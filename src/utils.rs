//! Shared utilities for transformer models and multi-component architectures

use candle_core::{Device, Error as CandleError, Tensor};
use std::collections::HashMap;

/// Utilities for creating attention masks used in transformer models
pub mod mask {
    use super::*;

    /// Create a causal attention mask for autoregressive generation
    ///
    /// This mask prevents tokens from attending to future positions in the sequence.
    ///
    /// # Arguments
    /// * `seq_len` - Sequence length for the mask
    /// * `device` - Device to create the tensor on
    ///
    /// # Returns
    /// Causal mask tensor with shape `(seq_len, seq_len)` where upper triangle is -inf
    pub fn create_causal_mask(seq_len: usize, device: &Device) -> Result<Tensor, CandleError> {
        let mut mask_data = vec![0.0f32; seq_len * seq_len];

        // Fill upper triangle with -inf for causal masking
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask_data[i * seq_len + j] = f32::NEG_INFINITY;
            }
        }

        Tensor::from_vec(mask_data, (seq_len, seq_len), device)
    }

    /// Create a causal mask for a specific position in the sequence
    ///
    /// This creates a mask row that allows attention to all previous positions
    /// up to and including the current position.
    ///
    /// # Arguments
    /// * `pos` - Current position in the sequence
    /// * `context_len` - Total context length
    /// * `device` - Device to create the tensor on
    ///
    /// # Returns  
    /// Position mask tensor with shape `(1, context_len)`
    pub fn create_position_mask(
        pos: usize,
        context_len: usize,
        device: &Device,
    ) -> Result<Tensor, CandleError> {
        let mut mask_data = vec![f32::NEG_INFINITY; context_len];

        // Allow attention to all positions up to and including current position
        for item in mask_data.iter_mut().take(pos.min(context_len - 1) + 1) {
            *item = 0.0;
        }

        Tensor::from_vec(mask_data, (1, context_len), device)
    }

    /// Create a rank-4 causal mask for CoreML models that expect specific shapes
    ///
    /// Some CoreML models require masks with rank-4 shapes like `(1, 1, 1, seq_len)`
    ///
    /// # Arguments
    /// * `pos` - Current position in the sequence  
    /// * `context_len` - Total context length
    /// * `device` - Device to create the tensor on
    ///
    /// # Returns
    /// Rank-4 position mask tensor with shape `(1, 1, 1, context_len)`
    pub fn create_rank4_position_mask(
        pos: usize,
        context_len: usize,
        device: &Device,
    ) -> Result<Tensor, CandleError> {
        let mut mask_data = vec![f32::NEG_INFINITY; context_len];

        // Allow attention to all positions up to and including current position
        for item in mask_data.iter_mut().take(pos.min(context_len - 1) + 1) {
            *item = 0.0;
        }

        Tensor::from_vec(mask_data, (1, 1, 1, context_len), device)
    }

    /// Create an update mask for stateful models indicating which position to update
    ///
    /// # Arguments
    /// * `pos` - Position to update
    /// * `context_len` - Total context length  
    /// * `device` - Device to create the tensor on
    ///
    /// # Returns
    /// Update mask with 1.0 at the target position, 0.0 elsewhere
    pub fn create_update_mask(
        pos: usize,
        context_len: usize,
        device: &Device,
    ) -> Result<Tensor, CandleError> {
        let mut mask_data = vec![0.0f32; context_len];
        if pos < context_len {
            mask_data[pos] = 1.0;
        }

        Tensor::from_vec(mask_data, (1, 1, context_len, 1), device)
    }
}

/// Utilities for sampling from model outputs
pub mod sampling {
    use super::*;
    use rand::Rng;

    /// Sample a token using temperature scaling
    ///
    /// Temperature controls randomness:
    /// - temperature = 0.0: Greedy sampling (most likely token)
    /// - temperature = 1.0: Standard sampling  
    /// - temperature > 1.0: More random
    /// - temperature < 1.0: More deterministic
    ///
    /// # Arguments
    /// * `logits` - Model output logits tensor
    /// * `temperature` - Temperature for scaling
    ///
    /// # Returns
    /// Sampled token ID
    pub fn sample_with_temperature(logits: &Tensor, temperature: f32) -> Result<i64, CandleError> {
        if temperature <= 0.0 {
            // Greedy sampling - return most likely token
            return greedy_sample(logits);
        }

        // Apply temperature scaling
        let temp_tensor = Tensor::new(&[temperature], logits.device())?;
        let scaled_logits = logits.broadcast_div(&temp_tensor)?;

        // Convert to probabilities via softmax
        let probs = candle_nn::ops::softmax_last_dim(&scaled_logits)?;
        let probs_vec = probs.to_vec1::<f32>()?;

        // Sample from the distribution
        let mut rng = rand::thread_rng();
        let random_val: f32 = rng.gen();

        let mut cumulative = 0.0;
        for (i, &prob) in probs_vec.iter().enumerate() {
            cumulative += prob;
            if random_val <= cumulative {
                return Ok(i as i64);
            }
        }

        // Fallback to last token if numerical issues
        Ok((probs_vec.len() - 1) as i64)
    }

    /// Greedy sampling - always return the most likely token
    pub fn greedy_sample(logits: &Tensor) -> Result<i64, CandleError> {
        let logits_vec = logits.to_vec1::<f32>()?;
        let max_idx = logits_vec
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        Ok(max_idx as i64)
    }

    /// Top-k sampling - sample from the k most likely tokens
    pub fn sample_top_k(logits: &Tensor, k: usize, temperature: f32) -> Result<i64, CandleError> {
        let logits_vec = logits.to_vec1::<f32>()?;

        // Get indices sorted by logit value (descending)
        let mut indexed_logits: Vec<(usize, f32)> = logits_vec
            .iter()
            .enumerate()
            .map(|(i, &logit)| (i, logit))
            .collect();
        indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Take top k
        let top_k = indexed_logits.into_iter().take(k).collect::<Vec<_>>();

        if top_k.is_empty() {
            return Ok(0);
        }

        if temperature <= 0.0 {
            // Return most likely from top-k
            return Ok(top_k[0].0 as i64);
        }

        // Create tensor with only top-k logits
        let mut filtered_logits = vec![f32::NEG_INFINITY; logits_vec.len()];
        for (idx, logit) in top_k {
            filtered_logits[idx] = logit;
        }

        let filtered_tensor = Tensor::from_vec(filtered_logits, logits.shape(), logits.device())?;
        sample_with_temperature(&filtered_tensor, temperature)
    }
}

/// Utilities for multi-component model orchestration
pub mod multi_component {
    use super::*;
    use crate::Config as CoreMLConfig;
    use std::path::Path;

    /// Trait for models that consist of multiple CoreML components
    pub trait MultiComponentModel {
        /// Load all model components from a directory
        fn load_components<P: AsRef<Path>>(path: P) -> Result<Self, CandleError>
        where
            Self: Sized;

        /// Run inference through the complete pipeline
        fn forward_pipeline(&self, inputs: &[&Tensor]) -> Result<Tensor, CandleError>;

        /// Get information about the loaded components
        fn component_info(&self) -> Vec<String>;
    }

    /// Builder for creating CoreML configurations for common component types
    pub struct ComponentConfigBuilder {
        base_config: CoreMLConfig,
    }

    impl ComponentConfigBuilder {
        pub fn new(vocab_size: usize, max_seq_len: usize) -> Self {
            Self {
                base_config: CoreMLConfig {
                    input_names: vec![],
                    output_name: String::new(),
                    max_sequence_length: max_seq_len,
                    vocab_size,
                    model_type: String::new(),
                },
            }
        }

        /// Create config for an embeddings component
        pub fn embeddings_config(mut self, model_type: &str) -> CoreMLConfig {
            self.base_config.input_names = vec!["input_ids".to_string()];
            self.base_config.output_name = "hidden_states".to_string();
            self.base_config.model_type = format!("{}-embeddings", model_type);
            self.base_config
        }

        /// Create config for an FFN/transformer component  
        pub fn ffn_config(mut self, model_type: &str, include_mask: bool) -> CoreMLConfig {
            self.base_config.input_names = vec!["hidden_states".to_string()];
            if include_mask {
                self.base_config.input_names.push("causal_mask".to_string());
            }
            self.base_config.output_name = "output_hidden_states".to_string();
            self.base_config.model_type = format!("{}-ffn", model_type);
            self.base_config
        }

        /// Create config for an LM head component
        pub fn lm_head_config(mut self, model_type: &str) -> CoreMLConfig {
            self.base_config.input_names = vec!["hidden_states".to_string()];
            self.base_config.output_name = "logits".to_string();
            self.base_config.model_type = format!("{}-lm-head", model_type);
            self.base_config
        }
    }

    /// Utility for combining chunked LM head outputs (e.g., from ANEMLL models)
    pub fn combine_chunked_logits(
        outputs: HashMap<String, Tensor>,
        num_chunks: usize,
    ) -> Result<Tensor, CandleError> {
        let mut chunks = Vec::new();

        for i in 1..=num_chunks {
            let key = format!("logits{}", i);
            if let Some(chunk) = outputs.get(&key) {
                chunks.push(chunk.clone());
            } else {
                return Err(CandleError::Msg(format!("Missing logits chunk: {}", key)));
            }
        }

        // Concatenate along vocabulary dimension (assumed to be last dimension)
        let chunk_refs: Vec<&Tensor> = chunks.iter().collect();
        Tensor::cat(&chunk_refs, chunks[0].dims().len() - 1)
    }
}
