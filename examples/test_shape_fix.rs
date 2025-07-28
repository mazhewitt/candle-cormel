//! Test that shape validation fixes work
//!
//! This is a minimal test to verify the tensor shape mismatch is resolved.

use candle_coreml::QwenModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Testing shape validation fix");
    println!("===============================");

    // Load the typo-fixer model from HuggingFace
    println!("📥 Loading typo-fixer model...");
    let mut model = QwenModel::load_typo_fixer(false)?; // No verbose download

    // Test simple single-token generation
    let test_text = "Hello";
    println!("🔍 Testing single token generation from: '{}'", test_text);
    
    // Just try to generate one next token
    let next_token = model.forward_text(test_text)?;
    println!("✅ Successfully generated next token: {}", next_token);

    // Try to decode it back
    let token_text = model.tokenizer().decode(&[next_token as u32], false)
        .map_err(|e| format!("Failed to decode token: {}", e))?;
    println!("📝 Next token decoded as: '{}'", token_text);

    println!("🎉 Shape validation fix verified - no tensor errors!");
    Ok(())
}