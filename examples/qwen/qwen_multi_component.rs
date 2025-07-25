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
use candle_coreml::{Config as CoreMLConfig, CoreMLModel, download_multi_component_model, MultiComponentConfig};
use clap::Parser;
use hf_hub::api::sync::Api;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const MAX_SEQUENCE_LENGTH: usize = 512;
const HIDDEN_SIZE: usize = 896; // Qwen 0.6B hidden dimension
const EOS_TOKEN_ID: i64 = 151645;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(
        long,
        default_value = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4"
    )]
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
}

impl MultiComponentQwen {
    async fn new(args: &Args) -> Result<Self> {
        let device = Device::Cpu;
        
        // Setup HuggingFace API 
        let api = Api::new()?;
        let repo = api.model(args.model_id.clone());

        // Configure multi-component download
        let download_config = MultiComponentConfig {
            components: vec![
                "qwen_embeddings.mlmodelc".to_string(),
                "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc".to_string(),
                "qwen_lm_head_lut6.mlmodelc".to_string(),
            ],
            additional_files: vec!["tokenizer.json".to_string()],
            verbose: args.verbose,
        };

        // Download all model components using the robust downloader
        if args.verbose {
            println!("📥 Downloading Qwen model components using multi-component downloader...");
        }
        
        let cache_dir = download_multi_component_model(&repo, &download_config)
            .map_err(|e| E::msg(format!(
                "🔧 Failed to download multi-component model: {}\n\
                \n\
                This might be because:\n\
                • The .mlmodelc directories require special handling\n\
                • Files are in Git LFS and need authentication\n\
                • Network connectivity issues\n\
                \n\
                💡 Alternative: Use the patterns demo that shows the same concepts:\n\
                   cargo run --example qwen_demo_patterns\n\
                \n\
                Original error: {}", e, e
            )))?;

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
            .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;

        // Configure and load embeddings model
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "embeddings".to_string(),
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };

        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)
            .map_err(|e| E::msg(format!("Failed to load embeddings model: {}", e)))?;

        // Configure and load FFN model  
        let ffn_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string(), "causal_mask".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: HIDDEN_SIZE, // FFN works with hidden dimensions
            model_type: "qwen-ffn".to_string(),
        };

        let ffn = CoreMLModel::load_from_file(&ffn_path, &ffn_config)
            .map_err(|e| E::msg(format!("Failed to load FFN model: {}", e)))?;

        // Configure and load LM head model
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits".to_string(),
            max_sequence_length: MAX_SEQUENCE_LENGTH,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head".to_string(),
        };

        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)
            .map_err(|e| E::msg(format!("Failed to load LM head model: {}", e)))?;

        if args.verbose {
            println!("✅ All model components loaded successfully");
        }

        Ok(Self {
            embeddings,
            ffn,
            lm_head,
            tokenizer,
            device,
            verbose: args.verbose,
        })
    }

    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self.tokenizer
            .encode(text, true)
            .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))?;
        
        let tokens: Vec<i64> = encoding
            .get_ids()
            .iter()
            .map(|&id| id as i64)
            .collect();
        
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
            .map_err(|e| E::msg(format!("Detokenization failed: {}", e)))
    }

    /// Run the prefill stage for a sequence of tokens
    fn prefill(&self, input_tokens: &[i64]) -> Result<Tensor> {
        if self.verbose {
            println!("🔄 Running prefill for {} tokens", input_tokens.len());
        }

        // Step 1: Convert tokens to embeddings
        let input_tensor = Tensor::from_vec(
            input_tokens.to_vec(),
            (1, input_tokens.len()),
            &self.device,
        )?;

        let embeddings_start = Instant::now();
        let hidden_states = self.embeddings.forward(&[&input_tensor])?;
        
        if self.verbose {
            println!("  • Embeddings: {:?} -> {:?} ({:?})", 
                input_tensor.shape(), hidden_states.shape(), embeddings_start.elapsed());
        }

        // Step 2: Process through FFN with causal mask
        let seq_len = input_tokens.len();
        let causal_mask = self.create_causal_mask(seq_len)?;
        
        let ffn_start = Instant::now();
        let processed_hidden = self.ffn.forward(&[&hidden_states, &causal_mask])?;
        
        if self.verbose {
            println!("  • FFN: {:?} -> {:?} ({:?})", 
                hidden_states.shape(), processed_hidden.shape(), ffn_start.elapsed());
        }

        Ok(processed_hidden)
    }

    /// Generate the next token given current hidden states
    fn generate_next_token(&self, hidden_states: &Tensor, temperature: f32) -> Result<i64> {
        // Get logits from LM head (only need the last position)
        let last_hidden = self.extract_last_position(hidden_states)?;
        
        let lm_head_start = Instant::now();
        let logits = self.lm_head.forward(&[&last_hidden])?;
        
        if self.verbose {
            println!("  • LM Head: {:?} -> {:?} ({:?})", 
                last_hidden.shape(), logits.shape(), lm_head_start.elapsed());
        }

        // Sample from logits
        let logits_vec = logits.to_vec2::<f32>()?;
        if logits_vec.is_empty() || logits_vec[0].len() != QWEN_VOCAB_SIZE {
            return Err(E::msg("Invalid logits shape from LM head"));
        }

        self.sample_token(&logits_vec[0], temperature)
    }

    /// Generate text using the multi-component pipeline
    fn generate(&self, prompt: &str, max_tokens: usize, temperature: f32) -> Result<String> {
        let input_tokens = self.tokenize(prompt)?;
        
        println!("🤖 ");
        io::stdout().flush().unwrap();

        // Prefill stage: process the input prompt
        let mut hidden_states = self.prefill(&input_tokens)?;
        let mut generated_tokens = input_tokens.clone();
        let mut generated_text = String::new();

        // Generation stage: generate tokens one by one
        for step in 0..max_tokens {
            let next_token = self.generate_next_token(&hidden_states, temperature)?;

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

            // For next iteration, we need to update hidden states
            // In a full implementation, this would involve extending the sequence
            // For now, we'll use a simplified approach
            if step < max_tokens - 1 {
                hidden_states = self.extend_hidden_states(&hidden_states, next_token)?;
            }
        }

        println!(); // New line after generation
        Ok(generated_text)
    }

    fn create_causal_mask(&self, seq_len: usize) -> Result<Tensor> {
        let mut mask_data = vec![0.0f32; seq_len * seq_len];
        
        // Fill upper triangle with -inf for causal masking
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask_data[i * seq_len + j] = f32::NEG_INFINITY;
            }
        }

        Tensor::from_vec(
            mask_data,
            (seq_len, seq_len),
            &self.device,
        ).map_err(Into::into)
    }

    fn extract_last_position(&self, hidden_states: &Tensor) -> Result<Tensor> {
        // Extract the last position from the sequence dimension
        let shape = hidden_states.shape();
        if shape.dims().len() != 3 {
            return Err(E::msg("Expected 3D hidden states tensor"));
        }

        let seq_len = shape.dims()[1];
        let last_idx = seq_len - 1;
        
        // Get slice [batch, last_position, hidden_dim]
        hidden_states.narrow(1, last_idx, 1).map_err(Into::into)
    }

    fn extend_hidden_states(&self, _current_hidden: &Tensor, new_token: i64) -> Result<Tensor> {
        // In a full implementation, this would:
        // 1. Convert new_token to embeddings using embeddings model
        // 2. Concatenate with existing hidden states
        // 3. Process through FFN with updated causal mask
        //
        // For now, return a placeholder
        // This is where the KV-cache optimization would be most beneficial
        
        let new_token_tensor = Tensor::from_vec(
            vec![new_token],
            (1, 1),
            &self.device,
        )?;

        // Convert to embeddings
        let new_embeddings = self.embeddings.forward(&[&new_token_tensor])?;
        
        // For simplicity, just return the new embeddings
        // In practice, you'd maintain and extend the full hidden state sequence
        Ok(new_embeddings)
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
        let scaled_logits: Vec<f32> = logits
            .iter()
            .map(|&x| x / temperature)
            .collect();

        let max_logit = scaled_logits
            .iter()
            .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        let exp_logits: Vec<f32> = scaled_logits
            .iter()
            .map(|&x| (x - max_logit).exp())
            .collect();
        
        let sum: f32 = exp_logits.iter().sum();
        let probabilities: Vec<f32> = exp_logits
            .iter()
            .map(|&x| x / sum)
            .collect();

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
    let model = MultiComponentQwen::new(args).await?;
    
    println!("💬 Chat started! Type 'quit' to exit.");
    println!("🎯 This demo shows multi-component model orchestration");
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

        // Generate response
        match model.generate(&prompt, args.max_tokens, args.temperature) {
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
async fn run_multi_component_chat(_args: &Args) -> Result<()> {
    println!("❌ Multi-component Qwen chat is only available on macOS with CoreML support.");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("🔧 Verbose mode enabled");
        println!("Config: {:#?}", args);
        println!();
    }

    run_multi_component_chat(&args).await
}