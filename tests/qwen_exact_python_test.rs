//! QwenModel with Exact Python Pipeline Test
//!
//! This test uses the EXACT same embeddings, inputs, and pipeline as Python
//! to verify that our architecture produces the exact "dog" prediction.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use std::fs::File;
use std::io::Read;

/// Load numpy tensor (handles int32 and float data)
fn load_numpy_tensor(path: &str, expected_shape: &[usize], device: &Device) -> Result<Tensor> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let data_start = if buffer.starts_with(b"\x93NUMPY") {
        let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
        10 + header_len
    } else {
        return Err(anyhow::Error::msg("Invalid .npy file format"));
    };
    
    let data = &buffer[data_start..];
    let expected_elements: usize = expected_shape.iter().product();
    
    if data.len() == expected_elements * 4 {
        if path.contains("input") || path.contains("token") || path.contains("position") {
            // int32 data
            let mut values = Vec::with_capacity(expected_elements);
            for chunk in data.chunks_exact(4) {
                let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                let int_val = i32::from_le_bytes(bytes);
                values.push(int_val as f32);
            }
            return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
        } else {
            // float32 data  
            let mut values = Vec::with_capacity(expected_elements);
            for chunk in data.chunks_exact(4) {
                let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                values.push(f32::from_le_bytes(bytes));
            }
            return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
        }
    } else if data.len() == expected_elements * 2 {
        // float16 data
        let mut values = Vec::with_capacity(expected_elements);
        for chunk in data.chunks_exact(2) {
            let half_bits = u16::from_le_bytes([chunk[0], chunk[1]]);
            values.push(half_to_f32(half_bits));
        }
        return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
    } else {
        return Err(anyhow::Error::msg(format!(
            "Data size mismatch: expected {} elements, got {} bytes",
            expected_elements, data.len()
        )));
    };
}

/// Convert float16 to float32
fn half_to_f32(half: u16) -> f32 {
    let sign = (half >> 15) & 0x1;
    let exp = (half >> 10) & 0x1f;
    let frac = half & 0x3ff;
    
    if exp == 0 {
        if frac == 0 {
            return if sign == 0 { 0.0 } else { -0.0 };
        } else {
            let f32_exp = -14i32;
            let f32_frac = (frac as f32) / 1024.0;
            let value = f32_frac * 2.0f32.powi(f32_exp);
            return if sign == 0 { value } else { -value };
        }
    } else if exp == 31 {
        if frac == 0 {
            return if sign == 0 { f32::INFINITY } else { f32::NEG_INFINITY };
        } else {
            return f32::NAN;
        }
    } else {
        let f32_exp = (exp as i32) - 15 + 127;
        let f32_frac = ((frac as u32) | 0x400) << 13;
        let f32_bits = ((sign as u32) << 31) | ((f32_exp as u32) << 23) | (f32_frac & 0x7fffff);
        return f32::from_bits(f32_bits);
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors"]
async fn test_qwen_exact_python_pipeline() -> Result<()> {
    println!("🎯 QWEN EXACT PYTHON PIPELINE TEST");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    
    println!("✅ QwenModel loaded");
    
    // STEP 1: Use EXACT Python embeddings for prefill
    println!("📥 Loading Python prefill embeddings...");
    let python_embeddings = load_numpy_tensor(
        "test_tensors/03_ffn_prefill_hidden_input.npy",
        &[1, 64, 1024],
        &device
    )?;
    println!("  Python embeddings shape: {:?}", python_embeddings.shape());
    
    // STEP 2: Reset states and run prefill with Python embeddings
    println!("⚡ Running prefill with EXACT Python embeddings...");
    qwen_model.reset_states()?;
    qwen_model.run_prefill_phase(&python_embeddings, 8)?; // 8 tokens in our prompt
    
    println!("✅ Prefill complete with Python embeddings");
    
    // STEP 3: Use EXACT Python token embedding for infer
    println!("📥 Loading Python infer token embedding...");
    let python_infer_embedding = load_numpy_tensor(
        "test_tensors/04_infer_token_embeddings.npy",
        &[1, 1, 1024],
        &device
    )?;
    println!("  Python infer embedding shape: {:?}", python_infer_embedding.shape());
    
    // STEP 4: Run infer with Python token embedding
    println!("⚡ Running infer with EXACT Python token embedding...");
    // CRITICAL FIX: Use position 7 (last token) instead of 8 (next token) to match Python
    let logits = qwen_model.generate_next_token_with_infer(&python_infer_embedding, 7)?;
    
    // STEP 5: Extract predicted token
    let flat_logits = logits.squeeze(0)?.squeeze(0)?;
    let logits_vec = flat_logits.to_vec1::<f32>()?;
    
    let next_token = logits_vec
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i as i64)
        .unwrap();
    
    println!("🎯 CRITICAL RESULT: Generated token {} (target: 5562 for 'dog')", next_token);
    
    // Decode the predicted token
    if let Ok(decoded) = qwen_model.tokenizer().decode(&[next_token as u32], false) {
        println!("📖 Token {} decodes to: '{}'", next_token, decoded);
    }
    if let Ok(decoded) = qwen_model.tokenizer().decode(&[5562], false) {
        println!("📖 Token 5562 decodes to: '{}'", decoded);
    }
    
    // Check top 5 predictions
    let mut indexed_logits: Vec<(usize, f32)> = logits_vec.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("🔍 Top 5 predictions:");
    for (i, (token_id, score)) in indexed_logits[0..5].iter().enumerate() {
        println!("  {}. Token {} (score: {:.4})", i+1, token_id, score);
        if *token_id == 5562 {
            println!("     ^ This is 'dog'! (token 5562)");
        }
    }
    
    // THE CRITICAL TEST
    if next_token == 5562 {
        println!("✅ 🎉 SUCCESS! QwenModel with exact Python pipeline predicts 'dog'!");
        println!("   Architecture is PERFECT - issue was in embedding generation!");
        Ok(())
    } else {
        // Check if dog is in top predictions
        let dog_rank = indexed_logits.iter().position(|(token_id, _)| *token_id == 5562);
        match dog_rank {
            Some(rank) => {
                let dog_score = indexed_logits[rank].1;
                let top_score = indexed_logits[0].1;
                println!("❌ QwenModel predicts token {} instead of 5562 ('dog')", next_token);
                println!("   'dog' is ranked #{} with score {:.4} vs top score {:.4}", rank + 1, dog_score, top_score);
                panic!("Expected 'dog' (token 5562) as #1, got token {} as #1", next_token);
            }
            None => {
                println!("❌ 'dog' (token 5562) not found in top predictions!");
                panic!("Token 5562 ('dog') not found in predictions");
            }
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_exact_python_pipeline_macos_requirement() {
    println!("❌ QwenModel exact Python pipeline test requires macOS");
}