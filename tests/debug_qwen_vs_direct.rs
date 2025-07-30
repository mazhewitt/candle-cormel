//! Debug test: Compare QwenModel infer vs Direct CoreML infer
//!
//! This test compares the exact same inputs processed through:
//! 1. Direct CoreML models (like our working isolated test)
//! 2. QwenModel granular methods
//!
//! This will pinpoint if the issue is in QwenModel wrapper or CoreML execution.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}, Config as CoreMLConfig, CoreMLModel};
use std::fs::File;
use std::io::Read;

/// Load numpy tensor (handles float data)
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
        // float32 data  
        let mut values = Vec::with_capacity(expected_elements);
        for chunk in data.chunks_exact(4) {
            let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
            values.push(f32::from_le_bytes(bytes));
        }
        return Tensor::from_vec(values, expected_shape, device).map_err(Into::into);
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
async fn test_debug_qwen_vs_direct() -> Result<()> {
    println!("🔬 DEBUG: QwenModel infer vs Direct CoreML infer");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let device = Device::Cpu;
    
    // Load test tensors
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    // METHOD 1: Direct CoreML (like our working isolated test)
    println!("\n🔧 METHOD 1: Direct CoreML models");
    
    let ffn_config_base = CoreMLConfig {
        input_names: vec![
            "hidden_states".to_string(),
            "position_ids".to_string(),
            "causal_mask".to_string(),
            "current_pos".to_string(),
        ],
        output_name: "output_hidden_states".to_string(),
        max_sequence_length: 512,
        vocab_size: 1024,
        model_type: "qwen-ffn".to_string(),
    };
    
    let ffn_path = model_dir.join("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc");
    let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;
    
    let mut ffn_infer_config = ffn_config_base.clone();
    ffn_infer_config.input_names.insert(1, "update_mask".to_string());
    let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;
    
    // Create shared state and run prefill
    let mut shared_state = ffn_prefill.make_state()?;
    let prefill_inputs = [&py_prefill_hidden, &py_prefill_position_ids, &py_prefill_causal_mask, &py_prefill_current_pos];
    let _direct_prefill_output = ffn_prefill.predict_with_state(&prefill_inputs, &mut shared_state)?;
    
    // Run infer with shared state
    let infer_inputs = [&py_infer_hidden, &py_infer_update_mask, &py_infer_position_ids, &py_infer_causal_mask, &py_infer_current_pos];
    let direct_infer_output = ffn_infer.predict_with_state(&infer_inputs, &mut shared_state)?;
    
    let direct_vec = direct_infer_output.to_vec3::<f32>()?;
    println!("Direct CoreML infer result: {:?}", &direct_vec[0][0][0..5]);
    
    // METHOD 2: QwenModel granular methods
    println!("\n🦀 METHOD 2: QwenModel granular methods");
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    // Reset states and run prefill
    qwen_model.reset_states()?;
    let _qwen_prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &py_prefill_hidden,
        &py_prefill_position_ids, 
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    
    // Run infer
    let qwen_infer_output = qwen_model.run_ffn_infer_with_inputs(
        &py_infer_hidden,
        &py_infer_update_mask,
        &py_infer_position_ids,
        &py_infer_causal_mask,
        &py_infer_current_pos
    )?;
    
    let qwen_vec = qwen_infer_output.to_vec3::<f32>()?;
    println!("QwenModel infer result: {:?}", &qwen_vec[0][0][0..5]);
    
    // Python reference
    let py_vec = py_infer_output.to_vec3::<f32>()?;
    println!("Python reference result: {:?}", &py_vec[0][0][0..5]);
    
    // Compare
    let mut direct_max_diff = 0.0f32;
    let mut qwen_max_diff = 0.0f32;
    
    for (i, ((d, q), p)) in direct_vec[0][0].iter().zip(qwen_vec[0][0].iter()).zip(py_vec[0][0].iter()).enumerate() {
        let direct_diff = (d - p).abs();
        let qwen_diff = (q - p).abs();
        
        direct_max_diff = direct_max_diff.max(direct_diff);
        qwen_max_diff = qwen_max_diff.max(qwen_diff);
        
        if i < 10 && (direct_diff > 1e-6 || qwen_diff > 1e-6) {
            println!("Index {}: Direct={:.6} QwenModel={:.6} Python={:.6} (DirectDiff={:.6}, QwenDiff={:.6})", 
                     i, d, q, p, direct_diff, qwen_diff);
        }
    }
    
    println!("\n📊 RESULTS:");
    println!("Direct CoreML max diff: {:.8}", direct_max_diff);
    println!("QwenModel max diff: {:.8}", qwen_max_diff);
    
    let direct_matches = direct_max_diff < 1e-6;
    let qwen_matches = qwen_max_diff < 1e-6;
    
    println!("Direct CoreML matches Python: {}", if direct_matches { "✅" } else { "❌" });
    println!("QwenModel matches Python: {}", if qwen_matches { "✅" } else { "❌" });
    
    if direct_matches && !qwen_matches {
        println!("🎯 CONCLUSION: QwenModel wrapper has a bug!");
        println!("   The direct CoreML execution works, but QwenModel doesn't.");
        println!("   Issue is in the QwenModel implementation, not CoreML execution.");
    } else if !direct_matches && !qwen_matches {
        println!("🎯 CONCLUSION: CoreML execution issue (affects both)");
        println!("   Both direct and QwenModel fail with similar differences.");
        println!("   Issue is at CoreML execution level, not wrapper level.");
    } else if direct_matches && qwen_matches {
        println!("🎉 CONCLUSION: Both methods work correctly!");
        println!("   If the unified test still fails, issue is elsewhere.");
    }
    
    // Fail the test if QwenModel doesn't match Python
    if !qwen_matches {
        println!("\n❌ TEST FAILED: QwenModel infer method produces wrong results");
        panic!("QwenModel infer method differs from Python reference by {:.8}", qwen_max_diff);
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_debug_qwen_vs_direct_macos_requirement() {
    println!("❌ Debug QwenModel vs Direct test requires macOS");
}