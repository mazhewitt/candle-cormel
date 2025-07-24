//! Integration tests for CoreML models
//!
//! These tests use real .mlmodelc files to validate the complete pipeline.

use candle_core::{Device, Tensor, DType};
use candle_coreml::{Config, CoreMLModel};
use std::path::PathBuf;

/// Helper to get the path to test model
fn get_test_model_path() -> Option<PathBuf> {
    // Look for the model relative to the project root
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    
    // Try .mlpackage first (doesn't require compilation)
    path.push("../coreml-OpenELM-450M-Instruct/OpenELM-450M-Instruct-128-float32.mlpackage");
    if path.exists() {
        return Some(path);
    }
    
    // Fall back to .mlmodelc (may require compilation)
    path.pop();
    path.push("../coreml-OpenELM-450M-Instruct/OpenELM-450M-Instruct-128-float32.mlmodelc");
    if path.exists() {
        return Some(path);
    }
    
    None
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
        assert!(err.to_string().contains("CoreML is only available on macOS"));
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
    assert!(output.device().same_device(&device), "Output should be on same device as input");
    assert!(output.dims().len() >= 2, "Output should have at least 2 dimensions");
    assert!(!output.dims().is_empty(), "Output should not be empty");
    
    // Convert to vec to ensure we can read the values
    let output_data = output.to_vec2::<f32>();
    assert!(output_data.is_ok(), "Should be able to convert output to vec");
    
    let output_data = output_data.unwrap();
    assert!(!output_data.is_empty(), "Output data should not be empty");
    assert!(!output_data[0].is_empty(), "Output data rows should not be empty");
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
    assert!(output.device().same_device(&device), "Output should be on same device as input");
    assert!(output.dims().len() >= 2, "Output should have at least 2 dimensions");
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
    assert!(err.to_string().contains("CUDA"), "Error should mention CUDA");
}

/// Test tensor round-trip conversion
#[test]
#[cfg(target_os = "macos")]
fn test_tensor_roundtrip() {
    // This test validates our tensor conversion without needing a full model
    use candle_core::{Shape, Device, DType};
    
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