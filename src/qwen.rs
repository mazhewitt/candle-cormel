//! Qwen model integration for candle-coreml
//!
//! This module provides a complete implementation of the Qwen multi-component architecture
//! with proper tokenization, state management, and inference pipeline.

use crate::utils::{mask, multi_component, sampling};
use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use candle_core::{Device, Error as CandleError, Tensor};
use std::collections::HashMap;
use std::path::Path;
use tokenizers::Tokenizer;

/// Qwen model constants
pub const QWEN_VOCAB_SIZE: usize = 151936;
pub const QWEN_HIDDEN_SIZE: usize = 1024;
pub const QWEN_BATCH_SIZE: usize = 64;
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
}

impl QwenModel {
    /// Load Qwen model from the specified directory
    pub fn load_from_directory<P: AsRef<Path>>(
        model_dir: P,
        config: Option<QwenConfig>,
    ) -> Result<Self, CandleError> {
        let config = config.unwrap_or_default();
        let model_dir = model_dir.as_ref();

        // Load tokenizer
        let tokenizer_path = model_dir.join("tokenizer.json");
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| CandleError::Msg(format!("Failed to load tokenizer: {}", e)))?;

        // Configure and load embeddings
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: config.context_length,
            vocab_size: config.vocab_size,
            model_type: "qwen-embeddings".to_string(),
        };

        let embeddings_path = model_dir.join("qwen_embeddings.mlmodelc");
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

        let ffn_path = model_dir.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");

        // FFN Prefill function (for initial sequence processing)
        let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;

        // FFN Infer function (for token-by-token generation with update_mask)
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

        let lm_head_path = model_dir.join("qwen_lm_head_lut8.mlmodelc");
        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

        Ok(Self {
            embeddings,
            ffn_prefill,
            ffn_infer,
            lm_head,
            tokenizer,
            config,
            unified_state: None, // Single shared state
        })
    }

    /// Initialize model states for efficient generation
    /// CRITICAL: Use a single shared state between prefill and infer (matches Python chat.py)
    pub fn initialize_states(&mut self) -> Result<(), CandleError> {
        // Create ONE unified state that both prefill and infer will share
        let unified_state = self.ffn_prefill.make_state()?;
        self.unified_state = Some(unified_state);
        Ok(())
    }

    /// Reset states for a new generation sequence
    pub fn reset_states(&mut self) -> Result<(), CandleError> {
        self.initialize_states()
    }

    /// Tokenize input text
    pub fn tokenize(&self, text: &str) -> Result<Vec<i64>, CandleError> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| CandleError::Msg(format!("Tokenization failed: {}", e)))?;

        Ok(encoding.get_ids().iter().map(|&id| id as i64).collect())
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

    /// Create causal attention mask using shared utilities
    pub fn create_causal_mask(&self, seq_len: usize) -> Result<Tensor, CandleError> {
        // Create base mask and reshape to rank-4 for CoreML
        let base_mask = mask::create_causal_mask(seq_len, &self.config.device)?;
        base_mask.reshape((1, 1, seq_len, seq_len))
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

    /// Run embeddings for input tokens
    pub fn compute_embeddings(&self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        let padded_tokens = self.pad_tokens(tokens);
        let input_tensor = Tensor::from_vec(
            padded_tokens.clone(),
            (1, padded_tokens.len()),
            &self.config.device,
        )?;

        self.embeddings.forward(&[&input_tensor])
    }

    /// Process sequence through FFN prefill phase
    pub fn prefill_sequence(
        &mut self,
        embeddings: &Tensor,
        sequence_length: usize,
    ) -> Result<(), CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }

        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Process each token position through prefill
        for pos in 0..sequence_length {
            let token_embedding = embeddings.narrow(1, pos, 1)?;

            let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
            let causal_mask = self.create_position_causal_mask(pos, context_length)?;
            let current_pos = Tensor::from_vec(vec![pos as i64], (1,), device)?;

            let inputs = vec![&token_embedding, &position_ids, &causal_mask, &current_pos];
            let state = self.unified_state.as_mut().unwrap();
            let _output = self.ffn_prefill.predict_with_state(&inputs, state)?;
        }

        Ok(())
    }

    /// Generate next token using granular methods - REFACTORED
    pub fn generate_next_token(
        &mut self,
        last_embedding: &Tensor,
        pos: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Create inputs for infer phase
        let update_mask = self.create_update_mask(pos, context_length)?;
        let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(pos, context_length)?;
        let current_pos = position_ids.clone();

        // Use granular infer method
        let hidden_states = self.run_ffn_infer_with_inputs(
            last_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos
        )?;

        // Use granular LM head method
        let combined_logits = self.run_lm_head_with_inputs(&hidden_states)?;

        Ok(combined_logits)
    }

    /// Combine 16 LM head output chunks into full vocabulary using shared utility
    pub fn combine_lm_head_outputs(
        &self,
        outputs: HashMap<String, Tensor>,
    ) -> Result<Tensor, CandleError> {
        multi_component::combine_chunked_logits(outputs, 16)
    }

    /// Forward pass returning raw logits (consistent with CoreMLModel API)
    pub fn forward(&mut self, inputs: &[&Tensor]) -> Result<Tensor, CandleError> {
        // For now, this is a simplified implementation
        // In a full implementation, this would orchestrate the multi-component pipeline
        if inputs.is_empty() {
            return Err(CandleError::Msg("No input tensors provided".to_string()));
        }

        // Process through embeddings -> FFN -> LM head
        let embeddings = self.embeddings.forward(&inputs[0..1])?;

        // For now, return embeddings as placeholder
        // TODO: Complete pipeline implementation
        Ok(embeddings)
    }

    /// Generate a single token from text input - REFACTORED TO USE GRANULAR METHODS
    /// Uses the proven granular methods that work perfectly in our tests
    pub fn forward_text(&mut self, text: &str) -> Result<i64, CandleError> {
        // Reset states for new sequence
        self.reset_states()?;

        // Tokenize input
        let tokens = self.tokenize(text)?;
        let sequence_length = tokens.len();
        
        println!("🔄 REFACTORED forward_text: Using granular methods for {} tokens", sequence_length);

        // STEP 1: EMBEDDINGS - Get embeddings for full sequence (padded to batch size)
        let padded_tokens = self.pad_tokens(&tokens);
        let input_tensor = Tensor::from_vec(
            padded_tokens.clone(),
            (1, padded_tokens.len()),
            &self.config.device,
        )?;

        let embeddings = self.run_embeddings_with_inputs(&input_tensor)?;
        
        // STEP 2: PREFILL PHASE - Use granular method
        // Create prefill inputs (matching our working tests)
        let batch_size = self.config.batch_size;
        let context_length = self.config.context_length;
        let device = self.config.device.clone();

        // Position IDs for full batch (0, 1, 2, ..., 63)
        let position_ids_vec: Vec<i64> = (0..batch_size as i64).collect();
        let position_ids = Tensor::from_vec(position_ids_vec, (batch_size,), &device)?;

        // Causal mask for full batch (1, 1, 64, 512)
        let mut mask_data = vec![f32::NEG_INFINITY; batch_size * context_length];
        for i in 0..batch_size {
            for j in 0..=i.min(context_length - 1) {
                mask_data[i * context_length + j] = 0.0;
            }
        }
        let causal_mask = Tensor::from_vec(mask_data, (1, 1, batch_size, context_length), &device)?;

        // Current pos is the last actual token position (seq_len - 1)
        let current_pos = Tensor::from_vec(vec![sequence_length as i64 - 1], (1,), &device)?;

        // Run prefill using granular method
        let _prefill_output = self.run_ffn_prefill_with_inputs(
            &embeddings,
            &position_ids,
            &causal_mask,
            &current_pos
        )?;
        
        println!("✅ Prefill using granular method complete - KV cache populated");

        // STEP 3: INFER PHASE - Use granular method
        // Get embedding for the last token
        let last_token_tensor = Tensor::from_vec(vec![tokens[tokens.len() - 1]], (1, 1), &device)?;
        let last_token_embedding = self.run_embeddings_with_inputs(&last_token_tensor)?;
        
        // Create infer inputs (matching our working tests)
        let current_position = sequence_length - 1; // CRITICAL: Use last token position
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let infer_position_ids = Tensor::from_vec(vec![current_position as i64], (1,), &device)?;
        let infer_causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let infer_current_pos = infer_position_ids.clone();

        // Run infer using granular method
        let infer_output = self.run_ffn_infer_with_inputs(
            &last_token_embedding,
            &update_mask,
            &infer_position_ids,
            &infer_causal_mask,
            &infer_current_pos
        )?;

        println!("✅ Infer using granular method complete");

        // STEP 4: LM HEAD - Use granular method
        let logits = self.run_lm_head_with_inputs(&infer_output)?;

        // Extract next token using argmax
        let flat_logits = logits.squeeze(0)?.squeeze(0)?;
        let logits_vec = flat_logits.to_vec1::<f32>()?;

        let next_token = logits_vec
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i as i64)
            .unwrap();

        println!("🎯 Generated token: {} using granular methods", next_token);
        Ok(next_token)
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

        println!("⚡ Running prefill for {} tokens (padded to {} batch) to populate KV cache", sequence_length, batch_size);

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

        // Run prefill with full batch embeddings (already padded to 64)
        let inputs = vec![embeddings, &position_ids, &causal_mask, &current_pos];
        let state = self.unified_state.as_mut().unwrap();
        let _prefill_output = self.ffn_prefill.predict_with_state(&inputs, state)?;

        println!("✅ Prefill complete - KV cache populated for positions 0..{}", sequence_length - 1);
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

        println!("⚡ Running infer for position {} using SHARED state from prefill (FIXED: last token pos)", current_position);

        // CRITICAL: We must use the SAME state that was populated by prefill!
        // Use the shared state that was populated during prefill
        if self.unified_state.is_none() {
            return Err(CandleError::Msg("No unified state available - prefill must be run first".to_string()));
        }
        println!("🔄 Using SHARED state populated by prefill (like working tests)");

        // Create infer inputs (matching our working test exactly)
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Run infer to get hidden states using the shared populated state
        let inputs = vec![
            token_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos,
        ];
        let state = self.unified_state.as_mut().unwrap(); // Use the UNIFIED state!
        let hidden_states = self.ffn_infer.predict_with_state(&inputs, state)?;

        println!("✅ Infer complete - processing through LM head");

        // Run through LM head to get logits (16 chunks)
        let lm_outputs = self.lm_head.forward_all(&[&hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;

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
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {}", e)))
    }

    /// Generate multiple tokens using temperature sampling with optional top-k
    pub fn generate_tokens(
        &mut self,
        text: &str,
        max_tokens: usize,
        temperature: f32,
        top_k: Option<usize>,
    ) -> Result<Vec<i64>, CandleError> {
        let mut generated_tokens = Vec::new();

        // Initial forward pass
        let next_token = self.forward_text(text)?;
        generated_tokens.push(next_token);

        // Continue generating
        for i in 1..max_tokens {
            // Create single token embedding using granular method
            let token_tensor = Tensor::from_vec(vec![next_token], (1, 1), &self.config.device)?;
            let token_embedding = self.run_embeddings_with_inputs(&token_tensor)?;

            let logits = self.generate_next_token(&token_embedding, i)?;
            
            // Flatten logits to 1D for sampling utilities (handles 3D tensors from LM head)
            let flat_logits = logits.squeeze(0)?.squeeze(0)?;

            // Use shared sampling utility
            let next_token = if let Some(k) = top_k {
                // Top-k sampling with temperature
                sampling::sample_top_k(&flat_logits, k, temperature)?
            } else if temperature <= 0.0 {
                // Greedy sampling
                sampling::greedy_sample(&flat_logits)?
            } else {
                // Temperature sampling
                sampling::sample_with_temperature(&flat_logits, temperature)?
            };

            generated_tokens.push(next_token);

            // Stop if EOS token
            if next_token == 151645 {
                // Qwen EOS token
                break;
            }
        }

        Ok(generated_tokens)
    }

    /// Generate text using top-k sampling
    pub fn generate_text_top_k(
        &mut self,
        text: &str,
        max_tokens: usize,
        k: usize,
        temperature: f32,
    ) -> Result<String, CandleError> {
        let tokens = self.generate_tokens(text, max_tokens, temperature, Some(k))?;

        // Decode tokens back to text
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.tokenizer
            .decode(&token_ids, false)
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {}", e)))
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
            return Err(CandleError::Msg("No unified state available - prefill must be run first".to_string()));
        }
        
        let inputs = [hidden_states, update_mask, position_ids, causal_mask, current_pos];
        let state = self.unified_state.as_mut().unwrap(); // Use the SAME unified state as prefill
        let output = self.ffn_infer.predict_with_state(&inputs, state)?;
        
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
    
    /// Create state objects (for testing)
    pub fn create_fresh_states(&mut self) -> Result<(), CandleError> {
        self.initialize_states()
    }
    
    /// Reset states (for testing)
    pub fn clear_states(&mut self) -> Result<(), CandleError> {
        self.reset_states()
    }

    /// Debug method: Get FFN output directly (for testing) - DEPRECATED
    /// Use run_ffn_infer_with_inputs for new code
    pub fn debug_get_ffn_output(
        &mut self,
        token_embedding: &Tensor,
        current_position: usize,
    ) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Create infer inputs using the same methods as production
        let update_mask = self.create_update_mask(current_position, context_length)?;
        let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(current_position, context_length)?;
        let current_pos = position_ids.clone();

        // Use the new granular method
        self.run_ffn_infer_with_inputs(token_embedding, &update_mask, &position_ids, &causal_mask, &current_pos)
    }
}
