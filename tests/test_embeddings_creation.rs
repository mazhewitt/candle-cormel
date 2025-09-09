//! Embeddings input tensor creation tests (no CoreML runtime required)

#[cfg(test)]
mod embeddings_creation_tests {
    use candle_core::Device;
    use candle_coreml::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
    use candle_coreml::QwenConfig;
    use std::collections::HashMap;

    fn base_model_config_with_embeddings(input_len: usize, hidden: usize) -> ModelConfig {
        // Minimal config with embeddings and ffn_prefill to drive seq length expectations
        let mut components = HashMap::new();

        // Embeddings shapes
        let mut emb_in = HashMap::new();
        emb_in.insert(
            "input_ids".into(),
            TensorConfig { name: "input_ids".into(), shape: vec![1, input_len], data_type: "INT32".into() },
        );
        let mut emb_out = HashMap::new();
        emb_out.insert(
            "hidden_states".into(),
            TensorConfig { name: "hidden_states".into(), shape: vec![1, input_len, hidden], data_type: "FLOAT16".into() },
        );
        components.insert(
            "embeddings".into(),
            ComponentConfig { file_path: None, inputs: emb_in, outputs: emb_out, functions: vec![], input_order: None },
        );

        // FFN prefill expects same seq len
        let mut pre_in = HashMap::new();
        pre_in.insert(
            "hidden_states".into(),
            TensorConfig { name: "hidden_states".into(), shape: vec![1, input_len, hidden], data_type: "FLOAT16".into() },
        );
        components.insert(
            "ffn_prefill".into(),
            ComponentConfig { file_path: None, inputs: pre_in, outputs: HashMap::new(), functions: vec!["prefill".into()], input_order: None },
        );

        ModelConfig {
            model_info: ModelInfo { model_id: Some("test/embeddings".into()), path: None, model_type: "qwen".into(), discovered_at: None },
            shapes: ShapeConfig { batch_size: 1, context_length: input_len, hidden_size: hidden, vocab_size: 1000 },
            components,
            naming: NamingConfig { embeddings_pattern: None, ffn_prefill_pattern: None, ffn_infer_pattern: None, lm_head_pattern: None },
            ffn_execution: Some("split".into()),
        }
    }

    #[test]
    fn test_full_sequence_embeddings_creation_padding_and_truncation() {
        let cfg = base_model_config_with_embeddings(128, 1024);
        let mut qcfg = QwenConfig::from_model_config(cfg);
        qcfg.device = Device::Cpu;

        // 1) Shorter than expected: should pad to 128
        let short_tokens: Vec<i64> = vec![1, 2, 3, 4, 5];
        let t = qcfg.create_embeddings_input_tensor(&short_tokens).expect("tensor");
        assert_eq!(t.dims(), &[1, 128]);

        // 2) Exact length: stays 128
        let exact_tokens: Vec<i64> = (0..128).map(|i| i as i64).collect();
        let t2 = qcfg.create_embeddings_input_tensor(&exact_tokens).expect("tensor");
        assert_eq!(t2.dims(), &[1, 128]);

        // 3) Longer than expected: should truncate to 128
        let long_tokens: Vec<i64> = (0..200).map(|i| i as i64).collect();
        let t3 = qcfg.create_embeddings_input_tensor(&long_tokens).expect("tensor");
        assert_eq!(t3.dims(), &[1, 128]);
    }

    #[test]
    fn test_single_token_embeddings_creation_for_unified_models() {
        // Model that expects seq_len=1 at FFN. Embeddings input could still be >1, but downstream is single-token mode.
        let cfg = base_model_config_with_embeddings(1, 768);
        let qcfg = QwenConfig::from_model_config(cfg);

        // Creating input for 1 token yields [1,1]
        let t = qcfg.create_embeddings_input_tensor(&[42]).expect("tensor");
        assert_eq!(t.dims(), &[1, 1]);
    }
}
