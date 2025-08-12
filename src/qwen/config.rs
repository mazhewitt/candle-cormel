//! Qwen model configuration and validation logic
//!
//! This module handles the configuration of Qwen models, including shape management,
//! model ID resolution, and factory methods for different model variants.

use crate::qwen::naming::ModelNamingConfig;
use crate::ModelConfig;
use candle_core::{Device, Error as CandleError};
use tracing::debug;

// NOTE: These constants are deprecated and will be removed.
// Use ModelConfig system instead for dynamic shape configuration.
#[deprecated(note = "Use ModelConfig.shapes instead")]
pub const QWEN_VOCAB_SIZE: usize = 151_936;
#[deprecated(note = "Use ModelConfig.shapes instead")]
pub const QWEN_HIDDEN_SIZE: usize = 1024;
#[deprecated(note = "Use ModelConfig.shapes instead")]
pub const QWEN_BATCH_SIZE: usize = 64;
#[deprecated(note = "Use ModelConfig.shapes instead")]
pub const QWEN_CONTEXT_LENGTH: usize = 512;

/// Configuration for Qwen model components
#[derive(Debug, Clone)]
pub struct QwenConfig {
    pub device: Device,
    pub naming: ModelNamingConfig,
    pub model_config: ModelConfig,

    // Deprecated fields - use model_config.shapes instead
    #[deprecated(note = "Use model_config.shapes.vocab_size instead")]
    pub vocab_size: usize,
    #[deprecated(note = "Use model_config.shapes.hidden_size instead")]
    pub hidden_size: usize,
    #[deprecated(note = "Use model_config.shapes.batch_size instead")]
    pub batch_size: usize,
    #[deprecated(note = "Use model_config.shapes.context_length instead")]
    pub context_length: usize,
}

impl Default for QwenConfig {
    fn default() -> Self {
        let model_config = ModelConfig::default();
        Self {
            device: Device::Cpu,
            naming: ModelNamingConfig::default(),
            // Copy from model_config for backward compatibility
            #[allow(deprecated)]
            vocab_size: model_config.shapes.vocab_size,
            #[allow(deprecated)]
            hidden_size: model_config.shapes.hidden_size,
            #[allow(deprecated)]
            batch_size: model_config.shapes.batch_size,
            #[allow(deprecated)]
            context_length: model_config.shapes.context_length,
            model_config,
        }
    }
}

impl QwenConfig {
    /// Create a QwenConfig from a ModelConfig (recommended approach)
    pub fn from_model_config(model_config: ModelConfig) -> Self {
        let naming = ModelNamingConfig::from_model_config(&model_config);
        Self {
            device: Device::Cpu,
            naming,
            // Copy for backward compatibility
            #[allow(deprecated)]
            vocab_size: model_config.shapes.vocab_size,
            #[allow(deprecated)]
            hidden_size: model_config.shapes.hidden_size,
            #[allow(deprecated)]
            batch_size: model_config.shapes.batch_size,
            #[allow(deprecated)]
            context_length: model_config.shapes.context_length,
            model_config,
        }
    }

    /// Create embeddings input tensor with proper shape from configuration
    pub fn create_embeddings_input_tensor(
        &self,
        tokens: &[i64],
    ) -> Result<candle_core::Tensor, CandleError> {
        let expected_shape = self.model_config.embeddings_input_shape().unwrap();
        let expected_len = expected_shape[1]; // [batch, seq_len] -> seq_len

        // Pad or truncate tokens to match expected length
        let mut padded_tokens = tokens.to_vec();
        padded_tokens.resize(expected_len, 0); // Pad with 0s

        candle_core::Tensor::from_vec(
            padded_tokens,
            (expected_shape[0], expected_shape[1]),
            &self.device,
        )
    }

    /// Create position IDs tensor for FFN prefill with proper shape
    pub fn create_ffn_position_ids_tensor(
        &self,
        positions: &[i64],
    ) -> Result<candle_core::Tensor, CandleError> {
        let expected_shape = self
            .model_config
            .get_tensor_shape("ffn_prefill", "position_ids", true)
            .unwrap();
        let expected_len = expected_shape[0];

        // Create position sequence up to expected length
        let mut position_ids = Vec::with_capacity(expected_len);
        for i in 0..expected_len {
            if i < positions.len() {
                position_ids.push(positions[i]);
            } else {
                position_ids.push(0); // Pad with 0s
            }
        }

        candle_core::Tensor::from_vec(position_ids, (expected_len,), &self.device)
    }

    /// Create causal mask tensor for FFN with proper shape
    pub fn create_ffn_causal_mask_tensor(
        &self,
        _batch_size: usize,
        _context_length: usize,
    ) -> Result<candle_core::Tensor, CandleError> {
        let expected_shape = self
            .model_config
            .get_tensor_shape("ffn_prefill", "causal_mask", true)
            .ok_or_else(|| {
                CandleError::Msg(
                    "No ffn_prefill causal_mask shape found in configuration".to_string(),
                )
            })?;
        let mask_batch_size = expected_shape[2];
        let mask_context_length = expected_shape[3];

        // For single-token sequential prefill (hidden_states shape [1,1,H]) we still need a full causal mask logically,
        // but the network expects shape [1,1,1,context_length]. We'll just build the expected shape directly.
        let mut mask_data = vec![f32::NEG_INFINITY; mask_batch_size * mask_context_length];
        for i in 0..mask_batch_size { // usually 1 in sequential mode
            for j in 0..=i.min(mask_context_length - 1) { // i will be 0 -> only j=0 set to 0.0
                mask_data[i * mask_context_length + j] = 0.0;
            }
        }
        candle_core::Tensor::from_vec(
            mask_data,
            (
                expected_shape[0],
                expected_shape[1],
                expected_shape[2],
                expected_shape[3],
            ),
            &self.device,
        )
    }

    /// Create single token hidden states tensor for LM head
    pub fn create_single_token_hidden_states(
        &self,
        _tokens: &[i64],
    ) -> Result<candle_core::Tensor, CandleError> {
        let expected_shape = self
            .model_config
            .get_tensor_shape("lm_head", "hidden_states", true)
            .unwrap();

        // Create dummy tensor with correct shape (would be filled by actual embeddings)
        let tensor_data = vec![0.0f32; expected_shape.iter().product()];
        let shape = (expected_shape[0], expected_shape[1], expected_shape[2]);

        candle_core::Tensor::from_vec(tensor_data, shape, &self.device)
    }

    /// Create position IDs tensor for inference (single position)
    pub fn create_infer_position_ids_tensor(
        &self,
        position: usize,
    ) -> Result<candle_core::Tensor, CandleError> {
        // For inference, use single position
        candle_core::Tensor::from_vec(vec![position as i64], (1,), &self.device)
    }

    /// Create causal mask tensor for inference
    pub fn create_infer_causal_mask_tensor(
        &self,
        position: usize,
        _context_length: usize,
    ) -> Result<candle_core::Tensor, CandleError> {
        let expected_shape = self
            .model_config
            .get_tensor_shape("ffn_prefill", "causal_mask", true)
            .unwrap();
        let mask_batch_size = expected_shape[2];
        let mask_context_length = expected_shape[3];

        // Create causal mask for inference - only the row for current position is active
        let mut mask_data = vec![f32::NEG_INFINITY; mask_batch_size * mask_context_length];

        // Set the row corresponding to the current position
        let row_idx = position.min(mask_batch_size - 1);
        for j in 0..=position.min(mask_context_length - 1) {
            mask_data[row_idx * mask_context_length + j] = 0.0;
        }

        candle_core::Tensor::from_vec(
            mask_data,
            (
                expected_shape[0],
                expected_shape[1],
                expected_shape[2],
                expected_shape[3],
            ),
            &self.device,
        )
    }

    /// Create position IDs tensor with mode detection (prefill vs infer)
    pub fn create_position_ids_with_mode_detection(
        &self,
        positions: &[i64],
        is_prefill: bool,
    ) -> Result<candle_core::Tensor, CandleError> {
        if is_prefill {
            // Use prefill shape (batch-sized)
            self.create_ffn_position_ids_tensor(positions)
        } else {
            // For infer, check if we have separate ffn_infer component
            if self.model_config.components.contains_key("ffn_infer") {
                // Use infer-specific shape
                if let Some(infer_shape) =
                    self.model_config
                        .get_tensor_shape("ffn_infer", "position_ids", true)
                {
                    if infer_shape[0] == 1 {
                        // Single position for infer - FORCE infer tensor creation
                        debug!("🔧 SHAPE FIX: Using infer position_ids tensor (shape [1])");
                        return self.create_infer_position_ids_tensor(positions[0] as usize);
                    } else {
                        debug!(
                            "⚠️ SHAPE WARNING: ffn_infer position_ids shape is not [1]: {:?}",
                            infer_shape
                        );
                    }
                } else {
                    debug!("⚠️ SHAPE WARNING: No shape found for ffn_infer position_ids");
                }
            } else {
                debug!("⚠️ SHAPE WARNING: No ffn_infer component found");
            }

            // CRITICAL FIX: For typo-fixer models, ALWAYS use infer tensor shape in infer mode
            // This prevents falling back to the prefill shape (64) when we need infer shape (1)
            if positions.len() == 1 {
                debug!("🔧 SHAPE FIX: Forcing infer position_ids tensor for single position");
                return self.create_infer_position_ids_tensor(positions[0] as usize);
            }

            // Fallback to prefill shape or model configuration (this should rarely happen now)
            debug!("⚠️ SHAPE WARNING: Falling back to prefill position_ids tensor");
            self.create_ffn_position_ids_tensor(positions)
        }
    }

    /// Create causal mask tensor with mode detection (prefill vs infer)
    pub fn create_causal_mask_with_mode_detection(
        &self,
        position: usize,
        context_length: usize,
        is_prefill: bool,
    ) -> Result<candle_core::Tensor, CandleError> {
        if is_prefill {
            // Use prefill shape (batch-sized)
            self.create_ffn_causal_mask_tensor(0, context_length)
        } else {
            // For infer, check if we have separate ffn_infer component with different mask shape
            if self.model_config.components.contains_key("ffn_infer") {
                if let Some(infer_mask_shape) =
                    self.model_config
                        .get_tensor_shape("ffn_infer", "causal_mask", true)
                {
                    if infer_mask_shape[2] == 1 {
                        // Single-row mask for infer
                        let mask_context_length = infer_mask_shape[3];
                        let mut mask_data = vec![f32::NEG_INFINITY; mask_context_length];

                        // Allow attention to positions up to current position
                        for item in mask_data
                            .iter_mut()
                            .take(position.min(mask_context_length - 1) + 1)
                        {
                            *item = 0.0;
                        }

                        return candle_core::Tensor::from_vec(
                            mask_data,
                            (
                                infer_mask_shape[0],
                                infer_mask_shape[1],
                                infer_mask_shape[2],
                                infer_mask_shape[3],
                            ),
                            &self.device,
                        );
                    }
                }
            }

            // Fallback to the original infer mask creation
            self.create_infer_causal_mask_tensor(position, context_length)
        }
    }

    /// Create a QwenConfig for a known model ID
    pub fn for_model_id(model_id: &str) -> Result<Self, CandleError> {
        let model_config = ModelConfig::get_builtin_config(model_id)
            .ok_or_else(|| CandleError::Msg(format!("Unknown model ID: {model_id}")))?;
        Ok(Self::from_model_config(model_config))
    }

    /// Create a QwenConfig for standard qwen models (legacy method)
    #[deprecated(note = "Use for_model_id() or from_model_config() instead")]
    pub fn for_standard_qwen() -> Self {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
        Self::for_model_id(model_id).unwrap_or_else(|_| {
            // Fallback to old behavior if model not found
            Self {
                naming: ModelNamingConfig::standard_qwen(),
                ..Default::default()
            }
        })
    }

    /// Create a QwenConfig with custom naming patterns
    pub fn with_custom_naming(base_prefix: &str, extension: &str) -> Self {
        Self {
            naming: ModelNamingConfig::custom(base_prefix, extension),
            ..Default::default()
        }
    }

    /// Create a QwenConfig for Bob's custom model
    pub fn for_bobs_model() -> Self {
        Self {
            naming: ModelNamingConfig::bobs_model(),
            ..Default::default()
        }
    }

    /// Set custom naming configuration
    pub fn with_naming(mut self, naming: ModelNamingConfig) -> Self {
        self.naming = naming;
        self
    }

    // Convenience methods for accessing shapes from model_config

    /// Get the batch size for this model
    pub fn batch_size(&self) -> usize {
        self.model_config.shapes.batch_size
    }

    /// Get the context length for this model  
    pub fn context_length(&self) -> usize {
        self.model_config.shapes.context_length
    }

    /// Get the hidden size for this model
    pub fn hidden_size(&self) -> usize {
        self.model_config.shapes.hidden_size
    }

    /// Get the vocabulary size for this model
    pub fn vocab_size(&self) -> usize {
        self.model_config.shapes.vocab_size
    }

    /// Get the embeddings input shape
    pub fn embeddings_input_shape(&self) -> Option<&Vec<usize>> {
        self.model_config.embeddings_input_shape()
    }

    /// Get the embeddings output shape
    pub fn embeddings_output_shape(&self) -> Option<&Vec<usize>> {
        self.model_config.embeddings_output_shape()
    }

    /// Check if this model has multipart logits
    pub fn has_multipart_logits(&self) -> bool {
        self.model_config.has_multipart_logits()
    }

    /// Get the number of logits parts
    pub fn logits_part_count(&self) -> usize {
        self.model_config.logits_part_count()
    }
}
#[cfg(test)]
mod tests {
    use crate::qwen::config::QwenConfig;
    use crate::ModelConfig;

    fn create_test_model_config_standard() -> ModelConfig {
        // Create a standard ANEMLL config (embeddings input [1, 64])
        ModelConfig::get_builtin_config("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")
            .expect("Standard ANEMLL config should be available")
    }

    fn create_test_qwen_config_standard() -> QwenConfig {
        let model_config = create_test_model_config_standard();
        QwenConfig::from_model_config(model_config)
    }

    #[test]
    fn test_model_config_loading() {
        let standard_config = create_test_model_config_standard();

        // Verify standard ANEMLL model shapes
        assert_eq!(standard_config.shapes.batch_size, 64);
        assert_eq!(standard_config.shapes.context_length, 512);
        assert_eq!(standard_config.shapes.hidden_size, 1024);
        assert_eq!(standard_config.shapes.vocab_size, 151936);

        // Verify embeddings input shape for standard model (should be [1, 64])
        let embeddings_input = standard_config.embeddings_input_shape().unwrap();
        assert_eq!(embeddings_input, &vec![1, 64]);
    }

    #[test]
    fn test_multipart_logits_detection() {
        let standard_config = create_test_model_config_standard();

        // Standard ANEMLL should have single logits output
        assert!(!standard_config.has_multipart_logits());
        assert_eq!(standard_config.logits_part_count(), 1);
    }

    #[test]
    fn test_qwen_config_accessor_methods() {
        let standard_config = create_test_qwen_config_standard();

        // Test standard ANEMLL accessor methods
        assert_eq!(standard_config.batch_size(), 64); // ANEMLL uses batch_size=64
        assert_eq!(standard_config.context_length(), 512);
        assert_eq!(standard_config.hidden_size(), 1024);
        assert_eq!(standard_config.vocab_size(), 151936);
    }

    #[test]
    fn test_dynamic_padding_logic_standard_model() {
        let config = create_test_qwen_config_standard();

        // Test dynamic padding logic using the ModelConfig directly
        if let Some(input_shape) = config.embeddings_input_shape() {
            let expected_length = input_shape[1]; // Should be 64 for standard ANEMLL
            assert_eq!(expected_length, 64);

            // Test single token padding logic
            let single_token = vec![123];
            let mut padded = single_token.clone();
            padded.resize(expected_length, 0);
            assert_eq!(padded.len(), 64);
            assert_eq!(padded[0], 123);
            assert_eq!(padded[1], 0); // Padding

            // Test multi-token padding logic
            let multi_tokens = [123, 456, 789];
            let mut padded = multi_tokens.to_vec();
            padded.resize(expected_length, 0);
            assert_eq!(padded.len(), 64);
            assert_eq!(&padded[0..3], [123, 456, 789]);
            assert_eq!(padded[3], 0); // Padding
        } else {
            panic!("Standard ANEMLL config should have embeddings input shape");
        }
    }

    #[test]
    fn test_dynamic_tensor_shape_logic() {
        let config = create_test_qwen_config_standard();

        // Test expected tensor shapes based on ModelConfig
        if let Some(input_shape) = config.embeddings_input_shape() {
            // Test embeddings input tensor shape expectations
            let expected_dims = (input_shape[0], input_shape[1]); // (1, 64) for standard ANEMLL
            assert_eq!(expected_dims, (1, 64));
        }

        // Test position tensor shape logic
        let positions = [0, 1, 2];
        let expected_pos_shape = (positions.len(),); // (3,)
        assert_eq!(expected_pos_shape, (3,));

        // Test update mask tensor shape expectations
        let context_length = config.context_length(); // 512 for standard model
        let expected_update_shape = (1, 1, context_length, 1); // (1, 1, 512, 1)
        assert_eq!(expected_update_shape, (1, 1, 512, 1));

        // Test causal mask tensor shape expectations
        let seq_len = 10;
        let expected_causal_shape = (1, 1, seq_len, context_length); // (1, 1, 10, 512)
        assert_eq!(expected_causal_shape, (1, 1, 10, 512));
    }

    #[test]
    fn test_qwen_config_for_model_id() {
        // Test loading by model ID
        let standard_config =
            QwenConfig::for_model_id("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4").unwrap();
        assert_eq!(standard_config.batch_size(), 64); // ANEMLL uses batch_size=64
        assert_eq!(standard_config.context_length(), 512);

        // Test unknown model ID
        let unknown_result = QwenConfig::for_model_id("unknown/model");
        assert!(unknown_result.is_err());
    }

    #[test]
    fn test_model_config_validation() {
        let standard_config = create_test_model_config_standard();

        // Config should be valid
        assert!(standard_config.validate().is_ok());
    }

    #[test]
    fn test_shape_backward_compatibility() {
        let config = create_test_qwen_config_standard();

        // Test that new accessor methods return the same values as ModelConfig
        assert_eq!(config.batch_size(), config.model_config.shapes.batch_size);
        assert_eq!(
            config.context_length(),
            config.model_config.shapes.context_length
        );
        assert_eq!(config.hidden_size(), config.model_config.shapes.hidden_size);
        assert_eq!(config.vocab_size(), config.model_config.shapes.vocab_size);

        // Test embeddings shape accessors
        assert_eq!(
            config.embeddings_input_shape().unwrap(),
            config.model_config.embeddings_input_shape().unwrap()
        );
        assert_eq!(
            config.embeddings_output_shape().unwrap(),
            config.model_config.embeddings_output_shape().unwrap()
        );

        // Test multipart logits accessors
        assert_eq!(
            config.has_multipart_logits(),
            config.model_config.has_multipart_logits()
        );
        assert_eq!(
            config.logits_part_count(),
            config.model_config.logits_part_count()
        );
    }

    #[test]
    fn test_tokenization_validation() {
        let standard_config = create_test_qwen_config_standard();

        // Create mock tokens that would exceed context length
        let long_tokens: Vec<i64> = (0..600).collect(); // Exceeds model

        // Test validation logic (without actual tokenization)
        assert!(long_tokens.len() > standard_config.context_length());

        // Test tokens within limits
        let short_tokens: Vec<i64> = (0..100).collect(); // Within model
        assert!(short_tokens.len() <= standard_config.context_length());
    }
}
