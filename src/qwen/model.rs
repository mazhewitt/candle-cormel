//! Core Qwen model struct and loading logic
//!
//! This module contains the QwenModel struct definition and all methods related to
//! model loading, component initialization, and state management.

use crate::qwen::config::QwenConfig;
use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use candle_core::{Error as CandleError, Tensor};
use std::path::Path;
use tokenizers::Tokenizer;
use tracing::{debug, warn};

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
}

impl QwenModel {
    /// Find model file using glob pattern from ModelConfig
    /// This is the new preferred method that works directly with ModelConfig patterns
    pub fn find_model_file_with_pattern<P: AsRef<Path>>(
        model_dir: P,
        file_pattern: &str,
    ) -> Result<std::path::PathBuf, CandleError> {
        let model_dir = model_dir.as_ref();

        debug!("Searching for model files in: {}", model_dir.display());
        debug!("File pattern: {}", file_pattern);

        // Read directory entries
        let entries = std::fs::read_dir(model_dir)
            .map_err(|e| CandleError::Msg(format!("Failed to read model directory: {e}")))?;

        // Find files matching the glob pattern
        let mut matching_files = Vec::new();
        for entry in entries {
            let entry = entry
                .map_err(|e| CandleError::Msg(format!("Failed to read directory entry: {e}")))?;
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            // Skip git directory
            if filename_str == ".git" {
                continue;
            }

            // Match against glob pattern
            if Self::matches_glob_pattern(&filename_str, file_pattern) {
                debug!("Found matching file: {} (pattern: {})", filename_str, file_pattern);
                matching_files.push(entry.path());
            }
        }

        match matching_files.len() {
            0 => Err(CandleError::Msg(format!(
                "No model file found matching pattern: {} in directory: {}",
                file_pattern,
                model_dir.display()
            ))),
            1 => {
                let path = &matching_files[0];
                debug!("Auto-detected model file: {}", path.display());
                Ok(path.clone())
            }
            _ => {
                warn!(
                    "Multiple files match pattern {}: {:?}. Using first one: {}",
                    file_pattern,
                    matching_files.iter().map(|p| p.display()).collect::<Vec<_>>(),
                    matching_files[0].display()
                );
                Ok(matching_files[0].clone())
            }
        }
    }

    /// Simple glob pattern matching - supports * wildcards
    fn matches_glob_pattern(filename: &str, pattern: &str) -> bool {
        // Handle exact matches
        if !pattern.contains('*') {
            return filename == pattern;
        }

        // Split pattern by * to get parts that must be present
        let parts: Vec<&str> = pattern.split('*').collect();
        
        if parts.is_empty() {
            return true;
        }

        // Check if filename starts with first part
        if !parts[0].is_empty() && !filename.starts_with(parts[0]) {
            return false;
        }

        // Check if filename ends with last part
        if let Some(last_part) = parts.last() {
            if !last_part.is_empty() && !filename.ends_with(last_part) {
                return false;
            }
        }

        // Check if all middle parts are present in order
        let mut search_from = if parts[0].is_empty() { 0 } else { parts[0].len() };
        for part in parts.iter().skip(1).take(parts.len() - 2) {
            if !part.is_empty() {
                if let Some(pos) = filename[search_from..].find(part) {
                    search_from += pos + part.len();
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Auto-detect model file using configurable naming patterns
    /// Supports arbitrary prefixes, suffixes, and file extensions
    pub fn find_model_file_with_config<P: AsRef<Path>>(
        model_dir: P,
        prefixes: &[String],
        suffix: &str,
        supported_extensions: &[String],
    ) -> Result<std::path::PathBuf, CandleError> {
        let model_dir = model_dir.as_ref();

        debug!("Searching for model files in: {}", model_dir.display());
        debug!("Prefixes: {:?}", prefixes);
        debug!("Suffix: {}", suffix);
        debug!("Supported extensions: {:?}", supported_extensions);

        // Read directory entries
        let entries = std::fs::read_dir(model_dir)
            .map_err(|e| CandleError::Msg(format!("Failed to read model directory: {e}")))?;

        // Find files matching the pattern
        let mut matching_files = Vec::new();
        for entry in entries {
            let entry = entry
                .map_err(|e| CandleError::Msg(format!("Failed to read directory entry: {e}")))?;
            let filename = entry.file_name();
            let filename_str = filename.to_string_lossy();

            // Skip git directory
            if filename_str == ".git" {
                continue;
            }

            // Check if filename matches any prefix/extension combination
            for prefix in prefixes {
                for extension in supported_extensions {
                    // Create test suffix by replacing the original extension with this one
                    let test_suffix = if suffix.contains('.') {
                        // Replace the extension part
                        let base_suffix = suffix.split('.').next().unwrap_or(suffix);
                        format!("{base_suffix}{extension}")
                    } else {
                        format!("{suffix}{extension}")
                    };

                    if filename_str.starts_with(prefix) && filename_str.ends_with(&test_suffix) {
                        debug!(
                            "Found matching file: {} (prefix: {}, suffix: {})",
                            filename_str, prefix, test_suffix
                        );
                        matching_files.push(entry.path());
                        break;
                    }
                }
            }
        }

        match matching_files.len() {
            0 => Err(CandleError::Msg(format!(
                "No model file found matching patterns: {:?}*{}*{:?} in directory: {}",
                prefixes,
                suffix,
                supported_extensions,
                model_dir.display()
            ))),
            1 => {
                let path = &matching_files[0];
                debug!("Auto-detected model file: {}", path.display());
                Ok(path.clone())
            }
            _ => {
                // Multiple matches - prefer by extension order (first in supported_extensions wins)
                let mut best_path = &matching_files[0];
                for extension in supported_extensions {
                    if let Some(path) = matching_files
                        .iter()
                        .find(|p| p.to_string_lossy().ends_with(extension))
                    {
                        best_path = path;
                        break;
                    }
                }
                warn!(
                    "Multiple model files found matching patterns: {:?}. Using: {}",
                    matching_files,
                    best_path.display()
                );
                Ok(best_path.clone())
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
        let embeddings_inputs = if let Some(emb_comp) = config.model_config.components.get("embeddings") {
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

        // Use ModelConfig component paths/patterns if available, otherwise fall back to naming config
        let embeddings_path = if let Some(embeddings_component) = config.model_config.components.get("embeddings") {
            if let Some(file_path) = &embeddings_component.file_path {
                // SAFE: Use explicit file path - no pattern matching risk
                actual_model_dir.join(file_path)
            } else if let Some(pattern) = &embeddings_component.file_pattern {
                // FALLBACK: Use pattern matching (legacy support)
                Self::find_model_file_with_pattern(actual_model_dir, pattern)?
            } else {
                Self::find_model_file_with_config(
                    actual_model_dir,
                    &config.naming.embeddings_prefixes,
                    &config.naming.embeddings_suffix,
                    &config.naming.supported_extensions,
                )?
            }
        } else {
            Self::find_model_file_with_config(
                actual_model_dir,
                &config.naming.embeddings_prefixes,
                &config.naming.embeddings_suffix,
                &config.naming.supported_extensions,
            )?
        };
        debug!(
            "Loading embeddings component from {}",
            embeddings_path.display()
        );
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;

        // Configure and load FFN models (both prefill and infer functions)
        let ffn_prefill_inputs = if let Some(ffn_prefill_comp) = config.model_config.components.get("ffn_prefill") {
            ffn_prefill_comp
                .input_order
                .clone()
                .unwrap_or_else(|| vec![
                    "hidden_states".to_string(),
                    "position_ids".to_string(),
                    "causal_mask".to_string(),
                    "current_pos".to_string(),
                ])
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

        // Auto-detect FFN model file using ModelConfig paths/patterns if available
        let ffn_path = if let Some(ffn_component) = config.model_config.components.get("ffn_prefill") {
            if let Some(file_path) = &ffn_component.file_path {
                // SAFE: Use explicit file path - no pattern matching risk
                actual_model_dir.join(file_path)
            } else if let Some(pattern) = &ffn_component.file_pattern {
                // FALLBACK: Use pattern matching (legacy support)
                Self::find_model_file_with_pattern(actual_model_dir, pattern)?
            } else {
                Self::find_model_file_with_config(
                    actual_model_dir,
                    &config.naming.ffn_prefixes,
                    &config.naming.ffn_suffix,
                    &config.naming.supported_extensions,
                )?
            }
        } else {
            Self::find_model_file_with_config(
                actual_model_dir,
                &config.naming.ffn_prefixes,
                &config.naming.ffn_suffix,
                &config.naming.supported_extensions,
            )?
        };

        // FFN Prefill function (for initial sequence processing)
        debug!("Loading FFN prefill component from {}", ffn_path.display());
        let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;

        // FFN Infer function (for token-by-token generation)
        // Check if there's a separate ffn_infer component, otherwise use the same file as prefill
        let (ffn_infer_path, ffn_infer_config) = if let Some(ffn_infer_component) = config.model_config.components.get("ffn_infer") {
            let infer_path = if let Some(file_path) = &ffn_infer_component.file_path {
                // SAFE: Use explicit file path for separate infer component
                actual_model_dir.join(file_path)
            } else if let Some(pattern) = &ffn_infer_component.file_pattern {
                // FALLBACK: Use pattern matching (legacy support)  
                Self::find_model_file_with_pattern(actual_model_dir, pattern)?
            } else {
                ffn_path.clone() // Fallback to prefill path
            };
            
            // Use infer-specific configuration
            let ffn_infer_inputs = if let Some(ffn_infer_comp) = config.model_config.components.get("ffn_infer") {
                ffn_infer_comp
                    .input_order
                    .clone()
                    .unwrap_or_else(|| vec![
                        "hidden_states".to_string(),
                        "position_ids".to_string(),
                        "causal_mask".to_string(),
                        "current_pos".to_string(),
                    ])
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
        
        debug!("Loading FFN infer component from {}", ffn_infer_path.display());
        let ffn_infer = CoreMLModel::load_with_function(&ffn_infer_path, &ffn_infer_config, "infer")?;

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

        // Auto-detect LM head model file using ModelConfig paths/patterns if available
        let lm_head_path = if let Some(lm_head_component) = config.model_config.components.get("lm_head") {
            if let Some(file_path) = &lm_head_component.file_path {
                // SAFE: Use explicit file path - no pattern matching risk
                actual_model_dir.join(file_path)
            } else if let Some(pattern) = &lm_head_component.file_pattern {
                // FALLBACK: Use pattern matching (legacy support)
                Self::find_model_file_with_pattern(actual_model_dir, pattern)?
            } else {
                Self::find_model_file_with_config(
                    actual_model_dir,
                    &config.naming.lm_head_prefixes,
                    &config.naming.lm_head_suffix,
                    &config.naming.supported_extensions,
                )?
            }
        } else {
            Self::find_model_file_with_config(
                actual_model_dir,
                &config.naming.lm_head_prefixes,
                &config.naming.lm_head_suffix,
                &config.naming.supported_extensions,
            )?
        };
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
            debug!(
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
        debug!(
            "🔍 PAD_TOKENS: Called with {} tokens: {:?}",
            tokens.len(),
            tokens
        );

        // Use the dynamic embeddings input shape from ModelConfig
        if let Some(input_shape) = self.config.embeddings_input_shape() {
            let expected_length = input_shape[1]; // Shape is [batch, seq_len]
            debug!(
                "✅ PAD_TOKENS: Found embeddings_input_shape: {input_shape:?}, expected_length: {expected_length}"
            );

            if tokens.len() <= expected_length {
                let mut padded = tokens.to_vec();
                padded.resize(expected_length, 0);
                debug!(
                    "✅ PAD_TOKENS: Padded {} tokens to {} (expected_length)",
                    tokens.len(),
                    padded.len()
                );
                padded
            } else {
                // Truncate if too long
                debug!(
                    "✂️ PAD_TOKENS: Truncating {} tokens to {} (expected_length)",
                    tokens.len(),
                    expected_length
                );
                let truncated = tokens[..expected_length].to_vec();
                debug!("✂️ PAD_TOKENS: Truncated to {} tokens", truncated.len());
                truncated
            }
        } else {
            // Fallback to old behavior if shape discovery failed
            debug!("❌ PAD_TOKENS: No embeddings input shape found in ModelConfig, using legacy padding");
            if tokens.len() == 1 {
                debug!(
                    "📦 PAD_TOKENS: Single token mode: keeping {} tokens",
                    tokens.len()
                );
                tokens.to_vec() // Single token mode (1, 1)
            } else {
                // Pad to batch size (1, 64)
                let mut padded = tokens.to_vec();
                let batch_size = self.config.batch_size();
                padded.resize(batch_size, 0);
                debug!("📦 PAD_TOKENS: Legacy multi-token mode: padded {} tokens to {} (batch_size: {})", tokens.len(), padded.len(), batch_size);
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

        let batch_size = self.config.batch_size(); // 64
        let context_length = self.config.context_length(); // 512
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
        let context_length = self.config.context_length();
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
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Run infer with the shared state to get next-step hidden states
        let hidden_states = self.run_ffn_infer_with_inputs(
            token_embedding,
            &position_ids,
            &causal_mask,
            &current_pos,
        )?;

        debug!("Infer complete - processing through LM head");

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
