//! Step-by-step tests for the Typo Fixer pipeline using corrected fixtures.
//!
//! These tests assert each stage of the pipeline against the JSON snapshots in
//! `tests/fixtures/flex_pipeline`. They only use candle-coreml public APIs and
//! will skip gracefully if the CoreML model/config aren't available.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

#[cfg(target_os = "macos")]
use candle_coreml::{ModelConfig, QwenConfig, QwenModel};

#[cfg(target_os = "macos")]
fn adjust_component_paths(model_config: &mut ModelConfig, base_dir: &Path) {
    for (_name, comp) in model_config.components.iter_mut() {
        if let Some(fp) = &comp.file_path {
            let p = PathBuf::from(fp);
            if !p.exists() {
                if let Some(fname) = p.file_name() {
                    let candidate = base_dir.join(fname);
                    if candidate.exists() {
                        comp.file_path = Some(candidate.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn load_fixture(path: &str) -> anyhow::Result<Value> {
    let s = fs::read_to_string(path)?;
    let v: Value = serde_json::from_str(&s)?;
    Ok(v)
}

#[cfg(target_os = "macos")]
fn load_fixture_lenient(path: &str) -> anyhow::Result<Value> {
    // Replace non-finite JSON tokens with large finite sentinels so serde_json can parse
    let mut s = fs::read_to_string(path)?;
    s = s.replace("-Infinity", "-1.0e38");
    s = s.replace("Infinity", "1.0e38");
    s = s.replace("NaN", "0.0");
    let v: Value = serde_json::from_str(&s)?;
    Ok(v)
}

#[cfg(target_os = "macos")]
fn fixtures_root() -> PathBuf {
    PathBuf::from("tests/fixtures/flex_pipeline")
}

#[cfg(target_os = "macos")]
fn try_load_model_from_env_or_fixtures(
    prompt: &mut String,
) -> anyhow::Result<Option<(QwenModel, ModelConfig)>> {
    use std::env;
    let env_model_dir = env::var_os("FLEX_MODEL_DIR").map(PathBuf::from);
    let env_config_path = env::var_os("FLEX_CONFIG_PATH").map(PathBuf::from);

    let fx = fixtures_root();
    // We expect a config.json and a model/ directory if using fixtures (optional)
    let default_model_dir = fx.join("model");
    let default_config_path = fx.join("config.json");
    // CLI config with absolute model paths
    let cli_config_path = PathBuf::from("typo-fixer-cli/configs/qwen-typo-fixer-ane.json");

    // Optionally load the prompt from the step-1 fixture if present
    let step1 = fx.join("corrected_step_1_tokens.json");
    if step1.exists() {
        if let Ok(v) = load_fixture(step1.to_string_lossy().as_ref()) {
            if let Some(p) = v["metadata"]["prompt"].as_str() {
                *prompt = p.to_string();
            }
        }
    }

    let (model_dir, config_path) = match (env_model_dir, env_config_path) {
        (Some(m), Some(c)) => (m, c),
        _ => {
            if cli_config_path.is_file() {
                let s = fs::read_to_string(&cli_config_path)?;
                let v: Value = serde_json::from_str(&s)?;
                if let Some(model_path) = v["model_info"]["path"].as_str() {
                    let mdir = PathBuf::from(model_path);
                    if mdir.is_dir() {
                        (mdir, cli_config_path)
                    } else if default_model_dir.is_dir() && default_config_path.is_file() {
                        (default_model_dir, default_config_path)
                    } else {
                        eprintln!(
                            "[typo-fixer] CLI model path not found, and no fixtures; skipping."
                        );
                        return Ok(None);
                    }
                } else {
                    (default_model_dir, default_config_path)
                }
            } else if default_model_dir.is_dir() && default_config_path.is_file() {
                (default_model_dir, default_config_path)
            } else {
                eprintln!(
					"[typo-fixer] FLEX_MODEL_DIR/FLEX_CONFIG_PATH not set and no fixtures model/config found, skipping."
				);
                return Ok(None);
            }
        }
    };

    let mut model_config = ModelConfig::load_from_file(&config_path)?;
    adjust_component_paths(&mut model_config, &model_dir);
    let qwen_config = QwenConfig::from_model_config(model_config.clone());
    let mut model = QwenModel::load_from_directory(&model_dir, Some(qwen_config))?;
    model.initialize_states()?;
    Ok(Some((model, model_config)))
}

#[cfg(target_os = "macos")]
#[test]
fn test_step5_prefill_lm_head_matches_fixture() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    let step5_path = fx.join("corrected_step_5_infer_and_logits.json");
    if !step1_path.exists() || !step5_path.exists() {
        eprintln!("[typo-fixer] Missing step-1/5 fixtures, skipping");
        return Ok(());
    }
    let v1 = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let v5 = load_fixture(step5_path.to_string_lossy().as_ref())?;
    let mut prompt = v1["metadata"]["prompt"].as_str().unwrap_or("").to_string();

    let Some((mut model, _model_config)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };

    let tokens: Vec<i64> = v1["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    let context_pos = v1["data"]["context_pos"].as_u64().unwrap_or(0) as usize;

    // Full-sequence prefill to get last hidden state
    let embeddings = model.compute_embeddings(&tokens)?;
    let position_ids = model.create_position_tensor((0..128i64).collect())?;
    let causal = model.config().create_causal_mask_with_mode_detection(
        0,
        model.config().context_length(),
        true,
    )?;
    use candle_core::Tensor;
    let current_pos_t =
        Tensor::from_vec(vec![(context_pos - 1) as i64], (1,), &model.config().device)?;
    model.initialize_states()?;
    let prefill_last =
        model.run_ffn_prefill_with_inputs(&embeddings, &position_ids, &causal, &current_pos_t)?;

    // Run LM head directly on prefill output
    let logits = model.run_lm_head_with_inputs(&prefill_last)?;
    let flat = logits.squeeze(0)?.squeeze(0)?;
    let scores = flat.to_vec1::<f32>()?;
    let (best_idx, _best) = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let expected_token_text = v5["data"]["top_predictions"]["tokens"][0]
        .as_str()
        .unwrap_or(".");
    let decoded = model
        .tokenizer()
        .decode(&[best_idx as u32], false)
        .unwrap_or_default();
    if decoded != expected_token_text {
        eprintln!(
            "[step5-prefill-lm] mismatch: got '{decoded}' (id={best_idx}), expected '{expected_token_text}'"
        );
        anyhow::bail!("prefill->lm_head path mismatch")
    }
    Ok(())
}
#[allow(dead_code)]
fn test_step5_infer_top1_matches_fixture_using_prefill_output() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    let step5_path = fx.join("corrected_step_5_infer_and_logits.json");
    if !step1_path.exists() || !step5_path.exists() {
        eprintln!("[typo-fixer] Missing step-1/5 fixtures, skipping");
        return Ok(());
    }
    let v1 = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let v5 = load_fixture(step5_path.to_string_lossy().as_ref())?;
    let mut prompt = v1["metadata"]["prompt"].as_str().unwrap_or("").to_string();

    let Some((mut model, _model_config)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };

    let tokens: Vec<i64> = v1["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    let context_pos = v1["data"]["context_pos"].as_u64().unwrap_or(0) as usize;

    // Build full-sequence prefill inputs manually and CAPTURE the output hidden state
    // 1) embeddings [1, 128, 1024]
    let embeddings = model.compute_embeddings(&tokens)?;
    // 2) position_ids [128]
    let position_ids = model.create_position_tensor((0..128i64).collect())?;
    // 3) causal mask [1,1,128,context]
    let causal = model.config().create_causal_mask_with_mode_detection(
        0,
        model.config().context_length(),
        true,
    )?;
    // 4) current_pos [1]
    use candle_core::Tensor;
    let current_pos_t =
        Tensor::from_vec(vec![(context_pos - 1) as i64], (1,), &model.config().device)?;

    model.initialize_states()?; // ensure unified state exists
    let prefill_last =
        model.run_ffn_prefill_with_inputs(&embeddings, &position_ids, &causal, &current_pos_t)?; // [1,1,1024]

    // Now run infer using the prefill output hidden_state as input
    let infer_pos_ids = model
        .config()
        .create_position_ids_with_mode_detection(&[(context_pos - 1) as i64], false)?;
    let infer_mask = model.config().create_causal_mask_with_mode_detection(
        context_pos - 1,
        model.config().context_length(),
        false,
    )?;
    let infer_out = model.run_ffn_infer_with_inputs(
        &prefill_last,
        &infer_pos_ids,
        &infer_mask,
        &infer_pos_ids,
    )?;
    let logits = model.run_lm_head_with_inputs(&infer_out)?;

    let flat = logits.squeeze(0)?.squeeze(0)?; // [vocab]
    let scores = flat.to_vec1::<f32>()?;
    let (best_idx, _best) = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    let expected_token_text = v5["data"]["top_predictions"]["tokens"][0]
        .as_str()
        .unwrap_or(".");
    let decoded = model
        .tokenizer()
        .decode(&[best_idx as u32], false)
        .unwrap_or_default();
    if decoded != expected_token_text {
        eprintln!(
            "[step5-prefill-out] mismatch: got '{decoded}' (id={best_idx}), expected '{expected_token_text}'"
        );
        anyhow::bail!("prefill-output infer path mismatch")
    }
    Ok(())
}
#[allow(dead_code)]
fn test_step1_tokenize_matches_fixture() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    if !step1_path.exists() {
        eprintln!("[typo-fixer] No step-1 fixture found, skipping");
        return Ok(());
    }
    let v = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let mut prompt = v["metadata"]["prompt"].as_str().unwrap_or("").to_string();

    let Some((model, _cfg)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };

    let tokens = model.tokenize(&prompt)?;
    let expected: Vec<i64> = v["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    assert_eq!(tokens, expected, "Tokenization mismatch with fixture");

    let ctx_pos = v["data"]["context_pos"].as_u64().unwrap_or(0) as usize;
    assert_eq!(tokens.len(), ctx_pos, "context_pos mismatch");
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_step2_causal_mask_matches_fixture_sample() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step2_path = fx.join("corrected_step_2_causal_mask.json");
    if !step2_path.exists() {
        eprintln!("[typo-fixer] No step-2 fixture found, skipping");
        return Ok(());
    }
    // Load model/config (for mask builder) BEFORE parsing large JSON
    let mut prompt = String::new();
    let Some((model, _cfg)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };
    let v = load_fixture_lenient(step2_path.to_string_lossy().as_ref())?;

    let context_length = v["metadata"]["context_length"].as_u64().unwrap_or(256) as usize;
    let mask = model
        .config()
        .create_causal_mask_with_mode_detection(0, context_length, true)?;
    let dims = mask.dims();
    assert_eq!(dims.len(), 4);

    // Compare a couple of rows from fixture to our mask
    let fx_rows = &v["data"]["causal_mask"][0][0];

    // Row 0 sample: expect [0.0, -Inf, -Inf, ...]
    let row0_expected: Vec<f32> = fx_rows[0]
        .as_array()
        .unwrap()
        .iter()
        .take(16)
        .map(|x| {
            if let Some(f) = x.as_f64() {
                let f = f as f32;
                if f < -1.0e30f32 {
                    f32::NEG_INFINITY
                } else {
                    f
                }
            } else {
                f32::NEG_INFINITY
            }
        })
        .collect();
    let row0_actual_t = mask.narrow(2, 0, 1)?.squeeze(2)?.squeeze(0)?.squeeze(0)?; // [context]
    let mut row0_actual = row0_actual_t.to_vec1::<f32>()?;
    if row0_actual.len() > 16 {
        row0_actual.truncate(16);
    }
    assert_eq!(row0_expected.len(), row0_actual.len());
    for (e, a) in row0_expected.iter().zip(row0_actual.iter()) {
        if e.is_infinite() {
            assert!(a.is_infinite() && a.is_sign_negative());
        } else {
            assert!((e - a).abs() < 1e-6);
        }
    }

    // Row 11 sample (last context position - 1): only if our mask has >=12 rows
    if dims[2] >= 12 {
        let row11_expected: Vec<f32> = fx_rows[11]
            .as_array()
            .unwrap()
            .iter()
            .take(16)
            .map(|x| {
                if let Some(f) = x.as_f64() {
                    let f = f as f32;
                    if f < -1.0e30f32 {
                        f32::NEG_INFINITY
                    } else {
                        f
                    }
                } else {
                    f32::NEG_INFINITY
                }
            })
            .collect();
        let row11_actual_t = mask.narrow(2, 11, 1)?.squeeze(2)?.squeeze(0)?.squeeze(0)?; // [context]
        let mut row11_actual = row11_actual_t.to_vec1::<f32>()?;
        if row11_actual.len() > 16 {
            row11_actual.truncate(16);
        }
        assert_eq!(row11_expected.len(), row11_actual.len());
        for (e, a) in row11_expected.iter().zip(row11_actual.iter()) {
            if e.is_infinite() {
                assert!(a.is_infinite() && a.is_sign_negative());
            } else {
                assert!((e - a).abs() < 1e-6);
            }
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_step3_embeddings_and_padding_match_fixture_minimal() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    let step3_path = fx.join("corrected_step_3_prefill_input.json");
    if !step1_path.exists() || !step3_path.exists() {
        eprintln!("[typo-fixer] Missing step-1/3 fixtures, skipping");
        return Ok(());
    }
    // Load model/config BEFORE parsing large JSON
    let v1 = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let mut prompt = v1["metadata"]["prompt"].as_str().unwrap_or("").to_string();
    let Some((model, model_config)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };
    let v3 = load_fixture_lenient(step3_path.to_string_lossy().as_ref())?;

    let expected_tokens: Vec<i64> = v1["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();

    // Compare padded input to fixture batch_input
    let padded = model
        .pad_tokens(&expected_tokens)
        .expect("pad_tokens should succeed with valid ModelConfig shapes");
    let batch_input_fx: Vec<i64> = v3["data"]["batch_input"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    assert_eq!(padded, batch_input_fx, "Embeddings input padding mismatch");

    // Run embeddings and assert shape matches config and fixture shape
    let emb_input = model.create_embeddings_input_tensor(&expected_tokens)?;
    let embeddings = model.run_embeddings_with_inputs(&emb_input)?;
    if let Some(shape) = model_config.embeddings_output_shape() {
        assert_eq!(embeddings.dims(), &shape[..]);
    }
    let dims = embeddings.dims().to_vec();
    assert_eq!(dims.len(), 3);
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_step4_prefill_sets_state_and_infer_ready() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    if !step1_path.exists() {
        eprintln!("[typo-fixer] Missing step-1 fixture, skipping");
        return Ok(());
    }
    let v1 = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let mut prompt = v1["metadata"]["prompt"].as_str().unwrap_or("").to_string();

    let Some((mut model, _model_config)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };

    let tokens: Vec<i64> = v1["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    let context_pos = v1["data"]["context_pos"].as_u64().unwrap_or(0) as usize;

    model.run_chatpy_prefill(&tokens, context_pos)?;

    // If prefill worked, infer hidden states for last position should be available
    let hs = model.get_infer_hidden_states(&tokens, context_pos)?;
    let d = hs.dims();
    assert_eq!(d.len(), 3);
    assert_eq!(d[0], 1);
    assert_eq!(d[1], 1);
    Ok(())
}

#[cfg(target_os = "macos")]
#[test]
fn test_step5_infer_top1_matches_fixture() -> anyhow::Result<()> {
    let fx = fixtures_root();
    let step1_path = fx.join("corrected_step_1_tokens.json");
    let step5_path = fx.join("corrected_step_5_infer_and_logits.json");
    if !step1_path.exists() || !step5_path.exists() {
        eprintln!("[typo-fixer] Missing step-1/5 fixtures, skipping");
        return Ok(());
    }
    let v1 = load_fixture(step1_path.to_string_lossy().as_ref())?;
    let v5 = load_fixture(step5_path.to_string_lossy().as_ref())?;
    let mut prompt = v1["metadata"]["prompt"].as_str().unwrap_or("").to_string();

    let Some((mut model, _model_config)) = try_load_model_from_env_or_fixtures(&mut prompt)? else {
        return Ok(());
    };

    let tokens: Vec<i64> = v1["data"]["input_ids"][0]
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_i64().unwrap())
        .collect();
    let context_pos = v1["data"]["context_pos"].as_u64().unwrap_or(0) as usize;

    // Prefill then infer+lm_head
    model.run_chatpy_prefill(&tokens, context_pos)?;
    let hidden_states = model.get_infer_hidden_states(&tokens, context_pos)?;

    // Debug: Show the infer hidden states tensor info
    eprintln!(
        "[step5] RUST infer hidden_states shape: {:?}",
        hidden_states.dims()
    );
    if hidden_states.dims().len() == 2 {
        // This is expected for fresh embeddings input to infer step (single token)
        eprintln!("[step5] RUST infer hidden_states[0,0]: single token embeddings for infer");
    } else if hidden_states.dims().len() == 3 && hidden_states.dims()[2] >= 16 {
        if let Ok(rust_slice) = hidden_states.narrow(2, 0, 16.min(hidden_states.dims()[2])) {
            if let Ok(rust_values) = rust_slice.to_vec3::<f32>() {
                let rust_first_16 = &rust_values[0][0];
                eprintln!("[step5] RUST infer hidden_states[0,0,0..16]: {rust_first_16:?}");
            }
        }
    }
    let position_ids = model
        .config()
        .create_position_ids_with_mode_detection(&[(context_pos - 1) as i64], false)?;
    let causal = model.config().create_causal_mask_with_mode_detection(
        context_pos - 1,
        model.config().context_length(),
        false,
    )?;
    let current_pos = position_ids.clone();

    // Debug: Print hidden states input to infer
    eprintln!(
        "[step5] infer input hidden_states shape: {:?}",
        hidden_states.dims()
    );
    if hidden_states.dims().len() == 2 {
        // This is expected for fresh embeddings (single token [1, 1])
        eprintln!("[step5] infer input: fresh single token embeddings");
    } else if hidden_states.dims().len() == 3 && hidden_states.dims()[2] >= 16 {
        if let Ok(hs_slice) = hidden_states.narrow(2, 0, 16.min(hidden_states.dims()[2])) {
            if let Ok(values) = hs_slice.to_vec3::<f32>() {
                eprintln!(
                    "[step5] infer input hidden_states[0,0,0..16]: {:?}",
                    &values[0][0]
                );
            }
        }
    }

    let infer_out =
        model.run_ffn_infer_with_inputs(&hidden_states, &position_ids, &causal, &current_pos)?;

    // Debug: Print infer output hidden states
    eprintln!(
        "[step5] infer output hidden_states shape: {:?}",
        infer_out.dims()
    );
    if infer_out.dims().len() == 3 && infer_out.dims()[2] >= 16 {
        if let Ok(out_slice) = infer_out.narrow(2, 0, 16.min(infer_out.dims()[2])) {
            if let Ok(values) = out_slice.to_vec3::<f32>() {
                eprintln!(
                    "[step5] infer output hidden_states[0,0,0..16]: {:?}",
                    &values[0][0]
                );
            }
        }
    } else {
        eprintln!(
            "[step5] infer output: unexpected shape or insufficient dimensions for debug display"
        );
    }

    // Compare with fixture infer output hidden states
    if let Some(fixture_infer_output) = v5["data"]["infer_output_hidden_states"].as_array() {
        if let Some(fixture_batch) = fixture_infer_output[0].as_array() {
            if let Some(fixture_seq) = fixture_batch[0].as_array() {
                let fixture_values: Vec<f32> = fixture_seq
                    .iter()
                    .take(16)
                    .filter_map(|v| v.as_f64().map(|f| f as f32))
                    .collect();
                if infer_out.dims().len() == 3 && infer_out.dims()[2] >= 16 {
                    if let Ok(rust_slice) = infer_out.narrow(2, 0, 16.min(infer_out.dims()[2])) {
                        if let Ok(rust_values) = rust_slice.to_vec3::<f32>() {
                            let rust_first_16 = &rust_values[0][0];
                            eprintln!(
                                "[step5] FIXTURE infer output[0,0,0..16]: {:?}",
                                &fixture_values
                            );
                            eprintln!("[step5] RUST    infer output[0,0,0..16]: {rust_first_16:?}");

                            // Check if they match within tolerance
                            let matches = fixture_values
                                .iter()
                                .zip(rust_first_16.iter())
                                .all(|(f, r)| (f - r).abs() < 0.1);
                            eprintln!("[step5] First 16 hidden states match fixture: {matches}");
                        }
                    }
                }
            }
        }
    }
    // Also inspect raw multipart outputs before combining
    let raw_outputs = model.lm_head.forward_all(&[&infer_out])?;
    // Compute chunk sizes and cumulative boundaries for debugging
    let mut chunk_sizes: Vec<usize> = Vec::new();
    let mut total_vocab = 0usize;
    for i in 1..=model.config().logits_part_count() {
        let key = format!("logits{i}");
        if let Some(t) = raw_outputs.get(&key) {
            let d = t.dims();
            if d.len() == 3 {
                let sz = d[2];
                chunk_sizes.push(sz);
                total_vocab += sz;
            }
        }
    }
    eprintln!(
        "[step5] lm_head chunk sizes = {:?} (sum = {}), model vocab_size = {}",
        chunk_sizes,
        total_vocab,
        model.config().vocab_size()
    );

    // Combine logits using the same path as runtime
    let logits = model.combine_lm_head_outputs(raw_outputs)?;

    let flat = logits.squeeze(0)?.squeeze(0)?; // [vocab]
    let scores = flat.to_vec1::<f32>()?;
    // Debug: show top-5 predictions (id, text, score)
    {
        let mut indexed: Vec<(usize, f32)> =
            scores.iter().enumerate().map(|(i, &s)| (i, s)).collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let top5 = indexed.iter().take(5).cloned().collect::<Vec<_>>();
        eprintln!("[typo-fixer][step5] top-5:");
        for (i, (idx, sc)) in top5.iter().enumerate() {
            let txt = model
                .tokenizer()
                .decode(&[*idx as u32], false)
                .unwrap_or_default();
            eprintln!("  {}. id={} text='{}' score={}", i + 1, idx, txt, sc);
        }
    }
    // Also inspect the fixture's expected token id (13) to see its score/rank
    if scores.len() > 13 {
        let dot_score = scores[13];
        let mut rank = 1usize;
        for s in &scores {
            if *s > dot_score {
                rank += 1;
            }
        }
        eprintln!(
            "[typo-fixer][step5] fixture '.' id=13 score={} rank={} of {}",
            dot_score,
            rank,
            scores.len()
        );
    }
    let (best_idx, _best) = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();

    // Prefer comparing decoded token text to avoid tokenizer ID differences
    let expected_token_text = v5["data"]["top_predictions"]["tokens"][0]
        .as_str()
        .unwrap_or(".");

    let decoded = model
        .tokenizer()
        .decode(&[best_idx as u32], false)
        .unwrap_or_default();

    // Print top-5 predictions for diagnosis
    let mut indexed: Vec<(usize, f32)> = scores.iter().enumerate().map(|(i, &s)| (i, s)).collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let top_k = 5.min(indexed.len());
    eprintln!("[step5] top-{top_k} predictions:");
    for (rank, (tid, sc)) in indexed.iter().take(top_k).enumerate() {
        let txt = model
            .tokenizer()
            .decode(&[*tid as u32], false)
            .unwrap_or_default();
        eprintln!("  {}. id={} text='{}' score={:.6}", rank + 1, tid, txt, sc);
    }

    if decoded != expected_token_text {
        // Also print the fixture's top-5 (if available) for context
        if let Some(arr) = v5["data"]["top_predictions"]["tokens"].as_array() {
            let fx_tokens: Vec<String> = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            eprintln!("[step5] fixture top tokens: {fx_tokens:?}");
        }
        anyhow::bail!(
            "Top-1 decoded token mismatch: got '{}' (id={}), expected '{}'",
            decoded,
            best_idx,
            expected_token_text
        );
    }
    Ok(())
}
