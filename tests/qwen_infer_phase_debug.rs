//! Focused test for QwenModel infer phase mismatch
//!
//! This test isolates the specific infer phase issue where:
//! - Prefill phase: ✅ Perfect match 
//! - Infer phase: ❌ Major difference (max diff: 55.18, mean: 4.24)
//!
//! Uses exact Python reference tensors to eliminate input preparation issues.

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

/// Compare tensors with detailed analysis
fn compare_tensors_detailed(name: &str, rust_tensor: &Tensor, python_tensor: &Tensor, tolerance: f32) -> Result<()> {
    let rust_vec = rust_tensor.to_vec3::<f32>()?;
    let python_vec = python_tensor.to_vec3::<f32>()?;
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    let mut max_diff_pos = (0, 0, 0);
    let mut large_diffs = Vec::new();
    
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
                
                // Collect large differences for analysis
                if diff > 1.0 {
                    large_diffs.push((i, j, k, *r_val, *py_val, diff));
                }
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("🔍 {} DETAILED ANALYSIS:", name.to_uppercase());
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements: {}", num_elements);
    println!("  Large differences (>1.0): {}", large_diffs.len());
    
    if max_diff < tolerance {
        println!("  ✅ MATCH! (max diff: {:.8} < tolerance: {:.8})", max_diff, tolerance);
    } else {
        println!("  ❌ DIFFER! (max diff: {:.8} > tolerance: {:.8})", max_diff, tolerance);
        println!("  Biggest difference at [{}, {}, {}]: Rust={:.6}, Python={:.6}", 
                max_diff_pos.0, max_diff_pos.1, max_diff_pos.2,
                rust_vec[max_diff_pos.0][max_diff_pos.1][max_diff_pos.2],
                python_vec[max_diff_pos.0][max_diff_pos.1][max_diff_pos.2]);
        
        // Show first few values for debugging
        println!("  First 5 values - Rust: {:?}", &rust_vec[0][0][0..5.min(rust_vec[0][0].len())]);
        println!("  First 5 values - Python: {:?}", &python_vec[0][0][0..5.min(python_vec[0][0].len())]);
        
        // Show top 5 largest differences
        large_diffs.sort_by(|a, b| b.5.partial_cmp(&a.5).unwrap());
        println!("  Top 5 largest differences:");
        for (idx, (i, j, k, r_val, py_val, diff)) in large_diffs.iter().take(5).enumerate() {
            println!("    {}. [{}, {}, {}]: Rust={:.6}, Python={:.6}, diff={:.6}", 
                     idx + 1, i, j, k, r_val, py_val, diff);
        }
        
        return Err(anyhow::Error::msg(format!(
            "{} phase mismatch: max diff {:.6} > tolerance {:.6}", 
            name, max_diff, tolerance
        )));
    }
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors"]
async fn test_qwen_infer_phase_isolation() -> Result<()> {
    println!("🎯 QWEN INFER PHASE ISOLATION TEST");
    println!("   Testing ONLY the infer phase issue using exact Python reference tensors");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    let tolerance = 1e-6; // Very strict
    
    println!("✅ QwenModel loaded");
    
    // ========== STEP 1: RUN PREFILL TO POPULATE STATE ==========
    println!("\n🔍 STEP 1: Running prefill to populate state");
    
    // Load prefill inputs (these should match perfectly based on unified test)
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    // Reset states and run prefill
    qwen_model.reset_states()?;
    let rust_prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &py_prefill_hidden,
        &py_prefill_position_ids, 
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    
    // Verify prefill still works perfectly
    let py_prefill_output = load_numpy_tensor("test_tensors/03_ffn_prefill_output.npy", &[1, 1, 1024], &device)?;
    compare_tensors_detailed("PREFILL VERIFICATION", &rust_prefill_output, &py_prefill_output, tolerance)?;
    println!("✅ Prefill phase works perfectly - state is populated correctly");
    
    // ========== STEP 2: ISOLATED INFER PHASE TEST ==========  
    println!("\n🔍 STEP 2: ISOLATED INFER PHASE - Using EXACT Python tensors");
    
    // Load the EXACT same infer inputs that Python used
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    println!("📊 Infer input tensor details:");
    println!("  - Hidden states shape: {:?}", py_infer_hidden.shape());
    println!("  - Update mask shape: {:?}", py_infer_update_mask.shape());
    println!("  - Position IDs: {:?}", py_infer_position_ids.to_vec1::<f32>()?);
    println!("  - Causal mask shape: {:?}", py_infer_causal_mask.shape());
    
    // Run Rust infer with EXACT Python inputs
    println!("⚡ Running Rust infer with exact Python inputs...");
    let rust_infer_output = qwen_model.run_ffn_infer_with_inputs(
        &py_infer_hidden,
        &py_infer_update_mask,
        &py_infer_position_ids,
        &py_infer_causal_mask,
        &py_infer_current_pos
    )?;
    
    // Load Python infer output
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    println!("📊 Output tensor shapes:");
    println!("  - Rust output: {:?}", rust_infer_output.shape());  
    println!("  - Python output: {:?}", py_infer_output.shape());
    
    // ========== STEP 3: DETAILED COMPARISON ==========
    println!("\n🔍 STEP 3: DETAILED INFER OUTPUT COMPARISON");
    
    // This should fail and show us exactly where the difference is
    match compare_tensors_detailed("INFER PHASE", &rust_infer_output, &py_infer_output, tolerance) {
        Ok(_) => {
            println!("🎉 UNEXPECTED SUCCESS! Infer phase now matches Python perfectly!");
            println!("   The issue may have been fixed or was environment-specific.");
        },
        Err(e) => {
            println!("❌ CONFIRMED: Infer phase mismatch detected");
            println!("   Error: {}", e);
            println!("\n🔍 DIAGNOSTIC INFO:");
            println!("   - Same state used for both prefill and infer: ✅");
            println!("   - Exact Python input tensors used: ✅");  
            println!("   - Prefill phase works perfectly: ✅");
            println!("   - Issue is specifically in infer CoreML model execution");
            
            // Print some raw values for debugging
            let rust_flat = rust_infer_output.flatten_all()?.to_vec1::<f32>()?;
            let python_flat = py_infer_output.flatten_all()?.to_vec1::<f32>()?;
            
            println!("\n📊 Raw tensor comparison (first 10 values):");
            for i in 0..10.min(rust_flat.len()) {
                let diff = (rust_flat[i] - python_flat[i]).abs();
                println!("  [{}]: Rust={:.6}, Python={:.6}, diff={:.6}", 
                         i, rust_flat[i], python_flat[i], diff);
            }
            
            return Err(anyhow::Error::msg("Infer phase produces different output than Python reference"));
        }
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_infer_phase_isolation_macos_requirement() {
    println!("❌ QwenModel infer phase isolation test requires macOS");
}