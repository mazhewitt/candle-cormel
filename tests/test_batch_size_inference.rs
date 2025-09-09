//! Batch size and shape inference unit tests
//!
//! These tests validate metadata-driven shape inference without requiring CoreML.

#[cfg(test)]
mod batch_size_inference_tests {
    use candle_coreml::config_generator::shape_inference::ShapeInference;
    use candle_coreml::model_config::{ComponentConfig, ShapeConfig, TensorConfig};
    use std::collections::HashMap;

    fn make_tensor(name: &str, shape: &[usize], dt: &str) -> TensorConfig {
        TensorConfig { name: name.to_string(), shape: shape.to_vec(), data_type: dt.to_uppercase() }
    }

    #[test]
    fn test_split_ffn_batch_size_inference() {
        // Split FFN: prefill [1,128,1024], infer [1,1,1024]
        let mut components: HashMap<String, ComponentConfig> = HashMap::new();

        // Minimal embeddings with shapes to satisfy validator
        let mut emb_in = HashMap::new();
        emb_in.insert("input_ids".into(), make_tensor("input_ids", &[1, 128], "int32"));
        let mut emb_out = HashMap::new();
        emb_out.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 128, 1024], "float16"));
        components.insert(
            "embeddings".into(),
            ComponentConfig { file_path: None, inputs: emb_in, outputs: emb_out, functions: vec![], input_order: None },
        );

        // Prefill
        let mut prefill_in = HashMap::new();
        prefill_in.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 128, 1024], "float16"));
        prefill_in.insert("position_ids".into(), make_tensor("position_ids", &[1, 128], "int32"));
        prefill_in.insert("causal_mask".into(), make_tensor("causal_mask", &[1, 1, 128, 128], "float16"));
        prefill_in.insert("current_pos".into(), make_tensor("current_pos", &[1], "int32"));
        let mut prefill_out = HashMap::new();
        prefill_out.insert("output_hidden_states".into(), make_tensor("output_hidden_states", &[1, 128, 1024], "float16"));
        components.insert(
            "ffn_prefill".into(),
            ComponentConfig { file_path: None, inputs: prefill_in, outputs: prefill_out, functions: vec!["prefill".into()], input_order: None },
        );

        // Infer
        let mut infer_in = HashMap::new();
        infer_in.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 1, 1024], "float16"));
        infer_in.insert("position_ids".into(), make_tensor("position_ids", &[1], "int32"));
        infer_in.insert("causal_mask".into(), make_tensor("causal_mask", &[1, 1, 1, 128], "float16"));
        infer_in.insert("current_pos".into(), make_tensor("current_pos", &[1], "int32"));
        let mut infer_out = HashMap::new();
        infer_out.insert("output_hidden_states".into(), make_tensor("output_hidden_states", &[1, 1, 1024], "float16"));
        components.insert(
            "ffn_infer".into(),
            ComponentConfig { file_path: None, inputs: infer_in, outputs: infer_out, functions: vec!["infer".into()], input_order: None },
        );

        // LM head
        let mut lm_in = HashMap::new();
        lm_in.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 1, 1024], "float16"));
        let mut lm_out = HashMap::new();
        lm_out.insert("logits".into(), make_tensor("logits", &[1, 1, 151_936], "float16"));
        components.insert(
            "lm_head".into(),
            ComponentConfig { file_path: None, inputs: lm_in, outputs: lm_out, functions: vec![], input_order: None },
        );

        let si = ShapeInference::new();
        let shapes = si.infer_shapes(&components).expect("shape inference");

        assert_eq!(shapes.batch_size, 1);
        assert_eq!(shapes.context_length, 128, "context length should derive from prefill seq dim");
        assert_eq!(shapes.hidden_size, 1024);
        assert!(shapes.vocab_size >= 30000);
    }

    #[test]
    fn test_unified_ffn_batch_size_inference() {
        // Unified FFN: single component (no infer)
        let mut components: HashMap<String, ComponentConfig> = HashMap::new();

        // Embeddings
        let mut emb_in = HashMap::new();
        emb_in.insert("input_ids".into(), make_tensor("input_ids", &[1, 32], "int32"));
        let mut emb_out = HashMap::new();
        emb_out.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 32, 768], "float16"));
        components.insert(
            "embeddings".into(),
            ComponentConfig { file_path: None, inputs: emb_in, outputs: emb_out, functions: vec![], input_order: None },
        );

        // FFN unified
        let mut ffn_in = HashMap::new();
        ffn_in.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 1, 768], "float16"));
        ffn_in.insert("position_ids".into(), make_tensor("position_ids", &[1], "int32"));
        ffn_in.insert("causal_mask".into(), make_tensor("causal_mask", &[1, 1, 1, 32], "float16"));
        ffn_in.insert("current_pos".into(), make_tensor("current_pos", &[1], "int32"));
        let mut ffn_out = HashMap::new();
        ffn_out.insert("output_hidden_states".into(), make_tensor("output_hidden_states", &[1, 1, 768], "float16"));
        components.insert(
            "ffn_prefill".into(),
            ComponentConfig { file_path: None, inputs: ffn_in, outputs: ffn_out, functions: vec![], input_order: None },
        );

        // LM head
        let mut lm_in = HashMap::new();
        lm_in.insert("hidden_states".into(), make_tensor("hidden_states", &[1, 1, 768], "float16"));
        let mut lm_out = HashMap::new();
        lm_out.insert("logits".into(), make_tensor("logits", &[1, 1, 50_000], "float16"));
        components.insert(
            "lm_head".into(),
            ComponentConfig { file_path: None, inputs: lm_in, outputs: lm_out, functions: vec![], input_order: None },
        );

        let si = ShapeInference::new();
        let shapes = si.infer_shapes(&components).expect("shape inference");

        assert_eq!(shapes.batch_size, 1);
        // Context length should pick the max sequence dim present => 32 (from embeddings)
        assert_eq!(shapes.context_length, 32);
        assert_eq!(shapes.hidden_size, 768);
        assert!(shapes.vocab_size >= 50_000);
    }

    #[test]
    fn test_batch_size_inference_edge_cases() {
        // Edge: minimal tensors but valid shapes
        let mut components: HashMap<String, ComponentConfig> = HashMap::new();

        let mut emb_in = HashMap::new();
        emb_in.insert("input_ids".into(), make_tensor("input_ids", &[2, 4], "int32"));
        let mut emb_out = HashMap::new();
        emb_out.insert("hidden_states".into(), make_tensor("hidden_states", &[2, 4, 16], "float16"));
        components.insert(
            "embeddings".into(),
            ComponentConfig { file_path: None, inputs: emb_in, outputs: emb_out, functions: vec![], input_order: None },
        );

        let si = ShapeInference::new();
        let shapes: ShapeConfig = si.infer_shapes(&components).expect("shape inference");
        // Batch should be derived from first dim (2)
        assert_eq!(shapes.batch_size, 2);
        assert_eq!(shapes.context_length, 4);
        assert_eq!(shapes.hidden_size, 16);
    }
}
