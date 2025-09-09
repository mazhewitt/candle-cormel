//! Qwen 0.6B Interactive Chat with ANE Acceleration
//!
//! This example demonstrates real-time chat using Anemll's ANE-optimized Qwen 0.6B model
//! with the new UnifiedModelLoader system that provides automatic config generation.
//!
//! Features:
//! - ANE-accelerated inference for maximum performance
//! - Automatic config generation and shape detection
//! - Universal model compatibility (works with any ANEMLL model)
//! - Real HuggingFace tokenizer integration
//! - Temperature and sampling controls
//! - Automatic model and tokenizer download
//! - Single-token and multi-token generation
//!
//! Usage:
//! ```bash
//! # Basic chat (single token completions)
//! cargo run --example qwen_chat
//!
//! # Multi-token generation
//! cargo run --example qwen_chat -- --temperature 0.7 --max-tokens 50
//!
//! # Use specific model variant
//! cargo run --example qwen_chat -- --model-id "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"
//! ```

use anyhow::{Error as E, Result};
use candle_coreml::{QwenModel, UnifiedModelLoader};
use clap::Parser;
use std::io::{self, Write};
use std::time::Instant;

const DEFAULT_TEMPERATURE: f32 = 0.7;
const DEFAULT_MAX_TOKENS: usize = 50;
const DEFAULT_TOP_K: usize = 50;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(
        long,
        default_value = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"
    )]
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

    /// Top-k sampling size (set 0 to disable and use pure temperature)
    #[arg(long, default_value_t = DEFAULT_TOP_K)]
    top_k: usize,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Use local model instead of downloading (deprecated with UnifiedModelLoader)
    #[arg(long)]
    local: bool,

    /// Path to local model directory (deprecated with UnifiedModelLoader)
    #[arg(long)]
    model_path: Option<String>,

    /// Enable single-token mode (like TDD test - predicts one token)
    #[arg(long)]
    single_token: bool,

    /// Use a simple chat-style prompt format (User:/Assistant:). Off by default.
    #[arg(long)]
    chat_format: bool,
}

struct QwenChatWrapper {
    model: QwenModel,
    temperature: f32,
    max_tokens: usize,
    single_token_mode: bool,
    top_k: Option<usize>,
}

impl QwenChatWrapper {
    fn new(
        model: QwenModel,
        temperature: f32,
        max_tokens: usize,
        single_token_mode: bool,
        top_k: Option<usize>,
    ) -> Self {
        Self {
            model,
            temperature,
            max_tokens,
            single_token_mode,
            top_k,
        }
    }

    fn detokenize(&self, tokens: &[i64]) -> Result<String> {
        let token_ids: Vec<u32> = tokens.iter().map(|&id| id as u32).collect();
        self.model
            .tokenizer()
            .decode(&token_ids, false)
            .map_err(|e| E::msg(format!("Detokenization failed: {e}")))
    }

    fn generate_response(&mut self, prompt: &str) -> Result<String> {
        let start_time = Instant::now();

        if self.single_token_mode {
            // Single token mode - like our TDD test that works perfectly
            println!("🎯 Single-token mode: Using validated forward_text() method");
            let next_token = self
                .model
                .forward_text(prompt)
                .map_err(|e| E::msg(format!("Single token generation failed: {e}")))?;

            let response = self.detokenize(&[next_token])?;
            let inference_time = start_time.elapsed();

            println!("⚡ Generated '{response}' in {inference_time:?}");
            Ok(response)
        } else {
            // Multi-token mode - prefer full decode to avoid per-token BPE artifacts
            println!("🚀 Multi-token mode: Using top-k + temperature sampling");
            println!("📝 Input: '{prompt}' (top_k={:?}, temp={:.2})", self.top_k, self.temperature);

            // Prefer top-k + temperature sampling to avoid repetitive greedy outputs
            // Normalize prompt (ensure trailing space in plain mode to reduce leading artifact join)
            let normalized_prompt = {
                let trimmed = prompt.trim_end();
                if !trimmed.ends_with(':')
                    && !trimmed.ends_with('.')
                    && !trimmed.ends_with('!')
                    && !trimmed.ends_with('?')
                    && !trimmed.ends_with(' ')
                {
                    format!("{trimmed} ")
                } else {
                    prompt.to_string()
                }
            };

            let generated_tokens = self
                .model
                .generate_tokens_topk_temp(
                    &normalized_prompt,
                    self.max_tokens,
                    self.temperature,
                    self.top_k,
                )
                .map_err(|e| E::msg(format!("Multi-token generation failed: {e}")))?;

            // Decode full text and then take just the suffix after the prompt
            let prompt_tokens = self
                .model
                .tokenizer()
                .encode(normalized_prompt.as_str(), true)
                .map_err(|e| E::msg(format!("Prompt tokenization failed: {e}")))?;
            let mut full_ids: Vec<u32> = prompt_tokens.get_ids().to_vec();
            full_ids.extend(generated_tokens.iter().map(|&t| t as u32));
            let full_text = self
                .model
                .tokenizer()
                .decode(&full_ids, false)
                .map_err(|e| E::msg(format!("Full decode failed: {e}")))?;
            let prompt_decoded = self
                .model
                .tokenizer()
                .decode(prompt_tokens.get_ids(), false)
                .unwrap_or_else(|_| normalized_prompt.clone());
            let mut response = full_text
                .strip_prefix(&prompt_decoded)
                .unwrap_or(&full_text)
                .to_string();
            // Final cleanup
            while response.starts_with([' ', '\n', '\t']) {
                response.remove(0);
            }
            let inference_time = start_time.elapsed();

            println!(
                "⚡ Generated {} tokens in {:?}",
                generated_tokens.len(),
                inference_time
            );

            // Print the fully decoded text once
            println!("🤖 {response}");
            Ok(response)
        }
    }
}

fn load_model_with_unified_loader(args: &Args) -> Result<QwenModel> {
    if let Some(_path) = &args.model_path {
        return Err(E::msg(
            "Local model paths not supported with UnifiedModelLoader.\n\
            Use the --model-id option with HuggingFace model IDs instead."
        ));
    }

    if args.local {
        return Err(E::msg(
            "Local model loading not supported with UnifiedModelLoader.\n\
            Use the --model-id option with HuggingFace model IDs instead."
        ));
    }

    println!(
        "🚀 Loading model with UnifiedModelLoader: {}",
        args.model_id
    );

    // Use the new UnifiedModelLoader
    let loader = UnifiedModelLoader::new()
        .map_err(|e| E::msg(format!("Failed to create UnifiedModelLoader: {e}")))?;
    
    let model = loader.load_model(&args.model_id)
        .map_err(|e| E::msg(format!("Failed to load model: {e}")))?;

    println!("✅ Model loaded successfully");
    Ok(model)
}

#[cfg(target_os = "macos")]
fn run_qwen_chat(args: &Args) -> Result<()> {
    println!("🦙 Qwen 0.6B Chat with ANE Acceleration");
    println!("=====================================");
    println!("Model: {}", args.model_id);
    println!("Temperature: {:.2}", args.temperature);
    println!("Max tokens: {}", args.max_tokens);
    println!();

    // Load model using the new UnifiedModelLoader
    println!("🔄 Loading QwenModel with UnifiedModelLoader...");
    println!("   🤖 Automatic config generation and shape detection");
    println!("   📦 This model has 4 components: embeddings, FFN prefill, FFN infer, LM head");
    println!("   ⏱️  Each component requires compilation (expect ~30-60s total)");
    println!("   💡 Set RUST_LOG=debug to see detailed component loading progress");
    let start_time = Instant::now();

    let qwen_model = load_model_with_unified_loader(args)?;

    println!("✅ QwenModel loaded in {:?}", start_time.elapsed());

    // Create chat wrapper
    let mut qwen = QwenChatWrapper::new(
        qwen_model,
        args.temperature,
        args.max_tokens,
        args.single_token,
        if args.top_k == 0 { None } else { Some(args.top_k) },
    );

    // Interactive chat loop
    println!("\n💬 Chat started! Type 'quit' to exit.");
    if args.single_token {
        println!("🎯 Single-token mode: Like TDD test - predicts next word only");
        println!("💡 Try: 'The quick brown fox jumps over the lazy' (should predict 'dog')");
    } else {
        println!("🚀 Multi-token mode: Generates full responses");
        if args.chat_format {
            println!("💡 Chat format enabled: prompts use 'User: <text>\\nAssistant:'");
        } else {
            println!("💡 Plain format: prompts are used directly (recommended for base LMs)");
        }
    }
    println!("⚡ Note: Uses new UnifiedModelLoader with automatic config generation");
    println!();

    loop {
        print!("👤 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;

        // Handle EOF (Ctrl+D or stdin closed)
        if bytes_read == 0 {
            println!("\n👋 Goodbye!");
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        // Format prompt based on mode
        let prompt = if args.single_token {
            // Single token mode - use raw input for completion
            input.to_string()
        } else if args.chat_format {
            // Optional chat-style format
            format!("User: {input}\nAssistant:")
        } else {
            // Plain prompt (default)
            input.to_string()
        };

        // Generate response
        match qwen.generate_response(&prompt) {
            Ok(_) => {
                println!(); // Extra line for readability
            }
            Err(e) => {
                println!("❌ Generation failed: {e}");
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
        println!("Config: {args:#?}");
        println!();
    }

    run_qwen_chat(&args)
}
