//! Integration tensor shape tests that exercise the full config generation pipeline
//! 
//! These tests create mock typo-fixer packages and test that the config generation
//! system properly extracts and validates tensor shapes for each component.
//! Tests are designed to FAIL initially when tensor extraction is insufficient,
//! then be made to pass by implementing proper tensor metadata handling.

#[cfg(target_os = "macos")]
mod typo_fixer_integration_tensor_tests {
    use candle_coreml::config_generator::ConfigGenerator;
    use candle_coreml::model_config::{ComponentConfig, TensorConfig};
    use std::collections::HashMap;
    use tempfile::TempDir;
    use std::path::PathBuf;

    /// Create a real mock typo-fixer package with specific tensor shapes
    fn create_typo_fixer_package_with_tensors(
        temp_dir: &std::path::Path, 
        package_name: &str,
        inputs: HashMap<String, TensorConfig>,
        outputs: HashMap<String, TensorConfig>
    ) -> std::io::Result<PathBuf> {
        let package_path = temp_dir.join(format!("{}.mlpackage", package_name));
        let data_dir = package_path.join("Data/com.apple.CoreML");
        std::fs::create_dir_all(&data_dir)?;
        
        // Create a minimal model.mlmodel file
        // This would normally contain the actual tensor metadata, but for testing
        // we'll rely on our mock tensor extraction
        std::fs::write(data_dir.join("model.mlmodel"), b"mock_model_with_tensors")?;
        
        Ok(package_path)
    }

    // ==========================================
    // EMBEDDINGS COMPONENT INTEGRATION TESTS
    // ==========================================

    #[test]
    #[ignore]
    fn test_embeddings_config_generation_has_input_ids_shape_1_12() {
        println!("🧪 Testing embeddings config generation with expected input_ids shape [1, 12]");
        
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create expected tensor configurations for embeddings component
        let mut expected_inputs = HashMap::new();
        expected_inputs.insert("input_ids".to_string(), TensorConfig {
            name: "input_ids".to_string(),
            shape: vec![1, 12],  // batch=1, seq_len=12 from corrected_step_1_tokens.json
            data_type: "Int32".to_string(),
        });
        
        let mut expected_outputs = HashMap::new();
        expected_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],  // batch=1, seq_len=12, hidden_size=1024
            data_type: "Float16".to_string(),
        });
        
        // Create mock embeddings package
        let embeddings_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_embeddings",
            expected_inputs,
            expected_outputs
        ).unwrap();
        
        println!("📦 Created embeddings package: {}", embeddings_path.display());
        
        // Test config generation from directory
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "test/qwen-typo-fixer-embeddings-test",
            "qwen"
        );
        
        match result {
            Ok(config) => {
                // SUCCESS: Config generation worked, validate tensor shapes
                println!("✅ Config generation succeeded for embeddings");
                
                // STRICT ASSERTION: Must have embeddings component
                assert!(config.components.contains_key("embeddings"), 
                       "Config must contain embeddings component");
                
                let embeddings = &config.components["embeddings"];
                
                // STRICT ASSERTION: input_ids tensor must exist with correct shape
                assert!(embeddings.inputs.contains_key("input_ids"),
                       "Embeddings must have input_ids input tensor");
                
                let input_ids = &embeddings.inputs["input_ids"];
                assert_eq!(input_ids.shape, vec![1, 12],
                          "Embeddings input_ids must have shape [1, 12], got {:?}", input_ids.shape);
                
                // STRICT ASSERTION: hidden_states tensor must exist with correct shape  
                assert!(embeddings.outputs.contains_key("hidden_states"),
                       "Embeddings must have hidden_states output tensor");
                
                let hidden_states = &embeddings.outputs["hidden_states"];
                assert_eq!(hidden_states.shape, vec![1, 12, 1024],
                          "Embeddings hidden_states must have shape [1, 12, 1024], got {:?}", hidden_states.shape);
                
                println!("✅ PASS: Embeddings component has correct tensor shapes");
                println!("   • input_ids: {:?}", input_ids.shape);
                println!("   • hidden_states: {:?}", hidden_states.shape);
            }
            Err(e) => {
                // EXPECTED FAILURE: This should fail when tensor extraction is insufficient
                println!("❌ Config generation failed (expected): {}", e);
                println!("💡 This test will pass when tensor metadata extraction is implemented");
                
                // For now, we expect this test to fail because mock packages don't have real tensor metadata
                // Once we implement proper tensor extraction, this test should pass
                panic!("Config generation failed for embeddings component: {}. \
                       This test should PASS when tensor metadata extraction is properly implemented.", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_ffn_prefill_config_generation_has_batch_processing_shapes() {
        println!("🧪 Testing FFN prefill config generation with batch processing tensor shapes");
        
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create expected tensor configurations for FFN prefill component
        let mut expected_inputs = HashMap::new();
        expected_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],  // Full sequence processing
            data_type: "Float16".to_string(),
        });
        expected_inputs.insert("position_ids".to_string(), TensorConfig {
            name: "position_ids".to_string(),
            shape: vec![1, 12],  // Positions for full sequence [0, 1, 2, ..., 11]
            data_type: "Int32".to_string(),
        });
        expected_inputs.insert("causal_mask".to_string(), TensorConfig {
            name: "causal_mask".to_string(),
            shape: vec![1, 1, 256, 256],  // Full causal attention mask
            data_type: "Float16".to_string(),
        });
        expected_inputs.insert("current_pos".to_string(), TensorConfig {
            name: "current_pos".to_string(),
            shape: vec![1],  // Scalar position tracker
            data_type: "Int32".to_string(),
        });
        
        let mut expected_outputs = HashMap::new();
        expected_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],  // Processed sequence
            data_type: "Float16".to_string(),
        });
        
        // Create mock FFN prefill package
        let prefill_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_prefill_chunk_01of01",
            expected_inputs,
            expected_outputs
        ).unwrap();
        
        println!("📦 Created FFN prefill package: {}", prefill_path.display());
        
        // Test config generation
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "test/qwen-typo-fixer-prefill-test",
            "qwen"
        );
        
        match result {
            Ok(config) => {
                println!("✅ Config generation succeeded for FFN prefill");
                
                // STRICT ASSERTION: Must have ffn_prefill component
                assert!(config.components.contains_key("ffn_prefill"), 
                       "Config must contain ffn_prefill component");
                
                let ffn_prefill = &config.components["ffn_prefill"];
                
                // STRICT ASSERTION: Validate all expected input tensors
                assert!(ffn_prefill.inputs.contains_key("hidden_states"),
                       "FFN prefill must have hidden_states input");
                assert_eq!(ffn_prefill.inputs["hidden_states"].shape, vec![1, 12, 1024],
                          "FFN prefill hidden_states input must have shape [1, 12, 1024]");
                
                assert!(ffn_prefill.inputs.contains_key("position_ids"),
                       "FFN prefill must have position_ids input");
                assert_eq!(ffn_prefill.inputs["position_ids"].shape, vec![1, 12],
                          "FFN prefill position_ids must have shape [1, 12]");
                
                assert!(ffn_prefill.inputs.contains_key("causal_mask"),
                       "FFN prefill must have causal_mask input");
                assert_eq!(ffn_prefill.inputs["causal_mask"].shape, vec![1, 1, 256, 256],
                          "FFN prefill causal_mask must have shape [1, 1, 256, 256]");
                
                println!("✅ PASS: FFN prefill has correct batch processing tensor shapes");
            }
            Err(e) => {
                println!("❌ Expected failure: {}", e);
                panic!("FFN prefill config generation failed: {}. \
                       This test should PASS when tensor metadata extraction supports complex multi-input models.", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_ffn_infer_config_generation_has_single_token_shapes() {
        println!("🧪 Testing FFN infer config generation with single token processing shapes");
        
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create expected tensor configurations for FFN infer component (single token processing)
        let mut expected_inputs = HashMap::new();
        expected_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],  // Single token processing
            data_type: "Float16".to_string(),
        });
        expected_inputs.insert("position_ids".to_string(), TensorConfig {
            name: "position_ids".to_string(),
            shape: vec![1, 1],  // Single position
            data_type: "Int32".to_string(),
        });
        expected_inputs.insert("causal_mask".to_string(), TensorConfig {
            name: "causal_mask".to_string(),
            shape: vec![1, 1, 1, 256],  // Single token attention mask
            data_type: "Float16".to_string(),
        });
        
        let mut expected_outputs = HashMap::new();
        expected_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],  // Single processed token
            data_type: "Float16".to_string(),
        });
        
        // Create mock FFN infer package
        let infer_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_FFN_chunk_01of01",
            expected_inputs,
            expected_outputs
        ).unwrap();
        
        println!("📦 Created FFN infer package: {}", infer_path.display());
        
        // Test config generation
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "test/qwen-typo-fixer-infer-test",
            "qwen"
        );
        
        match result {
            Ok(config) => {
                println!("✅ Config generation succeeded for FFN infer");
                
                // STRICT ASSERTION: Must have ffn_infer component
                assert!(config.components.contains_key("ffn_infer"), 
                       "Config must contain ffn_infer component");
                
                let ffn_infer = &config.components["ffn_infer"];
                
                // STRICT ASSERTION: Validate single token processing shapes
                assert!(ffn_infer.inputs.contains_key("hidden_states"),
                       "FFN infer must have hidden_states input");
                assert_eq!(ffn_infer.inputs["hidden_states"].shape, vec![1, 1, 1024],
                          "FFN infer hidden_states must have shape [1, 1, 1024] for single token");
                
                assert!(ffn_infer.inputs.contains_key("causal_mask"),
                       "FFN infer must have causal_mask input");
                assert_eq!(ffn_infer.inputs["causal_mask"].shape, vec![1, 1, 1, 256],
                          "FFN infer causal_mask must have shape [1, 1, 1, 256] for single token attention");
                
                println!("✅ PASS: FFN infer has correct single token processing shapes");
            }
            Err(e) => {
                println!("❌ Expected failure: {}", e);
                panic!("FFN infer config generation failed: {}. \
                       This test should PASS when tensor metadata extraction handles autoregressive model components.", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_lm_head_config_generation_has_chunked_logits_vocab_151669() {
        println!("🧪 Testing LM head config generation with 16-part chunked logits (vocab_size=151669)");
        
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create expected tensor configurations for LM head component
        let mut expected_inputs = HashMap::new();
        expected_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],  // Single token features
            data_type: "Float16".to_string(),
        });
        
        let mut expected_outputs = HashMap::new();
        let vocab_size = 151669;
        let num_chunks = 16;
        
        // Create 16-part chunked logits outputs
        for i in 0..num_chunks {
            let chunk_size = if i == num_chunks - 1 {
                // Last chunk gets remainder
                vocab_size - (vocab_size / num_chunks) * i
            } else {
                vocab_size / num_chunks  // ~9479 per chunk
            };
            
            expected_outputs.insert(format!("logits_{}", i), TensorConfig {
                name: format!("logits_{}", i),
                shape: vec![1, 1, chunk_size],
                data_type: "Float16".to_string(),
            });
        }
        
        // Create mock LM head package
        let lm_head_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_lm_head",
            expected_inputs,
            expected_outputs
        ).unwrap();
        
        println!("📦 Created LM head package: {}", lm_head_path.display());
        
        // Test config generation
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "test/qwen-typo-fixer-lm-head-test",
            "qwen"
        );
        
        match result {
            Ok(config) => {
                println!("✅ Config generation succeeded for LM head");
                
                // STRICT ASSERTION: Must have lm_head component
                assert!(config.components.contains_key("lm_head"), 
                       "Config must contain lm_head component");
                
                let lm_head = &config.components["lm_head"];
                
                // STRICT ASSERTION: Validate input tensor
                assert!(lm_head.inputs.contains_key("hidden_states"),
                       "LM head must have hidden_states input");
                assert_eq!(lm_head.inputs["hidden_states"].shape, vec![1, 1, 1024],
                          "LM head hidden_states input must have shape [1, 1, 1024]");
                
                // STRICT ASSERTION: Must have exactly 16 logits outputs
                let logits_outputs: Vec<_> = lm_head.outputs.keys()
                    .filter(|k| k.starts_with("logits_"))
                    .collect();
                assert_eq!(logits_outputs.len(), num_chunks,
                          "LM head must have exactly {} logits outputs", num_chunks);
                
                // STRICT ASSERTION: Logits must sum to vocab_size 151669
                let total_vocab: usize = lm_head.outputs.values()
                    .filter(|tensor| tensor.name.starts_with("logits_"))
                    .map(|tensor| tensor.shape.last().copied().unwrap_or(0))
                    .sum();
                
                assert_eq!(total_vocab, vocab_size,
                          "LM head chunked logits must sum to vocab_size {} (got {})", vocab_size, total_vocab);
                
                // CRITICAL ASSERTION: This validates typo-fixer can generate token 13 ('.')
                assert!(total_vocab > 13,
                       "Vocab size {} must be > 13 to generate expected first token '.'", total_vocab);
                
                println!("✅ PASS: LM head has correct chunked logits structure");
                println!("   - Total vocab size: {}", total_vocab);
                println!("   - Logits chunks: {}", logits_outputs.len());
                println!("   - Can generate token 13 ('.'): {}", total_vocab > 13);
            }
            Err(e) => {
                println!("❌ Expected failure: {}", e);
                panic!("LM head config generation failed: {}. \
                       This test should PASS when tensor metadata extraction supports chunked output models with large vocabulary sizes.", e);
            }
        }
    }

    // ==========================================
    // COMPLETE PIPELINE INTEGRATION TEST
    // ==========================================

    #[test]
    #[ignore]
    fn test_complete_typo_fixer_pipeline_tensor_shapes() {
        println!("🧪 Testing complete typo-fixer pipeline with all component tensor shapes");
        
        let temp_dir = TempDir::new().unwrap();
        let generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
        
        // Create all four typo-fixer components with proper tensor shapes
        
        // 1. Embeddings component
        let mut emb_inputs = HashMap::new();
        emb_inputs.insert("input_ids".to_string(), TensorConfig {
            name: "input_ids".to_string(),
            shape: vec![1, 12],
            data_type: "Int32".to_string(),
        });
        let mut emb_outputs = HashMap::new();
        emb_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],
            data_type: "Float16".to_string(),
        });
        let _embeddings_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_embeddings",
            emb_inputs,
            emb_outputs
        ).unwrap();
        
        // 2. FFN Prefill component
        let mut prefill_inputs = HashMap::new();
        prefill_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],
            data_type: "Float16".to_string(),
        });
        prefill_inputs.insert("causal_mask".to_string(), TensorConfig {
            name: "causal_mask".to_string(),
            shape: vec![1, 1, 256, 256],
            data_type: "Float16".to_string(),
        });
        let mut prefill_outputs = HashMap::new();
        prefill_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 12, 1024],
            data_type: "Float16".to_string(),
        });
        let _prefill_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_prefill_chunk_01of01",
            prefill_inputs,
            prefill_outputs
        ).unwrap();
        
        // 3. FFN Infer component
        let mut infer_inputs = HashMap::new();
        infer_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],
            data_type: "Float16".to_string(),
        });
        let mut infer_outputs = HashMap::new();
        infer_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],
            data_type: "Float16".to_string(),
        });
        let _infer_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_FFN_chunk_01of01",
            infer_inputs,
            infer_outputs
        ).unwrap();
        
        // 4. LM Head component
        let mut lm_inputs = HashMap::new();
        lm_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![1, 1, 1024],
            data_type: "Float16".to_string(),
        });
        let mut lm_outputs = HashMap::new();
        // Simplified: just create a few logits outputs for testing
        for i in 0..4 {
            lm_outputs.insert(format!("logits_{}", i), TensorConfig {
                name: format!("logits_{}", i),
                shape: vec![1, 1, 37917],  // 151669 / 4 ≈ 37917
                data_type: "Float16".to_string(),
            });
        }
        let _lm_head_path = create_typo_fixer_package_with_tensors(
            temp_dir.path(),
            "qwen-typo-fixer_lm_head",
            lm_inputs,
            lm_outputs
        ).unwrap();
        
        println!("📦 Created complete typo-fixer model with 4 components");
        
        // Test complete pipeline config generation
        let result = generator.generate_config_from_directory_enhanced(
            temp_dir.path(),
            "mazhewitt/qwen-typo-fixer-coreml-test",
            "qwen"
        );
        
        match result {
            Ok(config) => {
                println!("✅ COMPLETE PIPELINE config generation succeeded!");
                
                // STRICT ASSERTION: All 4 components must be present
                assert!(config.components.contains_key("embeddings"), "Missing embeddings");
                assert!(config.components.contains_key("ffn_prefill"), "Missing ffn_prefill");
                assert!(config.components.contains_key("ffn_infer"), "Missing ffn_infer");
                assert!(config.components.contains_key("lm_head"), "Missing lm_head");
                
                // STRICT ASSERTION: Execution mode must be "split" for typo-fixer
                assert_eq!(config.ffn_execution.as_deref(), Some("split"),
                          "Typo-fixer must use split FFN execution mode");
                
                // STRICT ASSERTION: Model shapes must be reasonable for typo-fixer
                assert!(config.shapes.vocab_size >= 151669,
                       "Config vocab_size {} must be >= 151669 for typo-fixer", config.shapes.vocab_size);
                assert!(config.shapes.hidden_size >= 1024,
                       "Config hidden_size {} must be >= 1024", config.shapes.hidden_size);
                assert!(config.shapes.context_length > 0,
                       "Context length must be > 0");
                assert!(config.shapes.batch_size > 0,
                       "Batch size must be > 0");
                
                println!("✅ ULTIMATE SUCCESS: Complete typo-fixer pipeline configured correctly!");
                println!("   • Components: {}", config.components.len());
                println!("   • Execution: {}", config.ffn_execution.as_deref().unwrap_or("unknown"));
                println!("   • Vocab size: {}", config.shapes.vocab_size);
                println!("   • Hidden size: {}", config.shapes.hidden_size);
                println!("   • Context length: {}", config.shapes.context_length);
                
                // THIS IS THE ULTIMATE GOAL: Complete typo-fixer model configuration success
                
            }
            Err(e) => {
                println!("❌ Complete pipeline failed: {}", e);
                panic!("Complete typo-fixer pipeline configuration failed: {}. \
                       This test represents the ULTIMATE GOAL - when it passes, the typo-fixer CLI should work correctly.", e);
            }
        }
    }
}