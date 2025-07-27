//! Complete Qwen demo using the unified Qwen library
//! 
//! This example demonstrates the full Qwen pipeline with proper tokenization,
//! state management, and text generation.

use anyhow::Result;
use candle_coreml::{QwenModel, QwenConfig};
use std::path::Path;

fn main() -> Result<()> {
    println!("🚀 Qwen Complete Demo - Unified Pipeline");
    println!("=========================================");
    
    let model_dir = Path::new("/Users/mazdahewitt/projects/candle-coreml/qwen-model");
    
    println!("📂 Loading Qwen model from: {}", model_dir.display());
    
    // Load model with default configuration
    let config = QwenConfig::default();
    let mut model = QwenModel::load_from_directory(model_dir, Some(config))?;
    
    println!("✅ Model loaded successfully!");
    println!("   Vocab size: {}", model.config().vocab_size);
    println!("   Hidden size: {}", model.config().hidden_size);
    println!("   Context length: {}", model.config().context_length);
    
    // Initialize states for efficient generation
    println!("\n🧠 Initializing model states...");
    model.initialize_states()?;
    println!("✅ States initialized!");
    
    // Test cases
    let test_cases = vec![
        "The capital of France is",
        "The quick brown fox jumps over the lazy",
        "Hello, my name is",
        "Artificial intelligence is",
    ];
    
    println!("\n🧪 Running test cases:");
    println!("=====================");
    
    for (i, prompt) in test_cases.iter().enumerate() {
        println!("\n{}. Testing: \"{}\"", i + 1, prompt);
        
        // Tokenize to show the process
        let tokens = model.tokenize(prompt)?;
        println!("   Tokens ({}): {:?}", tokens.len(), tokens);
        
        // Single token prediction
        println!("   🎯 Generating next token...");
        let start_time = std::time::Instant::now();
        
        let next_token = model.forward(prompt)?;
        let inference_time = start_time.elapsed();
        
        println!("   ✅ Next token: {} (in {:?})", next_token, inference_time);
        
        // Reset states for next test
        model.reset_states()?;
    }
    
    // Demonstrate multi-token generation
    println!("\n🔄 Multi-token generation demo:");
    println!("===============================");
    
    let prompt = "The quick brown fox";
    println!("Prompt: \"{}\"", prompt);
    
    let start_time = std::time::Instant::now();
    let generated_tokens = model.generate(prompt, 5)?;
    let total_time = start_time.elapsed();
    
    println!("Generated tokens: {:?}", generated_tokens);
    println!("Total time for 5 tokens: {:?}", total_time);
    println!("Average time per token: {:?}", total_time / 5);
    
    // Tokenizer demo
    println!("\n📝 Tokenizer analysis:");
    println!("======================");
    
    let test_texts = vec![
        "Hello world",
        "The quick brown fox",
        "Artificial intelligence",
        "Machine learning model",
    ];
    
    for text in test_texts {
        let tokens = model.tokenize(text)?;
        let padded = model.pad_tokens(&tokens);
        println!("\"{}\" -> {} tokens -> {} padded", text, tokens.len(), padded.len());
        println!("  Original: {:?}", tokens);
        if tokens.len() != padded.len() {
            println!("  Padded:   {:?}...", &padded[..tokens.len().min(8)]);
        }
    }
    
    println!("\n🎉 Demo completed successfully!");
    println!("✅ Qwen unified pipeline working with:");
    println!("   • Proper tokenization and padding");
    println!("   • Stateful FFN inference with KV cache");
    println!("   • Multi-output LM head (16 chunks)");
    println!("   • Complete autoregressive generation");
    
    Ok(())
}