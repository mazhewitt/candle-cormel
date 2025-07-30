//! FFN Prefill→Infer State Transition Test - Phase 4 of systematic debugging
//!
//! This test recreates the complete Python pipeline:
//! 1. Load embeddings output from Python
//! 2. Run through Rust FFN PREFILL to build up state (like Python does)
//! 3. Run through Rust FFN INFER with that populated state
//! 4. Verify the final output matches Python reference
//!
//! This tests the critical state continuity that's broken in our current implementation.

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
async fn test_ffn_prefill_infer_state_transition() -> Result<()> {
    println!("🔄 Testing Rust FFN Prefill→Infer State Transition");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Configure FFN models (both prefill and infer)
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
    
    // Load FFN prefill function
    let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config_base, "prefill")?;
    
    // Load FFN infer function (with update_mask)
    let mut ffn_infer_config = ffn_config_base.clone();
    ffn_infer_config.input_names.insert(1, "update_mask".to_string());
    let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_infer_config, "infer")?;
    
    println!("✅ FFN prefill and infer models loaded");
    
    // Check that test tensors exist
    let test_tensors_dir = "test_tensors";
    let required_tensors = [
        "03_ffn_prefill_hidden_input.npy",    // Embeddings output (batch of 64)
        "03_ffn_prefill_position_ids.npy",   // Position IDs for prefill
        "03_ffn_prefill_causal_mask.npy",    // Causal mask for prefill
        "03_ffn_prefill_current_pos.npy",    // Current position
        "04_infer_token_embeddings.npy",     // Single token embedding for infer
        "04_infer_update_mask.npy",          // Update mask for infer
        "04_infer_position_ids.npy",         // Position IDs for infer
        "04_infer_causal_mask.npy",          // Causal mask for infer
        "04_infer_ffn_output.npy",           // Reference final output
    ];
    
    for tensor_name in &required_tensors {
        let tensor_path = PathBuf::from(test_tensors_dir).join(tensor_name);
        if !tensor_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "Required tensor not found: {}. Run 'python3 qwen-chat-test.py --meta <path>' first",
                tensor_path.display()
            )));
        }
    }
    
    let device = Device::Cpu;
    
    println!("📥 Loading prefill input tensors...");
    
    // Load prefill inputs (these are the batch of 64 tokens)
    let prefill_hidden = load_numpy_tensor(
        &format!("{}/03_ffn_prefill_hidden_input.npy", test_tensors_dir),
        &[1, 64, 1024],
        &device
    )?;
    println!("  Prefill hidden states: {:?}", prefill_hidden.shape());
    
    let prefill_position_ids = load_numpy_int_tensor(
        &format!("{}/03_ffn_prefill_position_ids.npy", test_tensors_dir),
        &[64],
        &device
    )?;
    println!("  Prefill position IDs: {:?}", prefill_position_ids.shape());
    
    let prefill_causal_mask = load_numpy_tensor(
        &format!("{}/03_ffn_prefill_causal_mask.npy", test_tensors_dir),
        &[1, 1, 64, 512],
        &device
    )?;
    println!("  Prefill causal mask: {:?}", prefill_causal_mask.shape());
    
    let prefill_current_pos = load_numpy_int_tensor(
        &format!("{}/03_ffn_prefill_current_pos.npy", test_tensors_dir),
        &[1],
        &device
    )?;
    println!("  Prefill current pos: {:?}", prefill_current_pos.shape());
    
    // Create state for prefill
    println!("🔄 Creating shared state object...");
    let mut shared_state = ffn_prefill.make_state()?;
    
    // STEP 1: Run prefill phase to populate the state (like Python does)
    println!("⚡ STEP 1: Running FFN PREFILL to populate state...");
    
    let prefill_inputs = [
        &prefill_hidden,
        &prefill_position_ids,
        &prefill_causal_mask,
        &prefill_current_pos,
    ];
    
    let prefill_output = ffn_prefill.predict_with_state(&prefill_inputs, &mut shared_state)?;
    println!("  Prefill output shape: {:?}", prefill_output.shape());
    
    println!("✅ Prefill phase complete - state is now populated with KV-cache!");
    
    // STEP 2: Load infer inputs (single token)
    println!("📥 Loading infer input tensors...");
    
    let infer_hidden = load_numpy_tensor(
        &format!("{}/04_infer_token_embeddings.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("  Infer hidden states: {:?}", infer_hidden.shape());
    
    let infer_update_mask = load_numpy_tensor(
        &format!("{}/04_infer_update_mask.npy", test_tensors_dir),
        &[1, 1, 512, 1],
        &device
    )?;
    println!("  Infer update mask: {:?}", infer_update_mask.shape());
    
    let infer_position_ids = load_numpy_int_tensor(
        &format!("{}/04_infer_position_ids.npy", test_tensors_dir),
        &[1],
        &device
    )?;
    println!("  Infer position IDs: {:?}", infer_position_ids.shape());
    
    let infer_causal_mask = load_numpy_tensor(
        &format!("{}/04_infer_causal_mask.npy", test_tensors_dir),
        &[1, 1, 1, 512],
        &device
    )?;
    println!("  Infer causal mask: {:?}", infer_causal_mask.shape());
    
    // For current_pos in infer, use the same as position_ids
    let infer_current_pos = infer_position_ids.clone();
    
    // STEP 3: Run infer phase with the populated state
    println!("⚡ STEP 2: Running FFN INFER with populated state...");
    
    let infer_inputs = [
        &infer_hidden,
        &infer_update_mask,
        &infer_position_ids,
        &infer_causal_mask,
        &infer_current_pos,
    ];
    
    let rust_final_output = ffn_infer.predict_with_state(&infer_inputs, &mut shared_state)?;
    println!("  Final infer output shape: {:?}", rust_final_output.shape());
    
    // STEP 4: Load Python reference and compare
    println!("📊 Loading Python reference output...");
    let python_reference = load_numpy_tensor(
        &format!("{}/04_infer_ffn_output.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("  Python reference shape: {:?}", python_reference.shape());
    
    // Compare outputs
    println!("📊 Comparing Rust (with proper state) vs Python reference...");
    
    let rust_vec = rust_final_output.to_vec3::<f32>()?;
    let python_vec = python_reference.to_vec3::<f32>()?;
    
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
    
    println!("📈 CRITICAL COMPARISON RESULTS:");
    println!("  Max difference: {:.6}", max_diff);
    println!("  Mean difference: {:.6}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    // Show sample values for debugging
    println!("\n🔍 Sample values comparison:");
    println!("  Rust[0][0][0:5]: {:?}", &rust_vec[0][0][0..5.min(rust_vec[0][0].len())]);
    println!("  Python[0][0][0:5]: {:?}", &python_vec[0][0][0..5.min(python_vec[0][0].len())]);
    
    // Strict tolerance for state transition test
    let tolerance = 0.01; // Much stricter now that we have proper state
    
    if max_diff < tolerance {
        println!("✅ 🎉 STATE TRANSITION TEST PASSED!");
        println!("   Rust prefill→infer matches Python reference (max diff: {:.6})", max_diff);
        println!("   🔥 STATE MANAGEMENT IS NOW WORKING CORRECTLY! 🔥");
        Ok(())
    } else {
        println!("❌ STATE TRANSITION TEST FAILED!");
        println!("   Max difference: {:.6} > tolerance: {:.6}", max_diff, tolerance);
        
        if max_diff > 1.0 {
            panic!(
                "🚨 CRITICAL STATE MANAGEMENT FAILURE: Max diff {:.6} indicates broken state continuity between prefill and infer phases!",
                max_diff
            );
        } else {
            panic!(
                "⚠️ STATE MANAGEMENT ISSUE: Max diff {:.6} > tolerance {:.6}. State transition not matching Python reference.",
                max_diff, tolerance
            );
        }
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_ffn_prefill_infer_state_macos_requirement() {
    println!("❌ FFN prefill→infer state test requires macOS");
}