//! Qwen One-Pass Chat Example
//!
//! This example demonstrates a simple one-pass chat interface using the 
//! multi-component Qwen model from Anemll. It uses the clean git2+LFS downloader
//! to get the model and provides a basic chat interface.
//!
//! Based on: https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4
//!
//! Usage:
//! ```bash
//! # Interactive chat
//! cargo run --example qwen_one_pass_chat
//!
//! # Single prompt
//! cargo run --example qwen_one_pass_chat -- --prompt "What is the capital of France?"
//!
//! # With verbose output
//! cargo run --example qwen_one_pass_chat -- --verbose
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel, download_model};
use clap::Parser;
use std::io::{self, Write};
use std::time::Instant;
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const HIDDEN_SIZE: usize = 896;
const EOS_TOKEN_ID: i64 = 151645;
const MAX_GENERATION_LENGTH: usize = 50;
const SEQUENCE_LENGTH: usize = 16; // Working length from integration tests

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Single prompt to process (non-interactive mode)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Maximum number of tokens to generate
    #[arg(long, default_value_t = MAX_GENERATION_LENGTH)]
    max_tokens: usize,

    /// Model ID on HuggingFace Hub
    #[arg(long, default_value = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")]
    model_id: String,
}

/// Simple one-pass chat model using multi-component Qwen
struct QwenChat {
    embeddings: CoreMLModel,
    ffn: CoreMLModel,
    lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    device: Device,
    verbose: bool,
}

impl QwenChat {
    /// Create a new chat instance
    async fn new(args: &Args) -> Result<Self> {
        let device = Device::Cpu;
        
        if args.verbose {
            println!("🤖 Initializing Qwen Chat...");
            println!("📦 Model: {}", args.model_id);
        }
        
        // Download the model using clean git2+LFS approach
        let start_time = Instant::now();
        let cache_dir = download_model(&args.model_id, args.verbose)?;
        
        if args.verbose {
            println!("⏱️  Download time: {:?}", start_time.elapsed());
        }
        
        // Set up component paths
        let tokenizer_path = cache_dir.join("tokenizer.json");
        let embeddings_path = cache_dir.join("qwen_embeddings.mlmodelc");
        let ffn_path = cache_dir.join("qwen_FFN_PF_lut6_chunk_01of01.mlmodelc");
        let lm_head_path = cache_dir.join("qwen_lm_head_lut6.mlmodelc");

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;

        // Load model components
        let embeddings = Self::load_embeddings(&embeddings_path)?;
        let ffn = Self::load_ffn(&ffn_path)?;
        let lm_head = Self::load_lm_head(&lm_head_path)?;

        if args.verbose {
            println!("✅ All components loaded successfully!");
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

    /// Load embeddings model
    fn load_embeddings(path: &std::path::Path) -> Result<CoreMLModel> {
        let config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "embeddings".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };
        Ok(CoreMLModel::load_from_file(path, &config)?)
    }

    /// Load FFN model
    fn load_ffn(path: &std::path::Path) -> Result<CoreMLModel> {
        let config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string(), "causal_mask".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: HIDDEN_SIZE,
            model_type: "qwen-ffn".to_string(),
        };
        Ok(CoreMLModel::load_from_file(path, &config)?)
    }

    /// Load LM head model
    fn load_lm_head(path: &std::path::Path) -> Result<CoreMLModel> {
        let config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head".to_string(),
        };
        Ok(CoreMLModel::load_from_file(path, &config)?)
    }

    /// Tokenize input text
    fn tokenize(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self.tokenizer
            .encode(text, false)
            .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))?;
        
        Ok(encoding.get_ids().iter().map(|&id| id as i64).collect())
    }

    /// Detokenize tokens back to text
    fn detokenize(&self, tokens: &[i64]) -> Result<String> {
        let u32_tokens: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
        let text = self.tokenizer
            .decode(&u32_tokens, false)
            .map_err(|e| E::msg(format!("Detokenization failed: {}", e)))?;
        
        Ok(text)
    }

    /// Create causal attention mask
    fn create_causal_mask(&self, seq_len: usize) -> Result<Tensor> {
        let mut mask_data = vec![0.0f32; seq_len * seq_len];
        
        // Fill upper triangle with -inf for causal masking
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask_data[i * seq_len + j] = f32::NEG_INFINITY;
            }
        }
        
        Ok(Tensor::from_vec(mask_data, (seq_len, seq_len), &self.device)?)
    }

    /// Run one-pass generation
    fn generate_response(&self, prompt: &str, max_tokens: usize) -> Result<String> {
        if self.verbose {
            println!("🎯 Generating response for: \"{}\"", prompt);
        }

        let start_time = Instant::now();
        
        // Tokenize prompt
        let mut tokens = self.tokenize(prompt)?;
        
        // Pad to working sequence length
        while tokens.len() < SEQUENCE_LENGTH {
            tokens.push(0); // Pad with zeros
        }
        tokens.truncate(SEQUENCE_LENGTH);
        
        if self.verbose {
            println!("📝 Input tokens: {:?}", &tokens[..std::cmp::min(10, tokens.len())]);
        }

        // Generate tokens
        let mut generated_tokens = Vec::new();
        let mut current_tokens = tokens.clone();
        
        for step in 0..max_tokens {
            // Run forward pass
            let next_token = self.generate_next_token(&current_tokens)?;
            
            if next_token == EOS_TOKEN_ID {
                if self.verbose {
                    println!("🛑 EOS token generated, stopping");
                }
                break;
            }
            
            generated_tokens.push(next_token);
            
            if self.verbose {
                println!("🎲 Step {}: Generated token {}", step + 1, next_token);
            }
            
            // Update tokens for next iteration (sliding window)
            current_tokens.remove(0);
            current_tokens.push(next_token);
            
            // Print token immediately for streaming effect
            if let Ok(token_text) = self.detokenize(&[next_token]) {
                print!("{}", token_text);
                io::stdout().flush().unwrap();
            }
        }
        
        let generation_time = start_time.elapsed();
        
        // Decode generated tokens
        let response = self.detokenize(&generated_tokens)?;
        
        if self.verbose {
            println!();
            println!("⏱️  Generation time: {:?}", generation_time);
            println!("📊 Generated {} tokens", generated_tokens.len());
            if !generated_tokens.is_empty() {
                println!("⚡ Speed: {:.2} tokens/sec", 
                    generated_tokens.len() as f64 / generation_time.as_secs_f64());
            }
        }
        
        Ok(response)
    }

    /// Generate next token using the multi-component pipeline
    fn generate_next_token(&self, input_tokens: &[i64]) -> Result<i64> {
        let seq_len = input_tokens.len();
        
        // Convert to tensor
        let input_tensor = Tensor::from_vec(
            input_tokens.to_vec(),
            (1, seq_len),
            &self.device,
        )?;

        // Step 1: Embeddings
        let hidden_states = self.embeddings.forward(&[&input_tensor])?;
        
        // Step 2: Create causal mask and run FFN
        let causal_mask = self.create_causal_mask(seq_len)?;
        let processed_hidden = self.ffn.forward(&[&hidden_states, &causal_mask])?;
        
        // Step 3: LM Head (extract last position)
        let last_hidden = processed_hidden.narrow(1, seq_len - 1, 1)?;
        let logits = self.lm_head.forward(&[&last_hidden])?;
        
        // Step 4: Sample next token (argmax for simplicity)
        let logits_vec = logits.to_vec3::<f32>()?;
        let last_token_logits = &logits_vec[0][0];
        
        let next_token = last_token_logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i as i64)
            .ok_or_else(|| E::msg("Failed to select next token"))?;
        
        Ok(next_token)
    }

    /// Run interactive chat loop
    fn interactive_chat(&self, max_tokens: usize) -> Result<()> {
        println!("🤖 Qwen Chat - Interactive Mode");
        println!("Type 'quit' or 'exit' to stop");
        println!("===============================");
        
        loop {
            print!("\n👤 You: ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            
            if input.is_empty() {
                continue;
            }
            
            if input == "quit" || input == "exit" {
                println!("👋 Goodbye!");
                break;
            }
            
            print!("🤖 Qwen: ");
            match self.generate_response(input, max_tokens) {
                Ok(_response) => {
                    // Response is printed during generation for streaming effect
                    println!(); // Add newline after response
                }
                Err(e) => {
                    println!("❌ Error: {}", e);
                }
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🦙 Qwen One-Pass Chat Example");
    println!("=============================");
    
    // Initialize chat model
    let chat = QwenChat::new(&args).await?;
    
    match args.prompt {
        Some(prompt) => {
            // Single prompt mode
            println!("👤 Prompt: {}", prompt);
            print!("🤖 Response: ");
            let _response = chat.generate_response(&prompt, args.max_tokens)?;
            println!(); // Add final newline
        }
        None => {
            // Interactive mode
            chat.interactive_chat(args.max_tokens)?;
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sequence_length_constant() {
        assert_eq!(SEQUENCE_LENGTH, 16);
        assert!(SEQUENCE_LENGTH <= 512); // Should fit in max context
    }
    
    #[test]
    fn test_vocab_size_constant() {
        assert_eq!(QWEN_VOCAB_SIZE, 151936);
    }
}