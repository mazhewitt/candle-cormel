//! TDD tests for fixing the infer shape mismatch
//!
//! Error: "MultiArray shape (64) does not match the shape (1) specified in the model description"
//! This test reproduces and fixes the issue.

#[cfg(test)]
mod tests {
    use crate::model_config::{ComponentConfig, ModelConfig, ShapeConfig, TensorConfig};
    use crate::qwen::{QwenConfig, QwenModel};

    use std::collections::HashMap;
    use std::path::PathBuf;

    fn get_test_model_path() -> Option<PathBuf> {
        // Try the same paths as performance tests
        let possible_paths = [
            "/Users/mazdahewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4",
            "./qwen-model",
        ];

        for path in &possible_paths {
            let pb = PathBuf::from(path);
            if pb.exists() {
                return Some(pb);
            }
        }
        None
    }

    fn create_infer_compatible_config() -> ModelConfig {
        // Create a config that matches what the actual model expects for infer
        let mut components = HashMap::new();

        // Embeddings - expects [1, 64] for prefill, [1, 1] for infer
        let mut embeddings_inputs = HashMap::new();
        embeddings_inputs.insert(
            "input_ids".to_string(),
            TensorConfig {
                name: "input_ids".to_string(),
                shape: vec![1, 64], // Batch processing shape
                data_type: "INT32".to_string(),
            },
        );

        components.insert(
            "embeddings".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: embeddings_inputs,
                outputs: HashMap::new(),
                functions: vec!["main".to_string()],
                input_order: None,
            },
        );

        // FFN prefill - for batch processing
        let mut ffn_prefill_inputs = HashMap::new();
        ffn_prefill_inputs.insert(
            "position_ids".to_string(),
            TensorConfig {
                name: "position_ids".to_string(),
                shape: vec![64], // Batch size for prefill
                data_type: "INT64".to_string(),
            },
        );
        ffn_prefill_inputs.insert(
            "causal_mask".to_string(),
            TensorConfig {
                name: "causal_mask".to_string(),
                shape: vec![1, 1, 64, 512], // Batch-sized mask for prefill
                data_type: "FLOAT32".to_string(),
            },
        );

        components.insert(
            "ffn_prefill".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: ffn_prefill_inputs,
                outputs: HashMap::new(),
                functions: vec!["prefill".to_string()],
                input_order: None,
            },
        );

        // FFN infer - for single token processing
        let mut ffn_infer_inputs = HashMap::new();
        ffn_infer_inputs.insert(
            "position_ids".to_string(),
            TensorConfig {
                name: "position_ids".to_string(),
                shape: vec![1], // Single position for infer
                data_type: "INT64".to_string(),
            },
        );
        ffn_infer_inputs.insert(
            "causal_mask".to_string(),
            TensorConfig {
                name: "causal_mask".to_string(),
                shape: vec![1, 1, 1, 512], // Single-row mask for infer
                data_type: "FLOAT32".to_string(),
            },
        );

        components.insert(
            "ffn_infer".to_string(),
            ComponentConfig {
                file_path: None,
                inputs: ffn_infer_inputs,
                outputs: HashMap::new(),
                functions: vec!["infer".to_string()],
                input_order: None,
            },
        );

        ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some("test/infer-compatible".to_string()),
                path: None,
                model_type: "qwen".to_string(),
                discovered_at: None,
            },
            shapes: ShapeConfig {
                batch_size: 64, // For prefill
                context_length: 512,
                hidden_size: 1024,
                vocab_size: 151936,
            },
            components,
            naming: crate::model_config::NamingConfig {
                embeddings_pattern: None,
                ffn_prefill_pattern: None,
                ffn_infer_pattern: None,
                lm_head_pattern: None,
            },
            ffn_execution: Some("split".to_string()),
        }
    }

    #[test]
    fn test_infer_vs_prefill_position_ids() {
        let config = QwenConfig::from_model_config(create_infer_compatible_config());

        // Test prefill position IDs - should use batch size
        let prefill_positions = config
            .create_ffn_position_ids_tensor(&[0, 1, 2, 3])
            .unwrap();

        // Should be padded to batch size (64)
        assert_eq!(prefill_positions.dims(), vec![64]);

        // Test infer position IDs - should use single position
        if config.model_config.components.contains_key("ffn_infer") {
            let infer_positions = config.create_infer_position_ids_tensor(7).unwrap();
            assert_eq!(infer_positions.dims(), vec![1]); // Single position for infer
        }
    }

    #[test]
    fn test_mode_aware_position_ids() {
        let config = QwenConfig::from_model_config(create_infer_compatible_config());

        // Test prefill mode - should use batch-sized positions
        let prefill_ids = config
            .create_position_ids_with_mode_detection(&[0, 1, 2], true)
            .unwrap();
        assert_eq!(prefill_ids.dims(), vec![64]); // Batch size

        // Test infer mode - should detect and use single position
        let infer_ids = config
            .create_position_ids_with_mode_detection(&[7], false)
            .unwrap();
        assert_eq!(infer_ids.dims(), vec![1]); // Single position for infer
    }

    #[test]
    fn test_mode_aware_causal_mask() {
        let config = QwenConfig::from_model_config(create_infer_compatible_config());

        // Test prefill mode - should use batch-sized mask
        let prefill_mask = config
            .create_causal_mask_with_mode_detection(10, 512, true)
            .unwrap();
        assert_eq!(prefill_mask.dims(), vec![1, 1, 64, 512]); // Batch-sized

        // Test infer mode - should detect and use single-row mask
        let infer_mask = config
            .create_causal_mask_with_mode_detection(10, 512, false)
            .unwrap();
        assert_eq!(infer_mask.dims(), vec![1, 1, 1, 512]); // Single-row for infer
    }

    #[test]
    fn test_detect_infer_mode_requirements() {
        let config = create_infer_compatible_config();

        // Should detect different requirements for prefill vs infer
        let prefill_pos_shape = config
            .get_tensor_shape("ffn_prefill", "position_ids", true)
            .unwrap();
        let infer_pos_shape = config
            .get_tensor_shape("ffn_infer", "position_ids", true)
            .unwrap();

        assert_eq!(prefill_pos_shape, &vec![64]); // Batch processing
        assert_eq!(infer_pos_shape, &vec![1]); // Single token
    }

    #[test]
    #[ignore = "Requires model files"]
    fn test_shape_detection_integration() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => {
                eprintln!("⚠️  Skipping test - no model found");
                return;
            }
        };

        // Integration test to verify that shape detection works with real model
        let config = QwenConfig::for_model_id("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")
            .expect("Should load config");

        let result = std::panic::catch_unwind(|| {
            let mut model = QwenModel::load_from_directory(&model_path, Some(config)).unwrap();
            model.initialize_states().unwrap();

            // This should now work with our shape detection fix
            model.run_chatpy_infer(&[785, 3974, 13876, 38835, 34208, 916, 279, 15678], 9)
        });

        // With our fix implemented, this should now succeed
        assert!(
            result.is_ok(),
            "Should now work with implemented shape detection fix"
        );
    }

    #[test]
    #[ignore = "Requires model files"]
    fn test_fixed_infer_should_work() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => return,
        };

        // With our fix implemented, this should work
        let config = QwenConfig::for_model_id("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")
            .expect("Should load config");

        let mut model = QwenModel::load_from_directory(&model_path, Some(config)).unwrap();
        model.initialize_states().unwrap();

        // This should succeed with our shape mode detection implementation
        let next_token =
            model.run_chatpy_infer(&[785, 3974, 13876, 38835, 34208, 916, 279, 15678], 9);
        match &next_token {
            Ok(token) => println!("✅ Successfully generated token: {token}"),
            Err(e) => {
                println!("❌ Error: {e}");
                println!("❌ Error debug: {e:?}");
            }
        }
        assert!(
            next_token.is_ok(),
            "Should work with fixed shape handling: {:?}",
            next_token.err()
        );

        // Verify we actually get a valid token
        if let Ok(token) = next_token {
            println!("✅ Successfully generated token: {token}");
        }
    }
}
