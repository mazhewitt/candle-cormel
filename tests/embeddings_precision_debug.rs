//! Embeddings Precision Debug - Check if Float16 extraction is corrupting data
//!
//! This test checks if our embeddings extraction from CoreML is losing precision
//! due to Float16 to Float32 conversion issues.

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
#[ignore = "manual precision debug test"]
async fn test_embeddings_precision_debug() -> Result<()> {
    println!("🔬 EMBEDDINGS PRECISION DEBUG - Check Float16 extraction");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    let prompt = "The quick brown fox jumps over the lazy";
    
    // STEP 1: Get QwenModel embeddings (potentially corrupted by Float16 extraction)
    println!("🦀 Getting QwenModel embeddings...");
    let tokens = qwen_model.tokenize(prompt)?;
    let rust_embeddings = qwen_model.compute_embeddings(&tokens)?;
    
    println!("  Rust embeddings shape: {:?}", rust_embeddings.shape());
    let rust_vec = rust_embeddings.to_vec3::<f32>()?;
    
    // STEP 2: Load Python reference embeddings (ground truth)
    println!("🐍 Loading Python reference embeddings...");
    let python_embeddings = load_numpy_tensor(
        "test_tensors/03_ffn_prefill_hidden_input.npy",
        &[1, 64, 1024],
        &device
    )?;
    let python_vec = python_embeddings.to_vec3::<f32>()?;
    
    // STEP 3: Detailed precision comparison
    println!("🔍 DETAILED PRECISION COMPARISON:");
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    let mut large_diffs = 0;
    
    // Compare only the actual token positions (not padding)
    for i in 0..tokens.len().min(64) {
        for j in 0..1024 {
            let rust_val = rust_vec[0][i][j];
            let python_val = python_vec[0][i][j];
            let diff = (rust_val - python_val).abs();
            
            max_diff = max_diff.max(diff);
            total_diff += diff;
            num_elements += 1;
            
            if diff > 1e-4 { // Float16 precision threshold
                large_diffs += 1;
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📊 PRECISION ANALYSIS:");
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements compared: {}", num_elements);
    println!("  Elements with diff > 1e-4: {} ({:.2}%)", large_diffs, 100.0 * large_diffs as f32 / num_elements as f32);
    
    // STEP 4: Show specific examples of precision loss
    println!("\\n🔍 PRECISION LOSS EXAMPLES:");
    let mut examples_shown = 0;
    
    for i in 0..tokens.len().min(8) {
        for j in 0..20 {
            let rust_val = rust_vec[0][i][j];
            let python_val = python_vec[0][i][j];
            let diff = (rust_val - python_val).abs();
            
            if diff > 1e-4 && examples_shown < 10 {
                println!("  Position [{}, {}]: Rust={:.8}, Python={:.8}, diff={:.8}", 
                        i, j, rust_val, python_val, diff);
                examples_shown += 1;
            }
        }
    }
    
    // STEP 5: Diagnosis
    if max_diff < 1e-6 {
        println!("\\n✅ EMBEDDINGS PRECISION IS PERFECT!");
        println!("   The issue is not in Float16 extraction.");
    } else if max_diff < 1e-4 {
        println!("\\n⚠️ MINOR PRECISION LOSS detected (max diff: {:.8})", max_diff);
        println!("   This could be normal Float16→Float32 conversion precision loss.");
        println!("   Unlikely to cause major prediction differences.");
    } else {
        println!("\\n🚨 SIGNIFICANT PRECISION LOSS detected (max diff: {:.8})", max_diff);
        println!("   This Float16 extraction issue could explain the wrong predictions!");
        println!("   We need to fix the Float16 handling in extract_output!");
    }
    
    // STEP 6: Test with exact Python embeddings to confirm
    println!("\\n🧪 TESTING with exact Python embeddings...");
    
    // Reset and use Python embeddings for prefill
    qwen_model.reset_states()?;
    qwen_model.run_prefill_phase(&python_embeddings, tokens.len())?;
    
    // Use Python infer embedding too
    let python_infer_embedding = load_numpy_tensor(
        "test_tensors/04_infer_token_embeddings.npy",
        &[1, 1, 1024],
        &device
    )?;
    
    let logits = qwen_model.generate_next_token_with_infer(&python_infer_embedding, tokens.len())?;
    
    // Check prediction
    let flat_logits = logits.squeeze(0)?.squeeze(0)?;
    let logits_vec = flat_logits.to_vec1::<f32>()?;
    let next_token = logits_vec
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, _)| i as i64)
        .unwrap();
    
    println!("🎯 With exact Python embeddings: Token {} (should be 5562 for 'dog')", next_token);
    
    if next_token == 5562 {
        println!("✅ Using Python embeddings gives correct prediction!");
        println!("   Confirms the issue is in our embeddings extraction!");
    } else {
        println!("❌ Even with Python embeddings, prediction is wrong.");
        println!("   The issue might be deeper in the FFN pipeline.");
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_embeddings_precision_debug_macos_requirement() {
    println!("❌ Embeddings precision debug test requires macOS");
}