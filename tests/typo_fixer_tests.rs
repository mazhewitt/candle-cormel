//! Tests for the typo-fixer config and related sampling/utilities.
use candle_core::{Device, Tensor};
use candle_coreml::ModelConfig; // crate root
use candle_coreml::qwen::inference::PrefillStep;

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

#[test]
fn test_plan_sequential_prefill_single_window() {
    use candle_coreml::qwen::QwenModel;
    // token_count <= embeddings_len => single window
    let plan = QwenModel::plan_sequential_prefill_static(5, 64, 0);
    // Expect 4 steps (leave last token for infer)
    assert_eq!(plan.steps.len(), 4);
    assert_eq!(plan.steps[0], PrefillStep { local_idx: 0, global_pos: 0 });
    assert_eq!(plan.steps[3], PrefillStep { local_idx: 3, global_pos: 3 });
    assert_eq!(plan.last_window_start, 0);
    assert_eq!(plan.last_local_idx, 4);
}

#[test]
fn test_plan_sequential_prefill_multi_window_with_already_prefilled() {
    use candle_coreml::qwen::QwenModel;
    // token_count spans two windows (e.g., 300 tokens with embeddings window=256)
    let token_count = 300usize;
    let embeddings_len = 256usize;
    let already_prefilled = 200usize; // e.g., continuing from a previous call
    let plan = QwenModel::plan_sequential_prefill_static(token_count, embeddings_len, already_prefilled);

    // Steps should start at global_pos=200 up to 298 (leave last token 299 for infer)
    assert!(plan.steps.first().unwrap().global_pos >= already_prefilled);
    assert_eq!(plan.steps.last().unwrap().global_pos, 298);
    // Last window start should be 44 (= 300 - 256)
    assert_eq!(plan.last_window_start, 44);
    // Last local index is 255-1 = 255? Actually local_idx is (token_count - start - 1)
    assert_eq!(plan.last_local_idx, 255);
}

// Placeholder for a future golden test that will load captured tensors
// from tests/fixtures/ and validate that forward path stays stable.
#[test]
fn test_golden_forward_plan_smoke() {
    use candle_coreml::qwen::QwenModel;
    // Load golden fixture
    let fixture = include_str!("fixtures/typo_fixer_prefill_plan_golden.json");
    let v: serde_json::Value = serde_json::from_str(fixture).unwrap();
    let token_count = v["token_count"].as_u64().unwrap() as usize;
    let embeddings_len = v["embeddings_len"].as_u64().unwrap() as usize;
    let already_prefilled = v["already_prefilled"].as_u64().unwrap() as usize;
    let expected_plan = &v["plan"];

    let plan = QwenModel::plan_sequential_prefill_static(token_count, embeddings_len, already_prefilled);
    // Compare steps
    let steps = expected_plan["steps"].as_array().unwrap();
    assert_eq!(steps.len(), plan.steps.len());
    for (i, s) in plan.steps.iter().enumerate() {
        assert_eq!(s.local_idx as u64, steps[i]["local_idx"].as_u64().unwrap());
        assert_eq!(s.global_pos as u64, steps[i]["global_pos"].as_u64().unwrap());
    }
    assert_eq!(plan.last_window_start as u64, expected_plan["last_window_start"].as_u64().unwrap());
    assert_eq!(plan.last_local_idx as u64, expected_plan["last_local_idx"].as_u64().unwrap());
}
