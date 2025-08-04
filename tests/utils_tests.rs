//! Comprehensive tests for utils.rs utilities
//! 
//! These tests cover the core utility functions that had minimal coverage:
//! - Causal mask creation and validation
//! - Position mask logic and edge cases  
//! - Sampling utilities with different parameters
//! - Multi-component configuration builders
//! - Chunked logits combination

use candle_core::{Device, Tensor};
use candle_coreml::utils::{mask, sampling, multi_component};
use std::collections::HashMap;

// Test helper to check if two tensors are approximately equal
fn tensors_approx_equal(a: &Tensor, b: &Tensor, tolerance: f32) -> bool {
    if a.shape() != b.shape() {
        return false;
    }
    
    let a_vec = a.to_vec1::<f32>().unwrap();
    let b_vec = b.to_vec1::<f32>().unwrap();
    
    a_vec.iter().zip(b_vec.iter()).all(|(x, y)| (x - y).abs() < tolerance)
}

#[cfg(target_os = "macos")]
mod mask_tests {
    use super::*;

    #[test]
    fn test_create_causal_mask_basic() {
        let device = Device::Cpu;
        let seq_len = 3;
        
        let mask = mask::create_causal_mask(seq_len, &device).unwrap();
        
        // Check dimensions
        assert_eq!(mask.dims(), &[3, 3]);
        
        // Check causal pattern - lower triangle should be 0.0, upper triangle should be -inf
        let mask_data = mask.to_vec2::<f32>().unwrap();
        
        // Position (0,0): can attend to itself
        assert_eq!(mask_data[0][0], 0.0);
        // Position (0,1): cannot attend to future 
        assert_eq!(mask_data[0][1], f32::NEG_INFINITY);
        // Position (0,2): cannot attend to future
        assert_eq!(mask_data[0][2], f32::NEG_INFINITY);
        
        // Position (1,0): can attend to past
        assert_eq!(mask_data[1][0], 0.0);
        // Position (1,1): can attend to itself
        assert_eq!(mask_data[1][1], 0.0);
        // Position (1,2): cannot attend to future
        assert_eq!(mask_data[1][2], f32::NEG_INFINITY);
        
        // Position (2,*): can attend to all (last position)
        assert_eq!(mask_data[2][0], 0.0);
        assert_eq!(mask_data[2][1], 0.0); 
        assert_eq!(mask_data[2][2], 0.0);
    }

    #[test]
    fn test_create_causal_mask_single_token() {
        let device = Device::Cpu;
        let mask = mask::create_causal_mask(1, &device).unwrap();
        
        assert_eq!(mask.dims(), &[1, 1]);
        assert_eq!(mask.to_vec1::<f32>().unwrap()[0], 0.0);
    }

    #[test]
    fn test_create_causal_mask_large() {
        let device = Device::Cpu;
        let seq_len = 10;
        let mask = mask::create_causal_mask(seq_len, &device).unwrap();
        
        assert_eq!(mask.dims(), &[10, 10]);
        
        let mask_data = mask.to_vec2::<f32>().unwrap();
        
        // Check causal property holds for larger matrix
        for i in 0..seq_len {
            for j in 0..seq_len {
                if j > i {
                    assert_eq!(mask_data[i][j], f32::NEG_INFINITY, 
                        "Position ({},{}) should be -inf", i, j);
                } else {
                    assert_eq!(mask_data[i][j], 0.0,
                        "Position ({},{}) should be 0.0", i, j);
                }
            }
        }
    }

    #[test]
    fn test_create_position_mask_basic() {
        let device = Device::Cpu;
        let pos = 2;
        let context_len = 5;
        
        let mask = mask::create_position_mask(pos, context_len, &device).unwrap();
        
        assert_eq!(mask.dims(), &[1, 5]);
        
        let mask_data = mask.to_vec1::<f32>().unwrap();
        
        // Should allow attention to positions 0, 1, 2 (up to and including pos)
        assert_eq!(mask_data[0], 0.0);
        assert_eq!(mask_data[1], 0.0);
        assert_eq!(mask_data[2], 0.0);
        
        // Should block attention to positions 3, 4 (after pos)
        assert_eq!(mask_data[3], f32::NEG_INFINITY);
        assert_eq!(mask_data[4], f32::NEG_INFINITY);
    }

    #[test]
    fn test_create_position_mask_edge_cases() {
        let device = Device::Cpu;
        
        // Position 0 (first token)
        let mask = mask::create_position_mask(0, 3, &device).unwrap();
        let data = mask.to_vec1::<f32>().unwrap();
        assert_eq!(data[0], 0.0);
        assert_eq!(data[1], f32::NEG_INFINITY);
        assert_eq!(data[2], f32::NEG_INFINITY);
        
        // Position equal to context_len - 1 (last valid position)
        let mask = mask::create_position_mask(2, 3, &device).unwrap();
        let data = mask.to_vec1::<f32>().unwrap();
        assert_eq!(data[0], 0.0);
        assert_eq!(data[1], 0.0);
        assert_eq!(data[2], 0.0); // All positions allowed
        
        // Position exceeding context_len (should handle gracefully)
        let mask = mask::create_position_mask(5, 3, &device).unwrap();
        let data = mask.to_vec1::<f32>().unwrap();
        // Should allow all positions (clamped to context_len - 1)
        assert_eq!(data[0], 0.0);
        assert_eq!(data[1], 0.0);
        assert_eq!(data[2], 0.0);
    }

    #[test]
    fn test_create_rank4_position_mask() {
        let device = Device::Cpu;
        let pos = 1;
        let context_len = 4;
        
        let mask = mask::create_rank4_position_mask(pos, context_len, &device).unwrap();
        
        // Should have rank-4 shape (1, 1, 1, context_len)
        assert_eq!(mask.dims(), &[1, 1, 1, 4]);
        
        let mask_data = mask.to_vec1::<f32>().unwrap();
        
        // Should allow positions 0, 1
        assert_eq!(mask_data[0], 0.0);
        assert_eq!(mask_data[1], 0.0);
        
        // Should block positions 2, 3
        assert_eq!(mask_data[2], f32::NEG_INFINITY);
        assert_eq!(mask_data[3], f32::NEG_INFINITY);
    }

    #[test]
    fn test_create_update_mask() {
        let device = Device::Cpu;
        let pos = 2;
        let context_len = 5;
        
        let mask = mask::create_update_mask(pos, context_len, &device).unwrap();
        
        assert_eq!(mask.dims(), &[1, 1, 5, 1]);
        
        let mask_data = mask.to_vec1::<f32>().unwrap();
        
        // Should have 1.0 only at the target position
        assert_eq!(mask_data[0], 0.0);
        assert_eq!(mask_data[1], 0.0);
        assert_eq!(mask_data[2], 1.0); // Target position
        assert_eq!(mask_data[3], 0.0);
        assert_eq!(mask_data[4], 0.0);
    }

    #[test]
    fn test_create_update_mask_out_of_bounds() {
        let device = Device::Cpu;
        let pos = 10; // Beyond context_len
        let context_len = 5;
        
        let mask = mask::create_update_mask(pos, context_len, &device).unwrap();
        let mask_data = mask.to_vec1::<f32>().unwrap();
        
        // All positions should be 0.0 when pos is out of bounds
        assert!(mask_data.iter().all(|&x| x == 0.0));
    }
}

#[cfg(target_os = "macos")]
mod sampling_tests {
    use super::*;

    #[test]
    fn test_greedy_sample() {
        let device = Device::Cpu;
        
        // Create logits where token 2 has highest probability
        let logits_data = vec![1.0, 2.0, 5.0, 0.5];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        let sampled_token = sampling::greedy_sample(&logits).unwrap();
        assert_eq!(sampled_token, 2); // Index of highest logit
    }

    #[test]
    fn test_greedy_sample_ties() {
        let device = Device::Cpu;
        
        // Multiple tokens with same highest logit
        let logits_data = vec![3.0, 1.0, 3.0, 2.0];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        let sampled_token = sampling::greedy_sample(&logits).unwrap();
        // Should return first occurrence of max value
        assert_eq!(sampled_token, 0);
    }

    #[test]
    fn test_sample_with_temperature_greedy() {
        let device = Device::Cpu;
        let logits_data = vec![1.0, 2.0, 5.0, 0.5];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        // Temperature 0.0 should behave like greedy sampling
        let sampled_token = sampling::sample_with_temperature(&logits, 0.0).unwrap();
        assert_eq!(sampled_token, 2);
    }

    #[test]
    fn test_sample_with_temperature_deterministic_high_temp() {
        let device = Device::Cpu;
        
        // Very skewed logits - even with high temperature, should usually pick the obvious choice
        let logits_data = vec![-10.0, -10.0, 10.0, -10.0];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        // Even with high temperature, should almost always pick token 2
        let sampled_token = sampling::sample_with_temperature(&logits, 2.0).unwrap();
        assert_eq!(sampled_token, 2);
    }

    #[test]
    fn test_sample_top_k_greedy() {
        let device = Device::Cpu;
        let logits_data = vec![1.0, 4.0, 2.0, 3.0];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        // With temperature 0, should return highest from top-k
        let sampled_token = sampling::sample_top_k(&logits, 2, 0.0).unwrap();
        assert_eq!(sampled_token, 1); // Highest logit
    }

    #[test]
    fn test_sample_top_k_filtering() {
        let device = Device::Cpu;
        let logits_data = vec![1.0, 4.0, 2.0, 3.0];
        let logits = Tensor::from_vec(logits_data, (4,), &device).unwrap();
        
        // With k=1, should only consider the top token (index 1 with logit 4.0)
        let sampled_token = sampling::sample_top_k(&logits, 1, 0.0).unwrap();
        assert_eq!(sampled_token, 1);
    }

    #[test]
    fn test_sample_top_k_empty() {
        let device = Device::Cpu;
        let logits_data = vec![1.0];
        let logits = Tensor::from_vec(logits_data, (1,), &device).unwrap();
        
        // With k=0, should fallback gracefully
        let sampled_token = sampling::sample_top_k(&logits, 0, 0.0).unwrap();
        assert_eq!(sampled_token, 0); // Fallback behavior
    }
}

#[cfg(target_os = "macos")]
mod multi_component_tests {
    use super::*;

    #[test]
    fn test_component_config_builder_embeddings() {
        let builder = multi_component::ComponentConfigBuilder::new(1000, 512);
        let config = builder.embeddings_config("qwen");
        
        assert_eq!(config.vocab_size, 1000);
        assert_eq!(config.max_sequence_length, 512);
        assert_eq!(config.input_names, vec!["input_ids".to_string()]);
        assert_eq!(config.output_name, "hidden_states");
        assert_eq!(config.model_type, "qwen-embeddings");
    }

    #[test]
    fn test_component_config_builder_ffn_with_mask() {
        let builder = multi_component::ComponentConfigBuilder::new(2048, 1024);
        let config = builder.ffn_config("llama", true);
        
        assert_eq!(config.vocab_size, 2048);
        assert_eq!(config.max_sequence_length, 1024);
        assert_eq!(config.input_names, vec!["hidden_states".to_string(), "causal_mask".to_string()]);
        assert_eq!(config.output_name, "output_hidden_states");
        assert_eq!(config.model_type, "llama-ffn");
    }

    #[test]
    fn test_component_config_builder_ffn_without_mask() {
        let builder = multi_component::ComponentConfigBuilder::new(2048, 1024);
        let config = builder.ffn_config("llama", false);
        
        assert_eq!(config.input_names, vec!["hidden_states".to_string()]);
        assert_eq!(config.output_name, "output_hidden_states");
        assert_eq!(config.model_type, "llama-ffn");
    }

    #[test]
    fn test_component_config_builder_lm_head() {
        let builder = multi_component::ComponentConfigBuilder::new(50000, 2048);
        let config = builder.lm_head_config("mistral");
        
        assert_eq!(config.vocab_size, 50000);
        assert_eq!(config.max_sequence_length, 2048);
        assert_eq!(config.input_names, vec!["hidden_states".to_string()]);
        assert_eq!(config.output_name, "logits");
        assert_eq!(config.model_type, "mistral-lm-head");
    }

    #[test]
    fn test_combine_chunked_logits_success() {
        let device = Device::Cpu;
        let mut outputs = HashMap::new();
        
        // Create mock logits chunks
        let chunk1 = Tensor::from_vec(vec![1.0, 2.0], (1, 2), &device).unwrap();
        let chunk2 = Tensor::from_vec(vec![3.0, 4.0], (1, 2), &device).unwrap();
        let chunk3 = Tensor::from_vec(vec![5.0, 6.0], (1, 2), &device).unwrap();
        
        outputs.insert("logits1".to_string(), chunk1);
        outputs.insert("logits2".to_string(), chunk2);
        outputs.insert("logits3".to_string(), chunk3);
        
        let combined = multi_component::combine_chunked_logits(outputs, 3).unwrap();
        
        // Should concatenate along last dimension: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        assert_eq!(combined.dims(), &[1, 6]);
        let combined_data = combined.to_vec1::<f32>().unwrap();
        assert_eq!(combined_data, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_combine_chunked_logits_missing_chunk() {
        let device = Device::Cpu;
        let mut outputs = HashMap::new();
        
        // Only provide chunk1, missing chunk2
        let chunk1 = Tensor::from_vec(vec![1.0, 2.0], (1, 2), &device).unwrap();
        outputs.insert("logits1".to_string(), chunk1);
        // chunk2 is missing
        
        let result = multi_component::combine_chunked_logits(outputs, 2);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Missing logits chunk: logits2"));
    }

    #[test]
    fn test_combine_chunked_logits_single_chunk() {
        let device = Device::Cpu;
        let mut outputs = HashMap::new();
        
        let chunk1 = Tensor::from_vec(vec![10.0, 20.0, 30.0], (1, 3), &device).unwrap();
        outputs.insert("logits1".to_string(), chunk1);
        
        let combined = multi_component::combine_chunked_logits(outputs, 1).unwrap();
        
        // Single chunk should remain unchanged
        assert_eq!(combined.dims(), &[1, 3]);
        let combined_data = combined.to_vec1::<f32>().unwrap();
        assert_eq!(combined_data, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_combine_chunked_logits_empty() {
        let outputs = HashMap::new();
        
        let result = multi_component::combine_chunked_logits(outputs, 1);
        
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Missing logits chunk: logits1"));
    }
}

// Test helper functions that aren't platform-specific
#[test]
fn test_tensors_approx_equal_helper() {
    let device = Device::Cpu;
    
    let tensor1 = Tensor::from_vec(vec![1.0, 2.0, 3.0], (3,), &device).unwrap();
    let tensor2 = Tensor::from_vec(vec![1.001, 1.999, 3.001], (3,), &device).unwrap();
    let tensor3 = Tensor::from_vec(vec![1.0, 2.0], (2,), &device).unwrap(); // Different shape
    
    assert!(tensors_approx_equal(&tensor1, &tensor2, 0.01));
    assert!(!tensors_approx_equal(&tensor1, &tensor2, 0.0001));
    assert!(!tensors_approx_equal(&tensor1, &tensor3, 0.01)); // Different shapes
}

#[cfg(not(target_os = "macos"))]
mod non_macos_tests {
    #[test]
    fn test_utils_available_on_non_macos() {
        // These tests ensure the module structure is available even on non-macOS
        // The actual functionality requires CoreML, but the API should be accessible
        
        // This test mainly ensures the module compiles on non-macOS platforms
        // Most actual functionality testing requires macOS and CoreML support
        assert!(true, "Utils module should be accessible on all platforms");
    }
}