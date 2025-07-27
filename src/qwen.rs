//! Qwen model integration for candle-coreml
//!
//! This module provides a complete implementation of the Qwen multi-component architecture
//! with proper tokenization, state management, and inference pipeline.

use crate::{Config as CoreMLConfig, CoreMLModel, CoreMLState};
use crate::utils::{mask, sampling, multi_component};
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
    embeddings: CoreMLModel,
    ffn_prefill: CoreMLModel,
    ffn_infer: CoreMLModel,
    lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    config: QwenConfig,
    ffn_prefill_state: Option<CoreMLState>,
    ffn_infer_state: Option<CoreMLState>,
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
            ffn_prefill_state: None,
            ffn_infer_state: None,
        })
    }

    /// Initialize model states for efficient generation
    pub fn initialize_states(&mut self) -> Result<(), CandleError> {
        self.ffn_prefill_state = Some(self.ffn_prefill.make_state()?);
        self.ffn_infer_state = Some(self.ffn_infer.make_state()?);
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
    pub fn create_position_causal_mask(&self, pos: usize, context_length: usize) -> Result<Tensor, CandleError> {
        mask::create_rank4_position_mask(pos, context_length, &self.config.device)
    }

    /// Create update mask for FFN infer phase
    pub fn create_update_mask(&self, pos: usize, context_length: usize) -> Result<Tensor, CandleError> {
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
    pub fn prefill_sequence(&mut self, embeddings: &Tensor, sequence_length: usize) -> Result<(), CandleError> {
        if self.ffn_prefill_state.is_none() {
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
            let state = self.ffn_prefill_state.as_mut().unwrap();
            let _output = self.ffn_prefill.predict_with_state(&inputs, state)?;
        }

        Ok(())
    }

    /// Generate next token using FFN infer phase
    pub fn generate_next_token(&mut self, last_embedding: &Tensor, pos: usize) -> Result<Tensor, CandleError> {
        if self.ffn_infer_state.is_none() {
            self.initialize_states()?;
        }

        let context_length = self.config.context_length;
        let device = &self.config.device;

        // Create inputs for infer phase
        let update_mask = self.create_update_mask(pos, context_length)?;
        let position_ids = Tensor::from_vec(vec![pos as i64], (1,), device)?;
        let causal_mask = self.create_position_causal_mask(pos, context_length)?;
        let current_pos = Tensor::from_vec(vec![pos as i64], (1,), device)?;

        let inputs = vec![
            last_embedding,
            &update_mask,
            &position_ids,
            &causal_mask,
            &current_pos,
        ];
        let state = self.ffn_infer_state.as_mut().unwrap();
        let hidden_states = self.ffn_infer.predict_with_state(&inputs, state)?;

        // Run through LM head to get logits
        let lm_outputs = self.lm_head.forward_all(&[&hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;

        Ok(combined_logits)
    }

    /// Combine 16 LM head output chunks into full vocabulary using shared utility
    pub fn combine_lm_head_outputs(&self, outputs: HashMap<String, Tensor>) -> Result<Tensor, CandleError> {
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

    /// Generate a single token from text input (convenience method)
    pub fn forward_text(&mut self, text: &str) -> Result<i64, CandleError> {
        // Reset states for new sequence
        self.reset_states()?;

        // Tokenize input
        let tokens = self.tokenize(text)?;

        // Process each token through the FFN to build up context
        for (pos, &token_id) in tokens.iter().enumerate() {
            // Get embedding for single token
            let single_token_tensor =
                Tensor::from_vec(vec![token_id], (1, 1), &self.config.device)?;
            let token_embedding = self.embeddings.forward(&[&single_token_tensor])?;

            // Process through FFN infer (this builds up the KV cache)
            let _logits = self.generate_next_token(&token_embedding, pos)?;
        }

        // Generate next token after processing all input tokens
        let last_token_tensor =
            Tensor::from_vec(vec![tokens[tokens.len() - 1]], (1, 1), &self.config.device)?;
        let last_embedding = self.embeddings.forward(&[&last_token_tensor])?;
        let logits = self.generate_next_token(&last_embedding, tokens.len())?;

        // Extract next token using argmax
        let logits_vec = logits.to_vec3::<f32>()?;
        let next_token_logits = &logits_vec[0][0];

        let next_token = next_token_logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i as i64)
            .unwrap();

        Ok(next_token)
    }

    /// Generate text using temperature sampling
    pub fn generate_text(&mut self, text: &str, max_tokens: usize, temperature: f32) -> Result<String, CandleError> {
        let tokens = self.generate_tokens(text, max_tokens, temperature)?;
        
        // Decode tokens back to text
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.tokenizer
            .decode(&token_ids, false)
            .map_err(|e| CandleError::Msg(format!("Failed to decode tokens: {}", e)))
    }

    /// Generate multiple tokens using temperature sampling
    pub fn generate_tokens(&mut self, text: &str, max_tokens: usize, temperature: f32) -> Result<Vec<i64>, CandleError> {
        let mut generated_tokens = Vec::new();

        // Initial forward pass
        let next_token = self.forward_text(text)?;
        generated_tokens.push(next_token);

        // Continue generating
        for i in 1..max_tokens {
            // Create single token embedding
            let token_tensor = Tensor::from_vec(vec![next_token], (1, 1), &self.config.device)?;
            let token_embedding = self.embeddings.forward(&[&token_tensor])?;

            let logits = self.generate_next_token(&token_embedding, i)?;

            // Use shared sampling utility
            let next_token = if temperature <= 0.0 {
                sampling::greedy_sample(&logits)?
            } else {
                sampling::sample_with_temperature(&logits, temperature)?
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

    /// Get model configuration
    pub fn config(&self) -> &QwenConfig {
        &self.config
    }

    /// Get tokenizer reference
    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
}
