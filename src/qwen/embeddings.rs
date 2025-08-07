//! Embeddings computation and caching for Qwen models
//!
//! This module contains methods for computing, caching, and retrieving embeddings
//! with various optimization strategies for different model architectures.

use crate::qwen::model::QwenModel;
use candle_core::{Error as CandleError, Tensor};
use tracing::debug;

impl QwenModel {
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
        let input_tensor = self.create_embeddings_input_tensor(tokens)?;
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

        // Use single-token method for models with separate ffn_infer component
        let input_tensor = if self
            .config
            .model_config
            .components
            .contains_key("ffn_infer")
        {
            debug!("🔍 Using single-token embeddings input for separate ffn_infer component");
            self.create_single_token_embeddings_input(last_token)?
        } else {
            debug!("🔍 Using standard embeddings input for unified ffn component");
            self.create_embeddings_input_tensor(&[last_token])?
        };

        let result = self.embeddings.forward(&[&input_tensor])?;
        debug!(
            "✅ Single token embedding result shape: {:?}",
            result.dims()
        );
        Ok(result)
    }

    /// 🚀 OPTIMIZED: Get appropriate hidden states for inference based on model architecture
    pub fn get_infer_hidden_states(
        &mut self,
        tokens: &[i64],
        pos: usize,
    ) -> Result<Tensor, CandleError> {
        // Check if we have a separate FFN infer component
        if self
            .config
            .model_config
            .components
            .contains_key("ffn_infer")
        {
            // Typo-fixer style: Use single token embedding for separate infer component
            debug!("Using single token embedding for separate FFN infer component");
            self.get_last_token_embedding_optimized(tokens)
        } else {
            // Standard ANEMLL style: Use full sequence embeddings for unified FFN component
            debug!("Using full sequence embeddings for unified FFN component");
            self.get_full_sequence_embeddings_for_infer(tokens, pos)
        }
    }

    /// Get full sequence embeddings for inference (needed by standard ANEMLL model)
    pub fn get_full_sequence_embeddings_for_infer(
        &mut self,
        tokens: &[i64],
        _pos: usize,
    ) -> Result<Tensor, CandleError> {
        // Get the expected FFN input shape
        let expected_shape = if let Some(ffn_prefill_config) =
            self.config.model_config.components.get("ffn_prefill")
        {
            ffn_prefill_config
                .inputs
                .get("hidden_states")
                .map(|hidden_states_config| hidden_states_config.shape.clone())
        } else {
            None
        };

        if let Some(expected_shape) = expected_shape {
            let expected_seq_len = expected_shape[1]; // [1, 64, 1024] -> 64

            // Check if we have cached embeddings that match this sequence
            if let Some((cached_tokens, cached_embeddings)) = &self.last_sequence_embeddings {
                let current_tokens_len = tokens.len().min(expected_seq_len);

                // Check if the current tokens match the beginning of our cached tokens
                if cached_tokens.len() >= current_tokens_len
                    && cached_tokens[..current_tokens_len] == tokens[..current_tokens_len]
                {
                    // Extract the appropriate slice from cached embeddings
                    let cached_dims = cached_embeddings.dims();
                    if cached_dims.len() == 3 && cached_dims[1] >= current_tokens_len {
                        debug!("⚡ REUSING: Cached sequence embeddings for full context");

                        // Create a tensor with the expected shape, filled with cached data up to current position
                        // and padded with zeros for the rest
                        let mut result_data =
                            vec![0.0f32; expected_shape[0] * expected_shape[1] * expected_shape[2]];
                        let cached_data = cached_embeddings.to_vec3::<f32>()?;

                        // Copy the cached data up to current position
                        for i in 0..current_tokens_len {
                            for j in 0..expected_shape[2] {
                                result_data[i * expected_shape[2] + j] = cached_data[0][i][j];
                            }
                        }

                        return Tensor::from_vec(
                            result_data,
                            (expected_shape[0], expected_shape[1], expected_shape[2]),
                            &self.config.device,
                        );
                    }
                }
            }
        }

        // Fallback: recompute embeddings for current sequence
        debug!("💾 COMPUTING: Full sequence embeddings for inference");
        let input_tensor = self.create_embeddings_input_tensor(tokens)?;
        self.embeddings.forward(&[&input_tensor])
    }
}
