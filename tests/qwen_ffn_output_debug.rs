//! QwenModel FFN Output Debug - Compare vs Python FFN output
//!
//! This test runs the QwenModel FFN pipeline with exact Python inputs
//! and compares the FFN output to identify the exact mismatch.

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
async fn test_qwen_ffn_output_debug() -> Result<()> {
    println!("🔍 QWEN FFN OUTPUT DEBUG - Find the exact mismatch");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    
    println!("✅ QwenModel loaded");
    
    // STEP 1: Load Python reference FFN output (what should be produced)
    println!("📥 Loading Python reference FFN output...");
    let python_ffn_output = load_numpy_tensor(
        "test_tensors/04_infer_ffn_output.npy",
        &[1, 1, 1024],
        &device
    )?;
    println!("  Python FFN output shape: {:?}", python_ffn_output.shape());
    
    // STEP 2: Run QwenModel FFN pipeline with exact Python inputs
    println!("⚡ Running QwenModel FFN pipeline with Python inputs...");
    
    // Use exact Python embeddings for prefill
    let python_embeddings = load_numpy_tensor(
        "test_tensors/03_ffn_prefill_hidden_input.npy",
        &[1, 64, 1024],
        &device
    )?;
    
    qwen_model.reset_states()?;
    qwen_model.run_prefill_phase(&python_embeddings, 8)?;
    
    // Use exact Python token embedding for infer  
    let python_infer_embedding = load_numpy_tensor(
        "test_tensors/04_infer_token_embeddings.npy",
        &[1, 1, 1024],
        &device
    )?;
    
    // Run infer but CAPTURE the FFN output before LM head
    println!("⚡ Running infer to capture FFN output...");
    
    let current_position = 8;
    let rust_ffn_output = qwen_model.debug_get_ffn_output(&python_infer_embedding, current_position)?;
    
    println!("✅ QwenModel FFN output captured");
    println!("  Rust FFN output shape: {:?}", rust_ffn_output.shape());
    
    // STEP 3: Compare FFN outputs
    println!("📊 CRITICAL COMPARISON: Rust FFN vs Python FFN output");
    
    let rust_vec = rust_ffn_output.to_vec3::<f32>()?;
    let python_vec = python_ffn_output.to_vec3::<f32>()?;
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    
    for (r_batch, py_batch) in rust_vec.iter().zip(python_vec.iter()) {
        for (r_seq, py_seq) in r_batch.iter().zip(py_batch.iter()) {
            for (r_val, py_val) in r_seq.iter().zip(py_seq.iter()) {
                let diff = (r_val - py_val).abs();
                max_diff = max_diff.max(diff);
                total_diff += diff;
                num_elements += 1;
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📈 FFN OUTPUT COMPARISON RESULTS:");
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    // Show first few values
    println!("\\n🔍 First 10 values comparison:");
    println!("  Rust FFN[0][0][0:10]: {:?}", &rust_vec[0][0][0..10]);
    println!("  Python FFN[0][0][0:10]: {:?}", &python_vec[0][0][0..10]);
    
    if max_diff < 1e-6 {
        println!("✅ FFN OUTPUTS MATCH PERFECTLY!");
        println!("   Issue must be in LM head processing!");
    } else if max_diff < 0.01 {
        println!("⚠️ FFN OUTPUTS CLOSE but not identical (max diff: {:.8})", max_diff);
        println!("   Small differences could explain wrong prediction");
    } else {
        println!("❌ FFN OUTPUTS SIGNIFICANTLY DIFFERENT (max diff: {:.8})", max_diff);
        println!("   This explains why we get wrong prediction!");
        
        // Find where the biggest differences are
        let mut max_diff_pos = (0, 0);
        let mut max_diff_found = 0.0f32;
        for (i, (r_seq, py_seq)) in rust_vec[0].iter().zip(python_vec[0].iter()).enumerate() {
            for (j, (r_val, py_val)) in r_seq.iter().zip(py_seq.iter()).enumerate() {
                let diff = (r_val - py_val).abs();
                if diff > max_diff_found {
                    max_diff_found = diff;
                    max_diff_pos = (i, j);
                }
            }
        }
        
        println!("   Biggest difference at position [{}, {}]: Rust={:.6}, Python={:.6}, diff={:.6}", 
                max_diff_pos.0, max_diff_pos.1,
                rust_vec[0][max_diff_pos.0][max_diff_pos.1],
                python_vec[0][max_diff_pos.0][max_diff_pos.1],
                max_diff_found);
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_ffn_output_debug_macos_requirement() {
    println!("❌ QwenModel FFN output debug test requires macOS");
}