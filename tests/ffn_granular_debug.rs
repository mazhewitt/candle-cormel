//! FFN Granular Debug - Compare each step vs Python
//!
//! This test breaks down the FFN pipeline into individual steps:
//! 1. Compare prefill inputs (embeddings, position_ids, causal_mask, current_pos)
//! 2. Compare prefill output
//! 3. Compare infer inputs (token_embedding, update_mask, position_ids, causal_mask, current_pos)  
//! 4. Compare infer output
//!
//! This will pinpoint exactly where the divergence occurs.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, Config as CoreMLConfig, CoreMLModel};
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

/// Compare two tensors with detailed analysis
fn compare_tensors(name: &str, rust_tensor: &Tensor, python_tensor: &Tensor, tolerance: f32) -> Result<bool> {
    let rust_vec = rust_tensor.to_vec3::<f32>()?;
    let python_vec = python_tensor.to_vec3::<f32>()?;
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    let mut max_diff_pos = (0, 0, 0);
    
    for (i, (r_batch, py_batch)) in rust_vec.iter().zip(python_vec.iter()).enumerate() {
        for (j, (r_seq, py_seq)) in r_batch.iter().zip(py_batch.iter()).enumerate() {
            for (k, (r_val, py_val)) in r_seq.iter().zip(py_seq.iter()).enumerate() {
                let diff = (r_val - py_val).abs();
                if diff > max_diff {
                    max_diff = diff;
                    max_diff_pos = (i, j, k);
                }
                total_diff += diff;
                num_elements += 1;
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📊 {} COMPARISON:", name.to_uppercase());
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    if max_diff < tolerance {
        println!("  ✅ MATCH! (max diff: {:.8} < tolerance: {:.8})", max_diff, tolerance);
        return Ok(true);
    } else {
        println!("  ❌ DIFFER! (max diff: {:.8} > tolerance: {:.8})", max_diff, tolerance);
        println!("  Biggest difference at [{}, {}, {}]: Rust={:.6}, Python={:.6}", 
                max_diff_pos.0, max_diff_pos.1, max_diff_pos.2,
                rust_vec[max_diff_pos.0][max_diff_pos.1][max_diff_pos.2],
                python_vec[max_diff_pos.0][max_diff_pos.1][max_diff_pos.2]);
        
        // Show first few values for debugging
        println!("  First 5 values - Rust: {:?}", &rust_vec[0][0][0..5.min(rust_vec[0][0].len())]);
        println!("  First 5 values - Python: {:?}", &python_vec[0][0][0..5.min(python_vec[0][0].len())]);
        
        return Ok(false);
    }
}

/// Compare 2D tensors (for position_ids, etc.)
fn compare_tensors_2d(name: &str, rust_tensor: &Tensor, python_tensor: &Tensor, tolerance: f32) -> Result<bool> {
    // Handle different dimensionalities
    let (rust_vec, python_vec) = if rust_tensor.rank() == 1 && python_tensor.rank() == 1 {
        let r = rust_tensor.to_vec1::<f32>()?;
        let p = python_tensor.to_vec1::<f32>()?;
        (vec![r], vec![p])
    } else if rust_tensor.rank() == 2 || python_tensor.rank() == 2 {
        let r = if rust_tensor.rank() == 1 {
            vec![rust_tensor.to_vec1::<f32>()?]
        } else {
            rust_tensor.to_vec2::<f32>()?
        };
        let p = if python_tensor.rank() == 1 {
            vec![python_tensor.to_vec1::<f32>()?]
        } else {
            python_tensor.to_vec2::<f32>()?
        };
        (r, p)
    } else {
        return Err(anyhow::Error::msg(format!("Unsupported tensor ranks for {}: Rust={}, Python={}", name, rust_tensor.rank(), python_tensor.rank())));
    };
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    
    for (r_row, py_row) in rust_vec.iter().zip(python_vec.iter()) {
        for (r_val, py_val) in r_row.iter().zip(py_row.iter()) {
            let diff = (r_val - py_val).abs();
            max_diff = max_diff.max(diff);
            total_diff += diff;
            num_elements += 1;
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📊 {} COMPARISON:", name.to_uppercase());
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    if max_diff < tolerance {
        println!("  ✅ MATCH! (max diff: {:.8} < tolerance: {:.8})", max_diff, tolerance);
        Ok(true)
    } else {
        println!("  ❌ DIFFER! (max diff: {:.8} > tolerance: {:.8})", max_diff, tolerance);
        println!("  First 8 values - Rust: {:?}", &rust_vec[0][0..8.min(rust_vec[0].len())]);
        println!("  First 8 values - Python: {:?}", &python_vec[0][0..8.min(python_vec[0].len())]);
        Ok(false)
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors"]
async fn test_ffn_granular_debug() -> Result<()> {
    println!("🔬 FFN GRANULAR DEBUG - Step by step comparison");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Load FFN models
    let ffn_config_base = CoreMLConfig {
        input_names: vec![
            "hidden_states".to_string(),
            "position_ids".to_string(),
            "causal_mask".to_string(),
            "current_pos".to_string(),
        ],
        output_name: "output_hidden_states".to_string(),
        max_sequence_length: 512,
        vocab_size: 1024, // Hidden size
        model_type: "qwen-ffn".to_string(),
    };
    
    let ffn_path = model_dir.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");
    
    let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;
    
    let mut ffn_infer_config = ffn_config_base.clone();
    ffn_infer_config.input_names.insert(1, "update_mask".to_string());
    let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;
    
    println!("✅ FFN models loaded");
    
    let device = Device::Cpu;
    let tolerance = 1e-6; // Very strict
    
    // ========== STEP 1: PREFILL INPUTS COMPARISON ==========
    println!("\n🔍 STEP 1: PREFILL INPUTS COMPARISON");
    
    // Load Python prefill inputs
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    println!("📥 Python prefill inputs loaded");
    println!("  Hidden states: {:?}", py_prefill_hidden.shape());
    println!("  Position IDs: {:?}", py_prefill_position_ids.shape());
    println!("  Causal mask: {:?}", py_prefill_causal_mask.shape());
    println!("  Current pos: {:?}", py_prefill_current_pos.shape());
    
    // NOTE: For now, we'll assume these are our "ground truth" inputs
    // In a future step, we could compare against QwenModel-generated inputs
    
    // ========== STEP 2: PREFILL OUTPUT COMPARISON ==========
    println!("\n🔍 STEP 2: PREFILL OUTPUT COMPARISON");
    
    // Run Python prefill inputs through Rust FFN prefill
    let mut shared_state = ffn_prefill.make_state()?;
    
    let prefill_inputs = [&py_prefill_hidden, &py_prefill_position_ids, &py_prefill_causal_mask, &py_prefill_current_pos];
    let rust_prefill_output = ffn_prefill.predict_with_state(&prefill_inputs, &mut shared_state)?;
    
    // Load Python prefill output for comparison
    let py_prefill_output = load_numpy_tensor("test_tensors/03_ffn_prefill_output.npy", &[1, 1, 1024], &device)?;
    
    println!("📊 Comparing prefill outputs...");
    let prefill_match = compare_tensors("PREFILL OUTPUT", &rust_prefill_output, &py_prefill_output, tolerance)?;
    
    if !prefill_match {
        println!("🚨 PREFILL OUTPUTS DIFFER! This is the root cause.");
        return Ok(()); // Stop here if prefill already differs
    }
    
    // ========== STEP 3: INFER INPUTS COMPARISON ==========
    println!("\n🔍 STEP 3: INFER INPUTS COMPARISON");
    
    // Load Python infer inputs
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    println!("📥 Python infer inputs loaded");
    println!("  Token embeddings: {:?}", py_infer_hidden.shape());
    println!("  Update mask: {:?}", py_infer_update_mask.shape());
    println!("  Position IDs: {:?}", py_infer_position_ids.shape());
    println!("  Causal mask: {:?}", py_infer_causal_mask.shape());
    
    // ========== STEP 4: INFER OUTPUT COMPARISON ==========
    println!("\n🔍 STEP 4: INFER OUTPUT COMPARISON");
    
    // Run Python infer inputs through Rust FFN infer (using populated state)
    let infer_inputs = [&py_infer_hidden, &py_infer_update_mask, &py_infer_position_ids, &py_infer_causal_mask, &py_infer_current_pos];
    let rust_infer_output = ffn_infer.predict_with_state(&infer_inputs, &mut shared_state)?;
    
    // Load Python infer output for comparison  
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    println!("📊 Comparing infer outputs...");
    let infer_match = compare_tensors("INFER OUTPUT", &rust_infer_output, &py_infer_output, tolerance)?;
    
    if infer_match {
        println!("🎉 ALL FFN STEPS MATCH! The issue might be elsewhere.");
    } else {
        println!("🚨 INFER OUTPUTS DIFFER! Found the exact divergence point.");
    }
    
    // ========== SUMMARY ==========
    println!("\n📋 GRANULAR DEBUG SUMMARY:");
    println!("  Prefill output match: {}", if prefill_match { "✅" } else { "❌" });
    println!("  Infer output match: {}", if infer_match { "✅" } else { "❌" });
    
    if prefill_match && infer_match {
        println!("🔍 FFN pipeline is working correctly! Issue must be in QwenModel integration.");
    } else if !prefill_match {
        println!("🎯 Root cause: PREFILL phase differs from Python");
    } else if !infer_match {
        println!("🎯 Root cause: INFER phase differs from Python");
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_ffn_granular_debug_macos_requirement() {
    println!("❌ FFN granular debug test requires macOS");
}