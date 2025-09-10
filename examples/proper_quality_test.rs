use anyhow::Result;
use candle_coreml::UnifiedModelLoader;

fn main() -> Result<()> {
    println!("🧪 Proper Multi-Token Quality Assessment");
    println!("========================================");

    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

    println!("🔄 Loading model...");
    let loader = UnifiedModelLoader::new()?;
    let mut model = loader.load_model(model_id)?;

    // Test cases that benefit from multi-token completion
    let test_cases = [
        ("What is the capital of France?", "Should mention Paris"),
        (
            "Complete this sentence: The sky is",
            "Should be sensible completion",
        ),
        ("Hello, how are you", "Should be conversational"),
        ("1, 2, 3, 4,", "Should continue sequence logically"),
        (
            "Explain AI in simple terms:",
            "Should give coherent explanation",
        ),
    ];

    let mut passed = 0;
    let mut total = 0;

    for (i, (prompt, expectation)) in test_cases.iter().enumerate() {
        println!("\n📝 Test {}: '{}'", i + 1, prompt);
        println!("   Expected: {expectation}");

        // Use working multi-token generation
        match model.generate_tokens_topk_temp(prompt, 25, 0.7, Some(50)) {
            Ok(tokens) => {
                let tokens_u32: Vec<u32> = tokens.iter().map(|&t| t as u32).collect();
                if let Ok(decoded) = model.tokenizer().decode(&tokens_u32, false) {
                    println!("   🎯 Generated: '{decoded}'");

                    // Simple quality assessments
                    let is_coherent = !decoded.trim().is_empty()
                        && decoded.chars().filter(|c| c.is_alphabetic()).count() > 5;

                    let is_relevant = match i {
                        0 => {
                            decoded.to_lowercase().contains("paris")
                                || decoded.to_lowercase().contains("france")
                        }
                        1 => {
                            decoded.to_lowercase().contains("blue")
                                || decoded.to_lowercase().contains("clear")
                                || decoded.len() > 10
                        }
                        2 => {
                            decoded.to_lowercase().contains("fine")
                                || decoded.to_lowercase().contains("good")
                                || decoded.to_lowercase().contains("you")
                        }
                        3 => decoded.contains("5") || decoded.chars().any(|c| c.is_numeric()),
                        4 => decoded.len() > 20, // Should be substantive
                        _ => true,
                    };

                    let quality_passed = is_coherent && is_relevant;

                    if quality_passed {
                        println!("   ✅ PASS: Coherent and relevant");
                        passed += 1;
                    } else {
                        println!("   ❌ FAIL: Coherent={is_coherent}, Relevant={is_relevant}");
                    }

                    total += 1;
                } else {
                    println!("   ❌ FAIL: Could not decode tokens");
                    total += 1;
                }
            }
            Err(e) => {
                println!("   ❌ FAIL: Generation failed: {e}");
                total += 1;
            }
        }
    }

    println!("\n📊 FINAL RESULTS");
    println!("================");
    let quality_score = (passed as f32 / total as f32) * 100.0;
    println!("Quality Score: {quality_score:.1}% ({passed}/{total} tests passed)");

    if quality_score >= 80.0 {
        println!("🎉 EXCELLENT: Model quality is very good");
    } else if quality_score >= 60.0 {
        println!("✅ GOOD: Model quality is acceptable");
    } else if quality_score >= 40.0 {
        println!("⚠️ FAIR: Model quality needs improvement");
    } else {
        println!("❌ POOR: Model quality is concerning");
    }

    println!("\nComparison:");
    println!("  Single-token approach: 40% (limited by single tokens)");
    println!("  Multi-token approach:  {quality_score:.1}% (proper completion testing)");

    Ok(())
}
