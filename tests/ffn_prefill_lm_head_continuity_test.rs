//! FFN Prefill → LM Head Continuity Test
//!
//! This test verifies that our Rust FFN prefill→infer pipeline produces 
//! the EXACT same tensor that we feed into the LM head test.
//! This proves complete pipeline continuity: Embeddings → FFN → LM Head

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, Config as CoreMLConfig, CoreMLModel};
use std::path::PathBuf;

/// Load a numpy array from .npy file (handles float16 and float32)
fn load_numpy_tensor(path: &str, expected_shape: &[usize], device: &Device) -> Result<Tensor> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let data_start = if buffer.starts_with(b"\x93NUMPY") {
        let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
        10 + header_len
    } else {
        return Err(anyhow::Error::msg("Invalid .npy file format"));
    };
    
    let float_data = &buffer[data_start..];
    let expected_elements: usize = expected_shape.iter().product();
    
    let values = if float_data.len() == expected_elements * 2 {
        // float16 data
        let mut values = Vec::with_capacity(expected_elements);
        for chunk in float_data.chunks_exact(2) {
            let half_bits = u16::from_le_bytes([chunk[0], chunk[1]]);
            values.push(half_to_f32(half_bits));
        }
        values
    } else if float_data.len() == expected_elements * 4 {
        // float32 data
        let mut values = Vec::with_capacity(expected_elements);
        for chunk in float_data.chunks_exact(4) {
            let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
            values.push(f32::from_le_bytes(bytes));
        }
        values
    } else {
        return Err(anyhow::Error::msg(format!(
            "Data size mismatch: expected {} elements, got {} bytes",
            expected_elements, float_data.len()
        )));
    };
    
    Tensor::from_vec(values, expected_shape, device).map_err(Into::into)
}

/// Load int32 numpy array
fn load_numpy_int_tensor(path: &str, expected_shape: &[usize], device: &Device) -> Result<Tensor> {
    use std::fs::File;
    use std::io::Read;
    
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let data_start = if buffer.starts_with(b"\x93NUMPY") {
        let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
        10 + header_len
    } else {
        return Err(anyhow::Error::msg("Invalid .npy file format"));
    };
    
    let int_data = &buffer[data_start..];
    let expected_elements: usize = expected_shape.iter().product();
    
    if int_data.len() != expected_elements * 4 {
        return Err(anyhow::Error::msg(format!(
            "Int data size mismatch: expected {} x4 bytes, got {} bytes",
            expected_elements, int_data.len()
        )));
    }
    
    let mut values = Vec::with_capacity(expected_elements);
    for chunk in int_data.chunks_exact(4) {
        let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
        values.push(i32::from_le_bytes(bytes) as i64);
    }
    
    Tensor::from_vec(values, expected_shape, device).map_err(Into::into)
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
#[ignore = "requires test tensors - run manually after qwen-chat-test.py"]
async fn test_ffn_prefill_produces_lm_head_input() -> Result<()> {
    println!("🔗 Testing FFN Prefill produces exact LM Head input tensor");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Configure FFN models
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
    
    // Load FFN prefill and infer functions
    let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;
    
    let mut ffn_infer_config = ffn_config_base.clone();
    ffn_infer_config.input_names.insert(1, "update_mask".to_string());
    let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;
    
    println!("✅ FFN models loaded");
    
    let test_tensors_dir = "test_tensors";
    let device = Device::Cpu;
    
    // Load the expected LM head input (what our LM head test uses)
    println!("📥 Loading expected LM head input (from LM head test)...");
    let expected_lm_head_input = load_numpy_tensor(
        &format!("{}/05_lmhead_input.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("  Expected LM head input shape: {:?}", expected_lm_head_input.shape());
    
    // Now recreate this tensor using our FFN prefill→infer pipeline
    println!("🔄 Recreating this tensor using Rust FFN pipeline...");
    
    // Load prefill inputs
    println!("📥 Loading prefill inputs...");
    let prefill_hidden = load_numpy_tensor(
        &format!("{}/03_ffn_prefill_hidden_input.npy", test_tensors_dir),
        &[1, 64, 1024],
        &device
    )?;
    
    let prefill_position_ids = load_numpy_int_tensor(
        &format!("{}/03_ffn_prefill_position_ids.npy", test_tensors_dir),
        &[64],
        &device
    )?;
    
    let prefill_causal_mask = load_numpy_tensor(
        &format!("{}/03_ffn_prefill_causal_mask.npy", test_tensors_dir),
        &[1, 1, 64, 512],
        &device
    )?;
    
    let prefill_current_pos = load_numpy_int_tensor(
        &format!("{}/03_ffn_prefill_current_pos.npy", test_tensors_dir),
        &[1],
        &device
    )?;
    
    // Create state and run prefill
    println!("⚡ Running prefill phase...");
    let mut shared_state = ffn_prefill.make_state()?;
    
    let prefill_inputs = [
        &prefill_hidden,
        &prefill_position_ids,
        &prefill_causal_mask,
        &prefill_current_pos,
    ];
    
    let _prefill_output = ffn_prefill.predict_with_state(&prefill_inputs, &mut shared_state)?;
    println!("✅ Prefill complete - state populated");
    
    // Load infer inputs
    println!("📥 Loading infer inputs...");
    let infer_hidden = load_numpy_tensor(
        &format!("{}/04_infer_token_embeddings.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    
    let infer_update_mask = load_numpy_tensor(
        &format!("{}/04_infer_update_mask.npy", test_tensors_dir),
        &[1, 1, 512, 1],
        &device
    )?;
    
    let infer_position_ids = load_numpy_int_tensor(
        &format!("{}/04_infer_position_ids.npy", test_tensors_dir),
        &[1],
        &device
    )?;
    
    let infer_causal_mask = load_numpy_tensor(
        &format!("{}/04_infer_causal_mask.npy", test_tensors_dir),
        &[1, 1, 1, 512],
        &device
    )?;
    
    let infer_current_pos = infer_position_ids.clone();
    
    // Run infer phase
    println!("⚡ Running infer phase...");
    let infer_inputs = [
        &infer_hidden,
        &infer_update_mask,
        &infer_position_ids,
        &infer_causal_mask,
        &infer_current_pos,
    ];
    
    let rust_generated_lm_input = ffn_infer.predict_with_state(&infer_inputs, &mut shared_state)?;
    println!("✅ Infer complete - generated LM head input");
    println!("  Generated tensor shape: {:?}", rust_generated_lm_input.shape());
    
    // CRITICAL COMPARISON: Does our FFN pipeline produce the exact LM head input?
    println!("🔍 CRITICAL COMPARISON: FFN output vs Expected LM head input");
    
    let rust_vec = rust_generated_lm_input.to_vec3::<f32>()?;
    let expected_vec = expected_lm_head_input.to_vec3::<f32>()?;
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    
    for (r_batch, exp_batch) in rust_vec.iter().zip(expected_vec.iter()) {
        for (r_seq, exp_seq) in r_batch.iter().zip(exp_batch.iter()) {
            for (r_val, exp_val) in r_seq.iter().zip(exp_seq.iter()) {
                let diff = (r_val - exp_val).abs();
                max_diff = max_diff.max(diff);
                total_diff += diff;
                num_elements += 1;
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📊 PIPELINE CONTINUITY RESULTS:");
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    // Show first few values for verification
    println!("\n🔍 First 10 values comparison:");
    println!("  Rust FFN output[0][0][0:10]: {:?}", &rust_vec[0][0][0..10.min(rust_vec[0][0].len())]);
    println!("  Expected LM input[0][0][0:10]: {:?}", &expected_vec[0][0][0..10.min(expected_vec[0][0].len())]);
    
    // STRICT ASSERTION: Must be EXACTLY the same
    let tolerance = 1e-6; // Very strict tolerance
    
    if max_diff < tolerance {
        println!("✅ 🎉 PIPELINE CONTINUITY VERIFIED!");
        println!("   Rust FFN pipeline produces EXACT LM head input (max diff: {:.8})", max_diff);
        println!("   🔗 COMPLETE PIPELINE CONTINUITY: Embeddings → FFN → LM Head → 'dog' prediction!");
        
        // Extra verification: Assert specific slices match exactly
        assert_eq!(rust_vec[0][0][0], expected_vec[0][0][0], "First element must match exactly");
        assert_eq!(rust_vec[0][0][1], expected_vec[0][0][1], "Second element must match exactly");  
        assert_eq!(rust_vec[0][0][512], expected_vec[0][0][512], "Middle element must match exactly");
        assert_eq!(rust_vec[0][0][1023], expected_vec[0][0][1023], "Last element must match exactly");
        
        println!("✅ Slice assertions passed - key elements match exactly!");
        
        Ok(())
    } else {
        panic!(
            "❌ PIPELINE CONTINUITY BROKEN! Max diff: {:.8} > tolerance: {:.8}. FFN output doesn't match expected LM head input!",
            max_diff, tolerance
        );
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_ffn_lm_continuity_macos_requirement() {
    println!("❌ FFN→LM head continuity test requires macOS");
}