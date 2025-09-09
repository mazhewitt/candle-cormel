//! Test to verify that typo-fixer models correctly detect full-sequence prefill mode

#[cfg(test)]
mod prefill_detection_tests {
    use candle_coreml::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
    use std::collections::HashMap;

    #[test]
    fn test_typo_fixer_expects_full_sequence_prefill() {
        // Create a mock ModelConfig simulating typo-fixer model structure
        let mut components = HashMap::new();
        
        // Mock FFN Prefill component with sequence length = 128
        let mut ffn_prefill_inputs = HashMap::new();
        ffn_prefill_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 128, 1024],  // [batch=1, seq=128, hidden=1024]
            data_type: "FLOAT16".to_string(),
        });
        
        let ffn_prefill = ComponentConfig {
            file_path: Some("ffn_prefill.mlpackage".to_string()),
            inputs: ffn_prefill_inputs,
            outputs: HashMap::new(),
            functions: vec!["prefill".to_string()],
            input_order: None,
        };
        components.insert("ffn_prefill".to_string(), ffn_prefill);
        
        // Mock FFN Infer component with sequence length = 1
        let mut ffn_infer_inputs = HashMap::new();
        ffn_infer_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],  // [batch=1, seq=1, hidden=1024]
            data_type: "FLOAT16".to_string(),
        });
        
        let ffn_infer = ComponentConfig {
            file_path: Some("ffn_infer.mlpackage".to_string()),
            inputs: ffn_infer_inputs,
            outputs: HashMap::new(),
            functions: vec!["infer".to_string()],
            input_order: None,
        };
        components.insert("ffn_infer".to_string(), ffn_infer);
        
        // Create ModelConfig
        let model_config = ModelConfig {
            model_info: ModelInfo {
                model_id: Some("test-typo-fixer".to_string()),
                model_type: "qwen".to_string(),
                path: None,
                discovered_at: None,
            },
            shapes: ShapeConfig {
                batch_size: 1,
                context_length: 128,
                hidden_size: 1024,
                vocab_size: 30000,
            },
            components,
            naming: NamingConfig {
                embeddings_pattern: None,
                ffn_prefill_pattern: None,
                ffn_infer_pattern: None,
                lm_head_pattern: None,
            },
            ffn_execution: Some("split".to_string()),
        };
        
        // Test the detection functions
        println!("Testing prefill detection for typo-fixer model structure:");
        
        let is_single_token = model_config.prefill_is_single_token();
        println!("  prefill_is_single_token(): {}", is_single_token);
        assert!(!is_single_token, "Should NOT be single-token mode (seq_len=128, not 1)");
        
        let expects_full_seq = model_config.expects_full_sequence_prefill();
        println!("  expects_full_sequence_prefill(): {}", expects_full_seq);
        assert!(expects_full_seq, "Should expect full-sequence prefill (seq_len=128 > 1)");
        
        println!("✅ Typo-fixer model correctly detected as full-sequence prefill mode");
    }
    
    #[test]
    fn test_single_token_model_detection() {
        // Create a mock ModelConfig for a model that does single-token processing
        let mut components = HashMap::new();
        
        // Mock FFN component with single-token processing
        let mut ffn_inputs = HashMap::new();
        ffn_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],  // [batch=1, seq=1, hidden=1024]
            data_type: "FLOAT16".to_string(),
        });
        
        let ffn = ComponentConfig {
            file_path: Some("ffn.mlpackage".to_string()),
            inputs: ffn_inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        components.insert("ffn_prefill".to_string(), ffn);
        
        let model_config = ModelConfig {
            model_info: ModelInfo {
                model_id: Some("test-single-token".to_string()),
                model_type: "qwen".to_string(),
                path: None,
                discovered_at: None,
            },
            shapes: ShapeConfig {
                batch_size: 1,
                context_length: 1,
                hidden_size: 1024,
                vocab_size: 30000,
            },
            components,
            naming: NamingConfig {
                embeddings_pattern: None,
                ffn_prefill_pattern: None,
                ffn_infer_pattern: None,
                lm_head_pattern: None,
            },
            ffn_execution: Some("unified".to_string()),
        };
        
        let is_single_token = model_config.prefill_is_single_token();
        assert!(is_single_token, "Should be single-token mode (seq_len=1)");
        
        let expects_full_seq = model_config.expects_full_sequence_prefill();
        assert!(!expects_full_seq, "Should NOT expect full-sequence prefill (seq_len=1)");
        
        println!("✅ Single-token model correctly detected");
    }
}