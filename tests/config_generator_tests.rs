//! Comprehensive unit tests for config generator components
//!
//! These tests cover the core model detection and parsing logic:
//! - ManifestSource detection patterns
//! - Filename-based component role detection
//! - FFN execution mode inference
//! - Component name mapping
//! - Enhanced parsing strategy selection

use candle_coreml::{
    config_generator::{
        file_discovery::{FileDiscovery, ManifestSource},
        manifest_parser::ManifestParser,
        schema_extractor::{ComponentRole, SchemaExtractor},
    },
    model_config::{ComponentConfig, TensorConfig},
};
use std::collections::HashMap;

/// Test helper to create a mock ComponentConfig
fn create_mock_component_config(file_path: Option<String>) -> ComponentConfig {
    let mut inputs = HashMap::new();
    inputs.insert(
        "input".to_string(),
        TensorConfig {
            name: "input".to_string(),
            shape: vec![1, 512],
            data_type: "Float32".to_string(),
        },
    );

    let mut outputs = HashMap::new();
    outputs.insert(
        "output".to_string(),
        TensorConfig {
            name: "output".to_string(),
            shape: vec![1, 512, 1024],
            data_type: "Float32".to_string(),
        },
    );

    ComponentConfig {
        file_path,
        inputs,
        outputs,
        functions: vec![],
        input_order: None,
    }
}

#[cfg(target_os = "macos")]
mod file_discovery_tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_mock_package_structure(
        temp_dir: &TempDir,
        package_name: &str,
        manifest_type: &str,
    ) -> PathBuf {
        let package_path = temp_dir.path().join(package_name);
        std::fs::create_dir_all(&package_path).unwrap();

        match manifest_type {
            "metadata" => {
                let metadata_path = package_path.join("metadata.json");
                std::fs::write(&metadata_path, r#"{"format": "mlmodelc"}"#).unwrap();
            }
            "manifest" => {
                let manifest_path = package_path.join("Manifest.json");
                std::fs::write(&manifest_path, r#"[{"format": "mlpackage"}]"#).unwrap();
            }
            "model" => {
                let model_dir = package_path.join("Data").join("com.apple.CoreML");
                std::fs::create_dir_all(&model_dir).unwrap();
                let model_path = model_dir.join("model.mlmodel");
                std::fs::write(&model_path, b"mock model data").unwrap();
            }
            _ => {} // No manifest files
        }

        package_path
    }

    #[test]
    fn test_manifest_source_detection_metadata_json() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = create_mock_package_structure(&temp_dir, "test.mlmodelc", "metadata");

        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();

        match source {
            ManifestSource::MetadataJson(path) => {
                assert!(path.ends_with("metadata.json"));
            }
            _ => panic!("Expected MetadataJson source"),
        }
    }

    #[test]
    fn test_manifest_source_detection_manifest_json() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = create_mock_package_structure(&temp_dir, "test.mlpackage", "manifest");

        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();

        match source {
            ManifestSource::ManifestJson(path) => {
                assert!(path.ends_with("Manifest.json"));
            }
            _ => panic!("Expected ManifestJson source"),
        }
    }

    #[test]
    fn test_manifest_source_detection_model_file() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = create_mock_package_structure(&temp_dir, "test.mlpackage", "model");

        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();

        match source {
            ManifestSource::ModelFile(path) => {
                assert!(path.ends_with("model.mlmodel"));
            }
            _ => panic!("Expected ModelFile source"),
        }
    }

    #[test]
    fn test_manifest_source_detection_filename_only() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = create_mock_package_structure(&temp_dir, "test.mlpackage", "none");

        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();

        match source {
            ManifestSource::FilenameOnly => {
                // This is expected
            }
            _ => panic!("Expected FilenameOnly source"),
        }
    }

    #[test]
    fn test_typo_fixer_style_detection() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = create_mock_package_structure(&temp_dir, "test.mlpackage", "model");

        let file_discovery = FileDiscovery::new();
        assert!(file_discovery.is_typo_fixer_style(&package_path));

        // Test non-typo-fixer style
        let standard_path = create_mock_package_structure(&temp_dir, "standard.mlpackage", "manifest");
        assert!(!file_discovery.is_typo_fixer_style(&standard_path));
    }

    #[test]
    fn test_priority_order_metadata_over_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("test.mlmodelc");
        std::fs::create_dir_all(&package_path).unwrap();

        // Create both metadata.json and Manifest.json
        std::fs::write(package_path.join("metadata.json"), r#"{"format": "mlmodelc"}"#).unwrap();
        std::fs::write(package_path.join("Manifest.json"), r#"[{"format": "mlpackage"}]"#).unwrap();

        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();

        // Should prioritize metadata.json
        match source {
            ManifestSource::MetadataJson(_) => {
                // Expected behavior
            }
            _ => panic!("Expected MetadataJson to have priority over ManifestJson"),
        }
    }
}

#[cfg(target_os = "macos")]
mod schema_extractor_tests {
    use super::*;

    #[test]
    fn test_filename_based_component_detection_embeddings() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "qwen-typo-fixer_embeddings.mlpackage",
            "model_embeddings.mlmodelc",
            "transformer_embeddings_v2.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::Embeddings, "Failed for filename: {}", filename);
        }
    }

    #[test]
    fn test_filename_based_component_detection_lm_head() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "qwen-typo-fixer_lm_head.mlpackage",
            "model_lmhead.mlmodelc",
            "transformer_lm_head_v2.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::LmHead, "Failed for filename: {}", filename);
        }
    }

    #[test]
    fn test_filename_based_component_detection_ffn_unified() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc",
            "model_ffn_pf_layer.mlpackage",
            "transformer_FFN_PF.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::FfnUnified, "Failed for filename: {}", filename);
        }
    }

    #[test]
    fn test_filename_based_component_detection_ffn_prefill() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "qwen-typo-fixer_prefill_chunk_01of01.mlpackage",
            "model_prefill.mlmodelc",
            "transformer_prefill_layer.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::FfnPrefill, "Failed for filename: {}", filename);
        }
    }

    #[test]
    fn test_filename_based_component_detection_ffn_infer() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "qwen-typo-fixer_FFN_chunk_01of01.mlpackage",
            "model_infer_layer.mlmodelc",
            "transformer_ffn_chunk_01of01.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::FfnInfer, "Failed for filename: {}", filename);
        }
    }

    #[test]
    fn test_filename_based_component_detection_priority() {
        let schema_extractor = SchemaExtractor::new();

        // Test that FFN_PF takes priority over generic FFN
        let ffn_pf_filename = "model_FFN_PF_and_FFN.mlpackage";
        let role = schema_extractor.detect_component_role_from_filename(ffn_pf_filename);
        assert_eq!(role, ComponentRole::FfnUnified);

        // Test that prefill takes priority over generic FFN  
        let prefill_filename = "model_prefill_ffn.mlpackage";
        let role = schema_extractor.detect_component_role_from_filename(prefill_filename);
        assert_eq!(role, ComponentRole::FfnPrefill);
    }

    #[test]
    fn test_filename_based_component_detection_unknown() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            "random_model.mlpackage",
            "unknown_component.mlmodelc",
            "mystery_layer.mlpackage",
        ];

        for filename in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, ComponentRole::Unknown, "Failed for filename: {}", filename);
        }
    }
}

#[cfg(target_os = "macos")]
mod manifest_parser_tests {
    use super::*;

    #[test]
    fn test_role_to_component_name_mapping() {
        let parser = ManifestParser::new();

        assert_eq!(parser.role_to_component_name(&ComponentRole::Embeddings), "embeddings");
        assert_eq!(parser.role_to_component_name(&ComponentRole::LmHead), "lm_head");
        assert_eq!(parser.role_to_component_name(&ComponentRole::FfnPrefill), "ffn_prefill");
        assert_eq!(parser.role_to_component_name(&ComponentRole::FfnInfer), "ffn_infer");
        assert_eq!(parser.role_to_component_name(&ComponentRole::FfnUnified), "ffn_prefill");
        assert_eq!(parser.role_to_component_name(&ComponentRole::Unknown), "unknown");
    }

    #[test]
    fn test_infer_execution_mode_split_architecture() {
        let parser = ManifestParser::new();

        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("ffn_prefill".to_string(), create_mock_component_config(None)),
            ("ffn_infer".to_string(), create_mock_component_config(None)),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "split");
    }

    #[test]
    fn test_infer_execution_mode_unified_ffn_pf_pattern() {
        let parser = ManifestParser::new();

        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("ffn_prefill".to_string(), create_mock_component_config(Some("model_FFN_PF_lut8.mlpackage".to_string()))),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified");
    }

    #[test]
    fn test_infer_execution_mode_multi_function_unified() {
        let parser = ManifestParser::new();

        let mut config = create_mock_component_config(None);
        config.functions = vec!["prefill".to_string(), "infer".to_string()];

        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("ffn_prefill".to_string(), config),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified");
    }

    #[test]
    fn test_infer_execution_mode_fallback_unified() {
        let parser = ManifestParser::new();

        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("ffn_something".to_string(), create_mock_component_config(None)),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified");
    }

    #[test]
    fn test_infer_execution_mode_no_ffn_unified() {
        let parser = ManifestParser::new();

        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified");
    }

    #[test]
    fn test_parse_package_filename_only() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let package_path = temp_dir.path().join("qwen-typo-fixer_embeddings.mlpackage");
        std::fs::create_dir(&package_path).unwrap();

        let parser = ManifestParser::new();
        let schema_extractor = SchemaExtractor::new();
        
        let components = parser.parse_package_filename_only(&package_path, &schema_extractor).unwrap();
        
        assert_eq!(components.len(), 1);
        assert_eq!(components[0].0, "embeddings");
        assert!(components[0].1.file_path.is_some());
        assert!(components[0].1.inputs.is_empty());
        assert!(components[0].1.outputs.is_empty());
    }
}

#[cfg(target_os = "macos")]
mod integration_tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_typo_fixer_style_filename_detection_flow() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create typo-fixer style package structure
        let package_path = temp_dir.path().join("qwen-typo-fixer_embeddings.mlpackage");
        std::fs::create_dir_all(&package_path).unwrap();
        
        let model_dir = package_path.join("Data").join("com.apple.CoreML");
        std::fs::create_dir_all(&model_dir).unwrap();
        std::fs::write(model_dir.join("model.mlmodel"), b"mock model data").unwrap();

        // Test the detection flow
        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();
        
        match source {
            ManifestSource::ModelFile(path) => {
                assert!(path.ends_with("model.mlmodel"));
                
                // Test that it's detected as typo-fixer style
                assert!(file_discovery.is_typo_fixer_style(&package_path));
                
                // Test filename-based component detection
                let schema_extractor = SchemaExtractor::new();
                let role = schema_extractor.detect_component_role_from_filename("qwen-typo-fixer_embeddings.mlpackage");
                assert_eq!(role, ComponentRole::Embeddings);
                
                // Test component name mapping
                let parser = ManifestParser::new();
                let component_name = parser.role_to_component_name(&role);
                assert_eq!(component_name, "embeddings");
            }
            _ => panic!("Expected ModelFile source for typo-fixer style package"),
        }
    }

    #[test]
    fn test_standard_mlmodelc_detection_flow() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create standard .mlmodelc package structure
        let package_path = temp_dir.path().join("qwen_embeddings.mlmodelc");
        std::fs::create_dir_all(&package_path).unwrap();
        std::fs::write(package_path.join("metadata.json"), r#"{"format": "mlmodelc"}"#).unwrap();

        // Test the detection flow
        let file_discovery = FileDiscovery::new();
        let source = file_discovery.find_manifest_source(&package_path).unwrap();
        
        match source {
            ManifestSource::MetadataJson(path) => {
                assert!(path.ends_with("metadata.json"));
                
                // Test that it's NOT detected as typo-fixer style
                assert!(!file_discovery.is_typo_fixer_style(&package_path));
                
                // Test filename-based component detection
                let schema_extractor = SchemaExtractor::new();
                let role = schema_extractor.detect_component_role_from_filename("qwen_embeddings.mlmodelc");
                assert_eq!(role, ComponentRole::Embeddings);
            }
            _ => panic!("Expected MetadataJson source for standard .mlmodelc package"),
        }
    }

    #[test]
    fn test_anemll_ffn_pf_unified_mode_detection() {
        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
            ("ffn_prefill".to_string(), create_mock_component_config(Some("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc".to_string()))),
            ("lm_head".to_string(), create_mock_component_config(None)),
        ];

        let parser = ManifestParser::new();
        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified", "ANEMLL FFN_PF pattern should be detected as unified mode");
    }

    #[test]
    fn test_typo_fixer_split_mode_detection() {
        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(Some("qwen-typo-fixer_embeddings.mlpackage".to_string()))),
            ("ffn_prefill".to_string(), create_mock_component_config(Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()))),
            ("ffn_infer".to_string(), create_mock_component_config(Some("qwen-typo-fixer_FFN_chunk_01of01.mlpackage".to_string()))),
            ("lm_head".to_string(), create_mock_component_config(Some("qwen-typo-fixer_lm_head.mlpackage".to_string()))),
        ];

        let parser = ManifestParser::new();
        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "split", "Typo-fixer pattern should be detected as split mode");
    }
}

#[cfg(target_os = "macos")]
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_case_insensitive_filename_detection() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            ("Model_EMBEDDINGS.mlpackage", ComponentRole::Embeddings),
            ("qwen_LM_HEAD.mlmodelc", ComponentRole::LmHead),
            ("transformer_ffn_pf.mlpackage", ComponentRole::FfnUnified),
            ("model_PREFILL.mlpackage", ComponentRole::FfnPrefill),
        ];

        for (filename, expected_role) in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, expected_role, "Case insensitive detection failed for: {}", filename);
        }
    }

    #[test]
    fn test_complex_filename_patterns() {
        let schema_extractor = SchemaExtractor::new();

        let test_cases = vec![
            ("qwen-2.5-typo-fixer_embeddings_v2.1.mlpackage", ComponentRole::Embeddings),
            ("model_FFN_PF_optimized_lut8_chunk_01of01.mlmodelc", ComponentRole::FfnUnified),
            ("transformer_prefill_optimized_chunk_01of01.mlpackage", ComponentRole::FfnPrefill),
            ("model_name_with_FFN_chunk_01of01_optimized.mlpackage", ComponentRole::FfnInfer),
        ];

        for (filename, expected_role) in test_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, expected_role, "Complex pattern detection failed for: {}", filename);
        }
    }

    #[test]
    fn test_ambiguous_filename_resolution() {
        let schema_extractor = SchemaExtractor::new();

        // Test that more specific patterns take precedence
        let ambiguous_cases = vec![
            ("model_ffn_pf_prefill.mlpackage", ComponentRole::FfnUnified), // FFN_PF wins
            ("transformer_prefill_ffn.mlpackage", ComponentRole::FfnPrefill), // prefill wins
        ];

        for (filename, expected_role) in ambiguous_cases {
            let role = schema_extractor.detect_component_role_from_filename(filename);
            assert_eq!(role, expected_role, "Ambiguous resolution failed for: {}", filename);
        }
    }

    #[test]
    fn test_empty_and_invalid_inputs() {
        let schema_extractor = SchemaExtractor::new();
        let parser = ManifestParser::new();

        // Test empty filename
        let role = schema_extractor.detect_component_role_from_filename("");
        assert_eq!(role, ComponentRole::Unknown);

        // Test empty components list
        let mode = parser.infer_execution_mode(&[]);
        assert_eq!(mode, "unified");

        // Test components with missing file paths
        let components = vec![
            ("embeddings".to_string(), create_mock_component_config(None)),
        ];
        let mode = parser.infer_execution_mode(&components);
        assert_eq!(mode, "unified");
    }
}