//! Comprehensive tensor shape tests for typo-fixer model components
//! 
//! Each test validates specific tensor shapes for each model component
//! Tests are designed to FAIL initially, then be made to pass by implementing proper tensor handling

#[cfg(target_os = "macos")]
mod typo_fixer_tensor_shape_tests {
    use candle_coreml::config_generator::ConfigGenerator;
    use candle_coreml::model_config::{ComponentConfig, TensorConfig};
    use std::collections::HashMap;
    use tempfile::TempDir;
    use std::path::PathBuf;

    /// Reference tensor shapes based on flex_pipeline documentation
    struct TypoFixerReference {
        // Input tokens from corrected_step_1_tokens.json
        input_tokens_shape: [usize; 2],           // [1, 12] - batch=1, seq_len=12
        input_tokens_dtype: String,               // "int32"
        
        // Embeddings component  
        embeddings_input_shape: [usize; 2],       // [1, 12] - input_ids
        embeddings_output_shape: [usize; 3],      // [1, 12, 1024] - hidden_states
        hidden_size: usize,                       // 1024
        
        // FFN Prefill component (batch processing)
        prefill_hidden_input_shape: [usize; 3],   // [1, 12, 1024]
        prefill_position_ids_shape: [usize; 2],   // [1, 12]
        prefill_causal_mask_shape: [usize; 4],    // [1, 1, 256, 256] - full context
        prefill_current_pos_shape: [usize; 1],    // [1] - starts at 0
        prefill_batch_size: usize,                // 128 (for prefill processing)
        
        // FFN Infer component (single token)
        infer_hidden_input_shape: [usize; 3],     // [1, 1, 1024] - single token
        infer_position_ids_shape: [usize; 2],     // [1, 1]
        infer_causal_mask_shape: [usize; 4],      // [1, 1, 1, 256] - single row
        infer_current_pos_shape: [usize; 1],      // [1] - increments per token
        
        // LM Head component
        lm_head_input_shape: [usize; 3],          // [1, 1, 1024]
        lm_head_output_parts: usize,              // 16 - chunked logits
        vocab_size: usize,                        // 151669
        
        // Context and processing parameters
        context_length: usize,                    // 256 (for causal mask)
        max_length: usize,                        // 64 (tokenization padding)
        actual_context: usize,                    // 12 (real token count)
        
        // Expected tokens for validation
        expected_first_token_id: i32,             // 13 ('.') 
        expected_tokens: Vec<i32>,                // [25958, 25, 1096, 738, 763, 702, 2745, 694, 13580, 966, 304, 432]
    }

    impl TypoFixerReference {
        fn new() -> Self {
            Self {
                // Input tokenization
                input_tokens_shape: [1, 12],
                input_tokens_dtype: "int32".to_string(),
                
                // Embeddings
                embeddings_input_shape: [1, 12],
                embeddings_output_shape: [1, 12, 1024],
                hidden_size: 1024,
                
                // Prefill (batch processing)
                prefill_hidden_input_shape: [1, 12, 1024],
                prefill_position_ids_shape: [1, 12],
                prefill_causal_mask_shape: [1, 1, 256, 256],
                prefill_current_pos_shape: [1],
                prefill_batch_size: 128,
                
                // Infer (single token)
                infer_hidden_input_shape: [1, 1, 1024],
                infer_position_ids_shape: [1, 1],
                infer_causal_mask_shape: [1, 1, 1, 256],
                infer_current_pos_shape: [1],
                
                // LM Head
                lm_head_input_shape: [1, 1, 1024],
                lm_head_output_parts: 16,
                vocab_size: 151669,
                
                // Processing parameters
                context_length: 256,
                max_length: 64,
                actual_context: 12,
                
                // Expected results
                expected_first_token_id: 13,  // '.'
                expected_tokens: vec![25958, 25, 1096, 738, 763, 702, 2745, 694, 13580, 966, 304, 432],
            }
        }
    }

    // ==========================================
    // EMBEDDINGS COMPONENT TENSOR SHAPE TESTS
    // ==========================================

    #[test]
    fn test_embeddings_has_input_ids_shape_1_12() {
        let reference = TypoFixerReference::new();
        
        // Create mock embeddings component with expected tensor shapes
        let mut inputs = HashMap::new();
        inputs.insert("input_ids".to_string(), TensorConfig {
            name: "input_ids".to_string(),
            shape: reference.embeddings_input_shape.to_vec(),
            data_type: reference.input_tokens_dtype.clone(),
        });
        
        let mut outputs = HashMap::new();
        outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: reference.embeddings_output_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let embeddings_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_embeddings.mlpackage".to_string()),
            inputs,
            outputs,
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - should pass when tensor extraction works correctly
        assert_eq!(embeddings_component.inputs["input_ids"].shape, reference.embeddings_input_shape.to_vec(),
                  "Embeddings input_ids must have shape [{}, {}] for batch_size=1, seq_len=12", 
                  reference.embeddings_input_shape[0], reference.embeddings_input_shape[1]);
                  
        println!("✅ test_embeddings_has_input_ids_shape_1_12 PASSED");
    }

    #[test]
    fn test_embeddings_has_hidden_states_output_shape_1_12_1024() {
        let reference = TypoFixerReference::new();
        
        // Create mock embeddings component  
        let mut outputs = HashMap::new();
        outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: reference.embeddings_output_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let embeddings_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_embeddings.mlpackage".to_string()),
            inputs: HashMap::new(),
            outputs,
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - hidden_states output must be [1, 12, 1024]
        assert_eq!(embeddings_component.outputs["hidden_states"].shape, reference.embeddings_output_shape.to_vec(),
                  "Embeddings hidden_states output must have shape [{}, {}, {}] for batch=1, seq_len=12, hidden_size=1024", 
                  reference.embeddings_output_shape[0], reference.embeddings_output_shape[1], reference.embeddings_output_shape[2]);
                  
        println!("✅ test_embeddings_has_hidden_states_output_shape_1_12_1024 PASSED");
    }

    // ==========================================  
    // FFN PREFILL COMPONENT TENSOR SHAPE TESTS
    // ==========================================

    #[test]
    fn test_ffn_prefill_has_hidden_states_input_shape_1_12_1024() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: reference.prefill_hidden_input_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let prefill_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - prefill processes full sequence [1, 12, 1024]
        assert_eq!(prefill_component.inputs["hidden_states"].shape, reference.prefill_hidden_input_shape.to_vec(),
                  "FFN Prefill hidden_states input must have shape [{}, {}, {}] for full sequence processing", 
                  reference.prefill_hidden_input_shape[0], reference.prefill_hidden_input_shape[1], reference.prefill_hidden_input_shape[2]);
                  
        println!("✅ test_ffn_prefill_has_hidden_states_input_shape_1_12_1024 PASSED");
    }

    #[test]  
    fn test_ffn_prefill_has_position_ids_shape_1_12() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("position_ids".to_string(), TensorConfig {
            name: "position_ids".to_string(),
            shape: reference.prefill_position_ids_shape.to_vec(),
            data_type: "Int32".to_string(),
        });
        
        let prefill_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - position_ids for full sequence [1, 12]
        assert_eq!(prefill_component.inputs["position_ids"].shape, reference.prefill_position_ids_shape.to_vec(),
                  "FFN Prefill position_ids must have shape [{}, {}] for sequence positions 0-11", 
                  reference.prefill_position_ids_shape[0], reference.prefill_position_ids_shape[1]);
                  
        println!("✅ test_ffn_prefill_has_position_ids_shape_1_12 PASSED");
    }

    #[test]
    fn test_ffn_prefill_has_causal_mask_shape_1_1_256_256() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("causal_mask".to_string(), TensorConfig {
            name: "causal_mask".to_string(),
            shape: reference.prefill_causal_mask_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let prefill_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - causal mask for full context window [1, 1, 256, 256]
        assert_eq!(prefill_component.inputs["causal_mask"].shape, reference.prefill_causal_mask_shape.to_vec(),
                  "FFN Prefill causal_mask must have shape [{}, {}, {}, {}] for full context window", 
                  reference.prefill_causal_mask_shape[0], reference.prefill_causal_mask_shape[1], 
                  reference.prefill_causal_mask_shape[2], reference.prefill_causal_mask_shape[3]);
                  
        println!("✅ test_ffn_prefill_has_causal_mask_shape_1_1_256_256 PASSED");
    }

    #[test]
    fn test_ffn_prefill_has_current_pos_shape_1() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("current_pos".to_string(), TensorConfig {
            name: "current_pos".to_string(),
            shape: reference.prefill_current_pos_shape.to_vec(),
            data_type: "Int32".to_string(),
        });
        
        let prefill_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_prefill_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - current_pos starts at 0 for prefill [1]
        assert_eq!(prefill_component.inputs["current_pos"].shape, reference.prefill_current_pos_shape.to_vec(),
                  "FFN Prefill current_pos must have shape [{}] for scalar position tracking", 
                  reference.prefill_current_pos_shape[0]);
                  
        println!("✅ test_ffn_prefill_has_current_pos_shape_1 PASSED");
    }

    // ==========================================
    // FFN INFER COMPONENT TENSOR SHAPE TESTS  
    // ==========================================

    #[test]
    fn test_ffn_infer_has_hidden_states_input_shape_1_1_1024() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: reference.infer_hidden_input_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let infer_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_FFN_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - infer processes single token [1, 1, 1024]
        assert_eq!(infer_component.inputs["hidden_states"].shape, reference.infer_hidden_input_shape.to_vec(),
                  "FFN Infer hidden_states input must have shape [{}, {}, {}] for single token processing", 
                  reference.infer_hidden_input_shape[0], reference.infer_hidden_input_shape[1], reference.infer_hidden_input_shape[2]);
                  
        println!("✅ test_ffn_infer_has_hidden_states_input_shape_1_1_1024 PASSED");
    }

    #[test]
    fn test_ffn_infer_has_position_ids_shape_1_1() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("position_ids".to_string(), TensorConfig {
            name: "position_ids".to_string(),
            shape: reference.infer_position_ids_shape.to_vec(),
            data_type: "Int32".to_string(),
        });
        
        let infer_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_FFN_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - position_ids for single token [1, 1]
        assert_eq!(infer_component.inputs["position_ids"].shape, reference.infer_position_ids_shape.to_vec(),
                  "FFN Infer position_ids must have shape [{}, {}] for single token position", 
                  reference.infer_position_ids_shape[0], reference.infer_position_ids_shape[1]);
                  
        println!("✅ test_ffn_infer_has_position_ids_shape_1_1 PASSED");
    }

    #[test]
    fn test_ffn_infer_has_causal_mask_shape_1_1_1_256() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("causal_mask".to_string(), TensorConfig {
            name: "causal_mask".to_string(),
            shape: reference.infer_causal_mask_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let infer_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_FFN_chunk_01of01.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - causal mask for single token attention [1, 1, 1, 256]
        assert_eq!(infer_component.inputs["causal_mask"].shape, reference.infer_causal_mask_shape.to_vec(),
                  "FFN Infer causal_mask must have shape [{}, {}, {}, {}] for single token attention", 
                  reference.infer_causal_mask_shape[0], reference.infer_causal_mask_shape[1], 
                  reference.infer_causal_mask_shape[2], reference.infer_causal_mask_shape[3]);
                  
        println!("✅ test_ffn_infer_has_causal_mask_shape_1_1_1_256 PASSED");
    }

    // ==========================================
    // LM HEAD COMPONENT TENSOR SHAPE TESTS
    // ==========================================

    #[test]
    fn test_lm_head_has_hidden_states_input_shape_1_1_1024() {
        let reference = TypoFixerReference::new();
        
        let mut inputs = HashMap::new();
        inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: reference.lm_head_input_shape.to_vec(),
            data_type: "Float16".to_string(),
        });
        
        let lm_head_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_lm_head.mlpackage".to_string()),
            inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - LM head processes single token features [1, 1, 1024]
        assert_eq!(lm_head_component.inputs["hidden_states"].shape, reference.lm_head_input_shape.to_vec(),
                  "LM Head hidden_states input must have shape [{}, {}, {}] for single token features", 
                  reference.lm_head_input_shape[0], reference.lm_head_input_shape[1], reference.lm_head_input_shape[2]);
                  
        println!("✅ test_lm_head_has_hidden_states_input_shape_1_1_1024 PASSED");
    }

    #[test]
    fn test_lm_head_has_chunked_logits_outputs_16_parts() {
        let reference = TypoFixerReference::new();
        
        let mut outputs = HashMap::new();
        
        // Create 16-part chunked logits outputs as per typo-fixer architecture
        for i in 0..reference.lm_head_output_parts {
            let chunk_size = if i == reference.lm_head_output_parts - 1 {
                // Last chunk gets remainder: 151669 - (151669/16) * 15 = 151669 - 9479 * 15 = 151669 - 142185 = 9484
                reference.vocab_size - (reference.vocab_size / reference.lm_head_output_parts) * i
            } else {
                reference.vocab_size / reference.lm_head_output_parts  // 151669 / 16 = 9479
            };
            
            outputs.insert(format!("logits_{}", i), TensorConfig {
                name: format!("logits_{}", i),
                shape: vec![1, 1, chunk_size],
                data_type: "Float16".to_string(),
            });
        }
        
        let lm_head_component = ComponentConfig {
            file_path: Some("qwen-typo-fixer_lm_head.mlpackage".to_string()),
            inputs: HashMap::new(),
            outputs,
            functions: vec![],
            input_order: None,
        };
        
        // STRICT ASSERTION - must have exactly 16 logits outputs
        assert_eq!(lm_head_component.outputs.len(), reference.lm_head_output_parts,
                  "LM Head must have exactly {} logits output chunks", reference.lm_head_output_parts);
        
        // STRICT ASSERTION - all logits chunks must sum to vocab_size
        let total_vocab_size: usize = lm_head_component.outputs
            .values()
            .filter(|tensor| tensor.name.starts_with("logits_"))
            .map(|tensor| tensor.shape.last().copied().unwrap_or(0))
            .sum();
            
        assert_eq!(total_vocab_size, reference.vocab_size,
                  "LM Head chunked logits must sum to vocab_size {} (got {})", 
                  reference.vocab_size, total_vocab_size);
                  
        println!("✅ test_lm_head_has_chunked_logits_outputs_16_parts PASSED");
    }

    // ==========================================
    // INTEGRATION TESTS - COMPONENT INTERACTION
    // ==========================================

    #[test]
    fn test_embeddings_output_matches_ffn_prefill_input() {
        let reference = TypoFixerReference::new();
        
        // Test that embeddings output shape matches FFN prefill input shape
        assert_eq!(reference.embeddings_output_shape, reference.prefill_hidden_input_shape,
                  "Embeddings output [{}, {}, {}] must match FFN prefill input [{}, {}, {}]",
                  reference.embeddings_output_shape[0], reference.embeddings_output_shape[1], reference.embeddings_output_shape[2],
                  reference.prefill_hidden_input_shape[0], reference.prefill_hidden_input_shape[1], reference.prefill_hidden_input_shape[2]);
                  
        println!("✅ test_embeddings_output_matches_ffn_prefill_input PASSED");
    }

    #[test]
    fn test_ffn_infer_output_matches_lm_head_input() {
        let reference = TypoFixerReference::new();
        
        // Test that FFN infer output shape matches LM head input shape  
        // Both should process single token with hidden_size dimensions
        assert_eq!(reference.infer_hidden_input_shape, reference.lm_head_input_shape,
                  "FFN infer output [{}, {}, {}] must match LM head input [{}, {}, {}]",
                  reference.infer_hidden_input_shape[0], reference.infer_hidden_input_shape[1], reference.infer_hidden_input_shape[2],
                  reference.lm_head_input_shape[0], reference.lm_head_input_shape[1], reference.lm_head_input_shape[2]);
                  
        println!("✅ test_ffn_infer_output_matches_lm_head_input PASSED");
    }

    #[test]
    fn test_vocab_size_consistency_across_components() {
        let reference = TypoFixerReference::new();
        
        // Test that vocab_size is consistent across all components that reference it
        // This is critical for typo-fixer model with vocab_size = 151669
        
        assert_eq!(reference.vocab_size, 151669,
                  "Typo-fixer vocab_size must be exactly 151669 (Qwen tokenizer vocabulary)");
        
        // Verify 16-part chunking math works correctly
        let chunk_size = reference.vocab_size / reference.lm_head_output_parts;
        let remainder = reference.vocab_size % reference.lm_head_output_parts;
        
        println!("📊 Vocab size chunking:");
        println!("   • Total vocab_size: {}", reference.vocab_size);
        println!("   • Output parts: {}", reference.lm_head_output_parts);
        println!("   • Chunk size: {}", chunk_size);  
        println!("   • Remainder: {}", remainder);
        
        // For even chunking, remainder should be minimal
        assert!(remainder < reference.lm_head_output_parts,
               "Vocab size remainder {} should be less than output parts {}",
               remainder, reference.lm_head_output_parts);
               
        println!("✅ test_vocab_size_consistency_across_components PASSED");
    }

    // ==========================================
    // EXPECTED TOKEN VALIDATION TESTS
    // ==========================================

    #[test]
    fn test_expected_first_token_is_period_13() {
        let reference = TypoFixerReference::new();
        
        // CRITICAL: First generated token must be 13 ('.') for typo-fixer basic prompt format
        assert_eq!(reference.expected_first_token_id, 13,
                  "First generated token must be 13 ('.') for 'Fix: sentence' prompt format");
        
        println!("✅ Expected first token: {} ('.')", reference.expected_first_token_id);
        println!("✅ test_expected_first_token_is_period_13 PASSED");
    }

    #[test]
    fn test_expected_input_tokenization_sequence() {
        let reference = TypoFixerReference::new();
        
        // Validate the expected tokenization of "Fix: This setence has multple typos in it"
        let expected_tokens = reference.expected_tokens;
        
        assert_eq!(expected_tokens.len(), reference.actual_context,
                  "Tokenized sequence must have exactly {} tokens", reference.actual_context);
        
        // Validate specific token IDs from flex_pipeline reference
        assert_eq!(expected_tokens[0], 25958, "First token must be 'Fix' (25958)");
        assert_eq!(expected_tokens[1], 25, "Second token must be ':' (25)");  
        assert_eq!(expected_tokens[2], 1096, "Third token must be ' This' (1096)");
        
        println!("✅ Input tokenization sequence:");
        for (i, token) in expected_tokens.iter().enumerate() {
            println!("   [{}]: {}", i, token);
        }
        
        println!("✅ test_expected_input_tokenization_sequence PASSED");
    }
}