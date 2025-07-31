#!/usr/bin/env rust
//! Test what the Rust QwenModel predicts for "The quick brown fox jumps over the lazy"

use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing Rust QwenModel prediction for 'dog' completion");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let prompt = "The quick brown fox jumps over the lazy";
    println!("📝 Prompt: '{}'", prompt);
    
    let predicted_token = qwen_model.forward_text(prompt)?;
    let decoded = qwen_model.tokenizer().decode(&[predicted_token as u32], false)?;
    
    println!("🎯 Rust prediction: Token {} = '{}'", predicted_token, decoded);
    
    // Check if it's "dog" (token 5562)
    if predicted_token == 5562 {
        println!("🎉 SUCCESS! Rust correctly predicts 'dog'");
    } else {
        println!("❌ Different prediction. Expected: 5562 ('dog'), Got: {} ('{}')", predicted_token, decoded);
    }
    
    Ok(())
}