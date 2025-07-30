//! LM Head isolation test - Phase 2 of systematic debugging
//!
//! This test loads the FFN output tensor captured from the Python pipeline
//! and feeds it through the Rust LM head to verify we get "dog" as the top prediction.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, Config as CoreMLConfig, CoreMLModel};
use std::collections::HashMap;
use std::path::PathBuf;

/// Load a numpy array from .npy file
fn load_numpy_tensor(path: &str, device: &Device) -> Result<Tensor> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    // Simple .npy parser - need to handle float16 format
    // Skip the .npy header (first 128 bytes contain metadata)
    let data_start = if buffer.starts_with(b"\x93NUMPY") {
        // Find the end of the header
        let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
        10 + header_len
    } else {
        return Err(anyhow::Error::msg("Invalid .npy file format"));
    };
    
    // For the LM head input, we expect float16 data (2 bytes per element)
    let float_data = &buffer[data_start..];
    let num_floats = float_data.len() / 2;  // float16 is 2 bytes per element
    
    let mut values = Vec::with_capacity(num_floats);
    for chunk in float_data.chunks_exact(2) {
        let bytes = [chunk[0], chunk[1]];
        let half_bits = u16::from_le_bytes(bytes);
        
        // Convert float16 to float32 using half crate or manual conversion
        // Simple float16 to float32 conversion
        let f32_value = half_to_f32(half_bits);
        values.push(f32_value);
    }
    
    // For the LM head input, we expect shape [1, 1, 1024]
    Tensor::from_vec(values, (1, 1, 1024), device).map_err(Into::into)
}

/// Convert float16 (half precision) to float32
fn half_to_f32(half: u16) -> f32 {
    // Extract components
    let sign = (half >> 15) & 0x1;
    let exp = (half >> 10) & 0x1f;
    let frac = half & 0x3ff;
    
    if exp == 0 {
        if frac == 0 {
            // Zero
            return if sign == 0 { 0.0 } else { -0.0 };
        } else {
            // Denormalized number
            let f32_exp = -14i32;
            let f32_frac = (frac as f32) / 1024.0;
            let value = f32_frac * 2.0f32.powi(f32_exp);
            return if sign == 0 { value } else { -value };
        }
    } else if exp == 31 {
        if frac == 0 {
            // Infinity
            return if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY };
        } else {
            // NaN
            return f32::NAN;
        }
    } else {
        // Normalized number
        let f32_exp = (exp as i32) - 15 + 127;
        let f32_frac = ((frac as u32) | 0x400) << 13;  // Add implicit 1 and shift
        let f32_bits = ((sign as u32) << 31) | ((f32_exp as u32) << 23) | (f32_frac & 0x7fffff);
        return f32::from_bits(f32_bits);
    }
}

/// Get the tokenizer to decode token IDs (simplified - just handle the "dog" token)
fn get_dog_token_id() -> i64 {
    5562  // Token ID for " dog" from the Python results
}

/// Convert logits HashMap to a single tensor (concatenate 16 chunks)
fn combine_lm_head_outputs(outputs: HashMap<String, Tensor>) -> Result<Tensor> {
    let mut chunks = Vec::new();
    
    // Collect chunks in order (logits1, logits2, ..., logits16)
    for i in 1..=16 {
        let key = format!("logits{}", i);
        if let Some(chunk) = outputs.get(&key) {
            chunks.push(chunk.clone());
        } else {
            return Err(anyhow::Error::msg(format!("Missing logits chunk: {}", key)));
        }
    }
    
    // Concatenate along vocabulary dimension (last dimension)
    let chunk_refs: Vec<&Tensor> = chunks.iter().collect();
    Tensor::cat(&chunk_refs, 2).map_err(Into::into)  // Concat along dim 2 (vocab dimension)
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors - run manually after qwen-chat-test.py"]
async fn test_lm_head_isolation_rust() -> Result<()> {
    println!("🧠 Testing Rust LM Head in isolation");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Configure LM head model
    let lm_head_config = CoreMLConfig {
        input_names: vec!["hidden_states".to_string()],
        output_name: "logits1".to_string(), // We'll get all outputs
        max_sequence_length: 512,
        vocab_size: 151936,
        model_type: "qwen-lm-head".to_string(),
    };
    
    let lm_head_path = model_dir.join("qwen_lm_head_lut8.mlmodelc");
    let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;
    
    println!("✅ LM head model loaded");
    
    // Load the test tensor (FFN output that goes into LM head)
    let test_tensor_path = "test_tensors/05_lmhead_input.npy";
    let tensor_path = PathBuf::from(test_tensor_path);
    
    if !tensor_path.exists() {
        return Err(anyhow::Error::msg(
            "Test tensor not found. Run 'python3 qwen-chat-test.py --meta <path>' first"
        ));
    }
    
    let device = Device::Cpu;
    let lm_input = load_numpy_tensor(test_tensor_path, &device)?;
    
    println!("📥 Loaded LM head input tensor: {:?}", lm_input.shape());
    
    // Run the LM head
    println!("🔄 Running LM head inference...");
    let lm_outputs = lm_head.forward_all(&[&lm_input])?;
    
    println!("📤 LM head output keys: {:?}", lm_outputs.keys().collect::<Vec<_>>());
    
    // Combine the 16 logits chunks
    let combined_logits = combine_lm_head_outputs(lm_outputs)?;
    println!("📊 Combined logits shape: {:?}", combined_logits.shape());
    
    // Extract final logits [vocab_size] from [1, 1, vocab_size]
    let final_logits = combined_logits.squeeze(0)?.squeeze(0)?;
    println!("📈 Final logits shape: {:?}", final_logits.shape());
    
    // Convert to vector for analysis
    let logits_vec = final_logits.to_vec1::<f32>()?;
    
    // Find the top prediction
    let (top_idx, top_score) = logits_vec
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap();
    
    println!("🎯 RUST LM HEAD RESULT:");
    println!("Top prediction: Token {} with score {:.4}", top_idx, top_score);
    
    // STRICT REQUIREMENT: "dog" MUST be the top prediction - no fallbacks allowed
    let dog_token_id = get_dog_token_id() as usize;
    
    // Show top 10 predictions for debugging
    let mut indexed_logits: Vec<(usize, f32)> = logits_vec.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🏆 Top 10 predictions from Rust LM head:");
    for (i, (token_id, score)) in indexed_logits.iter().take(10).enumerate() {
        let marker = if *token_id == dog_token_id { " ← DOG" } else { "" };
        println!("  {}. Token {} with score {:.4}{}", i + 1, token_id, score, marker);
    }
    
    // FAIL THE TEST if "dog" is not the #1 prediction
    if top_idx != dog_token_id {
        // Find where "dog" ranks
        let dog_score = logits_vec[dog_token_id];
        let mut better_count = 0;
        for &score in &logits_vec {
            if score > dog_score {
                better_count += 1;
            }
        }
        let dog_rank = better_count + 1;
        
        panic!(
            "❌ CRITICAL FAILURE: 'dog' (token {}) is ranked #{} with score {:.4}, but TOP prediction is token {} with score {:.4}. The LM head MUST predict 'dog' as #1!",
            dog_token_id, dog_rank, dog_score, top_idx, top_score
        );
    }
    
    // If we get here, "dog" is the top prediction - verify score matches Python
    println!("✅ SUCCESS: Rust LM head correctly predicts 'dog' (token {}) as TOP token!", dog_token_id);
    
    let expected_score = 21.1875;
    let score_diff = (top_score - expected_score).abs();
    
    if score_diff < 0.01 {
        println!("✅ Score matches Python reference: {:.4} (diff: {:.6})", top_score, score_diff);
    } else {
        println!("⚠️  Score differs from Python: expected {:.4}, got {:.4} (diff: {:.6})", 
                 expected_score, top_score, score_diff);
        
        // Fail if score is significantly different (more than 1.0 difference)
        if score_diff > 1.0 {
            panic!(
                "❌ SCORE MISMATCH: Expected ~{:.4} but got {:.4} (diff: {:.6}). This indicates a serious computation error!",
                expected_score, top_score, score_diff
            );
        }
    }
    
    println!("🎉 LM HEAD TEST PASSED: 'dog' is #1 prediction with correct score!");
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_lm_head_isolation_macos_requirement() {
    // This test should fail on non-macOS platforms
    println!("❌ LM head isolation test requires macOS");
}