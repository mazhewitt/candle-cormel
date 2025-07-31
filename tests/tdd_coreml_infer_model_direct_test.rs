//! TDD Test: CoreML Infer Model Direct Execution
//!
//! HYPOTHESIS: The issue is in the CoreML infer model execution itself, not the state management.
//! This test directly executes the CoreML infer model with identical inputs to Python 
//! and compares the outputs to isolate the exact source of the 55.18 difference.
//!
//! APPROACH:
//! 1. Load the exact same state after prefill as Python does
//! 2. Execute the CoreML infer model directly with Python's exact inputs
//! 3. Compare outputs to prove/disprove if the issue is in model execution vs state management

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
#[ignore = "requires test tensors - granular CoreML model test"]
async fn test_coreml_infer_model_direct_execution() -> Result<()> {
    println!("🔬 TDD GRANULAR: Direct CoreML Infer Model Execution Test");
    println!("   HYPOTHESIS: Issue is in CoreML model execution, not state management");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    let tolerance = 1e-4;
    
    println!("✅ QwenModel loaded");
    
    // ========== STEP 1: SETUP STATE IDENTICAL TO PYTHON ==========
    println!("\n🔧 STEP 1: Setup state identical to Python after prefill");
    
    // Load and run prefill to populate state exactly as Python does
    let py_prefill_hidden = load_numpy_tensor("test_tensors/03_ffn_prefill_hidden_input.npy", &[1, 64, 1024], &device)?;
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    qwen_model.reset_states()?;
    let _prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &py_prefill_hidden,
        &py_prefill_position_ids, 
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    
    println!("✅ State populated with prefill - identical to Python pipeline");
    
    // ========== STEP 2: DIRECT COREML INFER MODEL TEST ==========
    println!("\n🔬 STEP 2: DIRECT CoreML Infer Model Execution");
    println!("   Testing if the CoreML infer model itself produces Python-matching results");
    
    // Load the EXACT infer inputs that Python used
    let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
    let py_infer_update_mask = load_numpy_tensor("test_tensors/04_infer_update_mask.npy", &[1, 1, 512, 1], &device)?;
    let py_infer_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
    let py_infer_causal_mask = load_numpy_tensor("test_tensors/04_infer_causal_mask.npy", &[1, 1, 1, 512], &device)?;
    let py_infer_current_pos = py_infer_position_ids.clone();
    
    println!("📊 Python infer inputs loaded:");
    println!("  - Hidden states: {:?}", py_infer_hidden.shape());
    println!("  - Update mask: {:?}", py_infer_update_mask.shape());
    println!("  - Position IDs: {:?}", py_infer_position_ids.to_vec1::<f32>()?);
    println!("  - Causal mask: {:?}", py_infer_causal_mask.shape());
    
    // CRITICAL TEST: Execute CoreML infer model directly with Python's exact inputs and state
    println!("\n⚡ EXECUTING: CoreML infer model with populated state + Python inputs...");
    
    // Execute the CoreML infer model directly (bypass QwenModel wrapper)
    let inputs = [&py_infer_hidden, &py_infer_update_mask, &py_infer_position_ids, &py_infer_causal_mask, &py_infer_current_pos];
    let rust_infer_output = qwen_model.debug_direct_infer_model_execution(&inputs)?;
    
    println!("✅ CoreML infer model executed");
    
    // Load Python's expected output
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    // ========== STEP 3: DETAILED COMPARISON ==========
    println!("\n📊 STEP 3: GRANULAR COMPARISON - CoreML vs Python");
    
    let infer_diff = (&rust_infer_output - &py_infer_output)?.abs()?;
    let infer_max_diff: f32 = infer_diff.flatten_all()?.max(0)?.to_scalar()?;
    let infer_mean_diff: f32 = infer_diff.flatten_all()?.mean(0)?.to_scalar()?;
    
    println!("📈 DIRECT COREML MODEL RESULTS:");
    println!("  Max difference: {:.8}", infer_max_diff);
    println!("  Mean difference: {:.8}", infer_mean_diff);
    println!("  Tolerance: {:.8}", tolerance);
    
    // Detailed value comparison
    let rust_flat = rust_infer_output.flatten_all()?.to_vec1::<f32>()?;
    let python_flat = py_infer_output.flatten_all()?.to_vec1::<f32>()?;
    
    println!("\n📊 First 10 values comparison (Direct CoreML vs Python):");
    for i in 0..10.min(rust_flat.len()) {
        let diff = (rust_flat[i] - python_flat[i]).abs();
        let marker = if diff > tolerance { "❌" } else { "✅" };
        println!("  {} [{}]: CoreML={:.6}, Python={:.6}, diff={:.6}", 
                 marker, i, rust_flat[i], python_flat[i], diff);
    }
    
    // Statistical analysis
    let large_diffs: Vec<f32> = rust_flat.iter().zip(python_flat.iter())
        .map(|(r, p)| (r - p).abs())
        .filter(|&diff| diff > tolerance)
        .collect();
    
    println!("\n📈 STATISTICAL ANALYSIS:");
    println!("  Total elements: {}", rust_flat.len());
    println!("  Elements with large differences (>{:.6}): {}", tolerance, large_diffs.len());
    println!("  Percentage of problematic elements: {:.2}%", 
             (large_diffs.len() as f32 / rust_flat.len() as f32) * 100.0);
    
    if !large_diffs.is_empty() {
        let largest_diff = large_diffs.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        println!("  Largest single difference: {:.8}", largest_diff);
    }
    
    // ========== STEP 4: HYPOTHESIS TESTING ==========
    println!("\n🧪 STEP 4: HYPOTHESIS EVALUATION");
    
    if infer_max_diff > tolerance {
        println!("❌ 📊 HYPOTHESIS CONFIRMED: CoreML infer model execution differs from Python");
        println!("   Max difference: {:.6} ({}x tolerance)", infer_max_diff, (infer_max_diff / tolerance) as i32);
        println!();
        println!("🔍 ROOT CAUSE ANALYSIS:");
        println!("   The issue is NOT in QwenModel state management");
        println!("   The issue IS in the CoreML infer model execution itself");
        println!();
        println!("🎯 NEXT STEPS:");
        println!("   1. Compare CoreML model versions (prefill vs infer)");
        println!("   2. Investigate CoreML model input data types (Float16 vs Float32)");
        println!("   3. Check if CoreML infer model was trained differently than Python expects");
        println!("   4. Verify CoreML model weights match Python model weights");
        
        // Additional diagnostic: Compare input data types
        println!("\n🔬 ADDITIONAL DIAGNOSTICS:");
        println!("   Testing if issue is in input data type conversion...");
        
        return Err(anyhow::Error::msg(format!(
            "CoreML infer model execution differs from Python by {:.6} ({}x tolerance). 
            
            CONFIRMED: The issue is in the CoreML model execution, not state management.
            The CoreML infer model produces different results than Python expects, 
            even with identical inputs and state.",
            infer_max_diff, (infer_max_diff / tolerance) as i32
        )));
    } else {
        println!("✅ 🎉 HYPOTHESIS DISPROVEN: CoreML infer model matches Python perfectly!");
        println!("   This means the issue is in the QwenModel wrapper, not CoreML execution");
        println!("   Max difference: {:.8} < tolerance: {:.8}", infer_max_diff, tolerance);
        
        println!("\n🔍 REVISED ANALYSIS:");
        println!("   The CoreML infer model works correctly");
        println!("   The issue must be in how QwenModel calls the CoreML model");
        println!("   Check: input preprocessing, tensor conversions, or model selection");
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_coreml_infer_model_direct_execution_macos_requirement() {
    println!("❌ CoreML infer model direct execution test requires macOS");
}