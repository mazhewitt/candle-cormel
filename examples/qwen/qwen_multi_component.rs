//! Qwen Multi-Component Architecture with ANE Optimization
//!
//! This example demonstrates the proper way to work with Anemll's multi-component
//! Qwen models, which split the architecture into separate CoreML models:
//! - Embeddings: Convert tokens to hidden states
//! - FFN (Feed-Forward Network): Process hidden states
//! - LM Head: Convert hidden states to logits
//!
//! This approach enables:
//! - Memory optimization through model chunking
//! - Better ANE utilization with specialized models
//! - Flexible deployment of model components
//!
//! Usage:
//! ```bash
//! # Interactive chat with multi-component model
//! cargo run --example qwen_multi_component
//!
//! # Enable verbose logging
//! cargo run --example qwen_multi_component -- --verbose
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_coreml::{download_model, Config as CoreMLConfig, CoreMLModel};
use clap::Parser;
use std::io::{self, Write};
use std::time::Instant;
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const MAX_SEQUENCE_LENGTH: usize = 512;
const HIDDEN_SIZE: usize = 1024; // Actual CoreML model hidden dimension (discovered via inspection)
const EOS_TOKEN_ID: i64 = 151645;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(long, default_value = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")]
    model_id: String,

    /// Temperature for sampling
    #[arg(long, default_value_t = 0.7)]
    temperature: f32,

    /// Maximum tokens to generate
    #[arg(long, default_value = "50")]
    max_tokens: usize,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

/// Multi-component Qwen model following Anemll architecture
struct MultiComponentQwen {
    embeddings: CoreMLModel,
    ffn: CoreMLModel,
    lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    device: Device,
    verbose: bool,
    // Add state tracking for autoregressive generation
    ffn_state: Option<candle_coreml::CoreMLState>,
    current_position: usize,
}

impl MultiComponentQwen {
    async fn new(args: &Args) -> Result<Self> {
        let device = Device::Cpu;

        // Download the complete model using the clean git2+LFS approach
        if args.verbose {
            println!("📥 Downloading Qwen model using clean git2+LFS downloader...");
        }

        let cache_dir = download_model(&args.model_id, args.verbose).map_err(|e| {
            E::msg(format!(
                "🔧 Failed to download model: {e}\n\
                \n\
                This might be because:\n\
                • Network connectivity issues\n\
                • Model repository is private\n\
                • Git LFS files are too large\n\
                \n\
                💡 Alternative: Use the patterns demo that shows the same concepts:\n\
                   cargo run --example qwen_demo_patterns\n\
                \n\
                Original error: {e}"
            ))
        })?;

        // Set up component paths
        let tokenizer_path = cache_dir.join("tokenizer.json");
        let embeddings_path = cache_dir.join("qwen_embeddings.mlmodelc");
        let ffn_path = cache_dir.join("qwen_FFN_PF_lut6_chunk_01of01.mlmodelc");
        let lm_head_path = cache_dir.join("qwen_lm_head_lut6.mlmodelc");

        if args.verbose {
            println!("✅ All components downloaded successfully:");
            println!("  • Cache directory: {}", cache_dir.display());
            println!("  • Embeddings: {}", embeddings_path.display());
            println!("  • FFN: {}", ffn_path.display());
            println!("  • LM Head: {}", lm_head_path.display());
            println!("  • Tokenizer: {}", tokenizer_path.display());
        }

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| E::msg(format!("Failed to load tokenizer: {e}")))?;

        // Configure and load embeddings model
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(), // Actual output name discovered via inspection
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };

        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)
            .map_err(|e| E::msg(format!("Failed to load embeddings model: {e}")))?;

        // Configure and load FFN model (requires MLState for KV-cache)
        let ffn_config = CoreMLConfig {
            input_names: vec![
                "hidden_states".to_string(),
                "position_ids".to_string(),
                "current_pos".to_string(),
                "causal_mask".to_string(),
            ],
            output_name: "output_hidden_states".to_string(), // Actual output name
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: HIDDEN_SIZE, // FFN works with hidden dimensions
            model_type: "qwen-ffn".to_string(),
        };

        let ffn = CoreMLModel::load_from_file(&ffn_path, &ffn_config)
            .map_err(|e| E::msg(format!("Failed to load FFN model: {e}")))?;

        // Configure and load LM head model (has 16 logits outputs that need concatenation)
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits1".to_string(), // We'll handle all 16 outputs manually
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head".to_string(),
        };

        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)
            .map_err(|e| E::msg(format!("Failed to load LM head model: {e}")))?;

        if args.verbose {
            println!("✅ All model components loaded successfully");
        }

        // Initialize FFN state for KV-cache
        let ffn_state = ffn.make_state()?;

        Ok(Self {
            embeddings,
            ffn,
            lm_head,
            tokenizer,
            device,
            verbose: args.verbose,
            ffn_state: Some(ffn_state),
            current_position: 0,
        })
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| E::msg(format!("Tokenization failed: {e}")))?;

        let tokens: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();

        if tokens.len() > MAX_SEQUENCE_LENGTH {
            return Err(E::msg(format!(
                "Input too long: {} tokens, max: {}",
                tokens.len(),
                MAX_SEQUENCE_LENGTH
            )));
        }

        Ok(tokens)
    }

    fn detokenize(&self, tokens: &[i64]) -> Result<String> {
        let ids: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
        self.tokenizer
            .decode(&ids, true)
            .map_err(|e| E::msg(format!("Detokenization failed: {e}")))
    }

    /// Process tokens one by one during prefill (discovered architecture)
    fn prefill(&mut self, input_tokens: &[i64]) -> Result<Tensor> {
        if self.verbose {
            println!(
                "🔄 Running prefill for {} tokens (single token processing)",
                input_tokens.len()
            );
        }

        let mut last_hidden = None;

        // Process each token individually through the pipeline
        for (pos, &token) in input_tokens.iter().enumerate() {
            if self.verbose {
                println!("  🔸 Processing token {token} at position {pos}");
            }

            // Step 1: Convert single token to embeddings (embeddings accepts length 1)
            let token_tensor = Tensor::from_vec(vec![token], (1, 1), &self.device)?;

            let hidden_states = self.embeddings.forward(&[&token_tensor])?;

            // Step 2: Process through FFN with state (single token: shape (1,1,1024))
            let processed_hidden = self.process_single_token_ffn(&hidden_states, pos)?;

            last_hidden = Some(processed_hidden);
        }

        // Update current position for generation phase
        self.current_position = input_tokens.len();

        last_hidden.ok_or_else(|| E::msg("No tokens to process"))
    }

    /// Process a single token through FFN with MLState
    fn process_single_token_ffn(
        &mut self,
        hidden_states: &Tensor,
        position: usize,
    ) -> Result<Tensor> {
        // Create position tensors
        let position_ids = Tensor::from_vec(vec![position as i64], (1,), &self.device)?;

        // current_pos should track the absolute position in the sequence
        let current_pos = Tensor::from_vec(vec![self.current_position as i64], (1,), &self.device)?;

        // Create causal mask that reflects the current context length
        let causal_mask = self.create_causal_mask_for_position(self.current_position)?;

        // Use FFN with state
        if let Some(ref mut state) = self.ffn_state {
            let processed_hidden = self.ffn.predict_with_state(
                &[hidden_states, &position_ids, &current_pos, &causal_mask],
                state,
            )?;

            if self.verbose {
                println!(
                    "    • FFN: {:?} -> {:?}",
                    hidden_states.shape(),
                    processed_hidden.shape()
                );
            }

            Ok(processed_hidden)
        } else {
            Err(E::msg("FFN state not initialized"))
        }
    }

    /// Create rank 4 causal mask for FFN that reflects current context
    fn create_causal_mask_for_position(&self, current_pos: usize) -> Result<Tensor> {
        // Shape: (1, 1, 1, 512) as discovered
        // The mask should indicate which positions are visible to the current token
        let mut mask_data = vec![f32::NEG_INFINITY; 512];

        // Allow access to all positions up to and including current position
        for item in mask_data.iter_mut().take(current_pos.min(511) + 1) {
            *item = 0.0;
        }

        // Reshape to required rank 4 format
        Tensor::from_vec(mask_data, (1, 1, 1, 512), &self.device).map_err(Into::into)
    }

    /// Generate the next token given current hidden states
    fn generate_next_token(&self, hidden_states: &Tensor, temperature: f32) -> Result<i64> {
        // LM head expects single token hidden states: (1, 1, 1024)
        let lm_head_start = Instant::now();

        // Get all outputs from LM head (need all 16 logits chunks)
        let all_outputs = self.lm_head.forward_all(&[hidden_states])?;

        if self.verbose {
            println!(
                "  • LM Head: {:?} -> {} outputs ({:?})",
                hidden_states.shape(),
                all_outputs.len(),
                lm_head_start.elapsed()
            );
        }

        // Concatenate all 16 logits chunks to form complete vocabulary
        let mut full_logits = Vec::with_capacity(QWEN_VOCAB_SIZE);

        for i in 1..=16 {
            let logits_key = format!("logits{i}");
            if let Some(chunk_tensor) = all_outputs.get(&logits_key) {
                let chunk_vec = chunk_tensor.to_vec3::<f32>()?;
                if !chunk_vec.is_empty() && !chunk_vec[0].is_empty() {
                    full_logits.extend_from_slice(&chunk_vec[0][0]);
                } else {
                    return Err(E::msg(format!(
                        "Invalid logits chunk shape for {logits_key}"
                    )));
                }
            } else {
                return Err(E::msg(format!("Missing logits chunk: {logits_key}")));
            }
        }

        if full_logits.len() != QWEN_VOCAB_SIZE {
            return Err(E::msg(format!(
                "Concatenated logits size {} != expected vocab size {}",
                full_logits.len(),
                QWEN_VOCAB_SIZE
            )));
        }

        if self.verbose {
            println!(
                "  • Concatenated {} logits chunks -> vocab size {}",
                16,
                full_logits.len()
            );
        }

        self.sample_token(&full_logits, temperature)
    }

    /// Generate hidden states for the next token
    fn generate_next_hidden_states(&mut self, next_token: i64) -> Result<Tensor> {
        // Process new token through embeddings + FFN pipeline
        let token_tensor = Tensor::from_vec(vec![next_token], (1, 1), &self.device)?;

        let hidden_states = self.embeddings.forward(&[&token_tensor])?;
        let processed_hidden =
            self.process_single_token_ffn(&hidden_states, self.current_position)?;

        // Update position for next iteration
        self.current_position += 1;

        Ok(processed_hidden)
    }

    /// Reset conversation state (important for multi-turn chat)
    #[allow(dead_code)]
    fn reset_conversation(&mut self) -> Result<()> {
        // Create fresh FFN state
        self.ffn_state = Some(self.ffn.make_state()?);
        self.current_position = 0;
        Ok(())
    }

    /// Generate text using the multi-component pipeline
    fn generate(&mut self, prompt: &str, max_tokens: usize, temperature: f32) -> Result<String> {
        // Only reset state if this is a completely new conversation
        // For now, let's NOT reset to see if that helps
        // self.reset_conversation()?;

        let input_tokens = self.tokenize(prompt)?;

        if self.verbose {
            println!("🎯 Input: {} tokens", input_tokens.len());
            println!(
                "📝 Tokens: {:?}",
                &input_tokens[..input_tokens.len().min(10)]
            );
        }

        println!("🤖 ");
        io::stdout().flush().unwrap();

        // Prefill stage: process the input prompt
        let mut hidden_states = self.prefill(&input_tokens)?;
        let mut generated_tokens = input_tokens.clone();
        let mut generated_text = String::new();

        // Generation stage: generate tokens one by one using single token pipeline
        for _step in 0..max_tokens {
            let next_token = self.generate_next_token(&hidden_states, temperature)?;

            if next_token == EOS_TOKEN_ID {
                break;
            }

            generated_tokens.push(next_token);

            // Decode and display the new token
            if let Ok(token_text) = self.detokenize(&[next_token]) {
                print!("{token_text}");
                io::stdout().flush().unwrap();
                generated_text.push_str(&token_text);
            }

            // For next iteration: process the new token through the pipeline
            hidden_states = self.generate_next_hidden_states(next_token)?;
        }

        println!(); // New line after generation
        Ok(generated_text)
    }

    fn sample_token(&self, logits: &[f32], temperature: f32) -> Result<i64> {
        if temperature == 0.0 {
            // Greedy sampling
            let (best_idx, _) = logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .ok_or_else(|| E::msg("Empty logits"))?;
            return Ok(best_idx as i64);
        }

        // Temperature sampling
        let scaled_logits: Vec<f32> = logits.iter().map(|&x| x / temperature).collect();

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

        Ok((probabilities.len() - 1) as i64)
    }
}

#[cfg(target_os = "macos")]
async fn run_multi_component_chat(args: &Args) -> Result<()> {
    println!("🦙 Multi-Component Qwen Chat with ANE");
    println!("====================================");
    println!("Model: {}", args.model_id);
    println!("Temperature: {:.2}", args.temperature);
    println!("Max tokens: {}", args.max_tokens);
    println!();

    // Load multi-component model
    let mut model = MultiComponentQwen::new(args).await?;

    println!("💬 Chat started! Type 'quit' to exit.");
    println!("🎯 This demo shows multi-component model orchestration with MLState");
    println!();

    loop {
        print!("👤 You: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\n👋 Goodbye!");
                break;
            }
            Ok(_) => {}
            Err(e) => {
                println!("❌ Error reading input: {e}");
                break;
            }
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            println!("👋 Goodbye!");
            break;
        }

        // Format as chat prompt (using Qwen chat template)
        let prompt = format!("<|im_start|>user\n{input}<|im_end|>\n<|im_start|>assistant\n");

        // Generate response
        match model.generate(&prompt, args.max_tokens, args.temperature) {
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
async fn run_multi_component_chat(_args: &Args) -> Result<()> {
    println!("❌ Multi-component Qwen chat is only available on macOS with CoreML support.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🔧 Verbose mode enabled");
        println!("Config: {args:#?}");
        println!();
    }

    run_multi_component_chat(&args).await
}
