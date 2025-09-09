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
            model_info: ModelInfo { model_id: Some("test/adapt".into()), path: None, model_type: "qwen".into(), discovered_at: None },
            shapes: ShapeConfig { batch_size: 1, context_length: prefill_seq.max(1), hidden_size: hidden, vocab_size: 1000 },
            components,
            naming: NamingConfig { embeddings_pattern: None, ffn_prefill_pattern: None, ffn_infer_pattern: None, lm_head_pattern: None },
            ffn_execution: if infer_seq.is_some() { Some("split".into()) } else { Some("unified".into()) },
        }
    }

    #[test]
    fn test_inference_path_selection_flags() {
        // Split model with full-sequence prefill
        let cfg_full = make_cfg(128, Some(1), 1024);
        assert!(cfg_full.expects_full_sequence_prefill());
        assert!(!cfg_full.prefill_is_single_token());

        // Unified single-token model
        let cfg_single = make_cfg(1, None, 768);
        assert!(!cfg_single.expects_full_sequence_prefill());
        assert!(cfg_single.prefill_is_single_token());
    }
}
