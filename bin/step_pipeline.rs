// Generic step-by-step CoreML pipeline demo for flex models
//
// This binary exercises the candle-coreml Qwen pipeline in distinct steps:
// 1) Load config + model
// 2) Tokenize input text
// 3) Build embeddings input and run embeddings
// 4) Run prefill (chunked or full-sequence depending on config)
// 5) Build infer inputs (single-token) and run FFN infer
// 6) Run LM head and print top predictions
//
// It prints tensor shapes at each step and decodes the top-5 tokens to help diagnose shape mismatches.

use std::env;
use std::path::PathBuf;

use candle_core::Tensor;
use candle_coreml::{ModelConfig, QwenConfig, QwenModel};

#[cfg(target_os = "macos")]
fn main() -> anyhow::Result<()> {
    // Simple args: --config <file.json> --model-dir <path> --text <prompt>
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    let mut config_path: Option<PathBuf> = None;
    let mut model_dir: Option<PathBuf> = None;
    let mut text: Option<String> = None;

    while let Some(arg) = args.first().cloned() {
        args.remove(0);
        match arg.as_str() {
            "--config" => {
                if let Some(v) = args.first().cloned() {
                    args.remove(0);
                    config_path = Some(PathBuf::from(v));
                }
            }
            "--model-dir" => {
                if let Some(v) = args.first().cloned() {
                    args.remove(0);
                    model_dir = Some(PathBuf::from(v));
                }
            }
            "--text" => {
                if let Some(v) = args.first().cloned() {
                    args.remove(0);
                    text = Some(v);
                }
            }
            _ => {
                // Allow positional: <model-dir> <config> <text>
                if model_dir.is_none() {
                    model_dir = Some(PathBuf::from(arg));
                } else if config_path.is_none() {
                    config_path = Some(PathBuf::from(arg));
                } else if text.is_none() {
                    text = Some(arg);
                }
            }
        }
    }

    if model_dir.is_none() || config_path.is_none() || text.is_none() {
        eprintln!("Usage: step_pipeline --model-dir <dir> --config <config.json> --text <prompt>\n       step_pipeline <model-dir> <config.json> <prompt>");
        std::process::exit(2);
    }

    let model_dir = model_dir.unwrap();
    let config_path = config_path.unwrap();
    let text = text.unwrap();

    println!("🔧 Loading ModelConfig: {}", config_path.display());
    let mut model_config = ModelConfig::load_from_file(&config_path)?;

    // If component file_paths are not absolute, resolve relative to model_dir
    for (name, comp) in model_config.components.iter_mut() {
        if let Some(fp) = &comp.file_path {
            let p = PathBuf::from(fp);
            if !p.exists() {
                if let Some(fname) = p.file_name() {
                    let candidate = model_dir.join(fname);
                    if candidate.exists() {
                        println!("🔄 Updated {} component path -> {}", name, candidate.display());
                        comp.file_path = Some(candidate.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    let qwen_config = QwenConfig::from_model_config(model_config.clone());

    println!("📦 Loading QwenModel from {}", model_dir.display());
    let mut model = QwenModel::load_from_directory(&model_dir, Some(qwen_config))?;

    println!("🚀 Initializing shared state and causal mask");
    model.initialize_states()?;

    println!("📝 Tokenizing input text: {:?}", text);
    let tokens = model.tokenize(&text)?;
    println!("   → token count: {}", tokens.len());

    println!("📦 Building embeddings input tensor…");
    let emb_input = model.create_embeddings_input_tensor(&tokens)?;
    println!("   shape: {:?}", emb_input.dims());

    println!("🧮 Running embeddings…");
    let embeddings = model.run_embeddings_with_inputs(&emb_input)?;
    println!("   embeddings shape: {:?}", embeddings.dims());

    println!("🧱 Running prefill (mode depends on config)…");
    model.run_chatpy_prefill(&tokens, tokens.len())?;
    println!("   ✅ Prefill complete");

    println!("🎯 Preparing single-token inputs for infer…");
    let last_idx = tokens.len().checked_sub(1).ok_or_else(|| anyhow::anyhow!("Empty input"))?;
    let hidden_states = model.get_infer_hidden_states(&tokens, tokens.len())?;
    println!("   hidden_states shape: {:?}", hidden_states.dims());

    let position_ids = model
        .config()
        .create_position_ids_with_mode_detection(&[last_idx as i64], false)?;
    println!("   position_ids shape: {:?}", position_ids.dims());

    let causal_mask = model
        .config()
        .create_causal_mask_with_mode_detection(last_idx, model.config().context_length(), false)?;
    println!("   causal_mask shape: {:?}", causal_mask.dims());

    let current_pos = position_ids.clone();

    println!("🧠 Running FFN infer…");
    let infer_out = model.run_ffn_infer_with_inputs(
        &hidden_states,
        &position_ids,
        &causal_mask,
        &current_pos,
    )?;
    println!("   infer hidden_states shape: {:?}", infer_out.dims());

    println!("🗣️  Running LM head…");
    let logits = model.run_lm_head_with_inputs(&infer_out)?;
    println!("   logits shape: {:?}", logits.dims());

    // Extract and print top-5 tokens
    let flat = logits.squeeze(0)?.squeeze(0)?; // [vocab]
    let scores = flat.to_vec1::<f32>()?;
    let mut idx_scores: Vec<(usize, f32)> = scores.iter().copied().enumerate().collect();
    idx_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("🏁 Top-5 predictions:");
    for (rank, (token_id, score)) in idx_scores.iter().take(5).enumerate() {
        let decoded = model
            .tokenizer()
            .decode(&[*token_id as u32], false)
            .unwrap_or_else(|_| "<?>".to_string());
        println!("  {}. id={}  score={:.6}  tok='{}'", rank + 1, token_id, score, decoded);
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This demo requires macOS/CoreML.");
}
