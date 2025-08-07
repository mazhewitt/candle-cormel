//! TDD tests for tensor shape configuration in QwenModel
//!
//! These tests define the expected behavior for tensor creation based on ModelConfig.
//! All tests should pass once proper shape discovery is implemented.

#[cfg(test)]
mod tests {
    use crate::model_config::{ComponentConfig, ModelConfig, ShapeConfig, TensorConfig};
    use crate::qwen::QwenConfig;

    use std::collections::HashMap;

    fn create_test_model_config() -> ModelConfig {
        let mut components = HashMap::new();

        // Embeddings component - expects batch-sized input
        let mut embeddings_inputs = HashMap::new();
        embeddings_inputs.insert(
            "input_ids".to_string(),
            TensorConfig {
                name: "input_ids".to_string(),
                shape: vec![1, 64], // [batch=1, seq_len=64]
                data_type: "INT32".to_string(),
            },
        );

        let mut embeddings_outputs = HashMap::new();
        embeddings_outputs.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 64, 1024], // [batch=1, seq_len=64, hidden=1024]
                data_type: "FLOAT16".to_string(),
            },
        );

        components.insert(
            "embeddings".to_string(),
            ComponentConfig {
                file_path: None,
                file_pattern: None,
                inputs: embeddings_inputs,
                outputs: embeddings_outputs,
                functions: vec!["main".to_string()],
            },
        );

        // FFN prefill component
        let mut ffn_prefill_inputs = HashMap::new();
        ffn_prefill_inputs.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 64, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );
        ffn_prefill_inputs.insert(
            "position_ids".to_string(),
            TensorConfig {
                name: "position_ids".to_string(),
                shape: vec![64], // Batch-sized position ids
                data_type: "INT64".to_string(),
            },
        );
        ffn_prefill_inputs.insert(
            "causal_mask".to_string(),
            TensorConfig {
                name: "causal_mask".to_string(),
                shape: vec![1, 1, 64, 512], // [1, 1, batch_size, context_length]
                data_type: "FLOAT32".to_string(),
            },
        );
        ffn_prefill_inputs.insert(
            "current_pos".to_string(),
            TensorConfig {
                name: "current_pos".to_string(),
                shape: vec![1],
                data_type: "INT64".to_string(),
            },
        );

        let mut ffn_prefill_outputs = HashMap::new();
        ffn_prefill_outputs.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 64, 1024],
                data_type: "FLOAT16".to_string(),
            },
        );

        components.insert(
            "ffn_prefill".to_string(),
            ComponentConfig {
                file_path: None,
                file_pattern: None,
                inputs: ffn_prefill_inputs,
                outputs: ffn_prefill_outputs,
                functions: vec!["prefill".to_string()],
            },
        );

        // LM head component - expects single token input
        let mut lm_head_inputs = HashMap::new();
        lm_head_inputs.insert(
            "hidden_states".to_string(),
            TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 1, 1024], // Single token
                data_type: "FLOAT16".to_string(),
            },
        );

        let mut lm_head_outputs = HashMap::new();
        lm_head_outputs.insert(
            "logits".to_string(),
            TensorConfig {
                name: "logits".to_string(),
                shape: vec![1, 1, 151936],
                data_type: "FLOAT32".to_string(),
            },
        );

        components.insert(
            "lm_head".to_string(),
            ComponentConfig {
                file_path: None,
                file_pattern: None,
                inputs: lm_head_inputs,
                outputs: lm_head_outputs,
                functions: vec!["main".to_string()],
            },
        );

        ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some("test/model".to_string()),
                path: None,
                model_type: "qwen".to_string(),
                discovered_at: None,
            },
            shapes: ShapeConfig {
                batch_size: 64,
                context_length: 512,
                hidden_size: 1024,
                vocab_size: 151936,
            },
            components,
            naming: crate::model_config::NamingConfig {
                embeddings_pattern: "*_embeddings.mlmodelc".to_string(),
                ffn_prefill_pattern: Some("*_FFN_PF_*_chunk_*.mlmodelc".to_string()),
                ffn_infer_pattern: None,
                lm_head_pattern: "*_lm_head_*.mlmodelc".to_string(),
            },
        }
    }

    #[test]
    fn test_tensor_creation_from_config() {
        let model_config = create_test_model_config();
        let config = QwenConfig::from_model_config(model_config);

        // Test embeddings input tensor creation
        let tokens = vec![1, 2, 3, 4];
        let embeddings_input = config.create_embeddings_input_tensor(&tokens).unwrap();

        // Should pad to expected batch size
        let expected_shape = config
            .model_config
            .embeddings_input_shape()
            .unwrap()
            .clone();
        assert_eq!(embeddings_input.dims(), expected_shape);

        // Test position ids tensor creation
        let position_ids = config
            .create_ffn_position_ids_tensor(&[0, 1, 2, 3])
            .unwrap();
        let expected_pos_shape = config
            .model_config
            .get_tensor_shape("ffn_prefill", "position_ids", true)
            .unwrap()
            .clone();
        assert_eq!(position_ids.dims(), expected_pos_shape);

        // Test causal mask tensor creation
        let causal_mask = config.create_ffn_causal_mask_tensor(64, 512).unwrap();
        let expected_mask_shape = config
            .model_config
            .get_tensor_shape("ffn_prefill", "causal_mask", true)
            .unwrap()
            .clone();
        assert_eq!(causal_mask.dims(), expected_mask_shape);
    }

    #[test]
    fn test_infer_tensor_shapes() {
        let model_config = create_test_model_config();
        let config = QwenConfig::from_model_config(model_config);

        // For inference, we need single token shapes
        let single_token_embeddings = config.create_single_token_hidden_states(&[42]).unwrap();

        // Should match LM head expected input shape
        let expected_lm_head_shape = config
            .model_config
            .get_tensor_shape("lm_head", "hidden_states", true)
            .unwrap()
            .clone();
        assert_eq!(single_token_embeddings.dims(), expected_lm_head_shape);

        // Position ids for inference should be single position
        let infer_position_ids = config.create_infer_position_ids_tensor(7).unwrap();
        assert_eq!(infer_position_ids.dims(), vec![1]);

        // Causal mask should maintain full dimensions
        let infer_causal_mask = config.create_infer_causal_mask_tensor(7, 512).unwrap();
        let expected_mask_shape = config
            .model_config
            .get_tensor_shape("ffn_prefill", "causal_mask", true)
            .unwrap()
            .clone();
        assert_eq!(infer_causal_mask.dims(), expected_mask_shape);
    }

    #[test]
    fn test_adaptive_batch_size() {
        // Test with different batch size
        let mut model_config = create_test_model_config();
        model_config.shapes.batch_size = 32; // Different batch size

        // Update component shapes to match
        if let Some(ffn_config) = model_config.components.get_mut("ffn_prefill") {
            ffn_config.inputs.get_mut("position_ids").unwrap().shape = vec![32];
            ffn_config.inputs.get_mut("causal_mask").unwrap().shape = vec![1, 1, 32, 512];
        }
        if let Some(embeddings_config) = model_config.components.get_mut("embeddings") {
            embeddings_config.inputs.get_mut("input_ids").unwrap().shape = vec![1, 32];
            embeddings_config
                .outputs
                .get_mut("hidden_states")
                .unwrap()
                .shape = vec![1, 32, 1024];
        }

        let config = QwenConfig::from_model_config(model_config);

        // Should create tensors with new batch size
        let tokens = vec![1, 2, 3];
        let embeddings_input = config.create_embeddings_input_tensor(&tokens).unwrap();
        assert_eq!(embeddings_input.dims()[1], 32); // Padded to batch_size=32

        let position_ids = config.create_ffn_position_ids_tensor(&[0, 1, 2]).unwrap();
        assert_eq!(position_ids.dims()[0], 32); // position_ids length matches batch_size
    }
}
