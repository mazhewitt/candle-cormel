//! Regression test: infer position_ids must be length-1 even if config mistakenly reports a vector.

use candle_coreml::model_config::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
use candle_coreml::QwenConfig;
use std::collections::HashMap;

fn misconfigured_infer_config() -> ModelConfig {
    let mut components = HashMap::new();

    // Minimal embeddings (not used here)
    components.insert(
        "embeddings".to_string(),
        ComponentConfig {
            file_path: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        },
    );

    // FFN prefill (not used here)
    components.insert(
        "ffn_prefill".to_string(),
        ComponentConfig {
            file_path: None,
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        },
    );

    // FFN infer with WRONG position_ids shape [128] instead of [1]
    let mut infer_inputs = HashMap::new();
    infer_inputs.insert(
        "position_ids".to_string(),
        TensorConfig { name: "position_ids".to_string(), shape: vec![128], data_type: "INT32".to_string() },
    );
    let mut infer_outputs = HashMap::new();
    infer_outputs.insert(
        "output_hidden_states".to_string(),
        TensorConfig { name: "output_hidden_states".to_string(), shape: vec![1, 1, 1024], data_type: "FLOAT16".to_string() },
    );
    components.insert(
        "ffn_infer".to_string(),
        ComponentConfig { file_path: None, inputs: infer_inputs, outputs: infer_outputs, functions: vec![], input_order: None },
    );

    // LM head
    let mut lm_in = HashMap::new();
    lm_in.insert(
        "hidden_states".to_string(),
        TensorConfig { name: "hidden_states".to_string(), shape: vec![1, 1, 1024], data_type: "FLOAT16".to_string() },
    );
    let mut lm_out = HashMap::new();
    lm_out.insert(
        "logits".to_string(),
        TensorConfig { name: "logits".to_string(), shape: vec![1, 1, 1000], data_type: "FLOAT32".to_string() },
    );
    components.insert(
        "lm_head".to_string(),
        ComponentConfig { file_path: None, inputs: lm_in, outputs: lm_out, functions: vec![], input_order: None },
    );

    ModelConfig {
        model_info: ModelInfo { model_id: Some("test/misconfigured-infer".to_string()), path: None, model_type: "qwen".to_string(), discovered_at: None },
        shapes: ShapeConfig { batch_size: 1, context_length: 128, hidden_size: 1024, vocab_size: 1000 },
        components,
        naming: NamingConfig { embeddings_pattern: None, ffn_prefill_pattern: None, ffn_infer_pattern: None, lm_head_pattern: None },
        ffn_execution: Some("split".to_string()),
    }
}

#[test]
fn infer_position_ids_should_be_length_one_even_if_config_vector() {
    let cfg = QwenConfig::from_model_config(misconfigured_infer_config());
    // Ask for infer-mode position ids
    let t = cfg
        .create_position_ids_with_mode_detection(&[7], false)
        .expect("tensor");
    assert_eq!(t.dims(), vec![1], "Infer position_ids must be [1], not a 128-length vector");
}
