//! TDD Test: Infer Phase State Continuity Issue
//!
//! RED: This test will fail, showing that infer phase doesn't properly continue from prefill state
//! GREEN: After fix, this test will pass
//! REFACTOR: Then we clean up any duplicate code
//!
//! HYPOTHESIS: The infer phase is not properly using the KV-cache state populated during prefill,
//! causing completely different results than the Python reference implementation.

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
#[ignore = "requires test tensors - RED phase test that WILL fail"]
async fn test_infer_state_continuity_tdd_red() -> Result<()> {
    println!("🔴 TDD RED: Testing infer phase state continuity - EXPECT THIS TO FAIL");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    let strict_tolerance = 1e-4; // Very strict - will fail with current implementation
    
    println!("✅ QwenModel loaded");
    
    // ========== STEP 1: RUN PREFILL AND CAPTURE STATE ==========
    println!("\n🔍 STEP 1: Run prefill and verify it works perfectly");
    
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    qwen_model.reset_states()?;
    let rust_prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &py_prefill_hidden,
        &py_prefill_position_ids, 
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    
    let py_prefill_output = load_numpy_tensor("test_tensors/03_ffn_prefill_output.npy", &[1, 1, 1024], &device)?;
    
    // Verify prefill still works (this should pass)
    let prefill_diff = (&rust_prefill_output - &py_prefill_output)?.abs()?;
    let prefill_max_diff: f32 = prefill_diff.flatten_all()?.max(0)?.to_scalar()?;
    
    println!("📊 Prefill max difference: {:.8}", prefill_max_diff);
    assert!(prefill_max_diff < 1e-6, "Prefill should work perfectly - if this fails, there's a bigger issue");
    
    println!("✅ Prefill works perfectly - KV-cache state is populated correctly");
    
    // ========== STEP 2: CRITICAL TEST - INFER PHASE STATE CONTINUITY ==========  
    println!("\n🔍 STEP 2: CRITICAL TEST - Infer phase with populated KV-cache state");
    
    // Load the EXACT infer inputs that Python used
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    println!("📊 Infer inputs loaded:");
    println!("  - Hidden states: {:?}", py_infer_hidden.shape());
    println!("  - Update mask: {:?}", py_infer_update_mask.shape());
    println!("  - Position IDs: {:?}", py_infer_position_ids.to_vec1::<f32>()?);
    
    // THIS IS THE CRITICAL TEST: Infer should continue from prefill state
    let rust_infer_output = qwen_model.run_ffn_infer_with_inputs(
        &py_infer_hidden,
        &py_infer_update_mask,
        &py_infer_position_ids,
        &py_infer_causal_mask,
        &py_infer_current_pos
    )?;
    
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    // ========== STEP 3: THE TEST THAT WILL FAIL ==========
    println!("\n🔍 STEP 3: State continuity validation - THIS WILL FAIL");
    
    let infer_diff = (&rust_infer_output - &py_infer_output)?.abs()?;
    let infer_max_diff: f32 = infer_diff.flatten_all()?.max(0)?.to_scalar()?;
    let infer_mean_diff: f32 = infer_diff.flatten_all()?.mean(0)?.to_scalar()?;
    
    println!("📊 INFER PHASE RESULTS:");
    println!("  Max difference: {:.8}", infer_max_diff);
    println!("  Mean difference: {:.8}", infer_mean_diff);
    println!("  Tolerance: {:.8}", strict_tolerance);
    
    // Show some values for debugging
    let rust_flat = rust_infer_output.flatten_all()?.to_vec1::<f32>()?;
    let python_flat = py_infer_output.flatten_all()?.to_vec1::<f32>()?;
    
    println!("\n📊 First 5 values comparison:");
    for i in 0..5.min(rust_flat.len()) {
        let diff = (rust_flat[i] - python_flat[i]).abs();
        println!("  [{}]: Rust={:.6}, Python={:.6}, diff={:.6}", 
                 i, rust_flat[i], python_flat[i], diff);
    }
    
    // ========== THE ASSERTION THAT WILL FAIL ==========
    if infer_max_diff > strict_tolerance {
        println!("\n❌ 🔴 TDD RED PHASE - TEST FAILING AS EXPECTED");
        println!("   DIAGNOSIS: Infer phase has {:.2}x higher difference than tolerance", 
                 infer_max_diff / strict_tolerance);
        println!("   ROOT CAUSE: KV-cache state continuity broken between prefill and infer");
        println!("   ARCHITECTURE ISSUE: Infer phase not properly using prefill-populated state");
        
        // Fail the test with a clear message about what needs to be fixed
        panic!("🔴 TDD RED: Infer phase state continuity broken! Max diff: {:.6} > tolerance: {:.6}. 
        
        DIAGNOSIS: The infer phase is not properly continuing from the KV-cache state populated during prefill.
        
        EXPECTED BEHAVIOR: After prefill populates the KV-cache with 64 tokens, infer should use that state 
        to process the next token with minimal computational difference from Python reference.
        
        CURRENT BEHAVIOR: Infer phase produces drastically different results ({}x tolerance), 
        indicating it's either:
        1. Not using the prefill-populated KV-cache state
        2. Corrupting/resetting the state between prefill and infer
        3. Using different computational paths than Python reference
        
        NEXT STEP: Fix the state continuity issue in run_ffn_infer_with_inputs()", 
               infer_max_diff, strict_tolerance, (infer_max_diff / strict_tolerance) as i32);
    } else {
        println!("✅ 🟢 UNEXPECTED: Test passed! State continuity is working correctly.");
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_infer_state_continuity_tdd_red_macos_requirement() {
    println!("❌ TDD infer state continuity test requires macOS");
}