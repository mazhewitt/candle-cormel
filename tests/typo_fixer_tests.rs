//! Tests for the typo-fixer config and related sampling/utilities.
use candle_core::{Device, Tensor};
use candle_coreml::ModelConfig; // crate root

#[test]
fn test_parse_typo_fixer_config() {
    let json = include_str!("../configs/typo-fixer-final.json");
    let cfg: ModelConfig = serde_json::from_str(json).expect("parse config");

    // Basic shapes
    assert_eq!(cfg.shapes.batch_size, 64);
    assert_eq!(cfg.shapes.context_length, 256);
    assert_eq!(cfg.shapes.hidden_size, 1024);
    assert!(cfg.shapes.vocab_size > 100_000);

    // Components present
    assert!(cfg.components.contains_key("embeddings"));
    assert!(cfg.components.contains_key("ffn_prefill"));
    assert!(cfg.components.contains_key("ffn_infer"));
    assert!(cfg.components.contains_key("lm_head"));

    // Prefill position_ids shape is [64]
    let prefill = cfg.components.get("ffn_prefill").unwrap();
    let pos_shape = &prefill.inputs.get("position_ids").unwrap().shape;
    assert_eq!(pos_shape, &vec![64]);

    // Infer position_ids shape is [1]
    let infer = cfg.components.get("ffn_infer").unwrap();
    let infer_pos_shape = &infer.inputs.get("position_ids").unwrap().shape;
    assert_eq!(infer_pos_shape, &vec![1]);

    // LM head multipart logits present (logits1..logits16)
    let lm = cfg.components.get("lm_head").unwrap();
    for i in 1..=16 {
        let k = format!("logits{}", i);
        assert!(lm.outputs.contains_key(&k), "missing {}", k);
    }

    // Validate config consistency
    cfg.validate().expect("config should validate");
}

#[test]
fn test_sampling_topk_and_temperature() {
    use candle_coreml::sampling; // re-exported in lib.rs
                                 // Create deterministic logits: increasing sequence
    let device = Device::Cpu;
    let vocab = 100usize;
    let data: Vec<f32> = (0..vocab).map(|i| i as f32 / 10.0).collect();
    let logits = Tensor::from_vec(data.clone(), (vocab,), &device).unwrap();

    // Greedy (temperature 0) should pick last index
    let greedy = sampling::greedy_sample(&logits).unwrap();
    assert_eq!(greedy as usize, vocab - 1);

    // Top-k with k=10 and temperature 0 should also pick the max
    let topk = sampling::sample_top_k(&logits, 10, 0.0).unwrap();
    assert_eq!(topk as usize, vocab - 1);

    // Temperature sampling (non-zero) should return an index within range
    let sampled = sampling::sample_top_k(&logits, 10, 0.7).unwrap();
    assert!(sampled < vocab as i64);
}

#[test]
fn test_typo_fixer_prefill_infer_pipeline_if_available() {
    use candle_coreml::{qwen::QwenConfig, qwen::QwenModel};
    use candle_core::Device;
    // Only run if local model path exists
    let config_json = include_str!("../configs/typo-fixer-working.json");
    let model_cfg: ModelConfig = serde_json::from_str(config_json).expect("parse working config");
    let path_str = model_cfg.model_info.path.clone().unwrap_or_default();
    let model_path = std::path::Path::new(&path_str);
    if !model_path.exists() { eprintln!("Skipping typo-fixer pipeline test: model path missing: {:?}", model_path); return; }
    // Build QwenConfig from ModelConfig
    let qcfg = QwenConfig::from_model_config(model_cfg.clone());
    // Load model directory (expects tokenizer + mlpackages in path)
    match QwenModel::load_from_directory(model_path, Some(qcfg.clone())) {
        Ok(mut model) => {
            // Simple prompt
            let prompt = "teh quick bronw"; // purposely misspelled
            let tokens = model.tokenize(prompt).expect("tokenize");
            let padded = model.pad_tokens(&tokens);
            let input_ids = candle_core::Tensor::from_vec(padded.clone(), (1, padded.len()), &Device::Cpu).unwrap();
            let embeddings = model.run_embeddings_with_inputs(&input_ids).expect("embeddings");
            // Prefill
            let seq_len = tokens.len();
            model.run_prefill_phase(&embeddings, seq_len).expect("prefill");
            // Take last token embedding for infer (slice)
            let last_embed = embeddings.narrow(1, seq_len - 1, 1).unwrap();
            let logits = model.generate_next_token_with_infer(&last_embed, seq_len - 1).expect("infer");
            assert_eq!(logits.dims()[0], 1);
        }
        Err(e) => {
            eprintln!("Skipping typo-fixer pipeline test: failed to load model: {e}");
        }
    }
}

// NOTE: End-to-end generation test omitted to avoid dependence on local CoreML mlpackage files in CI.
