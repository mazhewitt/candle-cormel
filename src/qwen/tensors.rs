//! Tensor creation and manipulation utilities for Qwen models
//!
//! This module contains all tensor creation functions that are used by the QwenModel
//! for preparing inputs, masks, and managing tensor operations.

use crate::qwen::model::QwenModel;
use crate::utils::mask;
use candle_core::{Error as CandleError, Tensor};
use tracing::debug;

impl QwenModel {
    /// Create input tensor for embeddings with proper shape validation
    pub fn create_embeddings_input_tensor(&self, tokens: &[i64]) -> Result<Tensor, CandleError> {
        let padded_tokens = self.pad_tokens(tokens);

        // Get expected shape from ModelConfig
        let shape = if let Some(input_shape) = self.config.embeddings_input_shape() {
            let tensor_shape = (input_shape[0], input_shape[1]); // [batch_size, seq_len]
            debug!(
                "🔍 SHAPE DEBUG: create_embeddings_input_tensor() - using ModelConfig shape: {:?} -> tensor shape: {:?}",
                input_shape, tensor_shape
            );
            tensor_shape
        } else {
            // Fallback shape
            let tensor_shape = (1, padded_tokens.len());
            debug!(
                "⚠️ SHAPE DEBUG: create_embeddings_input_tensor() - using fallback shape: {:?} (padded_tokens.len={})",
                tensor_shape, padded_tokens.len()
            );
            tensor_shape
        };

        debug!(
            "🎯 SHAPE DEBUG: Creating embeddings input tensor with shape {:?} for {} padded tokens (original: {} tokens)",
            shape, padded_tokens.len(), tokens.len()
        );

        // Validate tensor shape matches padded tokens length
        if shape.0 * shape.1 != padded_tokens.len() {
            return Err(CandleError::Msg(format!(
                "Shape mismatch: tensor shape {:?} requires {} elements, but got {} padded tokens",
                shape,
                shape.0 * shape.1,
                padded_tokens.len()
            )));
        }

        Tensor::from_vec(padded_tokens, shape, &self.config.device)
    }
    /// Create single-token embeddings input tensor for infer mode
    /// This produces [1, 1] shape regardless of the model's batch configuration
    pub fn create_single_token_embeddings_input(&self, token: i64) -> Result<Tensor, CandleError> {
        debug!(
            "🔍 SINGLE TOKEN: Creating [1, 1] shape input tensor for token {}",
            token
        );
        Tensor::from_vec(vec![token], (1, 1), &self.config.device)
    }
    /// Create position tensor with dynamic shape validation
    pub fn create_position_tensor(&self, positions: Vec<i64>) -> Result<Tensor, CandleError> {
        // Get expected position_ids shape from ModelConfig for FFN prefill component
        let expected_shape = if let Some(ffn_prefill_config) =
            self.config.model_config.components.get("ffn_prefill")
        {
            ffn_prefill_config
                .inputs
                .get("position_ids")
                .map(|position_ids_config| position_ids_config.shape.clone())
        } else {
            None
        };

        let (final_positions, shape) = if let Some(expected_shape) = expected_shape {
            let expected_len = expected_shape[0]; // Should be [64] -> 64

            if positions.len() == 1 {
                // Single token inference: create full position sequence up to current position
                let current_pos = positions[0];
                let mut full_positions = (0..expected_len as i64).collect::<Vec<i64>>();

                // Set positions up to current_pos, pad rest with 0
                if current_pos >= 0 && (current_pos as usize) < expected_len {
                    // Fill positions 0..=current_pos, leave rest as 0
                    for item in full_positions
                        .iter_mut()
                        .take(expected_len)
                        .skip(current_pos as usize + 1)
                    {
                        *item = 0;
                    }
                }

                debug!(
                    "Single token inference: creating position tensor with shape [{}] for current position {} (expected shape: {:?})",
                    expected_len, current_pos, expected_shape
                );
                (full_positions, (expected_len,))
            } else {
                // Multi-token case: use as provided but validate length
                let mut final_positions = positions;
                if final_positions.len() > expected_len {
                    debug!(
                        "Truncating position tensor from {} to {} positions",
                        final_positions.len(),
                        expected_len
                    );
                    final_positions.truncate(expected_len);
                } else if final_positions.len() < expected_len {
                    debug!(
                        "Padding position tensor from {} to {} positions",
                        final_positions.len(),
                        expected_len
                    );
                    final_positions.resize(expected_len, 0);
                }
                (final_positions, (expected_len,))
            }
        } else {
            // Fallback to original behavior if no model config available
            debug!("No FFN prefill position_ids shape found in ModelConfig, using legacy behavior");
            let len = positions.len();
            (positions, (len,))
        };

        debug!(
            "Creating position tensor with shape {:?} for positions (len={}): {:?}",
            shape,
            final_positions.len(),
            if final_positions.len() <= 10 {
                format!("{final_positions:?}")
            } else {
                format!(
                    "[{}, {}, ..., {}]",
                    final_positions[0],
                    final_positions[1],
                    final_positions[final_positions.len() - 1]
                )
            }
        );

        Tensor::from_vec(final_positions, shape, &self.config.device)
    }
    /// Create causal mask tensor with proper dimensions
    pub fn create_causal_mask_tensor(
        &self,
        seq_len: usize,
        context_len: usize,
    ) -> Result<Tensor, CandleError> {
        // Create causal mask data
        let mut mask_data = vec![f32::NEG_INFINITY; seq_len * context_len];
        for i in 0..seq_len {
            for j in 0..=i.min(context_len - 1) {
                mask_data[i * context_len + j] = 0.0;
            }
        }

        let shape = (1, 1, seq_len, context_len);
        debug!("Creating causal mask tensor with shape {:?}", shape);
        Tensor::from_vec(mask_data, shape, &self.config.device)
    }
    /// Create update mask tensor for FFN infer
    pub fn create_update_mask_tensor(&self, position: usize) -> Result<Tensor, CandleError> {
        let context_length = self.config.context_length();
        let mut mask_data = vec![0.0f32; context_length];
        if position < context_length {
            mask_data[position] = 1.0;
        }

        let shape = (1, 1, context_length, 1);
        debug!(
            "Creating update mask tensor with shape {:?} for position {}",
            shape, position
        );
        Tensor::from_vec(mask_data, shape, &self.config.device)
    }
    /// Create position slice of causal mask for single token processing
    pub fn create_position_causal_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        // Use the configuration-based approach
        self.config
            .create_infer_causal_mask_tensor(pos, context_length)
    }
    /// Create update mask for FFN infer phase
    pub fn create_update_mask(
        &self,
        pos: usize,
        context_length: usize,
    ) -> Result<Tensor, CandleError> {
        mask::create_update_mask(pos, context_length, &self.config.device)
    }
}
