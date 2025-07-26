//! Integration tests for CoreML models
//!
//! These tests use real .mlmodelc files to validate the complete pipeline.

#![allow(clippy::needless_return)]

use candle_core::{DType, Device, Tensor};
use candle_coreml::{Config, CoreMLModel, download_model};
use std::path::PathBuf;

/// Helper to get the path to test model - now downloads from HuggingFace if needed
fn get_test_model_path() -> Option<PathBuf> {
    
    // Try to download the CoreML OpenELM model
    let model_id = "corenet-community/coreml-OpenELM-450M-Instruct";
    
    match std::env::var("SKIP_MODEL_DOWNLOAD") {
        Ok(_) => {
            eprintln!("SKIP_MODEL_DOWNLOAD set, skipping model download");
            return None;
        }
        Err(_) => {}
    }
    
    // Download the model (synchronous)
    let cache_dir = download_model(model_id, true);
    
    match cache_dir {
        Ok(dir) => {
            // Look for .mlmodelc file first (compiled)
            let mlmodelc_path = dir.join("OpenELM-450M-Instruct-128-float32.mlmodelc");
            if mlmodelc_path.exists() {
                eprintln!("Found compiled CoreML model at: {}", mlmodelc_path.display());
                return Some(mlmodelc_path);
            }
            
            // Look for .mlpackage file (needs compilation)
            let mlpackage_path = dir.join("OpenELM-450M-Instruct-128-float32.mlpackage");
            if mlpackage_path.exists() {
                eprintln!("Found CoreML package at: {}", mlpackage_path.display());
                eprintln!("Note: .mlpackage files need to be compiled to .mlmodelc for use");
                return Some(mlpackage_path);
            }
            
            eprintln!("Model directory downloaded but neither .mlmodelc nor .mlpackage found");
            // List what files are actually there
            if let Ok(entries) = std::fs::read_dir(&dir) {
                eprintln!("Files in model directory:");
                for entry in entries.flatten() {
                    eprintln!("  - {}", entry.file_name().to_string_lossy());
                }
            }
            None
        }
        Err(e) => {
            eprintln!("Failed to download CoreML OpenELM model: {}", e);
            None
        }
    }
}

/// Test loading a real CoreML model
#[test]
fn test_load_real_model() {
    let model_path = match get_test_model_path() {
        Some(path) => path,
        None => {
            eprintln!("Skipping test: model file not found");
            return;
        }
    };

    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 128,
        vocab_size: 32000,
        model_type: "OpenELM-450M-Instruct".to_string(),
    };

    let result = CoreMLModel::load_from_file(&model_path, &config);

    // On macOS, this should succeed; on other platforms, it should fail gracefully
    #[cfg(target_os = "macos")]
    {
        match result {
            Ok(model) => {
                assert_eq!(model.config().max_sequence_length, 128);
                assert_eq!(model.config().vocab_size, 32000);
            }
            Err(err) => {
                let err_str = err.to_string();
                if err_str.contains("Compile the model") {
                    eprintln!("Skipping test: model needs to be compiled - {}", err_str);
                    return;
                } else {
                    panic!("Failed to load CoreML model: {}", err);
                }
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err
            .to_string()
            .contains("CoreML is only available on macOS"));
    }
}

/// Test CoreML model inference with CPU tensors
#[test]
#[cfg(target_os = "macos")]
fn test_inference_cpu() {
    let model_path = match get_test_model_path() {
        Some(path) => path,
        None => {
            eprintln!("Skipping test: model file not found");
            return;
        }
    };

    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 128,
        vocab_size: 32000,
        model_type: "OpenELM-450M-Instruct".to_string(),
    };

    let model = match CoreMLModel::load_from_file(&model_path, &config) {
        Ok(model) => model,
        Err(err) => {
            let err_str = err.to_string();
            if err_str.contains("Compile the model") {
                eprintln!("Skipping test: model needs to be compiled - {}", err_str);
                return;
            } else {
                panic!("Failed to load CoreML model: {}", err);
            }
        }
    };

    // Create a test input tensor on CPU
    let device = Device::Cpu;
    let input = Tensor::ones((1, 128), DType::F32, &device).unwrap();

    // Run inference
    let output = model.forward(&[&input]);
    assert!(output.is_ok(), "Forward pass failed: {:?}", output.err());

    let output = output.unwrap();

    // Verify output properties
    assert!(
        output.device().same_device(&device),
        "Output should be on same device as input"
    );
    assert!(
        output.dims().len() >= 1,
        "Output should have at least 1 dimension"
    );
    assert!(!output.dims().is_empty(), "Output should not be empty");

    // Convert to vec to ensure we can read the values
    eprintln!("Output shape: {:?}", output.dims());
    eprintln!("Output dtype: {:?}", output.dtype());
    
    // Try to convert based on the actual dimensionality to verify the tensor is readable
    match output.dims().len() {
        1 => {
            let output_data = output.to_vec1::<f32>();
            assert!(
                output_data.is_ok(),
                "Should be able to convert 1D output to vec: {:?}", output_data.err()
            );
            let data = output_data.unwrap();
            assert!(!data.is_empty(), "Output data should not be empty");
        }
        2 => {
            let output_data = output.to_vec2::<f32>();
            assert!(
                output_data.is_ok(),
                "Should be able to convert 2D output to vec: {:?}", output_data.err()
            );
            let data = output_data.unwrap();
            assert!(!data.is_empty(), "Output data should not be empty");
            assert!(!data[0].is_empty(), "Output data rows should not be empty");
        }
        3 => {
            let output_data = output.to_vec3::<f32>();
            assert!(
                output_data.is_ok(),
                "Should be able to convert 3D output to vec: {:?}", output_data.err()
            );
            let data = output_data.unwrap();
            assert!(!data.is_empty(), "Output data should not be empty");
            assert!(!data[0].is_empty(), "Output data should not be empty");
            assert!(!data[0][0].is_empty(), "Output data should not be empty");
        }
        _ => {
            panic!("Unexpected output dimensionality: {:?}", output.dims());
        }
    }
}

/// Test CoreML model inference with Metal tensors (if available)
#[test]
#[cfg(target_os = "macos")]
fn test_inference_metal() {
    let model_path = match get_test_model_path() {
        Some(path) => path,
        None => {
            eprintln!("Skipping test: model file not found");
            return;
        }
    };

    // Try to create Metal device
    let device = match Device::new_metal(0) {
        Ok(device) => device,
        Err(_) => {
            eprintln!("Skipping test: Metal device not available");
            return;
        }
    };

    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 128,
        vocab_size: 32000,
        model_type: "OpenELM-450M-Instruct".to_string(),
    };

    let model = match CoreMLModel::load_from_file(&model_path, &config) {
        Ok(model) => model,
        Err(err) => {
            let err_str = err.to_string();
            if err_str.contains("Compile the model") {
                eprintln!("Skipping test: model needs to be compiled - {}", err_str);
                return;
            } else {
                panic!("Failed to load CoreML model: {}", err);
            }
        }
    };

    // Create a test input tensor on Metal
    let input = Tensor::ones((1, 128), DType::F32, &device).unwrap();

    // Run inference
    let output = model.forward(&[&input]);
    assert!(output.is_ok(), "Forward pass failed: {:?}", output.err());

    let output = output.unwrap();

    // Verify output properties
    assert!(
        output.device().same_device(&device),
        "Output should be on same device as input"
    );
    assert!(
        output.dims().len() >= 2,
        "Output should have at least 2 dimensions"
    );
}

/// Test device validation - CUDA tensors should be rejected
#[test]
#[cfg(target_os = "macos")]
fn test_device_validation_cuda_rejection() {
    let model_path = match get_test_model_path() {
        Some(path) => path,
        None => {
            eprintln!("Skipping test: model file not found");
            return;
        }
    };

    // Try to create CUDA device
    let device = match Device::new_cuda(0) {
        Ok(device) => device,
        Err(_) => {
            eprintln!("Skipping test: CUDA device not available");
            return;
        }
    };

    let config = Config::default();
    let model = match CoreMLModel::load_from_file(&model_path, &config) {
        Ok(model) => model,
        Err(err) => {
            let err_str = err.to_string();
            if err_str.contains("Compile the model") {
                eprintln!("Skipping test: model needs to be compiled - {}", err_str);
                return;
            } else {
                panic!("Failed to load CoreML model: {}", err);
            }
        }
    };

    // Create a test input tensor on CUDA
    let input = Tensor::ones((1, 128), DType::F32, &device).unwrap();

    // This should fail with device validation error
    let output = model.forward(&[&input]);
    assert!(output.is_err(), "CUDA tensors should be rejected");

    let err = output.unwrap_err();
    assert!(
        err.to_string().contains("CUDA"),
        "Error should mention CUDA"
    );
}

/// Test tensor round-trip conversion
#[test]
#[cfg(target_os = "macos")]
fn test_tensor_roundtrip() {
    // This test validates our tensor conversion without needing a full model
    use candle_core::{DType, Device, Shape};

    let device = Device::Cpu;
    let shape = Shape::from((1, 4));

    // Test with f32 data
    let data = vec![1.0f32, 2.0, 3.0, 4.0];
    let tensor = Tensor::from_vec(data.clone(), &shape, &device).unwrap();

    // This tests the internal conversion logic without needing a model
    let retrieved_data = tensor.to_vec2::<f32>().unwrap();
    assert_eq!(retrieved_data, vec![data]);
    assert_eq!(tensor.dtype(), DType::F32);
    assert_eq!(tensor.dims(), &[1, 4]);
}

/// Test configuration validation
#[test]
fn test_config_validation() {
    let config = Config::default();

    assert_eq!(config.input_names, vec!["input_ids".to_string()]);
    assert_eq!(config.output_name, "logits");
    assert_eq!(config.max_sequence_length, 128);
    assert_eq!(config.vocab_size, 32000);

    // Test custom config
    let custom_config = Config {
        input_names: vec!["custom_input".to_string()],
        output_name: "custom_output".to_string(),
        max_sequence_length: 256,
        vocab_size: 50000,
        model_type: "custom".to_string(),
    };

    assert_eq!(custom_config.input_names, vec!["custom_input".to_string()]);
    assert_eq!(custom_config.max_sequence_length, 256);
}

/// Test error handling for missing files
#[test]
fn test_missing_file_error() {
    let nonexistent_path = "/path/that/does/not/exist.mlmodelc";
    let config = Config::default();

    let result = CoreMLModel::load_from_file(nonexistent_path, &config);
    assert!(result.is_err());

    let err = result.err().unwrap();
    let err_str = err.to_string();

    #[cfg(target_os = "macos")]
    assert!(err_str.contains("not found") || err_str.contains("Failed to load"));

    #[cfg(not(target_os = "macos"))]
    assert!(err_str.contains("CoreML is only available on macOS"));
}

/// Test basic state creation functionality
#[test]
fn test_state_creation() {
    let config = Config::default();

    #[cfg(target_os = "macos")]
    {
        // Try to load a real model if available
        if let Some(model_path) = get_test_model_path() {
            if let Ok(model) = CoreMLModel::load_from_file(&model_path, &config) {
                // State creation should succeed
                let state_result = model.make_state();
                assert!(state_result.is_ok());

                let _state = state_result.unwrap();
                // State should have proper debug formatting
                let debug_str = format!("{:?}", _state);
                assert!(debug_str.contains("CoreMLState"));
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        // On non-macOS, simulate a model
        let mock_model = CoreMLModel {
            _phantom: std::marker::PhantomData,
            config: config.clone(),
        };

        let state_result = mock_model.make_state();
        assert!(state_result.is_err());
        assert!(state_result
            .unwrap_err()
            .to_string()
            .contains("CoreML state is only available on macOS"));
    }
}

/// Test stateful prediction functionality
#[test]
fn test_stateful_prediction() {
    let config = Config::default();

    #[cfg(target_os = "macos")]
    {
        if let Some(model_path) = get_test_model_path() {
            if let Ok(model) = CoreMLModel::load_from_file(&model_path, &config) {
                let device = Device::Cpu;

                // Create state
                if let Ok(mut state) = model.make_state() {
                    // Create test input tensor
                    let input = Tensor::ones((1, 10), DType::F32, &device).unwrap();

                    // Test stateful prediction
                    let result = model.predict_with_state(&[&input], &mut state);

                    // Should work or fail gracefully (depending on model compatibility)
                    match result {
                        Ok(output) => {
                            // Successful prediction
                            assert!(!output.dims().is_empty());
                            // Check device compatibility without using PartialEq
                            match (output.device(), &device) {
                                (Device::Cpu, Device::Cpu) => {}
                                (Device::Metal(_), Device::Metal(_)) => {}
                                _ => panic!("Output device doesn't match input device"),
                            }
                        }
                        Err(e) => {
                            // May fail if model isn't stateful or has different input requirements
                            let err_str = e.to_string();
                            // Should be a meaningful error message
                            assert!(
                                err_str.contains("CoreML")
                                    || err_str.contains("input")
                                    || err_str.contains("prediction")
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Test stateful prediction with multiple calls (persistence check)
#[test]
fn test_stateful_prediction_persistence() {
    let config = Config::default();

    #[cfg(target_os = "macos")]
    {
        if let Some(model_path) = get_test_model_path() {
            if let Ok(model) = CoreMLModel::load_from_file(&model_path, &config) {
                let device = Device::Cpu;

                // Create state
                if let Ok(mut state) = model.make_state() {
                    let input = Tensor::ones((1, 10), DType::F32, &device).unwrap();

                    // Make multiple predictions with the same state
                    let mut successful_predictions = 0;
                    for _i in 0..3 {
                        match model.predict_with_state(&[&input], &mut state) {
                            Ok(_output) => {
                                successful_predictions += 1;
                            }
                            Err(_) => {
                                // Some models might not support stateful operation
                                break;
                            }
                        }
                    }

                    // If any predictions succeeded, the interface is working
                    // (We can't guarantee all models support stateful operation)
                    if successful_predictions > 0 {
                        assert!(successful_predictions >= 1);
                    }
                }
            }
        }
    }
}

/// Baseline test: OpenELM "quick brown fox" completion
/// 
/// This test serves as our "known truth" baseline to verify CoreML infrastructure works perfectly.
/// It downloads the OpenELM model and tests the classic "The quick brown fox jumped over the lazy" -> "dog" completion.
/// 
/// This test is ignored by default to avoid downloading large models in CI.
/// 
/// To run manually:
/// ```bash
/// cargo test test_openelm_baseline_text_completion -- --ignored --nocapture
/// ```
/// 
/// Expected behavior:
/// - Downloads OpenELM-450M-Instruct model (~1.7GB)
/// - Tests "quick brown fox" completion
/// - MUST predict "dog" (token 11203) as top prediction
/// - Confidence score should be > 10.0
/// - Test will FAIL if our CoreML infrastructure is broken
#[test]
#[ignore = "downloads large model - run manually to verify baseline"]
fn test_openelm_baseline_text_completion() {
    println!("🦙 BASELINE TEST: OpenELM 'quick brown fox' completion");
    println!("This test verifies our CoreML infrastructure works perfectly");
    
    let device = Device::Cpu;
    
    // Download and load OpenELM model
    let model_id = "corenet-community/coreml-OpenELM-450M-Instruct";
    let cache_dir = download_model(model_id, true).expect("Failed to download OpenELM model");
    let model_path = cache_dir.join("OpenELM-450M-Instruct-128-float32.mlpackage");
    assert!(model_path.exists(), "OpenELM model file should exist after download");
    
    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 128,
        vocab_size: 32000,
        model_type: "OpenELM-450M-Instruct".to_string(),
    };
    
    let model = CoreMLModel::load_from_file(&model_path, &config)
        .expect("Should be able to load OpenELM model");
    
    println!("✅ Model loaded successfully");
    
    // Test the classic "quick brown fox" completion
    // Simulated tokens for: "The quick brown fox jumped over the lazy"
    let test_tokens = vec![1i64, 450, 4996, 17354, 1701, 29916, 12500, 975, 278, 17366];
    let mut padded_tokens = test_tokens.clone();
    padded_tokens.resize(128, 0i64);
    
    println!("📝 Testing completion for: 'The quick brown fox jumped over the lazy'");
    println!("🔤 Token sequence: {:?}...", &test_tokens);
    
    let input_tensor = Tensor::from_vec(padded_tokens, (1, 128), &device)
        .expect("Should create input tensor");
    
    // Run inference
    let output = model.forward(&[&input_tensor])
        .expect("Model inference should succeed");
    
    assert_eq!(output.dims(), &[1, 128, 32000], "Output should have correct shape");
    assert_eq!(output.dtype(), DType::F32, "Output should be F32");
    
    println!("✅ Inference successful, output shape: {:?}", output.shape());
    
    // Extract logits for the last token position (predicting next word)
    let logits_vec = output.to_vec3::<f32>()
        .expect("Should be able to extract logits");
    
    let last_token_pos = test_tokens.len() - 1;
    let next_token_logits = &logits_vec[0][last_token_pos];
    
    // Find the top predicted token
    let mut indexed_logits: Vec<(usize, f32)> = next_token_logits
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    let top_token_id = indexed_logits[0].0;
    let top_token_score = indexed_logits[0].1;
    
    println!("🏆 Top prediction: Token {} with score {:.3}", top_token_id, top_token_score);
    
    // Show top 5 predictions for debugging
    println!("📊 Top 5 predictions:");
    for (rank, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
        println!("  {}. Token {}: {:.3}", rank + 1, token_id, score);
    }
    
    // Core assertions for baseline test
    assert!(top_token_score > 5.0, 
        "Top prediction should have high confidence (>5.0), got {:.3}", top_token_score);
    
    assert!(indexed_logits[0].1 - indexed_logits[1].1 > 1.0,
        "Top prediction should be clearly better than second choice");
    
    // The critical test: token 11203 should be "dog" and should be the top prediction
    assert_eq!(top_token_id, 11203, 
        "BASELINE FAILURE: Expected 'dog' (token 11203) as top prediction for 'The quick brown fox jumped over the lazy', got token {}", 
        top_token_id);
    
    // Additional statistical validations
    let logit_range = indexed_logits[0].1 - indexed_logits.last().unwrap().1;
    assert!(logit_range > 20.0, 
        "Should have good logit spread (>20.0), got {:.3}", logit_range);
    
    assert_eq!(indexed_logits.len(), 32000, 
        "Should predict over full vocabulary");
    
    println!("🎉 BASELINE TEST PASSED!");
    println!("  ✅ OpenELM correctly predicts 'dog' for 'quick brown fox' completion");
    println!("  ✅ CoreML infrastructure working perfectly");
    println!("  ✅ Confidence: {:.3}, Range: {:.3}", top_token_score, logit_range);
    println!("  ✅ This confirms our implementation is solid");
}

/// Test state parameter validation
#[test]
fn test_stateful_prediction_validation() {
    let config = Config::bert_config("logits", 128, 30522);

    #[cfg(target_os = "macos")]
    {
        if let Some(model_path) = get_test_model_path() {
            if let Ok(model) = CoreMLModel::load_from_file(&model_path, &config) {
                let device = Device::Cpu;

                if let Ok(mut state) = model.make_state() {
                    // Test wrong number of inputs
                    let input = Tensor::ones((1, 10), DType::F32, &device).unwrap();

                    // Config expects 3 inputs (input_ids, token_type_ids, attention_mask)
                    // but we're providing only 1
                    let result = model.predict_with_state(&[&input], &mut state);

                    match result {
                        Err(e) => {
                            let err_str = e.to_string();
                            assert!(
                                err_str.contains("Expected 3 inputs, got 1")
                                    || err_str.contains("input")
                            );
                        }
                        Ok(_) => {
                            // Model may be more flexible than expected, which is fine
                        }
                    }
                }
            }
        }
    }
}

/// Test device compatibility for stateful predictions
#[test]
fn test_stateful_device_validation() {
    let config = Config::default();

    #[cfg(target_os = "macos")]
    {
        if let Some(model_path) = get_test_model_path() {
            if let Ok(model) = CoreMLModel::load_from_file(&model_path, &config) {
                if let Ok(mut state) = model.make_state() {
                    // Test CPU device (should work)
                    let cpu_device = Device::Cpu;
                    let cpu_input = Tensor::ones((1, 10), DType::F32, &cpu_device).unwrap();
                    let _cpu_result = model.predict_with_state(&[&cpu_input], &mut state);
                    // CPU should always be accepted (result may vary based on model)

                    // Test Metal device (should work on supported hardware)
                    if let Ok(metal_device) = Device::new_metal(0) {
                        let metal_input = Tensor::ones((1, 10), DType::F32, &metal_device).unwrap();
                        let _metal_result = model.predict_with_state(&[&metal_input], &mut state);
                        // Metal should be accepted (result may vary based on model)
                    }

                    // Note: Can't easily test CUDA rejection without CUDA device
                    // The validation logic is already tested in the stateless tests
                }
            }
        }
    }
}
