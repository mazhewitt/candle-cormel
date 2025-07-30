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
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use clap::Parser;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

const QWEN_VOCAB_SIZE: usize = 151936;
const MAX_SEQUENCE_LENGTH: usize = 512;
const EOS_TOKEN_ID: i64 = 151645;
const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_MAX_TOKENS: usize = 50;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(long, default_value = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")]
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

struct QwenChatWrapper {
    model: QwenModel,
    config: ChatConfig,
}

struct ChatConfig {
    temperature: f32,
    max_tokens: usize,
    vocab_size: usize,
    max_sequence_length: usize,
}

impl QwenChatWrapper {
    fn new(model: QwenModel, temperature: f32, max_tokens: usize) -> Self {
        let config = ChatConfig {
            temperature,
            max_tokens,
            vocab_size: QWEN_VOCAB_SIZE,
            max_sequence_length: MAX_SEQUENCE_LENGTH,
        };

        Self {
            model,
            config,
        }
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        self.model.tokenize(text)
            .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))
    }

    fn detokenize(&self, tokens: &[i64]) -> Result<String> {
        self.model.detokenize(tokens)
            .map_err(|e| E::msg(format!("Detokenization failed: {}", e)))
    }

    fn generate_streaming(&mut self, prompt: &str) -> Result<String> {
        let tokens = self.tokenize(prompt)?;
        
        // Reset model state for new generation
        self.model.reset_states()
            .map_err(|e| E::msg(format!("Failed to reset model states: {}", e)))?;
        
        println!("🚀 Starting generation with QwenModel granular API");
        println!("📝 Input: '{}'", prompt);
        println!("🔢 Tokens: {} tokens", tokens.len());

        // STEP 1: Run embeddings for the input tokens
        let embeddings = self.model.compute_embeddings(&tokens)
            .map_err(|e| E::msg(format!("Embeddings failed: {}", e)))?;
        
        // STEP 2: Run prefill phase to populate KV cache
        self.model.run_prefill_phase(&embeddings, tokens.len())
            .map_err(|e| E::msg(format!("Prefill phase failed: {}", e)))?;
        
        let mut generated_tokens = tokens.clone();
        let mut generated_text = String::new();
        
        print!("🤖 ");
        io::stdout().flush().unwrap();

        // STEP 3: Generate tokens one by one using infer phase
        for step in 0..self.config.max_tokens {
            let current_position = generated_tokens.len() - 1;
            
            // Get last token embedding
            let last_token = generated_tokens[generated_tokens.len() - 1];
            let last_token_embedding = self.model.compute_embeddings(&[last_token])
                .map_err(|e| E::msg(format!("Last token embedding failed: {}", e)))?;
            
            // Generate next token using granular infer method
            let start_time = Instant::now();
            let next_token = self.model.generate_next_token_with_infer(&last_token_embedding, current_position)
                .map_err(|e| E::msg(format!("Token generation failed at step {}: {}", step, e)))?;
            let inference_time = start_time.elapsed();

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


fn download_model(args: &Args) -> Result<PathBuf> {
    if let Some(path) = &args.model_path {
        return Ok(PathBuf::from(path));
    }

    if args.local {
        let local_path = PathBuf::from("models/qwen");
        if local_path.exists() {
            return Ok(local_path);
        } else {
            return Err(E::msg(format!(
                "Local model directory not found at: {}\n\
                Run without --local to download from HuggingFace",
                local_path.display()
            )));
        }
    }

    println!("📥 Ensuring Qwen model from {} is available...", args.model_id);

    // Use the ensure_model_downloaded function from candle-coreml
    let model_dir = candle_coreml::ensure_model_downloaded(&args.model_id, false)
        .map_err(|e| E::msg(format!("Failed to download model: {}", e)))?;

    println!("✅ Model available at: {}", model_dir.display());
    Ok(model_dir)
}

#[cfg(target_os = "macos")]
fn run_qwen_chat(args: &Args) -> Result<()> {
    println!("🦙 Qwen 0.6B Chat with ANE Acceleration");
    println!("=====================================");
    println!("Model: {}", args.model_id);
    println!("Temperature: {:.2}", args.temperature);
    println!("Max tokens: {}", args.max_tokens);
    println!();

    // Download model directory
    let model_dir = download_model(args)?;

    // Load QwenModel using the granular API
    println!("🔄 Loading QwenModel with granular API...");
    let start_time = Instant::now();
    
    let qwen_config = QwenConfig::default();
    let qwen_model = QwenModel::load_from_directory(&model_dir, Some(qwen_config))
        .map_err(|e| E::msg(format!("Failed to load QwenModel: {}", e)))?;
    
    println!("✅ QwenModel loaded in {:?}", start_time.elapsed());

    // Create chat wrapper
    let mut qwen = QwenChatWrapper::new(qwen_model, args.temperature, args.max_tokens);

    // Interactive chat loop
    println!("\n💬 Chat started! Type 'quit' to exit.");
    println!("🎯 Tip: Try asking questions or starting conversations");
    println!("⚠️  Note: This uses the granular QwenModel API for fine-grained control");
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
