//! Integration tests for the generic, step-by-step CoreML pipeline.
//!
//! These tests validate the flex model pipeline using candle-coreml public APIs only.
//! No typo-fixer specific logic is used here.
//!
//! Provide environment variables to run full end-to-end checks:
//!   - FLEX_MODEL_DIR: path to directory containing CoreML components (or a coreml/ subdir)
//!   - FLEX_CONFIG_PATH: path to ModelConfig JSON that describes shapes and component files
//!   - FLEX_TEXT (optional): input text; default "Fix typos in this sentance."
//!
//! Example:
//!   FLEX_MODEL_DIR=/path/to/model \
//!   FLEX_CONFIG_PATH=/path/to/config.json \
//!   cargo test --tests flex_pipeline -- --nocapture

use std::env;
use std::path::{Path, PathBuf};

use candle_coreml::{ModelConfig, QwenConfig, QwenModel};

fn get_env_path(key: &str) -> Option<PathBuf> {
    env::var_os(key).map(PathBuf::from)
}

fn get_env_string(key: &str) -> Option<String> {
    env::var(key).ok()
}

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
fn try_load_from_env() -> anyhow::Result<Option<(QwenModel, ModelConfig, String)>> {
    // 1) Prefer explicit environment variables
    let env_model_dir = get_env_path("FLEX_MODEL_DIR");
    let env_config_path = get_env_path("FLEX_CONFIG_PATH");

    // 2) Fallback to provided typo-fixer CLI config (absolute model paths)
    let cli_config_path = PathBuf::from("typo-fixer-cli/configs/qwen-typo-fixer-ane.json");
    // 3) Fallback to repo fixtures if env is not set
    let fixtures_root = PathBuf::from("tests/fixtures/flex_pipeline");
    let default_model_dir = fixtures_root.join("model");
    let default_config_path = fixtures_root.join("config.json");

    let (model_dir, config_path) = match (env_model_dir, env_config_path) {
        (Some(m), Some(c)) => (m, c),
        _ => {
            if cli_config_path.is_file() {
                let s = std::fs::read_to_string(&cli_config_path)?;
                let v: serde_json::Value = serde_json::from_str(&s)?;
                let model_path = v["model_info"]["path"].as_str().unwrap_or("");
                let mdir = PathBuf::from(model_path);
                if mdir.is_dir() {
                    eprintln!(
                        "[flex-pipeline] Using CLI config: {} with model {}",
                        cli_config_path.display(),
                        mdir.display()
                    );
                    (mdir, cli_config_path)
                } else {
                    (default_model_dir.clone(), default_config_path.clone())
                }
            } else if default_model_dir.is_dir() && default_config_path.is_file() {
                eprintln!(
                    "[flex-pipeline] Using default fixtures: {} and {}",
                    default_model_dir.display(),
                    default_config_path.display()
                );
                (default_model_dir, default_config_path)
            } else {
                eprintln!("[flex-pipeline] FLEX_MODEL_DIR/FLEX_CONFIG_PATH not set and no fixtures found, skipping E2E test");
                return Ok(None);
            }
        }
    };

    let text =
        get_env_string("FLEX_TEXT").unwrap_or_else(|| "Fix typos in this sentance.".to_string());

    let mut model_config = ModelConfig::load_from_file(&config_path)?;
    adjust_component_paths(&mut model_config, &model_dir);
    let qwen_config = QwenConfig::from_model_config(model_config.clone());
    let mut model = QwenModel::load_from_directory(&model_dir, Some(qwen_config))?;
    model.initialize_states()?;
    Ok(Some((model, model_config, text)))
}

#[cfg(target_os = "macos")]
#[test]
fn test_flex_pipeline_shapes_and_infer() -> anyhow::Result<()> {
    let Some((mut model, model_config, text)) = try_load_from_env()? else {
        return Ok(()); // skipped
    };

    // Tokenize
    let tokens = model.tokenize(&text)?;
    assert!(!tokens.is_empty(), "Tokenization produced no tokens");
    assert!(
        tokens.len() <= model.config().context_length(),
        "Token count {} exceeds context length {}",
        tokens.len(),
        model.config().context_length()
    );

    // Embeddings input shape
    let emb_input = model.create_embeddings_input_tensor(&tokens)?;
    if let Some(shape) = model_config.embeddings_input_shape() {
        assert_eq!(
            emb_input.dims(),
            &shape[..],
            "Embeddings input dims mismatch"
        );
    } else {
        // Fallback: expect 2-D [1, seq]
        let dims = emb_input.dims();
        assert_eq!(dims.len(), 2);
        assert_eq!(dims[0], 1);
    }

    // Embeddings run -> check output
    let embeddings = model.run_embeddings_with_inputs(&emb_input)?;
    if let Some(shape) = model_config.embeddings_output_shape() {
        assert_eq!(
            embeddings.dims(),
            &shape[..],
            "Embeddings output dims mismatch"
        );
    } else {
        // Fallback: expect [1, seq, hidden]
        let dims = embeddings.dims();
        assert_eq!(dims.len(), 3);
        assert_eq!(dims[0], 1);
        assert_eq!(dims[2], model.config().hidden_size());
    }

    // Prefill phase
    model.run_chatpy_prefill(&tokens, tokens.len())?;

    // Infer hidden states
    let hidden_states = model.get_infer_hidden_states(&tokens, tokens.len())?;
    // Should match LM head input shape
    if let Some(hs_shape) = model_config.lm_head_input_shape() {
        assert_eq!(
            hidden_states.dims(),
            &hs_shape[..],
            "Infer hidden_states dims mismatch"
        );
    } else {
        let dims = hidden_states.dims();
        assert_eq!(dims.len(), 3);
        assert_eq!(dims[0], 1);
        assert_eq!(dims[1], 1);
        assert_eq!(dims[2], model.config().hidden_size());
    }

    // Position ids and causal mask for infer
    let last_idx = tokens.len() - 1;
    let pos_ids = model
        .config()
        .create_position_ids_with_mode_detection(&[last_idx as i64], false)?;
    // If infer position_ids shape exists, assert it; else minimally assert 1-D
    if let Some(s) = model_config.get_tensor_shape("ffn_infer", "position_ids", true) {
        assert_eq!(pos_ids.dims(), &s[..], "position_ids dims mismatch");
    } else {
        let dims = pos_ids.dims();
        assert_eq!(dims.len(), 1);
        assert_eq!(dims[0], 1);
    }

    let causal = model.config().create_causal_mask_with_mode_detection(
        last_idx,
        model.config().context_length(),
        false,
    )?;
    let c_dims = causal.dims();
    assert_eq!(c_dims.len(), 4, "causal_mask should be 4-D, got {c_dims:?}");

    // Infer run
    let current_pos = pos_ids.clone();
    let infer_out =
        model.run_ffn_infer_with_inputs(&hidden_states, &pos_ids, &causal, &current_pos)?;
    // Should match LM head input shape
    if let Some(hs_shape) = model_config.lm_head_input_shape() {
        assert_eq!(
            infer_out.dims(),
            &hs_shape[..],
            "FFN infer output dims mismatch"
        );
    } else {
        let dims = infer_out.dims();
        assert_eq!(dims.len(), 3);
        assert_eq!(dims[0], 1);
        assert_eq!(dims[1], 1);
        assert_eq!(dims[2], model.config().hidden_size());
    }

    // LM head logits
    let logits = model.run_lm_head_with_inputs(&infer_out)?;
    let l_dims = logits.dims();
    assert_eq!(l_dims.len(), 3);
    assert_eq!(l_dims[0], 1);
    assert_eq!(l_dims[1], 1);
    assert_eq!(l_dims[2], model.config().vocab_size());

    // Basic content sanity: get top-1 token id from logits
    let flat = logits.squeeze(0)?.squeeze(0)?; // [vocab]
    let scores = flat.to_vec1::<f32>()?;
    assert_eq!(scores.len(), model.config().vocab_size());
    let (best_idx, _best_score) = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .expect("non-empty logits");
    assert!(best_idx < model.config().vocab_size());
    Ok(())
}

// Platform-independent unit test for sequential prefill planning logic
#[test]
fn test_sequential_prefill_plan_static() {
    // token_count=100, embeddings_len=64, already_prefilled=0
    let plan = QwenModel::plan_sequential_prefill_static(100, 64, 0);
    assert!(!plan.steps.is_empty());
    assert!(plan.last_window_start <= 100);
    assert!(plan.last_local_idx < 64);

    // If some tokens already prefetched, ensure plan starts later
    let plan2 = QwenModel::plan_sequential_prefill_static(100, 64, 50);
    assert!(plan2
        .steps
        .iter()
        .all(|s| s.global_pos >= 50 && s.global_pos < 99));
}
