//! Test each phase individually with Python tensors to isolate the issue
//! 
//! Strategy: Work backwards from LM Head to find where Rust diverges from Python

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use std::fs::File;
use std::io::Read;

/// Load numpy tensor (reuse existing function)
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
        // Better detection: only treat as int32 if it's specifically input_ids, position_ids, etc.
        // NOT token_embeddings which is float32 data
        if (path.contains("input_ids") || path.contains("input_token") || 
            path.contains("position_ids") || path.contains("current_pos")) &&
            !path.contains("embeddings") {
            // int32 data
            let mut values = Vec::with_capacity(expected_elements);
            for chunk in data.chunks_exact(4) {
                let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                let int_val = i32::from_le_bytes(bytes);
                values.push(int_val as f32);
            }
            return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
        } else {
            // float32 data (including token_embeddings!)
            let mut values = Vec::with_capacity(expected_elements);
            for chunk in data.chunks_exact(4) {
                let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                values.push(f32::from_le_bytes(bytes));
            }
            return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
        }
    }
    
    Err(anyhow::Error::msg(format!(
        "Data size mismatch: expected {} elements, got {} bytes",
        expected_elements, data.len()
    )))
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors"]
async fn test_backwards_phase_isolation() -> Result<()> {
    println!("🔍 BACKWARDS PHASE ISOLATION TEST");
    println!("   Testing each phase individually with Python tensors");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    
    println!("✅ QwenModel loaded");
    
    // ========== PHASE 4: LM HEAD (FINAL PHASE) ==========
    println!("\n🧠 PHASE 4: LM HEAD - Does Python infer output produce 'dog'?");
    
    // Load Python's infer output (what should go into LM head)
    let python_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    println!("📊 Python infer output: shape={:?}", python_infer_output.shape());
    let sample = python_infer_output.to_vec3::<f32>()?[0][0][..5].to_vec();
    println!("📊 Sample values: {:?}", sample);
    
    // Run Rust LM Head on Python's infer output
    let rust_lm_output = qwen_model.run_lm_head_with_inputs(&python_infer_output)?;
    
    println!("📊 Rust LM output: shape={:?}", rust_lm_output.shape());
    
    // Extract predicted token and show top predictions to check for ties
    let flat_logits = rust_lm_output.squeeze(0)?.squeeze(0)?;
    let logits_vec = flat_logits.to_vec1::<f32>()?;
    
    // Get top 10 predictions to check for ties
    let mut indexed_logits: Vec<(usize, f32)> = logits_vec.iter().enumerate().map(|(i, &score)| (i, score)).collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("🔍 Top 10 Rust LM Head predictions:");
    for (rank, (token_id, score)) in indexed_logits.iter().take(10).enumerate() {
        let decoded = qwen_model.tokenizer().decode(&[*token_id as u32], false).unwrap_or("???".to_string());
        println!("  {}. Token {} ('{}'): {:.6}", rank + 1, token_id, decoded, score);
    }
    
    let predicted_token = indexed_logits[0].0 as i64;
    let decoded = qwen_model.tokenizer().decode(&[predicted_token as u32], false)
        .map_err(|e| anyhow::Error::msg(format!("Decode error: {}", e)))?;
    
    println!("🎯 LM Head result: Token {} = '{}'", predicted_token, decoded);
    
    if predicted_token == 5562 {
        println!("✅ SUCCESS! LM Head with Python infer output predicts 'dog'");
        println!("   ➜ Issue is in the infer phase or earlier");
    } else {
        println!("❌ FAIL! LM Head with Python infer output predicts '{}' instead of 'dog'", decoded);
        println!("   ➜ Issue is in the LM Head itself");
        
        // Compare with Python's expected LM output
        if let Ok(python_lm_output) = load_numpy_tensor("test_tensors/05_lmhead_combined_logits.npy", &[1, 1, 151936], &device) {
            let python_flat = python_lm_output.squeeze(0)?.squeeze(0)?;
            let python_logits = python_flat.to_vec1::<f32>()?;
            let python_predicted = python_logits
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(i, _)| i as i64)
                .unwrap();
            
            let python_decoded = qwen_model.tokenizer().decode(&[python_predicted as u32], false)
                .map_err(|e| anyhow::Error::msg(format!("Decode error: {}", e)))?;
            
            println!("🔍 Python LM Head predicts: Token {} = '{}'", python_predicted, python_decoded);
            
            // Compare logits at position 5562 (dog token)
            println!("🔍 Dog token (5562) logits:");
            println!("  Rust:   {:.6}", logits_vec[5562]);
            println!("  Python: {:.6}", python_logits[5562]);
        }
        
        return Ok(()); // Stop here if LM Head fails
    }
    
    // ========== PHASE 3: INFER ==========
    println!("\n⚡ PHASE 3: INFER - Does Python prefill + infer inputs produce correct output?");
    
    // First run prefill to populate state
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    qwen_model.reset_states()?;
    let _prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &py_prefill_hidden,
        &py_prefill_position_ids, 
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    println!("✅ Prefill complete - state populated");
    
    // Now run infer with Python inputs
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    let rust_infer_output = qwen_model.run_ffn_infer_with_inputs(
        &py_infer_hidden,
        &py_infer_update_mask,
        &py_infer_position_ids,
        &py_infer_causal_mask,
        &py_infer_current_pos
    )?;
    
    println!("📊 Rust infer output: shape={:?}", rust_infer_output.shape());
    let infer_sample = rust_infer_output.to_vec3::<f32>()?[0][0][..5].to_vec();
    println!("📊 Sample values: {:?}", infer_sample);
    
    // Compare with expected Python infer output
    let expected_infer_sample = python_infer_output.to_vec3::<f32>()?[0][0][..5].to_vec();
    println!("📊 Expected values: {:?}", expected_infer_sample);
    
    // Check if they match (within tolerance)
    let matches = rust_infer_output.to_vec3::<f32>()?[0][0].iter()
        .zip(python_infer_output.to_vec3::<f32>()?[0][0].iter())
        .all(|(r, p)| (r - p).abs() < 0.1);
    
    if matches {
        println!("✅ SUCCESS! Infer output matches Python");
        println!("   ➜ Issue is in the prefill phase or earlier");
    } else {
        println!("❌ FAIL! Infer output differs from Python");
        println!("   ➜ Issue is in the infer phase");
        
        // Test with Rust infer output through LM head
        let rust_lm_with_rust_infer = qwen_model.run_lm_head_with_inputs(&rust_infer_output)?;
        let flat = rust_lm_with_rust_infer.squeeze(0)?.squeeze(0)?;
        let logits = flat.to_vec1::<f32>()?;
        let token = logits.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).map(|(i, _)| i as i64).unwrap();
        let dec = qwen_model.tokenizer().decode(&[token as u32], false).unwrap_or_default();
        println!("🔍 LM Head with Rust infer output predicts: Token {} = '{}'", token, dec);
    }
    
    println!("\n📋 BACKWARDS ISOLATION SUMMARY:");
    println!("  This test isolates which phase is causing the 'dog' vs 'toJson' difference");
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_backwards_phase_isolation_macos_requirement() {
    println!("❌ Backwards phase isolation test requires macOS");
}