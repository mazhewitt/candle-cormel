//! Qwen 0.6B Interactive Chat with ANE Acceleration
//!
//! This example demonstrates real-time chat using Anemll's ANE-optimized Qwen 0.6B model
//! with streaming token generation powered by our CoreML inference engine.
//!
//! Features:
//! - ANE-accelerated inference for maximum performance
//! - Streaming generation with persistent KV-cache
//! - Real HuggingFace tokenizer integration
//! - Temperature and sampling controls
//! - Automatic model and tokenizer download
//!
//! Usage:
//! ```bash
//! # Basic chat
//! cargo run --example qwen_chat
//!
//! # Custom settings
//! cargo run --example qwen_chat -- --temperature 0.8 --max-tokens 100
//!
//! # Use specific model variant
//! cargo run --example qwen_chat -- --model-id "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_coreml::{get_local_or_remote_file, Config as CoreMLConfig, CoreMLModel};
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const MAX_SEQUENCE_LENGTH: usize = 512;
const EOS_TOKEN_ID: i64 = 151645;
const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_MAX_TOKENS: usize = 50;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(long, default_value = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")]
    model_id: String,

    /// Model revision (branch/tag)
    #[arg(long, default_value = "main")]
    revision: String,

    /// Temperature for sampling (0.0 = deterministic, 1.0 = very random)
    #[arg(long, default_value_t = DEFAULT_TEMPERATURE)]
    temperature: f32,

    /// Maximum tokens to generate per message
    #[arg(long, default_value_t = DEFAULT_MAX_TOKENS)]
    max_tokens: usize,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Use local model instead of downloading
    #[arg(long)]
    local: bool,

    /// Path to local model directory
    #[arg(long)]
    model_path: Option<String>,
}

struct QwenModel {
    model: CoreMLModel,
    tokenizer: Tokenizer,
    config: QwenConfig,
}

struct QwenConfig {
    temperature: f32,
    max_tokens: usize,
    vocab_size: usize,
    max_sequence_length: usize,
}

impl QwenModel {
    fn new(model: CoreMLModel, tokenizer: Tokenizer, temperature: f32, max_tokens: usize) -> Self {
        let config = QwenConfig {
            temperature,
            max_tokens,
            vocab_size: QWEN_VOCAB_SIZE,
            max_sequence_length: MAX_SEQUENCE_LENGTH,
        };

        Self {
            model,
            tokenizer,
            config,
        }
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))?;

        let tokens: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();

        if tokens.len() > self.config.max_sequence_length {
            return Err(E::msg(format!(
                "Input too long: {} tokens, max: {}",
                tokens.len(),
                self.config.max_sequence_length
            )));
        }

        Ok(tokens)
    }

    fn detokenize(&self, tokens: &[i64]) -> Result<String> {
        let ids: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
        self.tokenizer
            .decode(&ids, true)
            .map_err(|e| E::msg(format!("Detokenization failed: {}", e)))
    }

    fn generate_streaming(&self, prompt: &str) -> Result<String> {
        let tokens = self.tokenize(prompt)?;
        let device = Device::Cpu;

        // Create state for efficient streaming generation
        let mut state = self
            .model
            .make_state()
            .map_err(|e| E::msg(format!("Failed to create model state: {}", e)))?;

        let mut generated_tokens = tokens.clone();
        let mut generated_text = String::new();

        print!("🤖 ");
        io::stdout().flush().unwrap();

        for step in 0..self.config.max_tokens {
            // Prepare input tensor (single token for continuation)
            let input_len = if step == 0 { tokens.len() } else { 1 };
            let start_idx = generated_tokens.len() - input_len;
            let input_tokens = &generated_tokens[start_idx..];

            let input_tensor = Tensor::from_vec(input_tokens.to_vec(), (1, input_len), &device)?;

            // Run inference with state
            let start_time = Instant::now();
            let output = self
                .model
                .predict_with_state(&[&input_tensor], &mut state)
                .map_err(|e| E::msg(format!("Inference failed at step {}: {}", step, e)))?;
            let inference_time = start_time.elapsed();

            // Get logits and sample next token
            let logits = output.to_vec2::<f32>()?;
            if logits.is_empty() || logits[0].len() != self.config.vocab_size {
                return Err(E::msg(format!(
                    "Unexpected output shape: expected ({}, {}), got ({}, {})",
                    1,
                    self.config.vocab_size,
                    logits.len(),
                    logits.first().map_or(0, |row| row.len())
                )));
            }

            let next_token = self.sample_token(&logits[0])?;

            // Check for end of sequence
            if next_token == EOS_TOKEN_ID {
                break;
            }

            generated_tokens.push(next_token);

            // Decode and display the new token
            if let Ok(token_text) = self.detokenize(&[next_token]) {
                print!("{}", token_text);
                io::stdout().flush().unwrap();
                generated_text.push_str(&token_text);
            }

            if step % 10 == 0 && step > 0 {
                if let Some(duration_ms) = inference_time.as_millis().checked_sub(0) {
                    if duration_ms > 100 {
                        // Only show if inference takes notable time
                        print!(" [{}ms]", duration_ms);
                        io::stdout().flush().unwrap();
                    }
                }
            }
        }

        println!(); // New line after generation
        Ok(generated_text)
    }

    fn sample_token(&self, logits: &[f32]) -> Result<i64> {
        if logits.len() != self.config.vocab_size {
            return Err(E::msg(format!(
                "Invalid logits size: {} != {}",
                logits.len(),
                self.config.vocab_size
            )));
        }

        if self.config.temperature == 0.0 {
            // Greedy sampling
            let (best_idx, _) = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .ok_or_else(|| E::msg("Empty logits"))?;
            return Ok(best_idx as i64);
        }

        // Temperature sampling
        let scaled_logits: Vec<f32> = logits
            .iter()
            .map(|&x| x / self.config.temperature)
            .collect();

        // Apply softmax
        let max_logit = scaled_logits
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

        let exp_logits: Vec<f32> = scaled_logits
            .iter()
            .map(|&x| (x - max_logit).exp())
            .collect();

        let sum: f32 = exp_logits.iter().sum();
        let probabilities: Vec<f32> = exp_logits.iter().map(|&x| x / sum).collect();

        // Sample from distribution
        let random_value: f32 = rand::random();
        let mut cumsum = 0.0;

        for (idx, &prob) in probabilities.iter().enumerate() {
            cumsum += prob;
            if random_value <= cumsum {
                return Ok(idx as i64);
            }
        }

        // Fallback to last token (shouldn't happen)
        Ok((probabilities.len() - 1) as i64)
    }
}

fn download_tokenizer(api: &hf_hub::api::sync::ApiRepo) -> Result<Tokenizer> {
    println!("📥 Downloading Qwen tokenizer...");

    let tokenizer_file = get_local_or_remote_file("tokenizer.json", api)
        .map_err(|e| E::msg(format!("Failed to get tokenizer.json: {}", e)))?;

    let tokenizer = Tokenizer::from_file(&tokenizer_file)
        .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;

    println!("✅ Tokenizer loaded successfully");
    Ok(tokenizer)
}

fn download_model(args: &Args) -> Result<PathBuf> {
    if let Some(path) = &args.model_path {
        return Ok(PathBuf::from(path));
    }

    if args.local {
        let local_path = PathBuf::from("models/qwen/model.mlmodelc");
        if local_path.exists() {
            return Ok(local_path);
        } else {
            return Err(E::msg(format!(
                "Local model not found at: {}\n\
                Run without --local to download from HuggingFace",
                local_path.display()
            )));
        }
    }

    println!("📥 Downloading Qwen model from {}...", args.model_id);

    let repo = Repo::with_revision(
        args.model_id.clone(),
        RepoType::Model,
        args.revision.clone(),
    );
    let api = Api::new()?;
    let api_repo = api.repo(repo);

    // Look for the main model file
    let model_file = get_local_or_remote_file("model.mlmodelc", &api_repo)
        .or_else(|_| get_local_or_remote_file("model.mlpackage", &api_repo))
        .map_err(|e| {
            E::msg(format!(
                "Could not find model file in {}: {}\n\
            Expected model.mlmodelc or model.mlpackage",
                args.model_id, e
            ))
        })?;

    println!("✅ Model downloaded: {}", model_file.display());
    Ok(model_file)
}

#[cfg(target_os = "macos")]
fn run_qwen_chat(args: &Args) -> Result<()> {
    println!("🦙 Qwen 0.6B Chat with ANE Acceleration");
    println!("=====================================");
    println!("Model: {}", args.model_id);
    println!("Temperature: {:.2}", args.temperature);
    println!("Max tokens: {}", args.max_tokens);
    println!();

    // Setup HuggingFace API
    let repo = Repo::with_revision(
        args.model_id.clone(),
        RepoType::Model,
        args.revision.clone(),
    );
    let api = Api::new()?;
    let api_repo = api.repo(repo);

    // Download model and tokenizer
    let model_path = download_model(args)?;
    let tokenizer = download_tokenizer(&api_repo)?;

    // Configure CoreML model
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string()],
        output_name: "output".to_string(),
        max_sequence_length: MAX_SEQUENCE_LENGTH,
        vocab_size: QWEN_VOCAB_SIZE,
        model_type: "qwen-chat".to_string(),
    };

    // Load model
    println!("🔄 Loading CoreML model...");
    let start_time = Instant::now();
    let coreml_model = CoreMLModel::load_from_file(&model_path, &config)
        .map_err(|e| E::msg(format!("Failed to load model: {}", e)))?;
    println!("✅ Model loaded in {:?}", start_time.elapsed());

    // Create Qwen wrapper
    let qwen = QwenModel::new(coreml_model, tokenizer, args.temperature, args.max_tokens);

    // Interactive chat loop
    println!("\n💬 Chat started! Type 'quit' to exit.");
    println!("🎯 Tip: Try asking questions or starting conversations");
    println!();

    loop {
        print!("👤 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        // Format as chat prompt
        let prompt = format!("User: {}\nAssistant:", input);

        // Generate response with streaming
        match qwen.generate_streaming(&prompt) {
            Ok(_) => {
                println!(); // Extra line for readability
            }
            Err(e) => {
                println!("❌ Generation failed: {}", e);
            }
        }
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn run_qwen_chat(_args: &Args) -> Result<()> {
    println!("❌ Qwen chat is only available on macOS with CoreML support.");
    println!("\n💡 To use Qwen chat:");
    println!("   • Run on macOS");
    println!("   • Build with: cargo run --example qwen_chat");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🔧 Verbose mode enabled");
        println!("Config: {:#?}", args);
        println!();
    }

    run_qwen_chat(&args)
}
