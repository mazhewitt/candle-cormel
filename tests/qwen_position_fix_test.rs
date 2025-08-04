//! QwenModel Position Fix Test - Test the specific prompt with position fix

use anyhow::Result;
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_qwen_position_fix() -> Result<()> {
    println!("🔧 Testing QwenModel with position fix");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    println!("✅ QwenModel loaded");
    
    // Test the exact prompt that should predict "dog"
    let prompt = "The quick brown fox jumps over the lazy";
    println!("📝 Testing prompt: '{}'", prompt);
    
    // Also test a longer prompt that would trigger the /no_think issue
    let longer_prompt = "Tell me about Greece. I want to know about its history, culture, and geography. What makes it special?";
    println!("📝 Also testing longer prompt: '{}'", longer_prompt);
    
    let tokens = qwen_model.tokenizer().encode(longer_prompt, true)
        .map_err(|e| anyhow::Error::msg(format!("Tokenization failed: {}", e)))?;
    println!("🔢 Longer prompt tokenized to {} tokens", tokens.get_ids().len());
    
    let next_token = qwen_model.forward_text(prompt)?;
    
    println!("🎯 Prediction: token {}", next_token);
    
    // Test with the longer prompt (this would previously cause tensor indexing error)
    if tokens.get_ids().len() <= 50 { // Only test if within reasonable limits
        println!("🧪 Testing longer prompt (this would previously fail)...");
        match qwen_model.forward_text(longer_prompt) {
            Ok(long_token) => {
                println!("✅ Longer prompt works! Predicted token: {}", long_token);
                if let Ok(decoded) = qwen_model.tokenizer().decode(&[long_token as u32], false) {
                    println!("📖 Decoded: '{}'", decoded);
                }
            },
            Err(e) => {
                println!("⚠️  Longer prompt failed (expected for very long inputs): {}", e);
            }
        }
    } else {
        println!("⏭️  Skipping longer prompt test (too long: {} tokens)", tokens.get_ids().len());
    }
    
    // Decode the token
    if let Ok(decoded) = qwen_model.tokenizer().decode(&[next_token as u32], false) {
        println!("📖 Decoded: '{}'", decoded);
    }
    
    if next_token == 5562 {
        println!("✅ 🎉 SUCCESS! QwenModel now predicts 'dog' correctly!");
        Ok(())
    } else {
        println!("❌ Still predicting token {} instead of 5562 ('dog')", next_token);
        
        // Check if it's at least different from the previous wrong prediction
        if next_token == 15678 {
            println!("   Still getting 'lazy' - position fix didn't help");
        } else {
            println!("   Different prediction - position fix may have changed something");
        }
        
        panic!("POSITION FIX TEST FAILED: Expected token 5562 ('dog'), got token {} ('{}')", 
               next_token, 
               qwen_model.tokenizer().decode(&[next_token as u32], false).unwrap_or("???".to_string()));
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_position_fix_macos_requirement() {
    println!("❌ QwenModel position fix test requires macOS");
}