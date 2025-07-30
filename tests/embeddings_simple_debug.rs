//! Simple Embeddings Debug - Find why QwenModel embeddings differ from Python

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use std::fs::File;
use std::io::Read;

/// Load a numpy array from .npy file (handles int32 and float data)
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
    
    println!("📊 Loading {}: {} bytes for {} elements", path, data.len(), expected_elements);
    
    // Try different data types based on size
    if data.len() == expected_elements * 4 {
        // Could be int32 or float32
        if path.contains("input") || path.contains("token") || path.contains("position") {
            // Likely int32
            let mut values = Vec::with_capacity(expected_elements);
            for chunk in data.chunks_exact(4) {
                let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
                let int_val = i32::from_le_bytes(bytes);
                values.push(int_val as f32); // Convert to f32 for tensor
            }
            println!("  First 8 int32 values: {:?}", &values[0..8.min(values.len())]);
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
            "Data size mismatch: expected {} elements, got {} bytes ({}x2={}, {}x4={})",
            expected_elements, data.len(), expected_elements, expected_elements*2, expected_elements, expected_elements*4
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
#[ignore = "manual debug test"]
async fn test_simple_embeddings_debug() -> Result<()> {
    println!("🔍 SIMPLE EMBEDDINGS DEBUG");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let prompt = "The quick brown fox jumps over the lazy";
    println!("📝 Testing prompt: '{}'", prompt);
    
    // STEP 1: Check tokenization
    let tokens = qwen_model.tokenize(prompt)?;
    println!("🔢 Tokens: {:?} (length: {})", tokens, tokens.len());
    
    // Load Python embeddings input tokens (padded to 64)
    let device = Device::Cpu;
    let python_embeddings_input = load_numpy_tensor("test_tensors/02_embeddings_input.npy", &[1, 64], &device)?;
    let python_embeddings_vec = python_embeddings_input.to_vec2::<f32>()?;
    let python_tokens_i64: Vec<i64> = python_embeddings_vec[0][0..8].iter().map(|&x| x as i64).collect();
    
    println!("🐍 Python tokens: {:?}", python_tokens_i64);
    
    if tokens == python_tokens_i64 {
        println!("✅ TOKENIZATION MATCHES Python!");
    } else {
        println!("❌ TOKENIZATION DIFFERS from Python!");
        println!("   This is the root cause - different tokens = different embeddings!");
        return Ok(());
    }
    
    // STEP 2: Check embeddings (only if tokenization matches)
    let rust_embeddings = qwen_model.compute_embeddings(&tokens)?;
    println!("🦀 Rust embeddings shape: {:?}", rust_embeddings.shape());
    
    let python_embeddings = load_numpy_tensor(
        "test_tensors/03_ffn_prefill_hidden_input.npy",
        &[1, 64, 1024],
        &device
    )?;
    println!("🐍 Python embeddings shape: {:?}", python_embeddings.shape());
    
    // Compare first token embedding
    let rust_vec = rust_embeddings.to_vec3::<f32>()?;
    let python_vec = python_embeddings.to_vec3::<f32>()?;
    
    let mut max_diff = 0.0f32;
    for i in 0..tokens.len().min(8) {
        for j in 0..10 {  // Just first 10 dimensions
            let rust_val = rust_vec[0][i][j];
            let python_val = python_vec[0][i][j];
            let diff = (rust_val - python_val).abs();
            max_diff = max_diff.max(diff);
        }
    }
    
    println!("📊 First token embedding comparison (first 10 dims):");
    println!("  Rust[0][0][0:10]: {:?}", &rust_vec[0][0][0..10]);
    println!("  Python[0][0][0:10]: {:?}", &python_vec[0][0][0..10]);
    println!("  Max difference: {:.8}", max_diff);
    
    if max_diff < 1e-6 {
        println!("✅ EMBEDDINGS MATCH!");
    } else {
        println!("❌ EMBEDDINGS DIFFER! This explains the wrong prediction.");
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_simple_embeddings_debug_macos_requirement() {
    println!("❌ Simple embeddings debug test requires macOS");
}