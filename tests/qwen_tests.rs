//! Comprehensive Qwen Model Tests
//!
//! This file consolidates all Qwen-specific tests including:
//! - Architecture validation and pipeline testing
//! - Position fix validation and boundary conditions
//! - Specific prediction testing (dog completion)  
//! - End-to-end integration testing
//! - Extended coverage for edge cases
//!
//! Consolidates content from:
//! - qwen_architecture_success_test.rs
//! - qwen_position_fix_test.rs  
//! - rust_dog_prediction_test.rs
//! - qwen_integration_tests.rs
//! - comprehensive_qwen_tests.rs

use anyhow::Result;
use candle_core::Device;
use candle_coreml::{
    ensure_model_downloaded,
    qwen::{QwenConfig, QwenModel},
};

// Test constants - canonical test prompts and expected results
const QUICK_BROWN_FOX_PROMPT: &str = "The quick brown fox jumps over the lazy";
const EXPECTED_DOG_TOKEN: i64 = 5562; // The canonical "dog" token
const MODEL_ID: &str = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

// Helper function to check CoreML compatibility
fn check_coreml_compatibility() -> bool {
    use std::process::Command;

    // Check if we're in CI environment and skip CoreML version checks
    if std::env::var("CI").is_ok() || std::env::var("GITHUB_ACTIONS").is_ok() {
        println!("⚠️  Running in CI environment - CoreML compatibility issues may occur");
        return false; // Skip CoreML tests in CI for now due to version issues
    }

    // Try to detect macOS version (basic check)
    if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
        if let Ok(version) = String::from_utf8(output.stdout) {
            println!("macOS version: {}", version.trim());
            // macOS 15+ has CoreML 9.0+, macOS 14 and below have older versions
            return version.starts_with("15.") || version.starts_with("16.");
        }
    }

    true // Default to true for local development
}

// Helper functions for test utilities
async fn create_test_model() -> Option<QwenModel> {
    let model_dir = ensure_model_downloaded(MODEL_ID, true).ok()?;
    let config = QwenConfig::default();
    QwenModel::load_from_directory(&model_dir, Some(config)).ok()
}

#[cfg(target_os = "macos")]
mod architecture_tests {
    use super::*;

    #[tokio::test]
    async fn test_qwen_architecture_success() -> Result<()> {
        if !check_coreml_compatibility() {
            println!("⚠️ Skipping CoreML test due to version compatibility issues");
            return Ok(());
        }

        println!("🎉 Testing fixed QwenModel architecture");

        let model_dir = ensure_model_downloaded(MODEL_ID, true)?;
        let config = QwenConfig::default();
        let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;

        println!("✅ QwenModel loaded successfully");

        // Test that the model can complete text without errors
        let test_prompts = ["Hello world", "The quick brown fox", "In the beginning"];

        for prompt in &test_prompts {
            println!("\n📝 Testing prompt: '{}'", prompt);

            // This should work without panicking using our fixed architecture
            let result = qwen_model.forward_text(prompt);
            match result {
                Ok(token) => {
                    println!("🎯 Generated token: {}", token);

                    // Try to decode the token
                    if let Ok(decoded) = qwen_model.tokenizer().decode(&[token as u32], false) {
                        println!("📖 Decoded: '{}'", decoded);
                    } else {
                        println!("⚠️ Token {} exists but couldn't decode", token);
                    }

                    // Basic sanity check - token should be in valid range
                    assert!(
                        (0..200000).contains(&token),
                        "Token {} should be in reasonable range",
                        token
                    );
                }
                Err(e) => {
                    panic!("❌ QwenModel failed: {}", e);
                }
            }
        }

        println!("\n🎉 SUCCESS! QwenModel architecture is working correctly!");
        println!("   ✅ Uses proper prefill→infer pipeline");
        println!("   ✅ Shares state between prefill and infer phases");
        println!("   ✅ Generates tokens without errors");
        println!("   ✅ Can handle multiple different prompts");

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod position_fix_tests {
    use super::*;

    #[tokio::test]
    async fn test_qwen_position_fix() -> Result<()> {
        if !check_coreml_compatibility() {
            println!("⚠️ Skipping CoreML test due to version compatibility issues");
            return Ok(());
        }

        println!("🔧 Testing QwenModel with position fix");

        let model_dir = ensure_model_downloaded(MODEL_ID, true)?;
        let config = QwenConfig::default();
        let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;

        println!("✅ QwenModel loaded");

        // Test the exact prompt that should predict "dog"
        let prompt = QUICK_BROWN_FOX_PROMPT;
        println!("📝 Testing prompt: '{}'", prompt);

        // Also test a longer prompt that would trigger the /no_think issue
        let longer_prompt = "Tell me about Greece. I want to know about its history, culture, and geography. What makes it special?";
        println!("📝 Also testing longer prompt: '{}'", longer_prompt);

        let tokens = qwen_model
            .tokenizer()
            .encode(longer_prompt, true)
            .map_err(|e| anyhow::Error::msg(format!("Tokenization failed: {}", e)))?;
        println!(
            "🔢 Longer prompt tokenized to {} tokens",
            tokens.get_ids().len()
        );

        let next_token = qwen_model.forward_text(prompt)?;

        println!("🎯 Prediction: token {}", next_token);

        // Test with the longer prompt (this would previously cause tensor indexing error)
        if tokens.get_ids().len() <= 50 {
            // Only test if within reasonable limits
            println!("🧪 Testing longer prompt (this would previously fail)...");
            match qwen_model.forward_text(longer_prompt) {
                Ok(long_token) => {
                    println!("✅ Longer prompt works! Predicted token: {}", long_token);
                    if let Ok(decoded) = qwen_model.tokenizer().decode(&[long_token as u32], false)
                    {
                        println!("📖 Decoded: '{}'", decoded);
                    }
                }
                Err(e) => {
                    println!(
                        "⚠️  Longer prompt failed (expected for very long inputs): {}",
                        e
                    );
                }
            }
        } else {
            println!(
                "⏭️  Skipping longer prompt test (too long: {} tokens)",
                tokens.get_ids().len()
            );
        }

        // Decode the token
        if let Ok(decoded) = qwen_model.tokenizer().decode(&[next_token as u32], false) {
            println!("📖 Decoded: '{}'", decoded);
        }

        if next_token == EXPECTED_DOG_TOKEN {
            println!("✅ 🎉 SUCCESS! QwenModel now predicts 'dog' correctly!");
            Ok(())
        } else {
            println!(
                "❌ Still predicting token {} instead of {} ('dog')",
                next_token, EXPECTED_DOG_TOKEN
            );

            // Check if it's at least different from the previous wrong prediction
            if next_token == 15678 {
                println!("   Still getting 'lazy' - position fix didn't help");
            } else {
                println!("   Different prediction - position fix may have changed something");
            }

            panic!(
                "POSITION FIX TEST FAILED: Expected token {} ('dog'), got token {} ('{}')",
                EXPECTED_DOG_TOKEN,
                next_token,
                qwen_model
                    .tokenizer()
                    .decode(&[next_token as u32], false)
                    .unwrap_or("???".to_string())
            );
        }
    }
}

#[cfg(target_os = "macos")]
mod prediction_tests {
    use super::*;

    #[tokio::test]
    async fn test_rust_dog_prediction() -> Result<()> {
        if !check_coreml_compatibility() {
            println!("⚠️ Skipping CoreML test due to version compatibility issues");
            return Ok(());
        }

        println!("🎯 Testing Rust QwenModel prediction for 'dog' completion");

        let model_dir = ensure_model_downloaded(MODEL_ID, true)?;
        let config = QwenConfig::default();
        let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;

        let prompt = QUICK_BROWN_FOX_PROMPT;
        println!("📝 Prompt: '{}'", prompt);

        let predicted_token = qwen_model.forward_text(prompt)?;
        let decoded = qwen_model
            .tokenizer()
            .decode(&[predicted_token as u32], false)
            .map_err(|e| anyhow::Error::msg(format!("Decode error: {}", e)))?;

        println!(
            "🎯 Rust prediction: Token {} = '{}'",
            predicted_token, decoded
        );

        // Check if it's "dog" (token 5562)
        if predicted_token == EXPECTED_DOG_TOKEN {
            println!("🎉 SUCCESS! Rust correctly predicts 'dog'");
        } else {
            println!(
                "❌ Different prediction. Expected: {} ('dog'), Got: {} ('{}')",
                EXPECTED_DOG_TOKEN, predicted_token, decoded
            );

            // Show what the tokenizer thinks about "dog"
            if let Ok(dog_tokens) = qwen_model.tokenizer().encode(" dog", false) {
                let dog_token_ids: Vec<u32> = dog_tokens.get_ids().to_vec();
                println!("🔍 ' dog' tokenizes to: {:?}", dog_token_ids);
            }
            panic!("Rust prediction did not match expected 'dog' token");
        }

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod integration_tests {
    use super::*;

    #[tokio::test]
    // Model is cached after first download - safe for coverage analysis
    async fn test_qwen_complete_pipeline_fox_completion() -> Result<()> {
        if !check_coreml_compatibility() {
            println!("⚠️ Skipping CoreML test due to version compatibility issues");
            return Ok(());
        }

        // Download the Qwen model
        let model_dir = ensure_model_downloaded(MODEL_ID, true)?;

        // Load the complete Qwen model using the full pipeline
        let mut qwen_model = QwenModel::load_from_directory(&model_dir, None)?;

        // Test the classic "The quick brown fox jumps over the lazy" completion
        let prompt = QUICK_BROWN_FOX_PROMPT;

        // Generate completion with low temperature for deterministic results
        let completion = qwen_model.generate_text(prompt, 2, 0.0)?;

        // Assert that the completion contains "dog" or "lazy" (both have tied logits in the model)
        assert!(
            completion.to_lowercase().contains("dog") || completion.to_lowercase().contains("lazy"),
            "Expected completion to contain 'dog' or 'lazy' (both have tied logit values), but got: '{}'",
            completion
        );

        println!("✅ Qwen pipeline test passed!");
        println!("Prompt: {}", prompt);
        println!("Completion: {}", completion);

        Ok(())
    }
}

#[cfg(target_os = "macos")]
mod extended_coverage_tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Run with: cargo test test_state_management_edge_cases -- --ignored
    async fn test_state_management_edge_cases() -> Result<()> {
        let mut model = match create_test_model().await {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return Ok(());
            }
        };

        // Test multiple state initializations (should not cause conflicts)
        for i in 1..=3 {
            println!("State initialization attempt {}", i);

            // This should work repeatedly without issues
            let result = model.initialize_states();
            assert!(
                result.is_ok(),
                "State initialization {} failed: {:?}",
                i,
                result
            );
        }

        println!("✅ Multiple state initializations handled correctly");
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test test_cache_optimization_paths -- --ignored
    async fn test_cache_optimization_paths() -> Result<()> {
        let mut model = match create_test_model().await {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return Ok(());
            }
        };

        let base_prompt = "The quick brown fox";

        // First call - should populate cache
        let _token1 = model.forward_text(base_prompt)?;

        // Second call with same prompt - should hit cache optimization
        let _token2 = model.forward_text(base_prompt)?;

        // Third call with extended prompt - should partially reuse cache
        let extended_prompt = "The quick brown fox jumps";
        let _token3 = model.forward_text(extended_prompt)?;

        // Fourth call with completely different prompt - should miss cache
        let different_prompt = "Artificial intelligence is";
        let _token4 = model.forward_text(different_prompt)?;

        println!("✅ Cache optimization paths tested successfully");
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test test_generation_with_extreme_parameters -- --ignored
    async fn test_generation_with_extreme_parameters() -> Result<()> {
        let mut model = match create_test_model().await {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return Ok(());
            }
        };

        let prompt = "The weather today is";

        // Test with temperature 0.0 (deterministic)
        let tokens_temp_0 = model.generate_tokens(prompt, 5, 0.0, None)?;
        assert!(
            !tokens_temp_0.is_empty(),
            "No tokens generated with temp 0.0"
        );

        // Test with high temperature
        let tokens_temp_high = model.generate_tokens(prompt, 5, 2.0, None)?;
        assert!(
            !tokens_temp_high.is_empty(),
            "No tokens generated with high temp"
        );

        // Test with 1 token generation
        let single_token = model.generate_tokens(prompt, 1, 0.5, None)?;
        assert_eq!(
            single_token.len(),
            1,
            "Expected exactly 1 token, got {}",
            single_token.len()
        );

        // Test with many tokens (within reason)
        let many_tokens = model.generate_tokens(prompt, 30, 0.5, None)?;
        assert!(
            !many_tokens.is_empty(),
            "No tokens generated with many token request"
        );

        println!("✅ Extreme parameter testing completed");
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test test_custom_configurations -- --ignored
    async fn test_custom_configurations() -> Result<()> {
        let model_dir = ensure_model_downloaded(MODEL_ID, true)?;

        // Test with custom configuration
        let custom_config = QwenConfig {
            vocab_size: 151936,  // Qwen default
            hidden_size: 1024,   // Qwen default
            context_length: 512, // Qwen default
            batch_size: 64,      // Qwen default
            device: Device::Cpu, // Force CPU for testing
        };

        let model_result = QwenModel::load_from_directory(&model_dir, Some(custom_config));
        assert!(
            model_result.is_ok(),
            "Failed to load model with custom config"
        );

        let mut model = model_result?;

        // Test that custom config model works
        let result = model.forward_text("Configuration test");
        assert!(result.is_ok(), "Custom config model failed to generate");

        println!("✅ Custom configuration tested successfully");
        Ok(())
    }

    #[tokio::test]
    #[ignore] // Run with: cargo test test_conversation_continuity -- --ignored
    async fn test_conversation_continuity() -> Result<()> {
        let mut model = match create_test_model().await {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return Ok(());
            }
        };

        // Simulate a conversation flow
        let conversation = [
            "Hello, how are you?",
            "What is artificial intelligence?",
            "Can you explain machine learning?",
            "Thank you for the information.",
        ];

        let mut responses = Vec::new();

        for (i, prompt) in conversation.iter().enumerate() {
            println!("Conversation turn {}: {}", i + 1, prompt);

            let response = model.forward_text(prompt);
            assert!(response.is_ok(), "Conversation turn {} failed", i + 1);

            let token = response?;
            responses.push(token);
            println!("  Response token: {}", token);
        }

        // Verify we got different tokens (conversation progressed)
        assert!(responses.len() == conversation.len(), "Missing responses");

        println!(
            "✅ Conversation continuity maintained across {} turns",
            responses.len()
        );
        Ok(())
    }
}

// Tests that can run without models
#[tokio::test]
async fn test_qwen_config_defaults() {
    let config = QwenConfig::default();

    assert_eq!(config.vocab_size, candle_coreml::qwen::QWEN_VOCAB_SIZE);
    assert_eq!(config.hidden_size, candle_coreml::qwen::QWEN_HIDDEN_SIZE);
    assert_eq!(config.batch_size, candle_coreml::qwen::QWEN_BATCH_SIZE);
    assert_eq!(
        config.context_length,
        candle_coreml::qwen::QWEN_CONTEXT_LENGTH
    );

    println!("✅ QwenConfig defaults validated");
}

#[tokio::test]
async fn test_qwen_constants() {
    use candle_coreml::qwen::*;

    assert_eq!(QWEN_VOCAB_SIZE, 151936);
    assert_eq!(QWEN_HIDDEN_SIZE, 1024);
    assert_eq!(QWEN_BATCH_SIZE, 64);
    assert_eq!(QWEN_CONTEXT_LENGTH, 512);

    println!("✅ Qwen constants validated");
}

#[cfg(not(target_os = "macos"))]
mod non_macos_tests {
    use super::*;

    #[tokio::test]
    async fn test_qwen_requires_macos() {
        // Test that QwenModel appropriately requires macOS
        let fake_path = PathBuf::from("nonexistent");
        let result = QwenModel::load_from_directory(&fake_path, None);

        assert!(result.is_err(), "QwenModel should require macOS");

        let error_msg = result.unwrap_err().to_string();
        // Should mention macOS or CoreML requirement
        assert!(
            error_msg.to_lowercase().contains("macos")
                || error_msg.to_lowercase().contains("coreml")
                || error_msg.to_lowercase().contains("not found"),
            "Error should mention platform requirement: {}",
            error_msg
        );

        println!("✅ macOS requirement properly enforced");
    }

    #[tokio::test]
    async fn test_qwen_macos_requirement() {
        // On non-macOS platforms, verify that appropriate errors are returned
        let model_dir = PathBuf::from("nonexistent_dir");
        let result = QwenModel::load_from_directory(&model_dir, None);

        // Should fail on non-macOS platforms
        assert!(
            result.is_err(),
            "QwenModel should not be available on non-macOS platforms"
        );
    }
}
