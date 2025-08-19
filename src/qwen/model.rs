//! Core Qwen model struct and loading logic
//!
//! This module contains the QwenModel struct definition and all methods related to
//! model loading, component initialization, and state management.

use crate::qwen::config::QwenConfig;
use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use candle_core::{Error as CandleError, Tensor};
use std::path::Path;
use tokenizers::Tokenizer;
use tracing::{debug, trace, warn};

/// Complete Qwen model with all components and state management
pub struct QwenModel {
    pub embeddings: CoreMLModel,
    pub ffn_prefill: CoreMLModel,
    pub ffn_infer: CoreMLModel,
    pub lm_head: CoreMLModel,
    pub tokenizer: Tokenizer,
    pub config: QwenConfig,
    pub unified_state: Option<CoreMLState>, // Single shared state for both prefill and infer
    pub cached_causal_mask: Option<Tensor>, // Pre-computed causal mask (like chat.py)
    // Embeddings optimization
    pub last_sequence_embeddings: Option<(Vec<i64>, Tensor)>, // Cache last full sequence
    // 🚀 PERFORMANCE OPTIMIZATIONS: Pre-allocated tensors to avoid allocation in hot path
    pub cached_position_ids: Option<Tensor>, // Pre-computed position IDs for batch sizes
    pub cached_update_mask: Option<Tensor>,  // Pre-allocated update mask tensor
    pub cached_single_pos_tensor: Option<Tensor>, // Pre-allocated [1] tensor for current_pos
    pub last_single_token_prefill_len: Option<usize>, // How many context tokens have been prefetched into KV cache in single-token mode
    pub cached_prefill_output: Option<Tensor>, // Cache prefill output hidden states for infer input
}

impl QwenModel {
    /// Single-token prefill step used when prefill_is_single_token() is true.
    fn prefill_single_token_step(
        &mut self,
        embeddings: &Tensor,
        pos: usize,
        causal_mask_full: &Tensor,
    ) -> Result<(), CandleError> {
        let device = &self.config.device;
        let actual_seq = embeddings.dim(1)?;
        if pos >= actual_seq {
            // Stop early: we reached beyond real (unpadded) token sequence inside padded embeddings
            return Ok(());
        }
        // Narrow embeddings for current token
        let token_embed = embeddings.narrow(1, pos, 1).map_err(|e| {
            CandleError::Msg(format!(
                "Failed to narrow embeddings for prefill token {pos}: {e}"
            ))
        })?;
        // [1] position_ids tensor
        let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
        // Build a per-position single-row causal mask [1,1,1,context] that allows 0..=pos
        let context_length = self.config.context_length();
        let causal_mask = self
            .config
            .create_infer_causal_mask_tensor(pos, context_length)
            .unwrap_or_else(|_| causal_mask_full.clone());
        let current_pos = Tensor::from_vec(vec![pos as i64], (1,), device)?;
        let prefill_output = self.run_ffn_prefill_with_inputs(
            &token_embed,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

        // Cache the prefill output for use in get_infer_hidden_states
        self.cached_prefill_output = Some(prefill_output);
        Ok(())
    }

    /// Variant used for multi-chunk sequential prefill where embeddings is a window and `global_pos` is absolute token index.
    pub(crate) fn prefill_single_token_step_chunk(
        &mut self,
        embeddings_chunk: &Tensor,
        local_pos: usize,
        global_pos: usize,
        causal_mask_full: &Tensor,
    ) -> Result<(), CandleError> {
        let device = &self.config.device;
        let actual_seq = embeddings_chunk.dim(1)?;
        if local_pos >= actual_seq {
            return Ok(());
        }
        let token_embed = embeddings_chunk.narrow(1, local_pos, 1)?;
        let position_ids = Tensor::from_vec(vec![global_pos as i64], (1,), device)?;
        // Build a per-position single-row causal mask [1,1,1,context] that allows 0..=global_pos
        let context_length = self.config.context_length();
        let causal_mask = self
            .config
            .create_infer_causal_mask_tensor(global_pos, context_length)
            .unwrap_or_else(|_| causal_mask_full.clone());
        let current_pos = Tensor::from_vec(vec![global_pos as i64], (1,), device)?;
        let prefill_output = self.run_ffn_prefill_with_inputs(
            &token_embed,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

        // Cache the prefill output for use in get_infer_hidden_states
        self.cached_prefill_output = Some(prefill_output);
        Ok(())
    }

    /// Full-sequence prefill for CoreML models that expect fixed-length inputs (e.g., 128 tokens)
    /// This bypasses single-token processing and sends the complete sequence to CoreML
    pub(crate) fn prefill_full_sequence_chunk(
        &mut self,
        embeddings_chunk: &Tensor,
        max_global_pos: usize,
        causal_mask_full: &Tensor,
    ) -> Result<(), CandleError> {
        let device = &self.config.device;
        let seq_len = embeddings_chunk.dim(1)?;

        // Create position_ids for the full sequence [0, 1, 2, ..., seq_len-1]
        let position_ids_vec: Vec<i64> = (0..seq_len as i64).collect();
        let position_ids = self.create_position_tensor(position_ids_vec)?;

        // Use the full causal mask (should already be correctly sized for the sequence)
        let causal_mask = causal_mask_full.clone();

        // Set current_pos to 0 for prefill (matches Python behavior)
        // Python uses batch_pos=0 for prefill, not the last position
        let current_pos = Tensor::from_vec(vec![0i64], (1,), device)?;

    trace!(
            "🚀 FULL-SEQUENCE PREFILL: Processing full sequence with shape {:?}, max_pos: {}",
            embeddings_chunk.dims(),
            max_global_pos
        );

        // Debug: Print prefill inputs for comparison with Python
        trace!("🔍 PREFILL INPUTS DEBUG:");
        trace!("  embeddings shape: {:?}", embeddings_chunk.dims());
        trace!("  position_ids shape: {:?}", position_ids.dims());
        trace!("  causal_mask shape: {:?}", causal_mask.dims());
        trace!(
            "  current_pos: {:?}",
            current_pos.to_vec1::<i64>().unwrap_or_default()
        );
        if let Ok(pos_ids_vec) = position_ids.to_vec1::<i64>() {
            trace!(
                "  position_ids[0..16]: {:?}",
                &pos_ids_vec[..16.min(pos_ids_vec.len())]
            );
        }

        // Send the full sequence to CoreML prefill model
        let prefill_output = self.run_ffn_prefill_with_inputs(
            embeddings_chunk, // Full [1, 128, 1024] tensor
            &position_ids,    // [128] position IDs
            &causal_mask,     // Full causal mask
            &current_pos,     // [1] current position
        )?;

        // Cache the prefill output for use in get_infer_hidden_states
    self.cached_prefill_output = Some(prefill_output);
    trace!("✅ FULL-SEQUENCE PREFILL: Successfully processed full sequence");
        Ok(())
    }

    // Pattern/glob discovery removed. Explicit file paths are now required in ModelConfig.

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
        let embeddings_inputs =
            if let Some(emb_comp) = config.model_config.components.get("embeddings") {
                emb_comp
                    .input_order
                    .clone()
                    .unwrap_or_else(|| vec!["input_ids".to_string()])
            } else {
                vec!["input_ids".to_string()]
            };
        let embeddings_config = CoreMLConfig {
            input_names: embeddings_inputs,
            output_name: "hidden_states".to_string(),
            max_sequence_length: config.context_length(),
            vocab_size: config.vocab_size(),
            model_type: "qwen-embeddings".to_string(),
        };

        // Require explicit file path for embeddings
        let embeddings_component = config
            .model_config
            .components
            .get("embeddings")
            .ok_or_else(|| {
                CandleError::Msg("ModelConfig missing 'embeddings' component".to_string())
            })?;
        let embeddings_file = embeddings_component.file_path.as_ref().ok_or_else(|| {
            CandleError::Msg("ModelConfig.embeddings.file_path must be set".to_string())
        })?;
        let embeddings_path = actual_model_dir.join(embeddings_file);
        debug!(
            "Loading embeddings component from {}",
            embeddings_path.display()
        );
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;

        // Configure and load FFN models (both prefill and infer functions)
        let ffn_prefill_inputs =
            if let Some(ffn_prefill_comp) = config.model_config.components.get("ffn_prefill") {
                ffn_prefill_comp.input_order.clone().unwrap_or_else(|| {
                    vec![
                        "hidden_states".to_string(),
                        "position_ids".to_string(),
                        "causal_mask".to_string(),
                        "current_pos".to_string(),
                    ]
                })
            } else {
                vec![
                    "hidden_states".to_string(),
                    "position_ids".to_string(),
                    "causal_mask".to_string(),
                    "current_pos".to_string(),
                ]
            };
        let ffn_config_base = CoreMLConfig {
            input_names: ffn_prefill_inputs,
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: config.context_length(),
            vocab_size: config.hidden_size(),
            model_type: "qwen-ffn".to_string(),
        };

        // Require explicit file path for FFN prefill
        let ffn_component = config
            .model_config
            .components
            .get("ffn_prefill")
            .ok_or_else(|| {
                CandleError::Msg("ModelConfig missing 'ffn_prefill' component".to_string())
            })?;
        let ffn_file = ffn_component.file_path.as_ref().ok_or_else(|| {
            CandleError::Msg("ModelConfig.ffn_prefill.file_path must be set".to_string())
        })?;
        let ffn_path = actual_model_dir.join(ffn_file);

        // FFN Prefill function (for initial sequence processing)
        debug!("Loading FFN prefill component from {}", ffn_path.display());
        let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;

        // FFN Infer function (for token-by-token generation)
        // Check if there's a separate ffn_infer component, otherwise use the same file as prefill
        let (ffn_infer_path, ffn_infer_config) = if let Some(ffn_infer_component) =
            config.model_config.components.get("ffn_infer")
        {
            let infer_path = if let Some(file_path) = &ffn_infer_component.file_path {
                actual_model_dir.join(file_path)
            } else {
                return Err(CandleError::Msg("ModelConfig.ffn_infer.file_path must be set when 'ffn_infer' component is present".to_string()));
            };

            // Use infer-specific configuration
            let ffn_infer_inputs =
                if let Some(ffn_infer_comp) = config.model_config.components.get("ffn_infer") {
                    ffn_infer_comp.input_order.clone().unwrap_or_else(|| {
                        vec![
                            "hidden_states".to_string(),
                            "position_ids".to_string(),
                            "causal_mask".to_string(),
                            "current_pos".to_string(),
                        ]
                    })
                } else {
                    vec![
                        "hidden_states".to_string(),
                        "position_ids".to_string(),
                        "causal_mask".to_string(),
                        "current_pos".to_string(),
                    ]
                };
            let infer_config = CoreMLConfig {
                input_names: ffn_infer_inputs,
                output_name: "output_hidden_states".to_string(),
                max_sequence_length: 1, // Single token for inference
                vocab_size: config.hidden_size(),
                model_type: "qwen-ffn-infer".to_string(),
            };
            (infer_path, infer_config)
        } else {
            // Use same file as prefill with infer function
            (ffn_path.clone(), ffn_config_base.clone())
        };

        debug!(
            "Loading FFN infer component from {}",
            ffn_infer_path.display()
        );
        let ffn_infer =
            CoreMLModel::load_with_function(&ffn_infer_path, &ffn_infer_config, "infer")?;

        // Configure and load LM head
        let lm_output = config
            .model_config
            .lm_head_primary_output_name()
            .unwrap_or_else(|| "logits1".to_string());

        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: lm_output,
            max_sequence_length: config.context_length(),
            vocab_size: config.vocab_size(),
            model_type: "qwen-lm-head".to_string(),
        };

        // Require explicit file path for LM head
        let lm_head_component = config
            .model_config
            .components
            .get("lm_head")
            .ok_or_else(|| {
                CandleError::Msg("ModelConfig missing 'lm_head' component".to_string())
            })?;
        let lm_head_file = lm_head_component.file_path.as_ref().ok_or_else(|| {
            CandleError::Msg("ModelConfig.lm_head.file_path must be set".to_string())
        })?;
        let lm_head_path = actual_model_dir.join(lm_head_file);
        debug!("Loading LM head component from {}", lm_head_path.display());
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

        // Optional runtime config wiring validation
        if let Err(e) = config.model_config.validate_internal_wiring() {
            warn!(
                "ModelConfig internal wiring validation warning: {}. Proceeding with load.",
                e
            );
        }

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
            last_single_token_prefill_len: None,
            cached_prefill_output: None,
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
            let context_length = self.config.context_length();
            let causal_mask = self.create_full_causal_mask(context_length)?;
            self.cached_causal_mask = Some(causal_mask);
            trace!(
                "✅ Pre-computed causal mask for context length {}",
                context_length
            );
        }

        // 🚀 PERFORMANCE: Pre-allocate frequently used tensors to avoid allocation in hot path
        let device = &self.config.device;
        let context_length = self.config.context_length();
        let batch_size = self.config.batch_size();

        // Pre-allocate position IDs tensor for batch processing
        if self.cached_position_ids.is_none() {
            let position_ids_vec: Vec<i64> = (0..batch_size as i64).collect();
            let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), device)?;
            self.cached_position_ids = Some(position_ids);
            trace!(
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
            trace!(
                "✅ Pre-allocated update mask tensor for context length {}",
                context_length
            );
        }

        // Pre-allocate single position tensor for current_pos
        if self.cached_single_pos_tensor.is_none() {
            let single_pos = Tensor::from_vec(vec![0i64], (1,), device)?;
            self.cached_single_pos_tensor = Some(single_pos);
            trace!("✅ Pre-allocated single position tensor");
        }

        Ok(())
    }

    /// Create full causal mask once (like chat.py make_causal_mask)
    fn create_full_causal_mask(&self, context_length: usize) -> Result<Tensor, CandleError> {
        // Use the configuration-based approach
        self.config
            .create_ffn_causal_mask_tensor(self.config.batch_size(), context_length)
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
        if tokens.len() > self.config.context_length() {
            return Err(CandleError::Msg(format!(
                "Input too long: {} tokens exceeds maximum context length of {} tokens supported by the model. \
                Consider shortening your input.", 
                tokens.len(), self.config.context_length()
            )));
        }

        Ok(tokens)
    }

    /// Pad tokens to appropriate batch size for embeddings using dynamic configuration
    pub fn pad_tokens(&self, tokens: &[i64]) -> Vec<i64> {
    trace!(
            "🔍 PAD_TOKENS: Called with {} tokens: {:?}",
            tokens.len(),
            tokens
        );

        // Use the dynamic embeddings input shape from ModelConfig
        if let Some(input_shape) = self.config.embeddings_input_shape() {
            let expected_length = input_shape[1]; // Shape is [batch, seq_len]
            trace!(
                "✅ PAD_TOKENS: Found embeddings_input_shape: {input_shape:?}, expected_length: {expected_length}"
            );

            if tokens.len() <= expected_length {
                let mut padded = tokens.to_vec();
                padded.resize(expected_length, 0);
                trace!(
                    "✅ PAD_TOKENS: Padded {} tokens to {} (expected_length)",
                    tokens.len(),
                    padded.len()
                );
                padded
            } else {
                // Truncate if too long
                trace!(
                    "✂️ PAD_TOKENS: Truncating {} tokens to {} (expected_length)",
                    tokens.len(),
                    expected_length
                );
                let truncated = tokens[..expected_length].to_vec();
                trace!("✂️ PAD_TOKENS: Truncated to {} tokens", truncated.len());
                truncated
            }
        } else {
            // Fallback to old behavior if shape discovery failed
            trace!("❌ PAD_TOKENS: No embeddings input shape found in ModelConfig, using legacy padding");
            if tokens.len() == 1 {
                trace!(
                    "📦 PAD_TOKENS: Single token mode: keeping {} tokens",
                    tokens.len()
                );
                tokens.to_vec() // Single token mode (1, 1)
            } else {
                // Pad to batch size (1, 64)
                let mut padded = tokens.to_vec();
                let batch_size = self.config.batch_size();
                padded.resize(batch_size, 0);
                trace!("📦 PAD_TOKENS: Legacy multi-token mode: padded {} tokens to {} (batch_size: {})", tokens.len(), padded.len(), batch_size);
                padded
            }
        }
    }

    /// Get model configuration
    pub fn config(&self) -> &QwenConfig {
        &self.config
    }

    /// Get tokenizer reference
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
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

        let batch_size = self.config.batch_size();
        let context_length = self.config.context_length();
        let device = &self.config.device;

    trace!(
            "Running prefill for {} tokens (padded to {} batch) to populate KV cache",
            sequence_length, batch_size
        );

        // Branch: unified multi-token prefill vs single-token sequential prefill
        let single_token_mode = self.config.model_config.prefill_is_single_token();
        if single_token_mode {
            trace!(
                "⚙️ Prefill: single-token sequential mode ({} tokens)",
                sequence_length
            );
            // Prepare (and cache) full causal mask once
            if self.cached_causal_mask.is_none() {
                let full = self.create_full_causal_mask(context_length)?;
                self.cached_causal_mask = Some(full);
            }
            let causal_mask_full = self.cached_causal_mask.as_ref().unwrap().clone();
            // Iterate each token position and feed one token slice at a time
            for pos in 0..sequence_length {
                self.prefill_single_token_step(embeddings, pos, &causal_mask_full)?;
            }
            trace!(
                "✅ Prefill (sequential) complete - KV cache populated for 0..{}",
                sequence_length - 1
            );
        } else {
            // Original batched logic
            let expected_pos_len = self
                .config
                .model_config
                .get_tensor_shape("ffn_prefill", "position_ids", true)
                .map(|v| v[0])
                .unwrap_or(batch_size);
            let position_ids = if expected_pos_len == 1 {
                self.config
                    .create_position_ids_with_mode_detection(&[sequence_length as i64 - 1], true)?
            } else {
                let vec: Vec<i64> = (0..batch_size as i64).collect();
                Tensor::from_vec(vec, (batch_size,), device)?
            };
            let causal_mask = if let Some(mask) = &self.cached_causal_mask {
                mask.clone()
            } else {
                let m = self.create_full_causal_mask(context_length)?;
                self.cached_causal_mask = Some(m.clone());
                m
            };
            let current_pos = Tensor::from_vec(vec![sequence_length as i64 - 1], (1,), device)?;
            let prefill_output = self.run_ffn_prefill_with_inputs(
                embeddings,
                &position_ids,
                &causal_mask,
                &current_pos,
            )?;

            // Cache the prefill output for use in get_infer_hidden_states
            self.cached_prefill_output = Some(prefill_output);
            trace!(
                "✅ Prefill (batched) complete - KV cache populated for 0..{}",
                sequence_length - 1
            );
        }
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
        let context_length = self.config.context_length();

    trace!(
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
    trace!("Using SHARED state populated by prefill (like working tests)");

        // Create infer inputs (config-driven to support variant shapes)
        let position_ids = self
            .config
            .create_position_ids_with_mode_detection(&[current_position as i64], false)?;
        let causal_mask = self.config.create_causal_mask_with_mode_detection(
            current_position,
            context_length,
            false,
        )?;
        let current_pos = position_ids.clone();

        // Run infer with the shared state to get next-step hidden states
    trace!(
            "🔍 GENERATE_INFER: token_embedding shape={:?}",
            token_embedding.dims()
        );
    trace!(
            "🔍 GENERATE_INFER: position_ids shape={:?} vals={:?}",
            position_ids.dims(),
            position_ids.to_vec1::<i64>().unwrap_or_default()
        );
    trace!(
            "🔍 GENERATE_INFER: causal_mask shape={:?}",
            causal_mask.dims()
        );
    trace!(
            "🔍 GENERATE_INFER: current_pos shape={:?} vals={:?}",
            current_pos.dims(),
            current_pos.to_vec1::<i64>().unwrap_or_default()
        );
        let hidden_states = self.run_ffn_infer_with_inputs(
            token_embedding,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

    trace!("Infer complete - processing through LM head");

        // Run through LM head to get logits (using granular method)
        let combined_logits = self.run_lm_head_with_inputs(&hidden_states)?;

        Ok(combined_logits)
    }

    // ========== GRANULAR PIPELINE METHODS ==========
    // These methods expose each step of the pipeline for testing and debugging

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
