use anyhow::Result;
use candle_coreml::UnifiedModelLoader;

fn main() -> Result<()> {
    println!("🧠 Testing Qwen Thinking Model Behavior");
    println!("======================================");

    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

    println!("🔄 Loading model with UnifiedModelLoader...");
    let loader = UnifiedModelLoader::new()?;
    let mut model = loader.load_model(model_id)?;

    // Test prompts that should trigger thinking
    let test_cases = [
        "What is the capital of France?",
        "In one word, what is the capital of the UK?",
        "In a short sentence: Tell me about AI?",
        "The quick brown fox jumps over the lazy",
    ];

    for (i, prompt) in test_cases.iter().enumerate() {
        println!("\n📝 Test case {}: '{}'", i + 1, prompt);
        println!("{}", "=".repeat(50));

        // Method 1: Single token (current failing approach)
        println!("🔸 Single token generation:");
        let single_token = model.forward_text(prompt)?;
        if let Ok(decoded) = model.tokenizer().decode(&[single_token as u32], false) {
            println!("   Token {single_token}: '{decoded}'");
        }

        // Method 2: Multi-token generation (potential thinking behavior) - FIXED METHOD
        println!("🔸 Multi-token generation (25 tokens) - using working method:");
        match model.generate_tokens_topk_temp(prompt, 25, 0.7, Some(50)) {
            Ok(tokens) => {
                let tokens_u32: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
                if let Ok(decoded) = model.tokenizer().decode(&tokens_u32, false) {
                    println!("   Generated: '{decoded}'");

                    // Check for thinking patterns
                    let thinking_indicators =
                        ["think", "Thinking", "reason", "consider", "because"];
                    let has_thinking = thinking_indicators.iter().any(|&indicator| {
                        decoded.to_lowercase().contains(&indicator.to_lowercase())
                    });

                    if has_thinking {
                        println!("   ✅ Contains thinking patterns!");
                    } else {
                        println!("   ❌ No obvious thinking patterns");
                    }
                } else {
                    println!("   ❌ Could not decode tokens: {tokens:?}");
                }
            }
            Err(e) => {
                println!("   ❌ Multi-token generation failed: {e}");
            }
        }

        // Method 3: Explicit thinking prompt - FIXED METHOD
        println!("🔸 With thinking prompt (using working method):");
        let thinking_prompt = format!("Think step by step. {prompt}");
        match model.generate_tokens_topk_temp(&thinking_prompt, 30, 0.7, Some(50)) {
            Ok(tokens) => {
                let tokens_u32: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
                if let Ok(decoded) = model.tokenizer().decode(&tokens_u32, false) {
                    println!("   Generated: '{decoded}'");
                } else {
                    println!("   ❌ Could not decode thinking prompt tokens");
                }
            }
            Err(e) => {
                println!("   ❌ Thinking prompt generation failed: {e}");
            }
        }
    }

    // Test vocabulary for thinking-related tokens
    println!("\n🔍 Checking vocabulary for thinking-related tokens:");
    let thinking_tokens = [
        "Thinking", "thinking", "Think", "think", "reason", "because", "step",
    ];
    for token_text in &thinking_tokens {
        // Try to encode the token to see if it exists in vocabulary
        if let Ok(encoded) = model.tokenizer().encode(*token_text, false) {
            if !encoded.get_ids().is_empty() {
                println!(
                    "   ✅ '{}' → token IDs: {:?}",
                    token_text,
                    encoded.get_ids()
                );
            }
        } else {
            println!("   ❌ '{token_text}' not found in vocabulary");
        }
    }

    Ok(())
}
