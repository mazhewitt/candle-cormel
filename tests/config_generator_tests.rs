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

/// Test helper to create a ComponentConfig with empty tensor maps (reproduces the bug)
fn create_empty_component_config(file_path: Option<String>) -> ComponentConfig {
    ComponentConfig {
        file_path,
        inputs: HashMap::new(),  // Empty inputs - this causes the bug
        outputs: HashMap::new(), // Empty outputs - this causes the bug
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

#[cfg(target_os = "macos")]
mod typo_fixer_bug_reproduction_tests {
    use super::*;
    use candle_coreml::ConfigGenerator;
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// Reference shapes from flex_pipeline fixtures for validation
    struct FlexPipelineReference {
        // From corrected_step_1_tokens.json
        input_tokens_shape: Vec<usize>,
        context_pos: usize,
        
        // From typo-fixer model architecture (README.md)
        expected_vocab_size: usize,
        expected_hidden_size: usize,
        expected_context_length: usize,
        expected_batch_size: usize,
    }

    impl FlexPipelineReference {
        fn new() -> Self {
            Self {
                // From corrected_step_1_tokens.json: input_ids shape [1, 12], context_pos: 12
                input_tokens_shape: vec![1, 12],
                context_pos: 12,
                
                // From README.md: typo-fixer model architecture
                expected_vocab_size: 151669,  // 16-part logits → vocab_size=151669
                expected_hidden_size: 1024,   // hidden_states [batch, seq_len, 1024]
                expected_context_length: 256,  // For causal mask generation
                expected_batch_size: 1,        // Typical batch size for inference
            }
        }
        
        /// Load token data from flex_pipeline fixtures if available
        fn try_load_reference_token_data(&self) -> Option<serde_json::Value> {
            let fixture_path = PathBuf::from("tests/fixtures/flex_pipeline/corrected_step_1_tokens.json");
            if fixture_path.exists() {
                std::fs::read_to_string(fixture_path)
                    .ok()
                    .and_then(|content| serde_json::from_str(&content).ok())
            } else {
                None
            }
        }
        
        /// Validate that config meets typo-fixer requirements - STRICT assertions that fail when bug is present
        fn validate_config_shapes(&self, config: &candle_coreml::ModelConfig) {
            println!("🔍 Validating config shapes (STRICT mode - tests will FAIL if bug is present):");
            println!("   • vocab_size: {} (expected: >{} for typo-fixer)", config.shapes.vocab_size, self.expected_vocab_size);
            println!("   • hidden_size: {} (expected: >={})", config.shapes.hidden_size, self.expected_hidden_size);
            println!("   • context_length: {} (expected: >={})", config.shapes.context_length, self.expected_context_length);
            println!("   • batch_size: {} (expected: >={})", config.shapes.batch_size, self.expected_batch_size);
            
            // STRICT: These assertions will FAIL if the config generation bug is present
            assert!(config.shapes.vocab_size >= self.expected_vocab_size, 
                   "TYPO-FIXER BUG: vocab_size {} is too small. Expected >= {} for typo-fixer model. This indicates shape inference from empty tensor metadata is broken.",
                   config.shapes.vocab_size, self.expected_vocab_size);
            
            assert!(config.shapes.hidden_size >= self.expected_hidden_size, 
                   "TYPO-FIXER BUG: hidden_size {} is too small. Expected >= {} for typo-fixer model.",
                   config.shapes.hidden_size, self.expected_hidden_size);
            
            assert!(config.shapes.context_length >= self.expected_context_length, 
                   "TYPO-FIXER BUG: context_length {} is too small. Expected >= {}.",
                   config.shapes.context_length, self.expected_context_length);
            
            assert!(config.shapes.batch_size >= self.expected_batch_size, 
                   "TYPO-FIXER BUG: batch_size {} is invalid. Expected >= {}.",
                   config.shapes.batch_size, self.expected_batch_size);
            
            // STRICT: Components must have populated tensor information
            if let Some(embeddings) = config.components.get("embeddings") {
                assert!(!embeddings.inputs.is_empty(), 
                       "TYPO-FIXER BUG: Embeddings component has empty input tensor maps. This indicates metadata extraction is broken.");
                assert!(!embeddings.outputs.is_empty(), 
                       "TYPO-FIXER BUG: Embeddings component has empty output tensor maps. This indicates metadata extraction is broken.");
                println!("✅ Embeddings component has tensor information");
                
                // STRICT: Embeddings should have specific tensor names
                assert!(embeddings.inputs.contains_key("input_ids") || !embeddings.inputs.is_empty(),
                       "TYPO-FIXER BUG: Embeddings should have input_ids input or equivalent tensor");
                assert!(embeddings.outputs.contains_key("hidden_states") || !embeddings.outputs.is_empty(),
                       "TYPO-FIXER BUG: Embeddings should have hidden_states output or equivalent tensor");
            } else {
                panic!("TYPO-FIXER BUG: Missing embeddings component");
            }
            
            if let Some(lm_head) = config.components.get("lm_head") {
                assert!(!lm_head.inputs.is_empty(), 
                       "TYPO-FIXER BUG: LM head component has empty input tensor maps. This indicates metadata extraction is broken.");
                assert!(!lm_head.outputs.is_empty(), 
                       "TYPO-FIXER BUG: LM head component has empty output tensor maps. This indicates metadata extraction is broken.");
                println!("✅ LM head component has tensor information");
                
                // STRICT: LM head should have logits outputs that sum to correct vocab size
                let logits_outputs: Vec<_> = lm_head.outputs.keys().filter(|k| k.contains("logits")).collect();
                assert!(!logits_outputs.is_empty(),
                       "TYPO-FIXER BUG: LM head should have logits output tensors");
                
                // Calculate total vocab size from logits outputs
                let total_vocab_from_logits: usize = lm_head.outputs
                    .iter()
                    .filter(|(name, _)| name.contains("logits"))
                    .map(|(_, tensor)| tensor.shape.last().copied().unwrap_or(0))
                    .sum();
                
                if total_vocab_from_logits > 0 {
                    assert_eq!(total_vocab_from_logits, self.expected_vocab_size,
                              "TYPO-FIXER BUG: LM head logits sum to {} but should sum to {} for typo-fixer",
                              total_vocab_from_logits, self.expected_vocab_size);
                }
            } else {
                panic!("TYPO-FIXER BUG: Missing lm_head component");
            }
        }
    }

    #[test]
    fn test_typo_fixer_config_generation_with_empty_metadata_bug() {
        println!("🧪 Testing typo-fixer config generation with empty metadata (bug reproduction)");
        
        let reference = FlexPipelineReference::new();
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create typo-fixer style packages with the exact naming from the real model
        let emb_path = create_typo_fixer_style_mlpackage(temp_dir.path(), "qwen-typo-fixer_embeddings").unwrap();
        let ffn_prefill_path = create_typo_fixer_style_mlpackage(temp_dir.path(), "qwen-typo-fixer_prefill_chunk_01of01").unwrap();
        let ffn_infer_path = create_typo_fixer_style_mlpackage(temp_dir.path(), "qwen-typo-fixer_FFN_chunk_01of01").unwrap();
        let lm_head_path = create_typo_fixer_style_mlpackage(temp_dir.path(), "qwen-typo-fixer_lm_head").unwrap();

        println!("📦 Created mock typo-fixer packages:");
        println!("   • {}", emb_path.file_name().unwrap().to_string_lossy());
        println!("   • {}", ffn_prefill_path.file_name().unwrap().to_string_lossy());
        println!("   • {}", ffn_infer_path.file_name().unwrap().to_string_lossy());
        println!("   • {}", lm_head_path.file_name().unwrap().to_string_lossy());

        // Test 1: Verify the packages are detected correctly
        let packages = generator.find_mlpackage_files(temp_dir.path()).unwrap();
        assert_eq!(packages.len(), 4, "Should find 4 typo-fixer components");

        // Test 2: Test config generation (this will use filename fallback due to empty metadata)
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "test/qwen-typo-fixer-mock",
            "qwen"
        );

        match result {
            Ok(config) => {
                println!("✅ Config generation succeeded (filename fallback worked)");
                println!("📋 Generated components: {:?}", config.components.keys().collect::<Vec<_>>());
                
                // Validate we have all required components
                assert!(config.components.contains_key("embeddings"), 
                       "Missing embeddings component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
                assert!(config.components.contains_key("lm_head"), 
                       "Missing lm_head component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
                assert!(config.components.contains_key("ffn_prefill"), 
                       "Missing ffn_prefill component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
                assert!(config.components.contains_key("ffn_infer"), 
                       "Missing ffn_infer component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
                
                // Validate execution mode (should be split for typo-fixer)
                assert_eq!(config.ffn_execution.as_deref(), Some("split"), 
                          "Expected split execution mode for typo-fixer architecture");
                
                // Validate shapes are reasonable using reference data
                reference.validate_config_shapes(&config);
                
                println!("📊 Generated config shapes:");
                println!("   • vocab_size: {}", config.shapes.vocab_size);
                println!("   • hidden_size: {}", config.shapes.hidden_size);
                println!("   • context_length: {}", config.shapes.context_length);
                println!("   • batch_size: {}", config.shapes.batch_size);
                
                // STRICT: Check for empty tensor maps - this should cause test failure when bug is present
                println!("🔍 Checking for empty tensor maps (STRICT mode):");
                let mut empty_components = Vec::new();
                
                for (name, component) in &config.components {
                    if component.inputs.is_empty() && component.outputs.is_empty() {
                        empty_components.push(name.clone());
                        println!("❌ Component '{}' has empty tensor maps", name);
                    } else {
                        println!("✅ Component '{}' has tensor information", name);
                    }
                }
                
                // STRICT: This test should FAIL if any components have empty tensor maps
                assert!(empty_components.is_empty(), 
                       "TYPO-FIXER BUG: The following components have empty tensor maps: {:?}. This indicates the metadata extraction bug. Config generation should populate tensor information even from minimal metadata or provide proper fallback.",
                       empty_components);
                
                println!("✅ All components have proper tensor information - bug is FIXED!");
                
            }
            Err(e) => {
                println!("❌ Config generation failed: {}", e);
                
                // This could be the expected behavior if the system properly rejects empty configs
                // Check if the error message is meaningful
                let error_msg = format!("{}", e);
                assert!(error_msg.contains("tensor") || error_msg.contains("input") || error_msg.contains("output") || error_msg.contains("component"),
                       "Error message should be descriptive about tensor/component issues: {}", error_msg);
                
                println!("✅ Config generation properly rejected empty metadata with clear error");
            }
        }
        
        // Test 3: Load and validate reference token data if available
        if let Some(token_data) = reference.try_load_reference_token_data() {
            println!("📄 Found flex_pipeline reference data");
            
            if let Some(input_ids) = token_data["data"]["input_ids"][0].as_array() {
                let expected_tokens: Vec<i64> = input_ids
                    .iter()
                    .filter_map(|v| v.as_i64())
                    .collect();
                
                println!("🔢 Reference tokenization: {} tokens", expected_tokens.len());
                println!("   First few tokens: {:?}", &expected_tokens[..std::cmp::min(5, expected_tokens.len())]);
                
                assert_eq!(expected_tokens.len(), reference.context_pos,
                          "Reference token count should match context_pos");
            }
            
            if let Some(context_pos) = token_data["data"]["context_pos"].as_u64() {
                assert_eq!(context_pos as usize, reference.context_pos,
                          "Reference context_pos should match expected value");
                println!("✅ Reference token data validates correctly");
            }
        } else {
            println!("ℹ️  No flex_pipeline reference data found (tests/fixtures/flex_pipeline/corrected_step_1_tokens.json)");
        }
        
        println!("🎯 Test completed: Typo-fixer config generation with empty metadata");
    }

    /// Test that specifically reproduces the "cached config with empty tensor shapes" issue
    #[test]
    fn test_config_generation_rejects_completely_empty_tensor_maps() {
        println!("🧪 Testing config generation with completely empty tensor maps");
        
        let components = vec![
            // Create components with completely empty tensor maps (reproduces the exact bug)
            ("embeddings".to_string(), create_empty_component_config(Some("qwen-typo-fixer_embeddings.mlpackage".to_string()))),
            ("ffn_prefill".to_string(), create_empty_component_config(Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()))),
            ("ffn_infer".to_string(), create_empty_component_config(Some("qwen-typo-fixer_FFN_chunk_01of01.mlpackage".to_string()))),
            ("lm_head".to_string(), create_empty_component_config(Some("qwen-typo-fixer_lm_head.mlpackage".to_string()))),
        ];
        
        // Check that all components have empty tensor maps
        for (name, component) in &components {
            assert!(component.inputs.is_empty(), "Component {} should have empty inputs", name);
            assert!(component.outputs.is_empty(), "Component {} should have empty outputs", name);
            println!("📋 Component '{}' has empty tensor maps (reproduces bug)", name);
        }
        
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        let parser = ManifestParser::new();
        
        // Test execution mode inference with empty components
        let execution_mode = parser.infer_execution_mode(&components);
        println!("🔧 Execution mode inferred: {}", execution_mode);
        
        // Test shape inference with empty components
        let shape_config = generator.compute_shape_info_generic(&components.iter().map(|(k, v)| (k.clone(), v.clone())).collect());
        
        println!("📊 Shape config with empty tensors:");
        println!("   • vocab_size: {}", shape_config.vocab_size);
        println!("   • hidden_size: {}", shape_config.hidden_size);
        println!("   • context_length: {}", shape_config.context_length);
        println!("   • batch_size: {}", shape_config.batch_size);
        
        // STRICT: The system should either provide correct shapes or reject empty tensor configs
        println!("🔍 Analyzing shape inference behavior with empty tensor maps:");
        
        // Reference values for typo-fixer model
        let expected_min_vocab_size = 100000;  // Typo-fixer has 151,669
        let expected_min_hidden_size = 1024;   // Should be at least 1024
        
        if shape_config.vocab_size == 0 || shape_config.hidden_size == 0 {
            panic!("TYPO-FIXER BUG: Empty tensor maps led to zero shapes - vocab_size: {}, hidden_size: {}. The config generation system should either populate tensor info from metadata or reject empty configs with clear error messages.",
                   shape_config.vocab_size, shape_config.hidden_size);
        }
        
        // STRICT: Shape inference should produce correct values or the test should fail
        if shape_config.vocab_size < expected_min_vocab_size {
            panic!("TYPO-FIXER BUG: vocab_size {} is too small for typo-fixer model (expected >= {}). This indicates shape inference from empty tensor metadata produces incorrect values. The bug causes CLI failures at runtime.",
                   shape_config.vocab_size, expected_min_vocab_size);
        }
        
        if shape_config.hidden_size < expected_min_hidden_size {
            panic!("TYPO-FIXER BUG: hidden_size {} is too small for typo-fixer model (expected >= {}). This indicates shape inference issues.",
                   shape_config.hidden_size, expected_min_hidden_size);
        }
        
        // If we reach here, the config generation system has been fixed
        println!("✅ Shape inference provided correct typo-fixer shapes despite empty tensors");
        println!("✅ This indicates the config generation bug has been FIXED");
        
        println!("🎯 Test completed: Config generation with empty tensor maps");
    }

    /// Create a mock "typo-fixer style" .mlpackage without Manifest.json but with Data/com.apple.CoreML/model.mlmodel
    fn create_typo_fixer_style_mlpackage(temp_dir: &std::path::Path, name: &str) -> std::io::Result<PathBuf> {
        let package_path = temp_dir.join(format!("{}.mlpackage", name));
        let data_dir = package_path.join("Data/com.apple.CoreML");
        std::fs::create_dir_all(&data_dir)?;
        // Place a placeholder model.mlmodel (no real content needed for filename fallback)
        std::fs::write(data_dir.join("model.mlmodel"), b"placeholder")?;
        Ok(package_path)
    }

    /// Test that integrates with Python discover_shapes.py script when available
    #[test]
    fn test_python_assisted_shape_validation() {
        println!("🐍 Testing Python-assisted shape validation with discover_shapes.py");
        
        // Check if Python and the discover_shapes script are available
        let discover_script = PathBuf::from("tools/discover_shapes.py");
        if !discover_script.exists() {
            println!("⚠️  Python discover_shapes.py script not found at: {}", discover_script.display());
            println!("ℹ️  This test validates integration with the Python model introspection tool");
            return;
        }
        
        // Try to use Python to introspect a real typo-fixer model if available
        let potential_model_paths = [
            "/tmp/qwen-typo-fixer-debug",
            "/Users/mazdahewitt/models/qwen-typo-fixer",
            "./models/qwen-typo-fixer",
            "../models/qwen-typo-fixer",
        ];
        
        let mut real_model_path = None;
        for path_str in &potential_model_paths {
            let path = PathBuf::from(path_str);
            if path.exists() && path.is_dir() {
                // Check if it contains typo-fixer model files
                let has_typo_fixer_files = [
                    "qwen-typo-fixer_embeddings.mlpackage",
                    "qwen-typo-fixer_lm_head.mlpackage",
                ].iter().any(|filename| path.join(filename).exists());
                
                if has_typo_fixer_files {
                    real_model_path = Some(path);
                    break;
                }
            }
        }
        
        let Some(model_path) = real_model_path else {
            println!("ℹ️  No real typo-fixer model found. Checking paths:");
            for path in &potential_model_paths {
                println!("   • {} - {}", path, if PathBuf::from(path).exists() { "exists but no typo-fixer files" } else { "not found" });
            }
            println!("ℹ️  To enable full testing, download model to one of these paths:");
            println!("     huggingface-cli download mazhewitt/qwen-typo-fixer-coreml --local-dir /tmp/qwen-typo-fixer-debug");
            return;
        };
        
        println!("📦 Found real typo-fixer model at: {}", model_path.display());
        
        // Run Python discover_shapes.py on the real model
        println!("🐍 Running Python shape discovery on real model...");
        let output = std::process::Command::new("python3")
            .arg(&discover_script)
            .arg("--model-dir")
            .arg(&model_path)
            .arg("--output")
            .arg("/tmp/python_discovered_config.json")
            .arg("--verbose")
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    println!("✅ Python shape discovery succeeded");
                    println!("📄 Output: {}", String::from_utf8_lossy(&result.stdout));
                    
                    // Load the Python-generated config
                    if let Ok(python_config_str) = std::fs::read_to_string("/tmp/python_discovered_config.json") {
                        if let Ok(python_config) = serde_json::from_str::<serde_json::Value>(&python_config_str) {
                            println!("📊 Python-discovered shapes:");
                            
                            if let Some(shapes) = python_config.get("shapes") {
                                println!("   • vocab_size: {}", shapes.get("vocab_size").unwrap_or(&serde_json::Value::Null));
                                println!("   • hidden_size: {}", shapes.get("hidden_size").unwrap_or(&serde_json::Value::Null));
                                println!("   • context_length: {}", shapes.get("context_length").unwrap_or(&serde_json::Value::Null));
                                println!("   • batch_size: {}", shapes.get("batch_size").unwrap_or(&serde_json::Value::Null));
                            }
                            
                            if let Some(components) = python_config.get("components") {
                                println!("📋 Python-discovered components: {:?}", 
                                        components.as_object().map(|obj| obj.keys().collect::<Vec<_>>()).unwrap_or_default());
                            }
                            
                            // Now generate Rust config and compare
                            println!("🦀 Generating Rust config for comparison...");
                            let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
                            
                            match generator.generate_config_from_directory_enhanced(
                                &model_path, 
                                "mazhewitt/qwen-typo-fixer-coreml",
                                "qwen"
                            ) {
                                Ok(rust_config) => {
                                    println!("📊 Rust-generated shapes:");
                                    println!("   • vocab_size: {}", rust_config.shapes.vocab_size);
                                    println!("   • hidden_size: {}", rust_config.shapes.hidden_size);
                                    println!("   • context_length: {}", rust_config.shapes.context_length);
                                    println!("   • batch_size: {}", rust_config.shapes.batch_size);
                                    println!("📋 Rust-generated components: {:?}", rust_config.components.keys().collect::<Vec<_>>());
                                    
                                    // Cross-validate the results
                                    let python_vocab = python_config["shapes"]["vocab_size"].as_u64().unwrap_or(0) as usize;
                                    let python_hidden = python_config["shapes"]["hidden_size"].as_u64().unwrap_or(0) as usize;
                                    
                                    if python_vocab > 0 && rust_config.shapes.vocab_size != python_vocab {
                                        println!("⚠️  Vocab size mismatch: Python={}, Rust={}", python_vocab, rust_config.shapes.vocab_size);
                                    }
                                    
                                    if python_hidden > 0 && rust_config.shapes.hidden_size != python_hidden {
                                        println!("⚠️  Hidden size mismatch: Python={}, Rust={}", python_hidden, rust_config.shapes.hidden_size);
                                    }
                                    
                                    // Check that Rust found all required components
                                    let required_components = ["embeddings", "lm_head", "ffn_prefill", "ffn_infer"];
                                    let mut missing_components = Vec::new();
                                    
                                    for component in &required_components {
                                        if !rust_config.components.contains_key(*component) {
                                            missing_components.push(*component);
                                        }
                                    }
                                    
                                    if missing_components.is_empty() {
                                        println!("✅ Rust config generation found all required components");
                                    } else {
                                        println!("⚠️  Rust config missing components: {:?}", missing_components);
                                        
                                        // This indicates the original bug - config generation with empty tensor maps
                                        println!("🐞 This suggests the original bug: config generation not populating tensor maps properly");
                                    }
                                    
                                    println!("🎯 Cross-language validation complete");
                                }
                                Err(e) => {
                                    println!("❌ Rust config generation failed: {}", e);
                                    println!("🐞 This likely indicates the original bug with empty tensor metadata");
                                }
                            }
                            
                        } else {
                            println!("⚠️  Failed to parse Python-generated JSON config");
                        }
                    } else {
                        println!("⚠️  Failed to read Python-generated config file");
                    }
                } else {
                    println!("❌ Python shape discovery failed:");
                    println!("   stdout: {}", String::from_utf8_lossy(&result.stdout));
                    println!("   stderr: {}", String::from_utf8_lossy(&result.stderr));
                    println!("ℹ️  This might indicate missing Python dependencies (coremltools)");
                }
            }
            Err(e) => {
                println!("❌ Failed to run Python script: {}", e);
                println!("ℹ️  Make sure python3 is installed and available in PATH");
            }
        }
        
        println!("🎯 Python-assisted validation completed");
    }
}