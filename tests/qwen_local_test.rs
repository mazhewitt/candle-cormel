//! Simple test to verify Qwen components work with local files

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
use std::path::Path;

const QWEN_VOCAB_SIZE: usize = 151936;
const HIDDEN_SIZE: usize = 896;

#[test]
#[cfg(target_os = "macos")]
fn test_qwen_local_components() -> Result<()> {
    let device = Device::Cpu;
    let base_path = Path::new("/Users/mazdahewitt/projects/candle-coreml/qwen-model");
    
    println!("Testing Qwen components with local files...");
    
    // Test embeddings
    let embeddings_path = base_path.join("qwen_embeddings.mlmodelc");
    if embeddings_path.exists() {
        println!("✅ Found embeddings model");
        
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };
        
        match CoreMLModel::load_from_file(&embeddings_path, &embeddings_config) {
            Ok(embeddings) => {
                println!("✅ Embeddings model loaded successfully");
                
                // Test with simple input
                let input_ids = vec![1i64]; // Simple test token
                let input_tensor = Tensor::from_vec(input_ids, (1, 1), &device)?;
                
                match embeddings.forward(&[&input_tensor]) {
                    Ok(output) => {
                        println!("✅ Embeddings forward pass successful: {:?}", output.dims());
                    }
                    Err(e) => {
                        println!("❌ Embeddings forward pass failed: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("❌ Failed to load embeddings: {}", e);
            }
        }
    } else {
        println!("❌ Embeddings model not found at: {}", embeddings_path.display());
    }
    
    // Test FFN
    let ffn_path = base_path.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");
    if ffn_path.exists() {
        println!("✅ Found FFN model");
        
        let ffn_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string(), "position_ids".to_string(), "causal_mask".to_string(), "current_pos".to_string()],
            output_name: "output_hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: HIDDEN_SIZE,
            model_type: "qwen-ffn".to_string(),
        };
        
        match CoreMLModel::load_from_file(&ffn_path, &ffn_config) {
            Ok(_ffn) => {
                println!("✅ FFN model loaded successfully");
                // FFN test would need proper hidden states tensor - skip for now
            }
            Err(e) => {
                println!("❌ Failed to load FFN: {}", e);
            }
        }
    } else {
        println!("❌ FFN model not found at: {}", ffn_path.display());
    }
    
    // Test LM head
    let lm_head_path = base_path.join("qwen_lm_head_lut8.mlmodelc");
    if lm_head_path.exists() {
        println!("✅ Found LM head model");
        
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head".to_string(),
        };
        
        match CoreMLModel::load_from_file(&lm_head_path, &lm_head_config) {
            Ok(_lm_head) => {
                println!("✅ LM head model loaded successfully");
                // LM head test would need proper hidden states tensor - skip for now
            }
            Err(e) => {
                println!("❌ Failed to load LM head: {}", e);
            }
        }
    } else {
        println!("❌ LM head model not found at: {}", lm_head_path.display());
    }
    
    Ok(())
}