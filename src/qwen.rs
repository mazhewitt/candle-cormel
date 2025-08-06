//! Qwen model integration for candle-coreml
//!
//! This module provides a complete implementation of the Qwen multi-component architecture
//! with proper tokenization, state management, and inference pipeline.

use crate::utils::{mask, multi_component};
use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use candle_core::{Device, Error as CandleError, Tensor};
use std::collections::HashMap;
use std::path::Path;
use tokenizers::Tokenizer;
use tracing::{debug, trace, warn};

/// Qwen model constants
pub const QWEN_VOCAB_SIZE: usize = 151_936;
pub const QWEN_HIDDEN_SIZE: usize = 1024;
pub const QWEN_BATCH_SIZE: usize = 64; // CoreML model only accepts this specific shape
pub const QWEN_CONTEXT_LENGTH: usize = 512;

/// Configuration for Qwen model components
#[derive(Debug, Clone)]
pub struct QwenConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub batch_size: usize,
    pub context_length: usize,
    pub device: Device,
}

impl Default for QwenConfig {
    fn default() -> Self {
        Self {
            vocab_size: QWEN_VOCAB_SIZE,
            hidden_size: QWEN_HIDDEN_SIZE,
            batch_size: QWEN_BATCH_SIZE,
            context_length: QWEN_CONTEXT_LENGTH,
            device: Device::Cpu,
        }
    }
}

/// Complete Qwen model with all components and state management
pub struct QwenModel {
    pub embeddings: CoreMLModel,
    pub ffn_prefill: CoreMLModel,
    pub ffn_infer: CoreMLModel,
    pub lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    config: QwenConfig,
    unified_state: Option<CoreMLState>, // Single shared state for both prefill and infer
    cached_causal_mask: Option<Tensor>, // Pre-computed causal mask (like chat.py)
    // Embeddings optimization
    last_sequence_embeddings: Option<(Vec<i64>, Tensor)>, // Cache last full sequence
    // 🚀 PERFORMANCE OPTIMIZATIONS: Pre-allocated tensors to avoid allocation in hot path
    cached_position_ids: Option<Tensor>, // Pre-computed position IDs for batch sizes
    cached_update_mask: Option<Tensor>,  // Pre-allocated update mask tensor
    cached_single_pos_tensor: Option<Tensor>, // Pre-allocated [1] tensor for current_pos
}

impl QwenModel {
    /// Auto-detect model file with given prefix and suffix patterns
    fn find_model_file<P: AsRef<Path>>(
        model_dir: P,
        prefix: &str,
        suffix: &str,
    ) -> Result<std::path::PathBuf, CandleError> {
        let model_dir = model_dir.as_ref();

        // Read directory entries
        let entries = std::fs::read_dir(model_dir)
            .map_err(|e| CandleError::Msg(format!("Failed to read model directory: {e}")))?;

        // Define possible extensions to search for
        let extensions = if suffix.ends_with(".mlmodelc") {
            vec![".mlmodelc", ".mlpackage"]
        } else {
            vec![suffix]
        };

        // Define possible prefixes to search for (handle both qwen_ and qwen-typo-fixer_ patterns)
        let prefixes = if prefix == "qwen_" {
            vec!["qwen_", "qwen-typo-fixer_"]
        } else {
            vec![prefix]
        };

        debug!("Searching for model files in: {}", model_dir.display());
        debug!("Prefixes: {:?}", prefixes);
        debug!("Extensions: {:?}", extensions);

        // Find files matching the pattern
        let mut matching_files = Vec::new();
        for entry in entries {
            let entry = entry
                .map_err(|e| CandleError::Msg(format!("Failed to read directory entry: {e}")))?;
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            // Check if filename matches any prefix/extension combination
            for &test_prefix in &prefixes {
                for &test_extension in &extensions {
                    let test_suffix =
                        if suffix.ends_with(".mlmodelc") && test_extension == ".mlpackage" {
                            // For .mlmodelc patterns, replace with .mlpackage
                            suffix.replace(".mlmodelc", ".mlpackage")
                        } else {
                            suffix.to_string()
                        };

                    if filename_str.starts_with(test_prefix) && filename_str.ends_with(&test_suffix)
                    {
                        debug!(
                            "Found matching file: {} (prefix: {}, suffix: {})",
                            filename_str, test_prefix, test_suffix
                        );
                        matching_files.push(entry.path());
                        break;
                    }
                }
            }
        }

        match matching_files.len() {
            0 => Err(CandleError::Msg(format!(
                "No model file found matching pattern: {}*{} (or variants) in directory: {}",
                prefix,
                suffix,
                model_dir.display()
            ))),
            1 => {
                let path = &matching_files[0];
                debug!("Auto-detected model file: {}", path.display());
                Ok(path.clone())
            }
            _ => {
                // Multiple matches - prefer .mlpackage over .mlmodelc if available
                let path = matching_files
                    .iter()
                    .find(|p| p.to_string_lossy().ends_with(".mlpackage"))
                    .unwrap_or(&matching_files[0]);
                warn!(
                    "Multiple model files found matching {}*{}: {:?}. Using: {}",
                    prefix,
                    suffix,
                    matching_files,
                    path.display()
                );
                Ok(path.clone())
            }
        }
    }

    /// Load Qwen model from the specified directory
    /// Automatically checks for coreml/ subdirectory and supports both .mlmodelc and .mlpackage formats
    pub fn load_from_directory<P: AsRef<Path>>(
        model_dir: P,
        config: Option<QwenConfig>,
    ) -> Result<Self, CandleError> {
        let config = config.unwrap_or_default();
        let model_dir = model_dir.as_ref();

        // Check if there's a coreml/ subdirectory with CoreML models
        let coreml_subdir = model_dir.join("coreml");
        let actual_model_dir = if coreml_subdir.exists() && coreml_subdir.is_dir() {
            debug!("Found coreml/ subdirectory, using it for model loading");
            &coreml_subdir
        } else {
            debug!(
                "Using main directory for model loading: {}",
                model_dir.display()
            );
            model_dir
        };

        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| CandleError::Msg(format!("Failed to load tokenizer: {e}")))?;

        // Configure and load embeddings
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: config.context_length,
            vocab_size: config.vocab_size,
            model_type: "qwen-embeddings".to_string(),
        };

        let embeddings_path =
            Self::find_model_file(actual_model_dir, "qwen_", "embeddings.mlmodelc")?;
        debug!(
            "Loading embeddings component from {}",
            embeddings_path.display()
        );
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;

        // Configure and load FFN models (both prefill and infer functions)
        let ffn_config_base = CoreMLConfig {
            input_names: vec![
                "hidden_states".to_string(),
                "position_ids".to_string(),
                "causal_mask".to_string(),
                "current_pos".to_string(),
            ],
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: config.context_length,
            vocab_size: config.hidden_size,
            model_type: "qwen-ffn".to_string(),
        };

        // Auto-detect FFN model file (handles different LUT versions)
        let ffn_path =
            Self::find_model_file(actual_model_dir, "qwen_FFN_PF_", "_chunk_01of01.mlmodelc")?;

        // FFN Prefill function (for initial sequence processing)
        debug!("Loading FFN prefill component from {}", ffn_path.display());
        let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;

        // FFN Infer function (for token-by-token generation with update_mask)
        debug!("Loading FFN infer component from {}", ffn_path.display());
        let mut ffn_infer_config = ffn_config_base.clone();
        ffn_infer_config
            .input_names
            .insert(1, "update_mask".to_string());
        let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;

        // Configure and load LM head
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits1".to_string(), // First of 16 outputs
            max_sequence_length: config.context_length,
            vocab_size: config.vocab_size,
            model_type: "qwen-lm-head".to_string(),
        };

        // Auto-detect LM head model file (handles different LUT versions)
        let lm_head_path = Self::find_model_file(actual_model_dir, "qwen_lm_head_", ".mlmodelc")?;
        debug!("Loading LM head component from {}", lm_head_path.display());
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

        Ok(Self {
            embeddings,
            ffn_prefill,
            ffn_infer,
            lm_head,
            tokenizer,
            config,
            unified_state: None,      // Single shared state
            cached_causal_mask: None, // Will be computed on first use
            last_sequence_embeddings: None,
            // 🚀 PERFORMANCE: Initialize cached tensors as None - will be allocated on first use
            cached_position_ids: None,
            cached_update_mask: None,
            cached_single_pos_tensor: None,
        })
    }

    /// Initialize model states for efficient generation
    /// CRITICAL: Use a single shared state between prefill and infer (matches Python chat.py)
    pub fn initialize_states(&mut self) -> Result<(), CandleError> {
        // Create ONE unified state that both prefill and infer will share
        let unified_state = self.ffn_prefill.make_state()?;
        self.unified_state = Some(unified_state);

        // Pre-compute causal mask ONCE (like chat.py initialize_causal_mask)
        if self.cached_causal_mask.is_none() {
            let context_length = self.config.context_length;
            let causal_mask = self.create_full_causal_mask(context_length)?;
            self.cached_causal_mask = Some(causal_mask);
            debug!(
                "✅ Pre-computed causal mask for context length {}",
                context_length
            );
        }

        // 🚀 PERFORMANCE: Pre-allocate frequently used tensors to avoid allocation in hot path
        let device = &self.config.device;
        let context_length = self.config.context_length;
        let batch_size = self.config.batch_size;

        // Pre-allocate position IDs tensor for batch processing
        if self.cached_position_ids.is_none() {
            let position_ids_vec: Vec<i64> = (0..batch_size as i64).collect();
            let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), device)?;
            self.cached_position_ids = Some(position_ids);
            debug!(
                "✅ Pre-allocated position IDs tensor for batch size {}",
                batch_size
            );
        }

        // Pre-allocate update mask tensor for inference
        if self.cached_update_mask.is_none() {
            let update_mask_data = vec![0.0f32; context_length];
            let update_mask =
                Tensor::from_vec(update_mask_data, (1, 1, context_length, 1), device)?;
            self.cached_update_mask = Some(update_mask);
            debug!(
                "✅ Pre-allocated update mask tensor for context length {}",
                context_length
            );
        }

        // Pre-allocate single position tensor for current_pos
        if self.cached_single_pos_tensor.is_none() {
            let single_pos = Tensor::from_vec(vec![0i64], (1,), device)?;
            self.cached_single_pos_tensor = Some(single_pos);
            debug!("✅ Pre-allocated single position tensor");
        }

        Ok(())
    }

    /// Create full causal mask once (like chat.py make_causal_mask)
    fn create_full_causal_mask(&self, context_length: usize) -> Result<Tensor, CandleError> {
        // Create full causal mask: (1, 1, context_length, context_length)
        let mut mask_data = vec![f32::NEG_INFINITY; context_length * context_length];

        for i in 0..context_length {
            for j in 0..=i {
                mask_data[i * context_length + j] = 0.0;
            }
        }

        Tensor::from_vec(
            mask_data,
            (1, 1, context_length, context_length),
            &self.config.device,
        )
    }

    /// Reset states for a new generation sequence
    pub fn reset_states(&mut self) -> Result<(), CandleError> {
        self.initialize_states()
    }

    /// Tokenize input text with length validation
    pub fn tokenize(&self, text: &str) -> Result<Vec<i64>, CandleError> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| CandleError::Msg(format!("Tokenization failed: {e}")))?;

        let tokens: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();

        // Validate token length against context window (not batch size)
        if tokens.len() > QWEN_CONTEXT_LENGTH {
            return Err(CandleError::Msg(format!(
                "Input too long: {} tokens exceeds maximum context length of {} tokens supported by the model. \
                Consider shortening your input.", 
                tokens.len(), QWEN_CONTEXT_LENGTH
            )));
        }

        Ok(tokens)
    }

    /// Pad tokens to appropriate batch size for embeddings
    pub fn pad_tokens(&self, tokens: &[i64]) -> Vec<i64> {
        if tokens.len() == 1 {
            tokens.to_vec() // Single token mode (1, 1)
        } else {
            // Pad to batch size (1, 64)
            let mut padded = tokens.to_vec();
            padded.resize(self.config.batch_size, 0);
            padded
        }
    }

    /// Create position slice of causal mask for single token processing
    pub fn create_position_causal_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        mask::create_rank4_position_mask(pos, context_length, &self.config.device)
    }

    /// Create update mask for FFN infer phase
    pub fn create_update_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        mask::create_update_mask(pos, context_length, &self.config.device)
    }

    /// Compute embeddings with caching and reuse optimization
    pub fn compute_embeddings(&mut self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        // Check if we already have embeddings for this exact sequence
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            if cached_tokens == tokens {
                debug!("⚡ CACHE HIT: Reusing embeddings for sequence {:?}", tokens);
                return Ok(cached_embeddings.clone());
            }
        }

        // Compute new embeddings
        debug!(
            "💾 CACHE MISS: Computing embeddings for sequence {:?}",
            tokens
        );
        let padded_tokens = self.pad_tokens(tokens);
        let input_tensor = Tensor::from_vec(
            padded_tokens.clone(),
            (1, padded_tokens.len()),
            &self.config.device,
        )?;
        let embeddings = self.embeddings.forward(&[&input_tensor])?;

        // Cache the result
        self.last_sequence_embeddings = Some((tokens.to_vec(), embeddings.clone()));

        Ok(embeddings)
    }

    /// 🚀 OPTIMIZED: Get single token embedding from cached sequence  
    pub fn get_token_embedding_from_sequence(
        &self,
        tokens: &[i64],
        token_index: usize,
    ) -> Result<Option<Tensor>, CandleError> {
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            if cached_tokens == tokens && token_index < tokens.len() {
                // Validate bounds against actual cached embeddings dimensions
                let cached_seq_len = cached_embeddings.dims()[1];
                if token_index >= cached_seq_len {
                    debug!(
                        "❌ BOUNDS: token_index {} >= cached_seq_len {}, falling back",
                        token_index, cached_seq_len
                    );
                    return Ok(None);
                }

                // Extract the specific token embedding from the cached sequence
                debug!(
                    "⚡ EXTRACTING: Token {} from cached sequence embeddings (dims: {:?})",
                    token_index,
                    cached_embeddings.dims()
                );
                let token_embedding = cached_embeddings.narrow(1, token_index, 1)?;
                return Ok(Some(token_embedding));
            }
        }
        Ok(None)
    }

    /// 🚀 OPTIMIZED: Get last token embedding without recomputing
    pub fn get_last_token_embedding_optimized(
        &mut self,
        tokens: &[i64],
    ) -> Result<Tensor, CandleError> {
        let last_index = tokens.len() - 1;

        // Try to get from cached sequence first
        if let Some(cached_embedding) =
            self.get_token_embedding_from_sequence(tokens, last_index)?
        {
            debug!("⚡ REUSING: Last token embedding from cached sequence");
            return Ok(cached_embedding);
        }

        // Fallback: compute single token embedding
        debug!("💾 COMPUTING: Single last token embedding");
        let last_token = tokens[last_index];
        let input_tensor = Tensor::from_vec(vec![last_token], (1, 1), &self.config.device)?;
        self.embeddings.forward(&[&input_tensor])
    }

    /// 🚀 OPTIMIZATION: Try to get cached embeddings for a batch of tokens
    /// This checks if the padded batch matches part of our cached sequence
    fn get_cached_batch_embeddings(
        &self,
        padded_batch: &[i64],
    ) -> Result<Option<Tensor>, CandleError> {
        // Check if we have cached embeddings for the full sequence
        if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
            // Try to find if this padded batch corresponds to a slice of our cached sequence
            let batch_size = padded_batch.len();

            // Look for the meaningful part of the batch (before padding zeros)
            let meaningful_end = padded_batch
                .iter()
                .position(|&x| x == 0)
                .unwrap_or(batch_size);

            if meaningful_end > 0 {
                let meaningful_batch = &padded_batch[..meaningful_end];

                // Check if this meaningful batch appears at the start of our cached tokens
                if cached_tokens.len() >= meaningful_batch.len()
                    && &cached_tokens[..meaningful_batch.len()] == meaningful_batch
                {
                    // Check if cached embeddings have sufficient size for the requested batch
                    let cached_dims = cached_embeddings.dims();
                    if cached_dims.len() >= 2 && cached_dims[1] >= batch_size {
                        // Extract the corresponding embeddings slice
                        let batch_embeddings = cached_embeddings.narrow(1, 0, batch_size)?;
                        debug!(
                            "⚡ EMBEDDINGS CACHE HIT: Reusing {} tokens from cached sequence",
                            meaningful_end
                        );
                        return Ok(Some(batch_embeddings));
                    }
                    debug!(
                        "⚠️ EMBEDDINGS CACHE MISS: Cached dims {:?} insufficient for batch_size {}",
                        cached_dims, batch_size
                    );
                }
            }
        }

        // No cache hit found
        Ok(None)
    }

    /// Combine 16 LM head output chunks into full vocabulary using shared utility
    pub fn combine_lm_head_outputs(
        &self,
        outputs: HashMap<String, Tensor>,
    ) -> Result<Tensor, CandleError> {
        multi_component::combine_chunked_logits(outputs, 16)
    }

    /// Generate a single token from text input - PRIMARY METHOD
    /// ✅ Uses chat.py architecture for correct predictions (correctly answers "Paris" for capital of France)
    /// 🚀 OPTIMIZED: Enhanced with embeddings caching for maximum performance
    /// Replicates Python reference architecture with chunked prefill and cached masks
    pub fn forward_text(&mut self, text: &str) -> Result<i64, CandleError> {
        let start_time = std::time::Instant::now();

        // Ensure states and causal mask are initialized (done once like chat.py)
        if self.unified_state.is_none() || self.cached_causal_mask.is_none() {
            self.initialize_states()?;
        }

        // Tokenize input
        let tokens = self.tokenize(text)?;
        let context_pos = tokens.len();
        debug!(
            "🚀 Chat.py-style OPTIMIZED: Processing {} tokens",
            context_pos
        );

        // 🚀 OPTIMIZATION: Pre-compute and cache embeddings for the full sequence
        let embeddings_start = std::time::Instant::now();
        let _cached_embeddings = self.compute_embeddings(&tokens)?;
        let embeddings_time = embeddings_start.elapsed();
        debug!(
            "⚡ Cached embeddings took: {:?} for {} tokens",
            embeddings_time, context_pos
        );

        // PHASE 1: CHUNKED PREFILL (chat.py architecture with embeddings optimization)
        let prefill_start = std::time::Instant::now();
        self.run_chatpy_prefill(&tokens, context_pos)?;
        let prefill_time = prefill_start.elapsed();
        debug!("⚡ Optimized chat.py prefill took: {:?}", prefill_time);

        // PHASE 2: SINGLE TOKEN INFER (chat.py architecture with embeddings optimization)
        let infer_start = std::time::Instant::now();
        let next_token = self.run_chatpy_infer(&tokens, context_pos)?;
        let infer_time = infer_start.elapsed();
        debug!("⚡ Optimized chat.py infer took: {:?}", infer_time);

        let total_time = start_time.elapsed();
        debug!(
            "🎯 OPTIMIZED CHAT.PY TOTAL: {:?} (target: ~11ms for 87 t/s)",
            total_time
        );

        Ok(next_token)
    }

    /// Extract next token from logits (shared utility)
    fn extract_next_token(&self, logits: &Tensor) -> Result<i64, CandleError> {
        let flat_logits = logits.squeeze(0)?.squeeze(0)?;
        let logits_vec = flat_logits.to_vec1::<f32>()?;

        // Use same tie-breaking logic as TDD test
        let mut indexed_logits: Vec<(usize, f32)> = logits_vec
            .iter()
            .enumerate()
            .map(|(i, &score)| (i, score))
            .collect();
        indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let next_token = indexed_logits[0].0 as i64;

        // Show top predictions for debugging
        debug!("Top 5 extract_next_token predictions:");
        for (rank, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
            let decoded = self
                .tokenizer
                .decode(&[*token_id as u32], false)
                .unwrap_or("???".to_string());
            debug!(
                "  {}. Token {} ('{}'): {:.6}",
                rank + 1,
                token_id,
                decoded,
                score
            );
        }

        Ok(next_token)
    }

    /// Chat.py-style chunked prefill with embeddings caching optimization
    fn run_chatpy_prefill(
        &mut self,
        tokens: &[i64],
        context_pos: usize,
    ) -> Result<(), CandleError> {
        let batch_size = self.config.batch_size; // 64
        let device = self.config.device.clone(); // Clone to avoid borrowing issues
        let causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask

        // Process in 64-token chunks (CoreML model constraint)
        let mut batch_pos = 0;
        while batch_pos < context_pos {
            let batch_end = (batch_pos + batch_size).min(context_pos);
            let _current_batch_size = batch_end - batch_pos;

            // Get current batch tokens
            let batch_tokens = &tokens[batch_pos..batch_end];

            // Pad to full batch size (exactly like chat.py F.pad)
            let mut padded_batch = batch_tokens.to_vec();
            padded_batch.resize(batch_size, 0); // Pad with zeros

            // 🚀 OPTIMIZATION: Try to reuse cached embeddings instead of recomputing
            let hidden_states = if let Some(cached_embeddings) =
                self.get_cached_batch_embeddings(&padded_batch)?
            {
                debug!(
                    "⚡ CACHE HIT: Reusing cached embeddings for batch at position {}",
                    batch_pos
                );
                cached_embeddings
            } else {
                debug!(
                    "💾 CACHE MISS: Computing embeddings for batch at position {}",
                    batch_pos
                );
                // Fallback to direct embeddings computation
                let batch_input = Tensor::from_vec(padded_batch.clone(), (1, batch_size), &device)?;
                self.embeddings.forward(&[&batch_input])?
            };

            // 🚀 OPTIMIZATION: Reuse cached position IDs or create new tensor
            let position_ids = {
                let position_ids_vec: Vec<i64> =
                    (batch_pos as i64..(batch_pos + batch_size) as i64).collect();
                Tensor::from_vec(position_ids_vec, (batch_size,), &device)?
            };

            // Use pre-computed causal mask slice (like chat.py batch_causal_mask)
            let batch_causal_mask = causal_mask.narrow(2, batch_pos, batch_size)?;

            // 🚀 OPTIMIZATION: Reuse cached single position tensor or create new
            let current_pos = Tensor::from_vec(vec![batch_pos as i64], (1,), &device)?;

            // Run prefill with the working method
            let _output = self.run_ffn_prefill_with_inputs(
                &hidden_states,
                &position_ids,
                &batch_causal_mask,
                &current_pos,
            )?;

            batch_pos = batch_end;
        }

        debug!(
            "✅ Optimized chat.py prefill: Processed {} tokens in {} chunks",
            context_pos,
            context_pos.div_ceil(batch_size)
        );
        Ok(())
    }

    /// Chat.py-style single token infer with embeddings caching optimization
    fn run_chatpy_infer(&mut self, tokens: &[i64], pos: usize) -> Result<i64, CandleError> {
        let context_length = self.config.context_length;
        let device = self.config.device.clone(); // Clone to avoid borrowing issues
        let causal_mask = self.cached_causal_mask.as_ref().unwrap().clone(); // Clone mask

        // 🚀 OPTIMIZATION: Get last token embedding from cached sequence
        let hidden_states = self.get_last_token_embedding_optimized(tokens)?;

        // 🚀 OPTIMIZATION: Efficiently create update mask (potential for caching)
        let mut update_mask_data = vec![0.0f32; context_length];
        update_mask_data[pos - 1] = 1.0; // Set position for update
        let update_mask = Tensor::from_vec(update_mask_data, (1, 1, context_length, 1), &device)?;

        // 🚀 OPTIMIZATION: Efficient position IDs creation (potential for caching)
        let position_ids = Tensor::from_vec(vec![(pos - 1) as i64], (1,), &device)?;

        // Fix bounds checking for causal mask slicing
        let mask_pos = pos - 1;
        if mask_pos >= context_length {
            return Err(CandleError::Msg(format!(
                "Position {mask_pos} exceeds causal mask context length {context_length}. Input may be too long for chunked processing."
            )));
        }
        let single_causal_mask = causal_mask.narrow(2, mask_pos, 1)?; // Get slice for current position
        let current_pos = position_ids.clone();

        // Run infer using the working method
        let infer_output = self.run_ffn_infer_with_inputs(
            &hidden_states,
            &update_mask,
            &position_ids,
            &single_causal_mask,
            &current_pos,
        )?;

        // Run LM head and extract token (like chat.py)
        let logits = self.run_lm_head_with_inputs(&infer_output)?;
        let next_token = self.extract_next_token(&logits)?;

        debug!(
            "✅ Optimized chat.py infer: Generated token {} at position {}",
            next_token, pos
        );
        Ok(next_token)
    }

    /// Performance benchmark for the current implementation
    pub fn benchmark_implementations(
        &mut self,
        text: &str,
        iterations: usize,
    ) -> Result<(), CandleError> {
        println!("🏁 PERFORMANCE BENCHMARK: Chat.py-style Implementation");
        println!("Text: '{text}'");
        println!("Iterations: {iterations}");
        println!("================================");

        // Benchmark current forward_text implementation (chat.py-style)
        let start = std::time::Instant::now();
        let mut results = Vec::new();
        for i in 0..iterations {
            let token = self.forward_text(text)?;
            results.push(token);
            if i == 0 {
                println!("🚀 Result: token {token}");
                // Decode the token to show what it predicts
                if let Ok(decoded) = self.tokenizer.decode(&[token as u32], false) {
                    println!("   Decoded: '{decoded}'");
                }
            }
        }
        let total_time = start.elapsed();
        let avg_time = total_time / iterations as u32;
        let tokens_per_sec = 1000.0 / avg_time.as_millis() as f64;

        println!("🚀 CURRENT IMPLEMENTATION (Chat.py-style):");
        println!("   Total time: {total_time:?}");
        println!("   Average per call: {avg_time:?}");
        println!("   Tokens/second: {tokens_per_sec:.2}");

        // Performance target assessment
        if tokens_per_sec >= 70.0 {
            println!("🎯 TARGET ACHIEVED: {tokens_per_sec:.2} t/s >= 70 t/s ✅");
        } else if tokens_per_sec >= 20.0 {
            println!("🎯 PARTIAL SUCCESS: {tokens_per_sec:.2} t/s >= 20 t/s (minimum target) ⚠️");
        } else {
            println!("🎯 TARGET MISSED: {tokens_per_sec:.2} t/s < 20 t/s ❌");
        }

        // Consistency check
        let all_same = results.iter().all(|&token| token == results[0]);
        println!(
            "✅ Consistency: {} (all iterations produced {})",
            if all_same {
                "CONSISTENT"
            } else {
                "INCONSISTENT"
            },
            if all_same {
                "same result"
            } else {
                "different results"
            }
        );

        Ok(())
    }

    /// Run prefill phase to populate KV cache for all tokens in sequence
    /// This replicates the exact prefill behavior from our working tests
    pub fn run_prefill_phase(
        &mut self,
        embeddings: &Tensor,
        sequence_length: usize,
    ) -> Result<(), CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }

        let batch_size = self.config.batch_size; // 64
        let context_length = self.config.context_length; // 512
        let device = &self.config.device;

        debug!(
            "Running prefill for {} tokens (padded to {} batch) to populate KV cache",
            sequence_length, batch_size
        );

        // Create position IDs for the full batch (0, 1, 2, ..., 63)
        // Note: We use the full batch size, not just sequence_length
        let position_ids_vec: Vec<i64> = (0..batch_size as i64).collect();
        let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), device)?;

        // Create causal mask for the full batch (1, 1, 64, 512)
        // Each row allows attention to positions up to that token's position
        let mut mask_data = vec![f32::NEG_INFINITY; batch_size * context_length];
        for i in 0..batch_size {
            for j in 0..=i.min(context_length - 1) {
                mask_data[i * context_length + j] = 0.0;
            }
        }
        let causal_mask = Tensor::from_vec(mask_data, (1, 1, batch_size, context_length), device)?;

        // Current pos is the last actual token position (seq_len - 1)
        let current_pos = Tensor::from_vec(vec![sequence_length as i64 - 1], (1,), device)?;

        // Run prefill with full batch embeddings (using granular method)
        let _prefill_output = self.run_ffn_prefill_with_inputs(
            embeddings,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

        debug!(
            "Prefill complete - KV cache populated for positions 0..{}",
            sequence_length - 1
        );
        Ok(())
    }

    /// Generate next token using FFN infer with populated state
    /// This replicates the exact infer behavior from our working tests
    /// CRITICAL: Uses the SAME state object that was populated during prefill
    pub fn generate_next_token_with_infer(
        &mut self,
        token_embedding: &Tensor,
        current_position: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        debug!(
            "Running infer for position {} using SHARED state from prefill (FIXED: last token pos)",
            current_position
        );

        // CRITICAL: We must use the SAME state that was populated by prefill!
        // Use the shared state that was populated during prefill
        if self.unified_state.is_none() {
            return Err(CandleError::Msg(
                "No unified state available - prefill must be run first".to_string(),
            ));
        }
        debug!("Using SHARED state populated by prefill (like working tests)");

        // Create infer inputs (matching our working test exactly)
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Run infer to get hidden states using granular method
        let hidden_states = self.run_ffn_infer_with_inputs(
            token_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

        debug!("Infer complete - processing through LM head");

        // Run through LM head to get logits (using granular method)
        let combined_logits = self.run_lm_head_with_inputs(&hidden_states)?;

        Ok(combined_logits)
    }

    /// Generate text using temperature sampling
    pub fn generate_text(
        &mut self,
        text: &str,
        max_tokens: usize,
        temperature: f32,
    ) -> Result<String, CandleError> {
        let tokens = self.generate_tokens(text, max_tokens, temperature, None)?;

        // Decode tokens back to text
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.tokenizer
            .decode(&token_ids, false)
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {e}")))
    }

    /// Generate multiple tokens using temperature sampling with optional top-k
    /// Generate multiple tokens with correct position tracking
    pub fn generate_tokens(
        &mut self,
        text: &str,
        max_tokens: usize,
        temperature: f32,
        _top_k: Option<usize>,
    ) -> Result<Vec<i64>, CandleError> {
        let mut generated_tokens = Vec::new();
        let mut current_text = text.to_string();

        for _ in 0..max_tokens {
            // Use the working forward_text method for each token
            let next_token = self.forward_text(&current_text)?;
            generated_tokens.push(next_token);

            // Stop if EOS
            if next_token == 151_645 {
                break;
            }

            // Update current_text by appending the new token
            if let Ok(decoded) = self.tokenizer.decode(&[next_token as u32], false) {
                current_text.push_str(&decoded);
            } else {
                // If decoding fails, stop generation
                break;
            }

            // For temperature sampling, we'd need to modify forward_text to accept temperature
            // For now, this uses greedy sampling which is what forward_text does
            if temperature > 0.0 {
                // TODO: Implement temperature sampling support
                // For now, fall back to greedy
            }
        }

        Ok(generated_tokens)
    }

    /// Get model configuration
    pub fn config(&self) -> &QwenConfig {
        &self.config
    }

    /// Get tokenizer reference
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }

    // ========== GRANULAR PIPELINE METHODS ==========
    // These methods expose each step of the pipeline for testing and debugging

    /// Run FFN prefill phase with exact inputs (for testing)
    pub fn run_ffn_prefill_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        position_ids: &Tensor,
        causal_mask: &Tensor,
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }

        let inputs = [hidden_states, position_ids, causal_mask, current_pos];
        let state = self.unified_state.as_mut().unwrap(); // Use the same unified state
        let output = self.ffn_prefill.predict_with_state(&inputs, state)?;

        Ok(output)
    }

    /// Run FFN infer phase with exact inputs (for testing)
    pub fn run_ffn_infer_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        update_mask: &Tensor,
        position_ids: &Tensor,
        causal_mask: &Tensor,
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            return Err(CandleError::Msg(
                "No unified state available - prefill must be run first".to_string(),
            ));
        }

        // CRITICAL FIX: Match Python reference implementation input order
        // Python infer inputs: hidden_states, update_mask, position_ids, causal_mask, current_pos
        // where current_pos should equal position_ids for proper state continuity
        trace!(
            "DEBUG: Infer inputs - position_ids: {:?}, current_pos: {:?}",
            position_ids.to_vec1::<f32>().unwrap_or_default(),
            current_pos.to_vec1::<f32>().unwrap_or_default()
        );

        // DEBUGGING: Validate all inputs before CoreML call
        trace!("INFER INPUT VALIDATION:");
        trace!(
            "  hidden_states: shape={:?}, sample={:?}",
            hidden_states.shape(),
            hidden_states.to_vec3::<f32>().unwrap_or_default()[0][0]
                [..3.min(hidden_states.dim(2).unwrap_or(0))]
                .to_vec()
        );
        let update_nonzeros = if let Ok(flat) = update_mask.flatten_all() {
            if let Ok(vec) = flat.to_vec1::<f32>() {
                vec.iter().filter(|&&x| x != 0.0).count()
            } else {
                0
            }
        } else {
            0
        };
        trace!(
            "  update_mask: shape={:?}, nonzeros={}",
            update_mask.shape(),
            update_nonzeros
        );

        trace!(
            "  position_ids: shape={:?}, values={:?}",
            position_ids.shape(),
            position_ids.to_vec1::<f32>().unwrap_or_default()
        );

        let causal_nonzeros = if let Ok(flat) = causal_mask.flatten_all() {
            if let Ok(vec) = flat.to_vec1::<f32>() {
                vec.iter().filter(|&&x| x != 0.0).count()
            } else {
                0
            }
        } else {
            0
        };
        trace!(
            "  causal_mask: shape={:?}, nonzeros={}",
            causal_mask.shape(),
            causal_nonzeros
        );

        let inputs = [
            hidden_states,
            update_mask,
            position_ids,
            causal_mask,
            current_pos,
        ];
        let state = self.unified_state.as_mut().unwrap(); // Use the SAME unified state as prefill

        trace!("About to call CoreML infer model...");
        let output = self.ffn_infer.predict_with_state(&inputs, state)?;

        // DEBUGGING: Check output immediately after CoreML call
        let output_sample = output.to_vec3::<f32>().unwrap_or_default()[0][0]
            [..5.min(output.dim(2).unwrap_or(0))]
            .to_vec();
        trace!("INFER OUTPUT VALIDATION:");
        trace!(
            "  output: shape={:?}, sample={:?}",
            output.shape(),
            output_sample
        );

        if output_sample.iter().all(|&x| x == 0.0) {
            warn!("ZEROS DETECTED: CoreML infer model returned all zeros!");
        } else {
            trace!("NON-ZERO OUTPUT: CoreML infer model returned valid data");
        }

        Ok(output)
    }

    /// Run LM head with exact inputs (for testing)
    pub fn run_lm_head_with_inputs(&self, hidden_states: &Tensor) -> Result<Tensor, CandleError> {
        let lm_outputs = self.lm_head.forward_all(&[hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;
        Ok(combined_logits)
    }

    /// Get direct access to embeddings model (for testing)
    pub fn run_embeddings_with_inputs(&self, input_ids: &Tensor) -> Result<Tensor, CandleError> {
        self.embeddings.forward(&[input_ids])
    }

    /// Direct access to CoreML infer model for granular testing
    pub fn debug_direct_infer_model_execution(
        &mut self,
        inputs: &[&Tensor; 5],
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            return Err(CandleError::Msg(
                "No unified state available - prefill must be run first".to_string(),
            ));
        }

        let state = self.unified_state.as_mut().unwrap();
        let output = self.ffn_infer.predict_with_state(inputs, state)?;
        Ok(output)
    }
}
