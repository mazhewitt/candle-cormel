//! Common test helper functions and utilities
#![allow(dead_code)]

use candle_coreml::model_config::{
    ComponentConfig, ModelConfig, ModelInfo, ShapeConfig, TensorConfig,
};
use std::collections::HashMap;
use tempfile::TempDir;

/// Create a simple test ModelConfig for unit testing
pub fn create_test_model_config(
    batch_size: usize,
    context_length: usize,
    hidden_size: usize,
) -> ModelConfig {
    let mut components = HashMap::new();

    // Add embeddings component
    let mut inputs = HashMap::new();
    inputs.insert(
        "input_ids".to_string(),
        TensorConfig {
            name: "input_ids".to_string(),
            shape: vec![batch_size, context_length],
            data_type: "INT32".to_string(),
        },
    );

    let mut outputs = HashMap::new();
    outputs.insert(
        "hidden_states".to_string(),
        TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![batch_size, context_length, hidden_size],
            data_type: "FLOAT16".to_string(),
        },
    );

    components.insert(
        "embeddings".to_string(),
        ComponentConfig {
            file_path: None,
            inputs,
            outputs,
            functions: vec![],
            input_order: None,
        },
    );

    ModelConfig {
        model_info: ModelInfo {
            model_id: Some("test-model".into()),
            path: None,
            model_type: "qwen".into(),
            discovered_at: None,
        },
        shapes: ShapeConfig {
            batch_size,
            context_length,
            hidden_size,
            vocab_size: 151_936,
        },
        components,
        naming: Default::default(),
        ffn_execution: None,
    }
}

/// Create a temporary directory for test files
pub fn create_test_temp_dir() -> TempDir {
    tempfile::tempdir().expect("Failed to create temp dir")
}

/// Create a mock model package structure for testing
pub fn create_mock_model_package(
    temp_dir: &std::path::Path,
    package_name: &str,
) -> std::io::Result<std::path::PathBuf> {
    let package_path = temp_dir.join(format!("{package_name}.mlpackage"));
    let data_dir = package_path.join("Data/com.apple.CoreML");
    std::fs::create_dir_all(&data_dir)?;

    // Create minimal mock files
    std::fs::write(data_dir.join("model.mlmodel"), b"mock_model_data")?;
    std::fs::write(
        package_path.join("Manifest.json"),
        r#"{"itemInfoEntries":{}}"#,
    )?;

    Ok(package_path)
}
