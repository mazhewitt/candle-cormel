//! Unit tests for tensor operations and utilities
//! 
//! Tests pure tensor logic without requiring models or external dependencies

//! Tensor adaptation and inference path detection tests

#[cfg(test)]
mod tensor_adaptation_tests {
    use candle_coreml::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
    use std::collections::HashMap;

    fn make_cfg(prefill_seq: usize, infer_seq: Option<usize>, hidden: usize) -> ModelConfig {
        let mut components = HashMap::new();
        // Minimal embeddings to satisfy wiring
        let mut emb_in = HashMap::new();
        emb_in.insert("input_ids".into(), TensorConfig { name: "input_ids".into(), shape: vec![1, prefill_seq], data_type: "INT32".into() });
        let mut emb_out = HashMap::new();
        emb_out.insert("hidden_states".into(), TensorConfig { name: "hidden_states".into(), shape: vec![1, prefill_seq, hidden], data_type: "FLOAT16".into() });
        components.insert("embeddings".into(), ComponentConfig { file_path: None, inputs: emb_in, outputs: emb_out, functions: vec![], input_order: None });

        // FFN prefill (sequence = prefill_seq)
        let mut pre_in = HashMap::new();
        pre_in.insert("hidden_states".into(), TensorConfig { name: "hidden_states".into(), shape: vec![1, prefill_seq, hidden], data_type: "FLOAT16".into() });
        pre_in.insert("position_ids".into(), TensorConfig { name: "position_ids".into(), shape: vec![1, prefill_seq], data_type: "INT32".into() });
        pre_in.insert("causal_mask".into(), TensorConfig { name: "causal_mask".into(), shape: vec![1, 1, prefill_seq.max(1), prefill_seq.max(1)], data_type: "FLOAT16".into() });
        components.insert("ffn_prefill".into(), ComponentConfig { file_path: None, inputs: pre_in, outputs: HashMap::new(), functions: vec!["prefill".into()], input_order: None });

        // Optional infer component
        if let Some(s) = infer_seq {
            let mut infer_in = HashMap::new();
            infer_in.insert("hidden_states".into(), TensorConfig { name: "hidden_states".into(), shape: vec![1, s, hidden], data_type: "FLOAT16".into() });
            infer_in.insert("position_ids".into(), TensorConfig { name: "position_ids".into(), shape: vec![1], data_type: "INT32".into() });
            infer_in.insert("causal_mask".into(), TensorConfig { name: "causal_mask".into(), shape: vec![1, 1, s, prefill_seq.max(1)], data_type: "FLOAT16".into() });
            components.insert("ffn_infer".into(), ComponentConfig { file_path: None, inputs: infer_in, outputs: HashMap::new(), functions: vec!["infer".into()], input_order: None });
        }

        ModelConfig {
            model_info: ModelInfo { model_id: Some("test".into()), path: None, model_type: "qwen".into(), discovered_at: None },
            naming: NamingConfig::default(),
            shapes: ShapeConfig { batch_size: 1, context_length: prefill_seq, hidden_size: hidden, vocab_size: 151_936 },
            components,
            ffn_execution: None,
        }
    }

    #[test]
    fn test_prefill_only_config() {
        let cfg = make_cfg(12, None, 1024);
        // Should only have embeddings and ffn_prefill components
        assert!(cfg.components.contains_key("embeddings"));
        assert!(cfg.components.contains_key("ffn_prefill"));
        assert!(!cfg.components.contains_key("ffn_infer"));
    }

    #[test]  
    fn test_prefill_and_infer_config() {
        let cfg = make_cfg(12, Some(1), 1024);
        // Should have all three components
        assert!(cfg.components.contains_key("embeddings"));
        assert!(cfg.components.contains_key("ffn_prefill"));
        assert!(cfg.components.contains_key("ffn_infer"));
    }

    #[test]
    fn test_tensor_shapes_consistency() {
        let cfg = make_cfg(12, Some(1), 1024);
        
        let emb = &cfg.components["embeddings"];
        assert_eq!(emb.inputs["input_ids"].shape, vec![1, 12]);
        assert_eq!(emb.outputs["hidden_states"].shape, vec![1, 12, 1024]);
        
        let prefill = &cfg.components["ffn_prefill"];
        assert_eq!(prefill.inputs["hidden_states"].shape, vec![1, 12, 1024]);
        assert_eq!(prefill.inputs["position_ids"].shape, vec![1, 12]);
        
        let infer = &cfg.components["ffn_infer"];
        assert_eq!(infer.inputs["position_ids"].shape, vec![1]);
    }
}

// Consolidated embeddings creation tests
#[cfg(test)]
mod embeddings_creation_tests {
    use candle_core::Device;
    use candle_coreml::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
    use candle_coreml::QwenConfig;
    use std::collections::HashMap;

    fn create_simple_config() -> ModelConfig {
        let mut components = HashMap::new();
        let mut inputs = HashMap::new();
        inputs.insert("input_ids".to_string(), TensorConfig {
            name: "input_ids".to_string(),
            shape: vec![1, 12],
            data_type: "INT32".to_string(),
        });
        let mut outputs = HashMap::new();
        outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],
            data_type: "FLOAT16".to_string(),
        });
        components.insert("embeddings".to_string(), ComponentConfig {
            file_path: None,
            inputs,
            outputs,
            functions: vec![],
            input_order: None,
        });

        ModelConfig {
            model_info: ModelInfo { model_id: Some("test-model".into()), path: None, model_type: "qwen".into(), discovered_at: None },
            naming: NamingConfig::default(),
            shapes: ShapeConfig { batch_size: 1, context_length: 12, hidden_size: 1024, vocab_size: 151_936 },
            components,
            ffn_execution: None,
        }
    }

    #[test]
    fn test_embeddings_input_tensor_creation() {
        let config = create_simple_config();
        let qwen_config = QwenConfig::from_model_config(config);
        
        let tokens = vec![1i64, 2, 3];
        let result = qwen_config.create_embeddings_input_tensor(&tokens);
        assert!(result.is_ok(), "Should create embeddings input tensor successfully");
    }

    #[test]
    fn test_single_token_embeddings() {
        let config = create_simple_config();
        let qwen_config = QwenConfig::from_model_config(config);
        
        let result = qwen_config.create_single_token_embeddings_input(42);
        assert!(result.is_ok(), "Should create single token embeddings input successfully");
    }
}
