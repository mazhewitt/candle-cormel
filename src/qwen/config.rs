//! Qwen model configuration and validation logic
//!
//! This module handles the configuration of Qwen models, including shape management,
//! model ID resolution, and factory methods for different model variants.

use crate::qwen::naming::ModelNamingConfig;
use crate::ModelConfig;
use candle_core::{Device, Error as CandleError};
use tracing::debug;
use std::collections::HashMap;

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
        self.model_config.create_embeddings_input_tensor(tokens, &self.device)
    }

    /// Create position IDs tensor for FFN prefill with proper shape
    pub fn create_ffn_position_ids_tensor(
        &self,
        positions: &[i64],
    ) -> Result<candle_core::Tensor, CandleError> {
        self.model_config.create_ffn_position_ids_tensor(positions, &self.device)
    }

    /// Create causal mask tensor for FFN with proper shape
    pub fn create_ffn_causal_mask_tensor(
        &self,
        batch_size: usize,
        context_length: usize,
    ) -> Result<candle_core::Tensor, CandleError> {
        self.model_config.create_ffn_causal_mask_tensor(batch_size, context_length, &self.device)
    }

    /// Create single token hidden states tensor for LM head
    pub fn create_single_token_hidden_states(
        &self,
        tokens: &[i64],
    ) -> Result<candle_core::Tensor, CandleError> {
        self.model_config.create_single_token_hidden_states(tokens, &self.device)
    }

    /// Create position IDs tensor for inference (single position)
    pub fn create_infer_position_ids_tensor(
        &self,
        position: usize,
    ) -> Result<candle_core::Tensor, CandleError> {
        self.model_config.create_infer_position_ids_tensor(position as i64, &self.device)
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
            // For infer, prefer the ffn_infer expected shape if available
            if let Some(infer_shape) =
                self.model_config
                    .get_tensor_shape("ffn_infer", "position_ids", true)
            {
                // Commonly a 1-D vector shape
                if infer_shape.len() == 1 {
                    // Regardless of reported length, inference expects a single position id [1]
                    if infer_shape[0] != 1 {
                        debug!(
                            "⚠️ Config reports infer position_ids len={} but infer expects [1]; overriding to [1]",
                            infer_shape[0]
                        );
                    }
                    return candle_core::Tensor::from_vec(vec![positions[0]], (1,), &self.device);
                } else {
                    // Non 1-D shapes: fall back to create_infer_position_ids_tensor which honors configured shape
                    debug!(
                        "⚠️ Uncommon ffn_infer position_ids shape {:?}, using create_infer_position_ids_tensor",
                        infer_shape
                    );
                    return self.create_infer_position_ids_tensor(positions[0] as usize);
                }
            }

            // If infer shape unknown but prefill carries a vector length, use that length
            if let Some(prefill_shape) =
                self.model_config
                    .get_tensor_shape("ffn_prefill", "position_ids", true)
            {
                if prefill_shape.len() == 1 && prefill_shape[0] > 1 {
                    let len = prefill_shape[0];
                    debug!(
                        "🔧 Using prefill position_ids length {} for infer (no infer shape found)",
                        len
                    );
                    let vec: Vec<i64> = (0..len as i64).collect();
                    return candle_core::Tensor::from_vec(vec, (len,), &self.device);
                }
            }

            // Next best: derive expected length from prefill hidden_states seq_len if fixed (>1)
            if let Some(hs_shape) =
                self.model_config
                    .get_tensor_shape("ffn_prefill", "hidden_states", true)
            {
                if hs_shape.len() == 3 {
                    let seq_len = hs_shape[1];
                    if seq_len > 1 {
                        debug!(
                            "🔧 Using prefill hidden_states seq_len {} for infer position_ids",
                            seq_len
                        );
                        let vec: Vec<i64> = (0..seq_len as i64).collect();
                        return candle_core::Tensor::from_vec(vec, (seq_len,), &self.device);
                    }
                }
            }

            // Or from embeddings input shape [batch, seq_len]
            if let Some(emb_in_shape) = self.model_config.embeddings_input_shape() {
                if emb_in_shape.len() == 2 {
                    let seq_len = emb_in_shape[1];
                    if seq_len > 1 {
                        debug!(
                            "🔧 Using embeddings input seq_len {} for infer position_ids",
                            seq_len
                        );
                        let vec: Vec<i64> = (0..seq_len as i64).collect();
                        return candle_core::Tensor::from_vec(vec, (seq_len,), &self.device);
                    }
                }
            }

            // Final fallback: build a full-length vector matching context_length
            let len = self.model_config.shapes.context_length;
            debug!(
                "⚠️ No explicit infer/prefill shape for position_ids; using context-length vector of {}",
                len
            );
            let vec: Vec<i64> = (0..len as i64).collect();
            candle_core::Tensor::from_vec(vec, (len,), &self.device)
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
    
    /// Create current position tensor for FFN (delegates to ModelConfig)
    pub fn create_current_pos_tensor(
        &self,
        position: i64,
    ) -> Result<candle_core::Tensor, CandleError> {
        self.model_config.create_current_pos_tensor(position, &self.device)
    }

    /// Create a QwenConfig for a known model ID (deprecated - use UnifiedModelLoader instead)
    #[deprecated(note = "Use UnifiedModelLoader to load models dynamically instead of hardcoded configs")]
    pub fn for_model_id(model_id: &str) -> Result<Self, CandleError> {
        // This method is deprecated. Users should use UnifiedModelLoader which automatically
        // downloads models and generates configs dynamically.
        Err(CandleError::Msg(format!(
            "for_model_id is deprecated. Use UnifiedModelLoader to load model '{}' dynamically",
            model_id
        )))
    }

    /// Create a QwenConfig for standard qwen models (legacy method)
    

    

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

impl Default for QwenConfig {
    fn default() -> Self {
        // Minimal default ModelConfig with standard Qwen shapes and no components.
        // Loading a model without an explicit config will still fail later if component
        // file paths are required, but providing Default maintains API compatibility.
        let model_config = crate::model_config::ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some("default/qwen".to_string()),
                path: None,
                model_type: "qwen".to_string(),
                discovered_at: None,
            },
            shapes: crate::model_config::ShapeConfig {
                batch_size: 1,
                context_length: QWEN_CONTEXT_LENGTH,
                hidden_size: QWEN_HIDDEN_SIZE,
                vocab_size: QWEN_VOCAB_SIZE,
            },
            components: HashMap::new(),
            naming: crate::model_config::NamingConfig {
                embeddings_pattern: None,
                ffn_prefill_pattern: None,
                ffn_infer_pattern: None,
                lm_head_pattern: None,
            },
            ffn_execution: None,
        };

        let naming = ModelNamingConfig::default();
        Self {
            device: Device::Cpu,
            naming,
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
#[cfg(test)]
mod tests {
    use crate::qwen::config::QwenConfig;
    use crate::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
    use std::collections::HashMap;

    fn create_test_model_config_standard() -> ModelConfig {
        // Build a reasonable synthetic config with split FFN (prefill + infer)
        let mut components: HashMap<String, ComponentConfig> = HashMap::new();

        // Embeddings: input_ids [1, 64] -> hidden_states [1, 64, 1024]
        let mut emb_in = HashMap::new();
        emb_in.insert(
            "input_ids".to_string(),
            TensorConfig {
                name: "input_ids".to_string(),
                shape: vec![1, 64],
                data_type: "INT32".to_string(),
            },
        );
        let mut emb_out = HashMap::new();
        emb_out.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 64, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        components.insert(
            "embeddings".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: emb_in,
                outputs: emb_out,
                functions: vec![],
                input_order: None,
            },
        );

        // FFN prefill: hidden_states [1, 64, 1024], position_ids [64], causal_mask [1,1,64,64] -> output_hidden_states [1,1,1024]
        let mut ffn_prefill_in = HashMap::new();
        ffn_prefill_in.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 64, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        ffn_prefill_in.insert(
            "position_ids".to_string(),
            TensorConfig {
                name: "position_ids".to_string(),
                shape: vec![64],
                data_type: "INT32".to_string(),
            },
        );
        ffn_prefill_in.insert(
            "causal_mask".to_string(),
            TensorConfig {
                name: "causal_mask".to_string(),
                shape: vec![1, 1, 64, 64],
                data_type: "FLOAT32".to_string(),
            },
        );
        let mut ffn_prefill_out = HashMap::new();
        ffn_prefill_out.insert(
            "output_hidden_states".to_string(),
            TensorConfig {
                name: "output_hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        components.insert(
            "ffn_prefill".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: ffn_prefill_in,
                outputs: ffn_prefill_out,
                functions: vec![],
                input_order: None,
            },
        );

        // FFN infer (single-token): hidden_states [1,1,1024], position_ids [1] -> output_hidden_states [1,1,1024]
        let mut ffn_infer_in = HashMap::new();
        ffn_infer_in.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        ffn_infer_in.insert(
            "position_ids".to_string(),
            TensorConfig {
                name: "position_ids".to_string(),
                shape: vec![1],
                data_type: "INT32".to_string(),
            },
        );
        let mut ffn_infer_out = HashMap::new();
        ffn_infer_out.insert(
            "output_hidden_states".to_string(),
            TensorConfig {
                name: "output_hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        components.insert(
            "ffn_infer".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: ffn_infer_in,
                outputs: ffn_infer_out,
                functions: vec![],
                input_order: None,
            },
        );

        // LM head: hidden_states [1,1,1024] -> logits [1,1,151936]
        let mut lm_in = HashMap::new();
        lm_in.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        let mut lm_out = HashMap::new();
        lm_out.insert(
            "logits".to_string(),
            TensorConfig {
                name: "logits".to_string(),
                shape: vec![1, 1, 151_936],
                data_type: "FLOAT32".to_string(),
            },
        );
        components.insert(
            "lm_head".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: lm_in,
                outputs: lm_out,
                functions: vec![],
                input_order: None,
            },
        );

        ModelConfig {
            model_info: ModelInfo {
                model_id: Some("test/model".to_string()),
                path: Some("/test".to_string()),
                model_type: "qwen".to_string(),
                discovered_at: None,
            },
            shapes: ShapeConfig {
                batch_size: 1,
                context_length: 512,
                hidden_size: 1024,
                vocab_size: 151_936,
            },
            components,
            naming: NamingConfig {
                embeddings_pattern: None,
                ffn_prefill_pattern: None,
                ffn_infer_pattern: None,
                lm_head_pattern: None,
            },
            ffn_execution: Some("split".to_string()),
        }
    }

    fn create_test_qwen_config_standard() -> QwenConfig {
        let mc = create_test_model_config_standard();
        QwenConfig::from_model_config(mc)
    }

    #[test]
    fn test_model_config_loading() {
        let standard_config = create_test_qwen_config_standard().model_config.clone();

        assert_eq!(standard_config.shapes.batch_size, 1);
        assert_eq!(standard_config.shapes.context_length, 512);
        assert_eq!(standard_config.shapes.hidden_size, 1024);
        assert_eq!(standard_config.shapes.vocab_size, 151_936);

        // Embeddings input shape should be available in this synthetic config
        assert_eq!(standard_config.embeddings_input_shape(), Some(&vec![1, 64]));
    }

    #[test]
    fn test_multipart_logits_detection() {
        let mut config = create_test_model_config_standard();
        assert!(!config.has_multipart_logits());
        assert_eq!(config.logits_part_count(), 1);

        // Convert lm_head to multipart logits
        if let Some(lm_head) = config.components.get_mut("lm_head") {
            lm_head.outputs.clear();
            lm_head.outputs.insert(
                "logits1".to_string(),
                TensorConfig {
                    name: "logits1".to_string(),
                    shape: vec![1, 1, 10],
                    data_type: "FLOAT32".to_string(),
                },
            );
            lm_head.outputs.insert(
                "logits2".to_string(),
                TensorConfig {
                    name: "logits2".to_string(),
                    shape: vec![1, 1, 20],
                    data_type: "FLOAT32".to_string(),
                },
            );
        }
        assert!(config.has_multipart_logits());
        assert_eq!(config.logits_part_count(), 2);
    }

    #[test]
    fn test_qwen_config_accessor_methods() {
        let cfg = create_test_qwen_config_standard();
        assert_eq!(cfg.batch_size(), 1);
        assert_eq!(cfg.context_length(), 512);
        assert_eq!(cfg.hidden_size(), 1024);
        assert_eq!(cfg.vocab_size(), 151_936);
        assert_eq!(cfg.embeddings_input_shape(), Some(&vec![1, 64]));
        assert_eq!(cfg.embeddings_output_shape(), Some(&vec![1, 64, 1024]));
    }

    #[test]
    fn test_model_config_validation() {
        let mc = create_test_model_config_standard();
        // Should be valid and wiring consistent for our synthetic shapes
        assert!(mc.validate().is_ok());
        assert!(mc.validate_internal_wiring().is_ok());
    }
}
