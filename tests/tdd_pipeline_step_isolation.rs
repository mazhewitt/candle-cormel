//! TDD Pipeline Step Isolation Test
//!
//! This test follows TDD methodology to isolate exactly which pipeline step
//! causes the Rust vs Python difference in the end-to-end prediction.
//!
//! Strategy:
//! 1. RED: Test fails showing the difference
//! 2. GREEN: Identify the specific step that differs  
//! 3. REFACTOR: Fix that step
//!
//! We'll test each step using Rust-generated inputs from previous steps,
//! comparing against Python reference outputs.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use std::fs::File;
use std::io::Read;

/// Load numpy tensor with fixed data type detection
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
        // Fixed detection logic
        if (path.contains("input_ids") || path.contains("input_token") || 
            path.contains("position_ids") || path.contains("current_pos")) &&
            !path.contains("embeddings") {
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
    }
    
    Err(anyhow::Error::msg(format!(
        "Data size mismatch: expected {} elements, got {} bytes",
        expected_elements, data.len()
    )))
}

/// Compare tensors with tolerance
fn tensors_match(rust_tensor: &Tensor, python_tensor: &Tensor, tolerance: f32, step_name: &str) -> Result<bool> {
    let rust_vec = rust_tensor.flatten_all()?.to_vec1::<f32>()?;
    let python_vec = python_tensor.flatten_all()?.to_vec1::<f32>()?;
    
    if rust_vec.len() != python_vec.len() {
        println!("❌ {}: Shape mismatch - Rust: {}, Python: {}", step_name, rust_vec.len(), python_vec.len());
        return Ok(false);
    }
    
    let mut max_diff = 0.0f32;
    let mut total_diff = 0.0f32;
    let mut large_diffs = 0;
    
    for (r, p) in rust_vec.iter().zip(python_vec.iter()) {
        let diff = (r - p).abs();
        max_diff = max_diff.max(diff);
        total_diff += diff;
        if diff > tolerance {
            large_diffs += 1;
        }
    }
    
    let mean_diff = total_diff / rust_vec.len() as f32;
    let matches = max_diff <= tolerance;
    
    println!("📊 {} COMPARISON:", step_name.to_uppercase());
    println!("  Max difference: {:.8}", max_diff);
    println!("  Mean difference: {:.8}", mean_diff);
    println!("  Large differences (>{:.1e}): {}", tolerance, large_diffs);
    println!("  Sample Rust: {:?}", &rust_vec[..5.min(rust_vec.len())]);
    println!("  Sample Python: {:?}", &python_vec[..5.min(python_vec.len())]);
    
    if matches {
        println!("  ✅ MATCH! (max diff: {:.8} <= tolerance: {:.8})", max_diff, tolerance);
    } else {
        println!("  ❌ DIFFER! (max diff: {:.8} > tolerance: {:.8})", max_diff, tolerance);
    }
    
    Ok(matches)
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors"]
async fn test_tdd_pipeline_step_isolation() -> Result<()> {
    println!("🔬 TDD PIPELINE STEP ISOLATION TEST");
    println!("   Testing each step systematically to find the divergence point");
    
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    let device = Device::Cpu;
    let tolerance = 1e-4; // Reasonable tolerance for float32
    
    println!("✅ QwenModel loaded");
      
    // The input prompt that Python processes
    let prompt = "The quick brown fox jumps over the lazy";
    let tokens = qwen_model.tokenize(prompt)?;
    println!("📝 Prompt: '{}' -> tokens: {:?}", prompt, tokens);
    
    // ========== TDD STEP 1: EMBEDDINGS ==========
    println!("\n🔬 TDD STEP 1: EMBEDDINGS");
    println!("   Testing: Does Rust embeddings match Python embeddings?");
    
    // Load Python's input tokens (should match our tokens)
    let py_input_tokens = load_numpy_tensor("test_tensors/01_input_tokens.npy", &[1, 8], &device)?;
    let py_input_vec = py_input_tokens.to_vec2::<f32>()?[0].iter().map(|&x| x as i64).collect::<Vec<_>>();
    println!("🔍 Python tokens: {:?}", py_input_vec);
    println!("🔍 Rust tokens:   {:?}", tokens);
    
    if tokens != py_input_vec {
        println!("❌ TOKENIZATION DIFFERS! This explains the end-to-end difference.");
        println!("   Rust and Python are processing different token sequences.");
        panic!("TOKENIZATION FAILURE: Rust tokens {:?} != Python tokens {:?}", tokens, py_input_vec);
    }
    println!("✅ Tokenization matches");
    
    // Test embeddings: Rust vs Python
    let rust_embeddings = qwen_model.compute_embeddings(&tokens)?;
    let py_embeddings = load_numpy_tensor("test_tensors/02_embeddings_output.npy", &[1, 64, 1024], &device)?;
    
    // Only compare the actual sequence length, not the padding
    let seq_len = tokens.len();
    let rust_seq_embeddings = rust_embeddings.narrow(1, 0, seq_len)?;
    let py_seq_embeddings = py_embeddings.narrow(1, 0, seq_len)?;
    
    let embeddings_match = tensors_match(&rust_seq_embeddings, &py_seq_embeddings, tolerance, "EMBEDDINGS")?;
    
    if !embeddings_match {
        println!("❌ TDD STEP 1 FAILED: Embeddings differ");
        println!("   Root cause: Embeddings step produces different output");
        println!("   This explains why the end-to-end pipeline differs");
        panic!("EMBEDDINGS FAILURE: QwenModel embeddings do not match Python reference. Fix embeddings before testing other components.");
    }
    
    // ========== TDD STEP 2: PREFILL ==========
    println!("\n🔬 TDD STEP 2: PREFILL");  
    println!("   Testing: Does Rust prefill with Rust embeddings produce expected output?");
    
    // Run Rust prefill using the SAME approach as working tests
    qwen_model.reset_states()?;
    
    // Load Python prefill inputs to match the exact tensor shapes and values
    let py_prefill_position_ids = load_numpy_tensor("test_tensors/03_ffn_prefill_position_ids.npy", &[64], &device)?;
    let py_prefill_causal_mask = load_numpy_tensor("test_tensors/03_ffn_prefill_causal_mask.npy", &[1, 1, 64, 512], &device)?;
    let py_prefill_current_pos = load_numpy_tensor("test_tensors/03_ffn_prefill_current_pos.npy", &[1], &device)?;
    
    // Run prefill with Rust embeddings but Python masks/positions (to isolate embedding vs mask differences)
    let rust_prefill_output = qwen_model.run_ffn_prefill_with_inputs(
        &rust_embeddings,
        &py_prefill_position_ids,
        &py_prefill_causal_mask,
        &py_prefill_current_pos
    )?;
    
    // Compare prefill outputs
    let py_prefill_output = load_numpy_tensor("test_tensors/03_ffn_prefill_output.npy", &[1, 1, 1024], &device)?;
    let prefill_match = tensors_match(&rust_prefill_output, &py_prefill_output, tolerance, "PREFILL")?;
    
    if !prefill_match {
        println!("❌ TDD STEP 2 FAILED: Prefill with Rust embeddings differs from Python");
        println!("   Root cause: Even with Python position/masks, Rust embeddings cause different prefill output");
        panic!("PREFILL FAILURE: QwenModel prefill does not match Python reference. This is the infinite values bug - check run_ffn_prefill_with_inputs().");
    }
    
    println!("✅ TDD STEP 2 PASSED: Prefill with Rust embeddings matches Python prefill output");
    
    // Get the final token embedding for infer phase
    let last_token_tensor = Tensor::from_vec(vec![tokens[tokens.len() - 1]], (1, 1), &device)?;
    let last_token_embedding = qwen_model.run_embeddings_with_inputs(&last_token_tensor)?;
    
    // Create infer inputs using Rust's approach
    let current_position = seq_len;
    let update_mask = qwen_model.create_update_mask(current_position, qwen_model.config().context_length)?;
    let position_ids = Tensor::from_vec(vec![current_position as i64], (1,), &device)?;
    let causal_mask = qwen_model.create_position_causal_mask(current_position, qwen_model.config().context_length)?;
    let current_pos = position_ids.clone();
    
    // Run infer phase using the populated state
    let rust_infer_output = qwen_model.run_ffn_infer_with_inputs(
        &last_token_embedding,
        &update_mask,
        &position_ids,
        &causal_mask,
        &current_pos
    )?;
    
    // Compare against Python's expected infer output
    let py_infer_output = load_numpy_tensor("test_tensors/04_infer_ffn_output.npy", &[1, 1, 1024], &device)?;
    
    let infer_match = tensors_match(&rust_infer_output, &py_infer_output, tolerance, "INFER (end-to-end)")?;
    
    if !infer_match {
        println!("❌ TDD STEP 2 FAILED: End-to-end infer differs from Python");
        println!("   Root cause: Pipeline integration produces different infer output");
        println!("   Even though individual steps work, the integration differs");
        
        // Additional debugging: Let's see what inputs we're generating vs Python
        println!("\n🔍 DEBUGGING INFER INPUTS:");
        
        let py_infer_hidden = load_numpy_tensor("test_tensors/04_infer_token_embeddings.npy", &[1, 1, 1024], &device)?;
        let hidden_match = tensors_match(&last_token_embedding, &py_infer_hidden, tolerance, "INFER HIDDEN INPUT")?;
        
        if !hidden_match {
            println!("   ➜ Issue: Rust generates different token embeddings for infer phase");
        }
        
        let py_position_ids = load_numpy_tensor("test_tensors/04_infer_position_ids.npy", &[1], &device)?;
        let pos_match = tensors_match(&position_ids, &py_position_ids, 0.1, "INFER POSITION IDS")?;
        
        if !pos_match {  
            println!("   ➜ Issue: Rust uses different position IDs for infer phase");
        }
        
        panic!("INFER FAILURE: QwenModel infer does not match Python reference. This is the 55.18 difference bug - check run_ffn_infer_with_inputs().");
    }
    
    // ========== TDD STEP 3: LM HEAD ==========
    println!("\n🔬 TDD STEP 3: LM HEAD");
    println!("   Testing: Does Rust LM head with Rust infer output predict correctly?");
    
    let rust_lm_output = qwen_model.run_lm_head_with_inputs(&rust_infer_output)?;
    
    // Extract top prediction
    let flat_logits = rust_lm_output.squeeze(0)?.squeeze(0)?;
    let logits_vec = flat_logits.to_vec1::<f32>()?;
    
    // Get top 5 predictions
    let mut indexed_logits: Vec<(usize, f32)> = logits_vec.iter().enumerate().map(|(i, &score)| (i, score)).collect();
    indexed_logits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("🔍 Top 5 Rust end-to-end predictions:");
    for (rank, (token_id, score)) in indexed_logits.iter().take(5).enumerate() {
        let decoded = qwen_model.tokenizer().decode(&[*token_id as u32], false).unwrap_or("???".to_string());
        println!("  {}. Token {} ('{}'): {:.6}", rank + 1, token_id, decoded, score);
    }
    
    let predicted_token = indexed_logits[0].0 as i64;
    let decoded = qwen_model.tokenizer().decode(&[predicted_token as u32], false).unwrap_or("???".to_string());
    
    println!("🎯 END-TO-END RESULT: Token {} = '{}'", predicted_token, decoded);
    
    if predicted_token == 5562 {
        println!("🎉 TDD SUCCESS! End-to-end pipeline now predicts 'dog' correctly!");
    } else {
        println!("❌ TDD STEP 3 FAILED: Still predicts '{}' instead of 'dog'", decoded);
        println!("   The pipeline integration issue persists despite individual steps working");
    }
    
    println!("\n📋 TDD PIPELINE ANALYSIS COMPLETE");
    println!("   This systematic test identifies the exact step causing divergence");
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_tdd_pipeline_step_isolation_macos_requirement() {
    println!("❌ TDD pipeline step isolation test requires macOS");
}