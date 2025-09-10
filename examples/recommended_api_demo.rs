//! Demonstration of the recommended Qwen API design
//!
//! This example shows the new primary APIs that provide the best user experience:
//! - `complete_text()`: Simple, working API for most users
//! - `generate_text_with_params()`: Power-user API with full control
//!
//! These APIs use proven methods internally and provide decoded text directly,
//! avoiding the token-handling complexity that caused issues in the past.

use anyhow::Result;
use candle_coreml::UnifiedModelLoader;

fn main() -> Result<()> {
    println!("🚀 Qwen Recommended API Demo");
    println!("============================");

    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

    println!("🔄 Loading model...");
    let loader = UnifiedModelLoader::new()?;
    let mut model = loader.load_model(model_id)?;

    println!("✅ Model loaded successfully!\n");

    // ========================================
    // PRIMARY API: complete_text()
    // ========================================

    println!("📝 PRIMARY API: complete_text()");
    println!("-------------------------------");
    println!("✨ Simple API with good defaults (temp=0.7, top_k=50)");
    println!("✨ Returns decoded text directly - no token handling needed");

    let test_prompts = [
        "What is the capital of France?",
        "Complete this sentence: The sky is",
        "Hello, how are you",
    ];

    for prompt in &test_prompts {
        println!("\n🔸 Prompt: '{prompt}'");
        match model.complete_text(prompt, 30) {
            Ok(response) => {
                println!("   Response: '{response}'");
            }
            Err(e) => {
                println!("   ❌ Error: {e}");
            }
        }
    }

    // ========================================
    // POWER USER API: generate_text_with_params()
    // ========================================

    println!("\n\n🔧 POWER USER API: generate_text_with_params()");
    println!("----------------------------------------------");
    println!("🎛️ Full control over temperature and top-k sampling");
    println!("🎛️ Perfect for fine-tuning generation behavior");

    let advanced_tests = [
        ("Creative completion", "Once upon a time", 0.9, Some(20)),
        ("Deterministic completion", "2 + 2 =", 0.0, None),
        ("Balanced completion", "Explain AI:", 0.7, Some(50)),
    ];

    for (description, prompt, temperature, top_k) in &advanced_tests {
        println!("\n🔸 {description}: '{prompt}'");
        println!("   Parameters: temp={temperature}, top_k={top_k:?}");
        match model.generate_text_with_params(prompt, 25, *temperature, *top_k) {
            Ok(response) => {
                println!("   Response: '{response}'");
            }
            Err(e) => {
                println!("   ❌ Error: {e}");
            }
        }
    }

    // ========================================
    // COMPARISON WITH OLD APIS
    // ========================================

    println!("\n\n📊 API COMPARISON");
    println!("================");
    println!("✅ NEW: complete_text() - Simple, working, user-friendly");
    println!("✅ NEW: generate_text_with_params() - Full control, reliable");
    println!("⚠️  OLD: generate_tokens() - DEPRECATED (ignores temperature)");
    println!("🔧 OLD: forward_text() - Hidden (debug/advanced use only)");

    println!("\n💡 Recommendation: Use complete_text() for most applications");
    println!("💡 Power users: Use generate_text_with_params() for fine control");

    Ok(())
}
