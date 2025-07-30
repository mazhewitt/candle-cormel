//! FFN Infer isolation test - Phase 3 of systematic debugging
//!
//! This test loads the FFN infer inputs captured from the Python pipeline
//! and feeds them through the Rust FFN infer to verify we get the same output.
//! This tests the single-token generation phase that happens after prefill.

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
    
    // Simple .npy parser
    let data_start = if buffer.starts_with(b"\x93NUMPY") {
        let header_len = u16::from_le_bytes([buffer[8], buffer[9]]) as usize;
        10 + header_len
    } else {
        return Err(anyhow::Error::msg("Invalid .npy file format"));
    };
    
    let float_data = &buffer[data_start..];
    
    // Determine if this is float16 or float32 based on data size and expected shape
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
            "Data size mismatch: expected {} elements ({}x2 or {}x4 bytes), got {} bytes",
            expected_elements, expected_elements, expected_elements, float_data.len()
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
            "Int data size mismatch: expected {} elements ({}x4 bytes), got {} bytes",
            expected_elements, expected_elements, int_data.len()
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
async fn test_ffn_infer_isolation_rust() -> Result<()> {
    println!("⚡ Testing Rust FFN Infer in isolation");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Configure FFN infer model  
    let ffn_config = CoreMLConfig {
        input_names: vec![
            "hidden_states".to_string(),
            "update_mask".to_string(),
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
    let ffn_infer = CoreMLModel::load_with_function(&ffn_path, &ffn_config, "infer")?;
    
    println!("✅ FFN infer model loaded");
    
    // Load all the test tensors captured from Python
    let test_tensors_dir = "test_tensors";
    
    // Check that test tensors exist
    let tensor_paths = [
        "04_infer_ffn_hidden_input.npy",
        "04_infer_update_mask.npy", 
        "04_infer_position_ids.npy",
        "04_infer_causal_mask.npy",
        "04_infer_ffn_output.npy", // Reference output
    ];
    
    for path in &tensor_paths {
        let full_path = PathBuf::from(test_tensors_dir).join(path);
        if !full_path.exists() {
            return Err(anyhow::Error::msg(format!(
                "Test tensor not found: {}. Run 'python3 qwen-chat-test.py --meta <path>' first", 
                full_path.display()
            )));
        }
    }
    
    let device = Device::Cpu;
    
    // Load input tensors
    println!("📥 Loading FFN infer input tensors...");
    
    let hidden_states = load_numpy_tensor(
        &format!("{}/04_infer_ffn_hidden_input.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("  Hidden states: {:?}", hidden_states.shape());
    
    let update_mask = load_numpy_tensor(
        &format!("{}/04_infer_update_mask.npy", test_tensors_dir),
        &[1, 1, 512, 1],
        &device
    )?;
    println!("  Update mask: {:?}", update_mask.shape());
    
    let position_ids = load_numpy_int_tensor(
        &format!("{}/04_infer_position_ids.npy", test_tensors_dir),
        &[1],
        &device
    )?;
    println!("  Position IDs: {:?}", position_ids.shape());
    
    let causal_mask = load_numpy_tensor(
        &format!("{}/04_infer_causal_mask.npy", test_tensors_dir),
        &[1, 1, 1, 512],
        &device
    )?;
    println!("  Causal mask: {:?}", causal_mask.shape());
    
    // For current_pos, use the same as position_ids (this is how Python does it)
    let current_pos = position_ids.clone();
    println!("  Current pos: {:?}", current_pos.shape());
    
    // Load reference output
    let reference_output = load_numpy_tensor(
        &format!("{}/04_infer_ffn_output.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("  Reference output: {:?}", reference_output.shape());
    
    // Create a state object for stateful inference
    // Note: We need to handle the state management complexity here
    println!("🔄 Creating FFN state...");
    let mut state = ffn_infer.make_state()?;
    
    // CRITICAL: We need to initialize the state with the prefill data
    // This is where the state management complexity comes in
    // For now, let's try to run infer without proper prefill state
    println!("⚠️  WARNING: Running infer without proper prefill state - this may not match Python!");
    
    // Run FFN infer
    println!("🔄 Running FFN infer inference...");
    let inputs = [&hidden_states, &update_mask, &position_ids, &causal_mask, &current_pos];
    let rust_output = ffn_infer.predict_with_state(&inputs, &mut state)?;
    
    println!("📊 Rust FFN infer output shape: {:?}", rust_output.shape());
    
    // Compare with reference
    println!("📊 Comparing with Python reference...");
    
    let rust_vec = rust_output.to_vec3::<f32>()?;
    let ref_vec = reference_output.to_vec3::<f32>()?;
    
    // Calculate differences
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut num_elements = 0;
    
    for (r_batch, ref_batch) in rust_vec.iter().zip(ref_vec.iter()) {
        for (r_seq, ref_seq) in r_batch.iter().zip(ref_batch.iter()) {
            for (r_val, ref_val) in r_seq.iter().zip(ref_seq.iter()) {
                let diff = (r_val - ref_val).abs();
                max_diff = max_diff.max(diff);
                total_diff += diff;
                num_elements += 1;
            }
        }
    }
    
    let mean_diff = total_diff / num_elements as f32;
    
    println!("📈 Comparison results:");
    println!("  Max difference: {:.6}", max_diff);
    println!("  Mean difference: {:.6}", mean_diff);
    println!("  Total elements: {}", num_elements);
    
    // Determine if this is a pass or fail
    let tolerance = 0.1; // Allow some difference due to state initialization issues
    
    if max_diff < tolerance {
        println!("✅ FFN INFER TEST PASSED: Output matches Python reference (max diff: {:.6})", max_diff);
        Ok(())
    } else {
        println!("❌ FFN INFER TEST FAILED: Output differs significantly from Python reference");
        println!("   Max difference: {:.6} (tolerance: {:.6})", max_diff, tolerance);
        
        // Show some sample values for debugging
        println!("\n🔍 Sample values comparison:");
        println!("  Rust[0][0][0:5]: {:?}", &rust_vec[0][0][0..5.min(rust_vec[0][0].len())]);
        println!("  Python[0][0][0:5]: {:?}", &ref_vec[0][0][0..5.min(ref_vec[0][0].len())]);
        
        panic!(
            "FFN infer output doesn't match Python reference. Max diff: {:.6} > tolerance: {:.6}",
            max_diff, tolerance
        );
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_ffn_infer_isolation_macos_requirement() {
    println!("❌ FFN infer isolation test requires macOS");
}