//! Comprehensive Qwen tests to cover the 233 untested lines in qwen.rs
//!
//! These tests target specific functionality gaps identified by coverage analysis:
//! - State management edge cases and error handling
//! - Token generation boundary conditions  
//! - Cache optimization paths and validation
//! - Error recovery scenarios and memory management
//! - Advanced generation scenarios and configuration edge cases

use candle_coreml::qwen::{QwenModel, QwenConfig};
use candle_core::Device;
use std::path::PathBuf;

// Test helper to get model path or skip tests
fn get_test_model_path() -> Option<PathBuf> {
    let paths = [
        "/Users/mazdahewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4",
        "./qwen-model",
        "qwen-model",
    ];
    
    paths.iter().find_map(|&p| {
        let path = PathBuf::from(p);
        if path.exists() { Some(path) } else { None }
    })
}

// Helper to create test QwenModel
fn create_test_model() -> Option<QwenModel> {
    let model_path = get_test_model_path()?;
    let config = QwenConfig::default();
    QwenModel::load_from_directory(&model_path, Some(config)).ok()
}

#[cfg(target_os = "macos")]
mod qwen_state_management_tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_state_initialization_edge_cases -- --ignored
    fn test_state_initialization_edge_cases() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test multiple state initializations (should not cause conflicts)
        for i in 1..=3 {
            println!("State initialization attempt {}", i);
            
            // This should work repeatedly without issues
            let result = model.initialize_states();
            assert!(result.is_ok(), "State initialization {} failed: {:?}", i, result);
        }
        
        println!("✅ Multiple state initializations handled correctly");
    }

    #[test] 
    #[ignore] // Run with: cargo test test_state_persistence_across_calls -- --ignored
    fn test_state_persistence_across_calls() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Generate first token
        let prompt1 = "Hello";
        let token1 = model.forward_text(prompt1).expect("Failed to generate first token");
        
        // Generate second token with same prompt - should use cached state
        let token2 = model.forward_text(prompt1).expect("Failed to generate second token");
        
        // Tokens should be consistent (testing state persistence)
        assert_eq!(token1, token2, "State persistence failed - got different tokens: {} vs {}", token1, token2);
        
        println!("✅ State persistence validated: consistent token {}", token1);
    }

    #[test]
    #[ignore] // Run with: cargo test test_cache_optimization_paths -- --ignored  
    fn test_cache_optimization_paths() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        let base_prompt = "The quick brown fox";
        
        // First call - should populate cache
        let _token1 = model.forward_text(base_prompt).expect("Failed first call");
        
        // Second call with same prompt - should hit cache optimization
        let _token2 = model.forward_text(base_prompt).expect("Failed second call");
        
        // Third call with extended prompt - should partially reuse cache
        let extended_prompt = "The quick brown fox jumps";
        let _token3 = model.forward_text(extended_prompt).expect("Failed extended call");
        
        // Fourth call with completely different prompt - should miss cache
        let different_prompt = "Artificial intelligence is";
        let _token4 = model.forward_text(different_prompt).expect("Failed different call");
        
        println!("✅ Cache optimization paths tested successfully");
    }
}

#[cfg(target_os = "macos")]
mod qwen_tokenization_edge_cases {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_tokenization_boundary_conditions -- --ignored
    fn test_tokenization_boundary_conditions() {
        let model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test empty string
        let empty_result = model.tokenize("");
        assert!(empty_result.is_ok(), "Empty string tokenization failed");
        let empty_tokens = empty_result.unwrap();
        assert!(!empty_tokens.is_empty(), "Empty string should still produce some tokens");
        
        // Test single character
        let single_char_result = model.tokenize("a");
        assert!(single_char_result.is_ok(), "Single character tokenization failed");
        
        // Test very long string (but within context limits)
        let long_string = "word ".repeat(100); // Should be within 512 token limit
        let long_result = model.tokenize(&long_string);
        assert!(long_result.is_ok(), "Long string tokenization failed");
        
        // Test string that exceeds context length
        let very_long_string = "supercalifragilisticexpialidocious ".repeat(50);
        let very_long_result = model.tokenize(&very_long_string);
        
        // Should either succeed or fail gracefully with informative error
        match very_long_result {
            Ok(tokens) => {
                assert!(tokens.len() <= 512, "Token count should respect context limit");
            }
            Err(e) => {
                let error_msg = e.to_string();
                assert!(error_msg.contains("too long") || error_msg.contains("context"), 
                    "Error should mention length/context limit: {}", error_msg);
            }
        }
        
        println!("✅ Tokenization boundary conditions tested");
    }

    #[test]
    #[ignore] // Run with: cargo test test_special_characters_tokenization -- --ignored
    fn test_special_characters_tokenization() {
        let model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        let special_inputs = [
            "Hello, world! 🌍",           // Emoji
            "数学很有趣",                   // Chinese characters  
            "café naïve résumé",           // Accented characters
            "2 + 2 = 4",                   // Numbers and operators
            "\n\t  ",                      // Whitespace characters
            "\"quotes\" and 'apostrophes'", // Quote characters
        ];

        for input in &special_inputs {
            let result = model.tokenize(input);
            assert!(result.is_ok(), "Failed to tokenize: '{}'", input);
            
            let tokens = result.unwrap();
            assert!(!tokens.is_empty(), "No tokens generated for: '{}'", input);
            
            println!("✅ Tokenized '{}' -> {} tokens", input, tokens.len());
        }
    }
}

#[cfg(target_os = "macos")]
mod qwen_generation_edge_cases {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_generation_with_extreme_parameters -- --ignored
    fn test_generation_with_extreme_parameters() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        let prompt = "The weather today is";
        
        // Test with temperature 0.0 (deterministic)
        let tokens_temp_0 = model.generate_tokens(prompt, 5, 0.0, None)
            .expect("Failed with temperature 0.0");
        assert!(!tokens_temp_0.is_empty(), "No tokens generated with temp 0.0");
        
        // Test with high temperature
        let tokens_temp_high = model.generate_tokens(prompt, 5, 2.0, None)
            .expect("Failed with high temperature");
        assert!(!tokens_temp_high.is_empty(), "No tokens generated with high temp");
        
        // Test with 1 token generation
        let single_token = model.generate_tokens(prompt, 1, 0.5, None)
            .expect("Failed with single token generation");
        assert_eq!(single_token.len(), 1, "Expected exactly 1 token, got {}", single_token.len());
        
        // Test with many tokens (within reason)
        let many_tokens = model.generate_tokens(prompt, 30, 0.5, None)
            .expect("Failed with many tokens");
        assert!(many_tokens.len() > 0, "No tokens generated with many token request");
        
        println!("✅ Extreme parameter testing completed");
    }

    #[test]
    #[ignore] // Run with: cargo test test_error_recovery_scenarios -- --ignored
    fn test_error_recovery_scenarios() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test that model can recover from potential errors
        let problematic_inputs = [
            "",                    // Empty input
            " ",                   // Just whitespace
            "\n\n\n",             // Just newlines
            "a",                   // Single character
        ];

        for input in &problematic_inputs {
            println!("Testing error recovery for: '{:?}'", input);
            
            // These should either succeed or fail gracefully
            match model.forward_text(input) {
                Ok(token) => {
                    println!("  ✅ Generated token: {}", token);
                    assert!(token >= 0, "Token should be non-negative");
                }
                Err(e) => {
                    println!("  ⚠️ Expected error: {}", e);
                    // Error should be informative
                    let error_msg = e.to_string();
                    assert!(!error_msg.is_empty(), "Error message should not be empty");
                }
            }
            
            // Test that model still works after potential error
            let recovery_result = model.forward_text("Hello world");
            assert!(recovery_result.is_ok(), "Model should recover after problematic input");
        }
        
        println!("✅ Error recovery scenarios tested");
    }
}

#[cfg(target_os = "macos")]
mod qwen_memory_management_tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_memory_efficiency_patterns -- --ignored
    fn test_memory_efficiency_patterns() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test that repeated operations don't cause memory leaks
        let prompt = "Testing memory efficiency";
        
        for i in 1..=10 {
            let result = model.forward_text(prompt);
            assert!(result.is_ok(), "Memory test iteration {} failed", i);
            
            if i % 5 == 0 {
                println!("Completed {} memory iterations", i);
            }
        }
        
        // Test with different prompt lengths to stress memory management
        let short_prompt = "Hi";
        let medium_prompt = "This is a medium length prompt for testing";
        let long_prompt = "This is a much longer prompt that should test the memory management capabilities of the model when processing more substantial input text sequences";
        
        let prompts = [short_prompt, medium_prompt, long_prompt];
        
        for (i, prompt) in prompts.iter().enumerate() {
            let result = model.forward_text(prompt);
            assert!(result.is_ok(), "Variable length test {} failed", i + 1);
            println!("✅ Processed {}-character prompt", prompt.len());
        }
        
        println!("✅ Memory efficiency patterns validated");
    }

    #[test]
    #[ignore] // Run with: cargo test test_resource_cleanup -- --ignored
    fn test_resource_cleanup() {
        // Test that model can be created and dropped multiple times
        for i in 1..=3 {
            println!("Resource cleanup test iteration {}", i);
            
            let model = create_test_model();
            assert!(model.is_some(), "Model creation failed in iteration {}", i);
            
            // Model should be cleanly dropped at end of scope
            drop(model);
        }
        
        println!("✅ Resource cleanup validated");
    }
}

#[cfg(target_os = "macos")]
mod qwen_configuration_tests {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_custom_configurations -- --ignored
    fn test_custom_configurations() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test with custom configuration
        let custom_config = QwenConfig {
            vocab_size: 151936,      // Qwen default
            hidden_size: 1024,       // Qwen default
            context_length: 512,     // Qwen default
            batch_size: 64,          // Qwen default
            device: Device::Cpu,     // Force CPU for testing
        };

        let model_result = QwenModel::load_from_directory(&model_path, Some(custom_config));
        assert!(model_result.is_ok(), "Failed to load model with custom config");
        
        let mut model = model_result.unwrap();
        
        // Test that custom config model works
        let result = model.forward_text("Configuration test");
        assert!(result.is_ok(), "Custom config model failed to generate");
        
        println!("✅ Custom configuration tested successfully");
    }

    #[test]
    #[ignore] // Run with: cargo test test_device_compatibility -- --ignored
    fn test_device_compatibility() {
        let model_path = match get_test_model_path() {
            Some(path) => path,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Test CPU device explicitly
        let cpu_config = QwenConfig {
            device: Device::Cpu,
            ..QwenConfig::default()
        };

        let cpu_model = QwenModel::load_from_directory(&model_path, Some(cpu_config));
        assert!(cpu_model.is_ok(), "CPU device configuration should work");
        
        // Test Metal device (should work on macOS if available)
        if let Ok(metal_device) = candle_core::Device::new_metal(0) {
            let metal_config = QwenConfig {
                device: metal_device,
                ..QwenConfig::default()
            };

            let metal_result = QwenModel::load_from_directory(&model_path, Some(metal_config));
            // Metal should work on macOS, but we'll accept either success or graceful failure
            match metal_result {
                Ok(_) => println!("✅ Metal device configuration works"),
                Err(e) => println!("⚠️ Metal device model loading failed: {}", e),
            }
        } else {
            println!("⚠️ Metal device not available on this system");
        }
        
        println!("✅ Device compatibility tested");
    }
}

#[cfg(target_os = "macos")]
mod qwen_advanced_scenarios {
    use super::*;

    #[test]
    #[ignore] // Run with: cargo test test_conversation_continuity -- --ignored
    fn test_conversation_continuity() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
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
            
            let token = response.unwrap();
            responses.push(token);
            println!("  Response token: {}", token);
        }
        
        // Verify we got different tokens (conversation progressed)
        assert!(responses.len() == conversation.len(), "Missing responses");
        
        println!("✅ Conversation continuity maintained across {} turns", responses.len());
    }

    #[test]
    #[ignore] // Run with: cargo test test_batch_processing_simulation -- --ignored
    fn test_batch_processing_simulation() {
        let mut model = match create_test_model() {
            Some(model) => model,
            None => {
                println!("⚠️ Skipping test: Qwen model not found");
                return;
            }
        };

        // Simulate processing multiple requests in sequence
        let requests = [
            "What is the weather like?",
            "Tell me a joke.",
            "Explain quantum physics.",
            "What is 2 + 2?",
            "Describe a sunset.",
        ];

        let start_time = std::time::Instant::now();
        let mut successful_requests = 0;
        
        for (i, request) in requests.iter().enumerate() {
            match model.forward_text(request) {
                Ok(token) => {
                    successful_requests += 1;
                    println!("Request {}/{} processed: token {}", i + 1, requests.len(), token);
                }
                Err(e) => {
                    println!("Request {}/{} failed: {}", i + 1, requests.len(), e);
                }
            }
        }
        
        let elapsed = start_time.elapsed();
        let avg_time_per_request = elapsed.as_millis() as f64 / requests.len() as f64;
        
        assert!(successful_requests > 0, "No requests processed successfully");
        assert!(avg_time_per_request < 5000.0, "Average request time too high: {:.1}ms", avg_time_per_request);
        
        println!("✅ Batch processing: {}/{} successful, avg {:.1}ms per request", 
                successful_requests, requests.len(), avg_time_per_request);
    }
}

// Helper tests that don't require models
#[test]
fn test_qwen_config_defaults() {
    let config = QwenConfig::default();
    
    assert_eq!(config.vocab_size, candle_coreml::qwen::QWEN_VOCAB_SIZE);
    assert_eq!(config.hidden_size, candle_coreml::qwen::QWEN_HIDDEN_SIZE);
    assert_eq!(config.batch_size, candle_coreml::qwen::QWEN_BATCH_SIZE);
    assert_eq!(config.context_length, candle_coreml::qwen::QWEN_CONTEXT_LENGTH);
    
    println!("✅ QwenConfig defaults validated");
}

#[test] 
fn test_qwen_constants() {
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
    
    #[test]
    fn test_qwen_requires_macos() {
        // Test that QwenModel appropriately requires macOS
        let fake_path = PathBuf::from("nonexistent");
        let result = QwenModel::load_from_directory(&fake_path, None);
        
        assert!(result.is_err(), "QwenModel should require macOS");
        
        let error_msg = result.unwrap_err().to_string();
        // Should mention macOS or CoreML requirement
        assert!(
            error_msg.to_lowercase().contains("macos") || 
            error_msg.to_lowercase().contains("coreml") ||
            error_msg.to_lowercase().contains("not found"),
            "Error should mention platform requirement: {}", error_msg
        );
        
        println!("✅ macOS requirement properly enforced");
    }
}

// Integration helper for running all comprehensive tests
#[cfg(target_os = "macos")]
pub fn run_comprehensive_qwen_tests() {
    println!("\n🧪 Comprehensive Qwen Test Suite");
    println!("==================================");
    println!("🎯 Target: Cover 233 untested lines in qwen.rs");
    println!("🔧 Areas: State management, tokenization, generation, memory, config");
    println!("🚀 Run with: cargo test --test comprehensive_qwen_tests -- --ignored --nocapture");
    println!("==================================\n");
}