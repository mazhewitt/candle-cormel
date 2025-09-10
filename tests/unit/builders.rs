//! Comprehensive tests for builder.rs
//!
//! These tests cover the CoreMLModelBuilder pattern that had 0% coverage:
//! - Builder creation and configuration
//! - HuggingFace integration (mocked for testing)
//! - Error handling for various scenarios
//! - Configuration management and validation

use candle_coreml::{Config, CoreMLModelBuilder};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

// Helper to create a minimal valid CoreML config for testing
fn create_test_config() -> Config {
    Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 512,
        vocab_size: 151936,
        model_type: "test-model".to_string(),
    }
}

// Helper to create a mock model file structure for testing
fn create_mock_model_structure() -> tempfile::TempDir {
    let temp_dir = tempdir().unwrap();
    let model_path = temp_dir.path().join("model.mlmodelc");

    // Create a directory structure that looks like a CoreML model
    fs::create_dir_all(&model_path).unwrap();

    // Create some mock files that would be in a real CoreML model
    fs::write(
        model_path.join("metadata.json"),
        r#"{"model_type": "CoreML"}"#,
    )
    .unwrap();
    fs::write(model_path.join("weights.bin"), b"mock weights data").unwrap();

    temp_dir
}

#[test]
fn test_builder_new_creation() {
    let config = create_test_config();
    let model_path = PathBuf::from("/fake/path/model.mlmodelc");

    let builder = CoreMLModelBuilder::new(&model_path, config.clone());

    // Test that config is accessible
    assert_eq!(builder.config().input_names, config.input_names);
    assert_eq!(builder.config().output_name, config.output_name);
    assert_eq!(builder.config().vocab_size, config.vocab_size);
    assert_eq!(
        builder.config().max_sequence_length,
        config.max_sequence_length
    );
    assert_eq!(builder.config().model_type, config.model_type);
}

#[test]
fn test_builder_new_with_different_path_types() {
    let config = create_test_config();

    // Test with PathBuf
    let path_buf = PathBuf::from("/test/path.mlmodelc");
    let builder1 = CoreMLModelBuilder::new(&path_buf, config.clone());
    assert_eq!(builder1.config().model_type, "test-model");

    // Test with string slice
    let builder2 = CoreMLModelBuilder::new("/test/path2.mlmodelc", config.clone());
    assert_eq!(builder2.config().model_type, "test-model");

    // Test with Path reference
    let path = Path::new("/test/path3.mlmodelc");
    let builder3 = CoreMLModelBuilder::new(path, config);
    assert_eq!(builder3.config().model_type, "test-model");
}

#[cfg(target_os = "macos")]
#[test]
fn test_build_model_with_valid_structure() {
    let config = create_test_config();
    let temp_dir = create_mock_model_structure();
    let model_path = temp_dir.path().join("model.mlmodelc");

    let builder = CoreMLModelBuilder::new(&model_path, config);

    // This will fail because it's not a real CoreML model, but it tests the path
    let result = builder.build_model();

    // Should get a meaningful error about CoreML loading, not path issues
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    // Should not be a "file not found" error since we created the structure
    assert!(!error_msg.contains("No such file"));
}

#[cfg(not(target_os = "macos"))]
#[test]
fn test_build_model_non_macos() {
    let config = create_test_config();
    let model_path = PathBuf::from("/fake/path/model.mlmodelc");

    let builder = CoreMLModelBuilder::new(&model_path, config);

    // On non-macOS, should get appropriate error
    let result = builder.build_model();
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("CoreML") || error_msg.contains("macOS"));
}

#[test]
fn test_config_accessibility() {
    let mut config = create_test_config();
    config.vocab_size = 50000;
    config.max_sequence_length = 1024;

    let model_path = PathBuf::from("/test/model.mlmodelc");
    let builder = CoreMLModelBuilder::new(&model_path, config);

    // Test that we can access and verify config values
    let retrieved_config = builder.config();
    assert_eq!(retrieved_config.vocab_size, 50000);
    assert_eq!(retrieved_config.max_sequence_length, 1024);
    assert_eq!(retrieved_config.input_names, vec!["input_ids".to_string()]);
    assert_eq!(retrieved_config.output_name, "logits");
    assert_eq!(retrieved_config.model_type, "test-model");
}

// Mock tests for HuggingFace integration - these test error handling paths
// without actually needing network access or real HF models

#[test]
fn test_load_from_hub_invalid_model_id() {
    // Test with clearly invalid model ID that would fail API creation or repo access
    let result =
        CoreMLModelBuilder::load_from_hub("invalid/model/id/with/too/many/slashes", None, None);

    assert!(result.is_err());
    if let Err(error) = result {
        let error_msg = error.to_string();
        assert!(
            error_msg.contains("Failed to create HF API") || error_msg.contains("Failed to get")
        );
    }
}

#[test]
fn test_load_from_hub_empty_model_id() {
    // Test with empty model ID
    let result = CoreMLModelBuilder::load_from_hub("", None, None);

    assert!(result.is_err());
    // Should fail at API or repo creation stage
}

#[test]
fn test_load_from_hub_with_specific_filenames() {
    // Test the parameter handling for specific filenames
    // This will fail due to network/auth, but tests the parameter flow
    let result = CoreMLModelBuilder::load_from_hub(
        "nonexistent/model",
        Some("custom_model.mlmodelc"),
        Some("custom_config.json"),
    );

    assert!(result.is_err());
    if let Err(error) = result {
        let error_msg = error.to_string();
        // Should attempt to access the custom filenames we specified
        assert!(
            error_msg.contains("custom_config.json")
                || error_msg.contains("Failed to create HF API")
                || error_msg.contains("Failed to get config file")
        );
    }
}

// Test configuration validation indirectly through builder
#[test]
fn test_config_variations() {
    let model_path = PathBuf::from("/test/model.mlmodelc");

    // Test with minimal config
    let minimal_config = Config {
        input_names: vec!["tokens".to_string()],
        output_name: "predictions".to_string(),
        max_sequence_length: 128,
        vocab_size: 1000,
        model_type: "minimal".to_string(),
    };

    let builder = CoreMLModelBuilder::new(&model_path, minimal_config);
    assert_eq!(builder.config().max_sequence_length, 128);
    assert_eq!(builder.config().vocab_size, 1000);

    // Test with complex config
    let complex_config = Config {
        input_names: vec![
            "input_ids".to_string(),
            "attention_mask".to_string(),
            "position_ids".to_string(),
        ],
        output_name: "logits".to_string(),
        max_sequence_length: 2048,
        vocab_size: 32000,
        model_type: "complex-transformer".to_string(),
    };

    let builder2 = CoreMLModelBuilder::new(&model_path, complex_config);
    assert_eq!(builder2.config().input_names.len(), 3);
    assert_eq!(builder2.config().max_sequence_length, 2048);
    assert_eq!(builder2.config().vocab_size, 32000);
}

// Test edge cases and error conditions
#[test]
fn test_builder_with_empty_config_fields() {
    let model_path = PathBuf::from("/test/model.mlmodelc");

    // Test with empty input names (edge case)
    let empty_inputs_config = Config {
        input_names: vec![], // Empty
        output_name: "output".to_string(),
        max_sequence_length: 512,
        vocab_size: 1000,
        model_type: "empty-inputs".to_string(),
    };

    let builder = CoreMLModelBuilder::new(&model_path, empty_inputs_config);
    assert!(builder.config().input_names.is_empty());

    // Test with empty output name (edge case)
    let empty_output_config = Config {
        input_names: vec!["input".to_string()],
        output_name: String::new(), // Empty
        max_sequence_length: 512,
        vocab_size: 1000,
        model_type: "empty-output".to_string(),
    };

    let builder2 = CoreMLModelBuilder::new(&model_path, empty_output_config);
    assert!(builder2.config().output_name.is_empty());
}

#[test]
fn test_builder_with_extreme_config_values() {
    let model_path = PathBuf::from("/test/model.mlmodelc");

    // Test with very large values
    let large_config = Config {
        input_names: vec!["input".to_string()],
        output_name: "output".to_string(),
        max_sequence_length: 100000, // Very large
        vocab_size: 1000000,         // Very large
        model_type: "large-model".to_string(),
    };

    let builder = CoreMLModelBuilder::new(&model_path, large_config);
    assert_eq!(builder.config().max_sequence_length, 100000);
    assert_eq!(builder.config().vocab_size, 1000000);

    // Test with minimum values
    let minimal_config = Config {
        input_names: vec!["in".to_string()],
        output_name: "out".to_string(),
        max_sequence_length: 1, // Minimum
        vocab_size: 1,          // Minimum
        model_type: "min".to_string(),
    };

    let builder2 = CoreMLModelBuilder::new(&model_path, minimal_config);
    assert_eq!(builder2.config().max_sequence_length, 1);
    assert_eq!(builder2.config().vocab_size, 1);
}

// Test path handling variations
#[test]
fn test_builder_path_variations() {
    let config = create_test_config();

    // Test with relative path
    let builder1 = CoreMLModelBuilder::new("./model.mlmodelc", config.clone());
    assert_eq!(builder1.config().model_type, "test-model");

    // Test with absolute path
    let builder2 = CoreMLModelBuilder::new("/absolute/path/model.mlmodelc", config.clone());
    assert_eq!(builder2.config().model_type, "test-model");

    // Test with nested path
    let builder3 = CoreMLModelBuilder::new("models/subfolder/model.mlmodelc", config.clone());
    assert_eq!(builder3.config().model_type, "test-model");

    // Test with different extensions
    let builder4 = CoreMLModelBuilder::new("model.mlpackage", config);
    assert_eq!(builder4.config().model_type, "test-model");
}

// Integration test combining builder features
#[test]
fn test_builder_complete_workflow() {
    let config = Config {
        input_names: vec!["input_ids".to_string(), "attention_mask".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 1024,
        vocab_size: 50000,
        model_type: "workflow-test".to_string(),
    };

    let model_path = PathBuf::from("/workflow/test/model.mlmodelc");

    // Create builder
    let builder = CoreMLModelBuilder::new(&model_path, config);

    // Verify all config aspects
    let retrieved_config = builder.config();
    assert_eq!(retrieved_config.input_names.len(), 2);
    assert_eq!(retrieved_config.input_names[0], "input_ids");
    assert_eq!(retrieved_config.input_names[1], "attention_mask");
    assert_eq!(retrieved_config.output_name, "logits");
    assert_eq!(retrieved_config.max_sequence_length, 1024);
    assert_eq!(retrieved_config.vocab_size, 50000);
    assert_eq!(retrieved_config.model_type, "workflow-test");

    // Attempt to build (will fail without real model, but tests the interface)
    let build_result = builder.build_model();
    assert!(build_result.is_err()); // Expected to fail without real model file
}
