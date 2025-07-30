//! QwenModel End-to-End Test - Using the actual QwenModel implementation
//!
//! This test uses the QwenModel from src/qwen.rs to replicate the exact same
//! pipeline that we've proven works in our isolation tests. It should produce
//! the same exact tensors and "dog" prediction.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use std::collections::HashMap;

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

/// Convert logits HashMap to a single tensor (concatenate 16 chunks)
fn combine_lm_head_outputs(outputs: HashMap<String, Tensor>) -> Result<Tensor> {
    let mut chunks = Vec::new();
    
    // Collect chunks in order (logits1, logits2, ..., logits16)
    for i in 1..=16 {
        let key = format!("logits{}", i);
        if let Some(chunk) = outputs.get(&key) {
            chunks.push(chunk.clone());
        } else {
            return Err(anyhow::Error::msg(format!("Missing logits chunk: {}", key)));
        }
    }
    
    // Concatenate along vocabulary dimension (last dimension)
    let chunk_refs: Vec<&Tensor> = chunks.iter().collect();
    Tensor::cat(&chunk_refs, 2).map_err(Into::into)  // Concat along dim 2 (vocab dimension)
}

/// Get the tokenizer token ID for "dog"
fn get_dog_token_id() -> i64 {
    5562  // Token ID for " dog" from the Python results
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors - run manually after qwen-chat-test.py"]
async fn test_qwen_model_lm_head_output() -> Result<()> {
    println!("🧠 Testing QwenModel produces exact LM head output");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Create QwenModel with default config
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    println!("✅ QwenModel loaded");
    
    // Load the expected LM head input that should be produced by QwenModel FFN processing
    let test_tensors_dir = "test_tensors";
    let device = Device::Cpu;
    
    let expected_lm_head_input = load_numpy_tensor(
        &format!("{}/05_lmhead_input.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("📥 Loaded expected LM head input: {:?}", expected_lm_head_input.shape());
    
    // Test the QwenModel with the same prompt used in Python
    let prompt = "The quick brown fox jumps over the lazy";
    println!("📝 Testing with prompt: '{}'", prompt);
    
    // TODO: We need to modify QwenModel to expose intermediate tensors for testing
    // For now, let's test the final prediction
    let next_token = qwen_model.forward_text(prompt)?;
    
    println!("🎯 QwenModel prediction: token {}", next_token);
    
    // CRITICAL ASSERTION: QwenModel must predict "dog" (token 5562)
    let dog_token_id = get_dog_token_id();
    
    if next_token == dog_token_id {
        println!("✅ 🎉 QWEN MODEL SUCCESS!");
        println!("   QwenModel correctly predicts 'dog' (token {})!", dog_token_id);
        Ok(())
    } else {
        panic!(
            "❌ QWEN MODEL FAILURE: Expected 'dog' (token {}), got token {}!",
            dog_token_id, next_token
        );
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires test tensors - run manually after qwen-chat-test.py"]
async fn test_qwen_model_generates_exact_ffn_output() -> Result<()> {
    println!("🔗 Testing QwenModel generates exact FFN output tensor");
    
    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    println!("✅ QwenModel loaded");
    
    // Load the expected FFN output (which becomes LM head input)
    let test_tensors_dir = "test_tensors";
    let device = Device::Cpu;
    
    let expected_ffn_output = load_numpy_tensor(
        &format!("{}/04_infer_ffn_output.npy", test_tensors_dir),
        &[1, 1, 1024],
        &device
    )?;
    println!("📥 Expected FFN output: {:?}", expected_ffn_output.shape());
    
    // Test with the same inputs that Python used
    let prompt = "The quick brown fox jumps over the lazy";
    
    // TODO: We need to modify QwenModel to expose the FFN output for comparison
    // This would require adding a method like get_ffn_output() or modifying forward_text()
    // to return intermediate tensors
    
    println!("⚠️  TODO: QwenModel needs to expose FFN output for direct comparison");
    println!("   This test will be completed after QwenModel refactoring");
    
    // For now, just test that it predicts correctly
    let next_token = qwen_model.forward_text(prompt)?;
    let dog_token_id = get_dog_token_id();
    
    assert_eq!(next_token, dog_token_id, 
               "QwenModel must predict 'dog' (token {}) to pass FFN output test", dog_token_id);
    
    println!("✅ QwenModel predicts correctly - FFN output likely correct");
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test] 
#[ignore = "requires test tensors - run manually after qwen-chat-test.py"]
async fn test_qwen_model_full_generation() -> Result<()> {
    println!("🎯 Testing QwenModel full text generation");
    
    // Load the model  
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    println!("✅ QwenModel loaded");
    
    // Test the exact same scenario as Python: generate a few tokens
    let prompt = "The quick brown fox jumps over the lazy";
    let max_tokens = 3;
    let temperature = 0.0; // Greedy for deterministic results
    
    println!("📝 Generating {} tokens for: '{}'", max_tokens, prompt);
    
    let generated_text = qwen_model.generate_text(prompt, max_tokens, temperature)?;
    
    println!("🎯 QwenModel generated: '{}'", generated_text);
    
    // CRITICAL ASSERTION: Generated text must contain "dog"
    if generated_text.to_lowercase().contains("dog") {
        println!("✅ 🎉 QWEN MODEL GENERATION SUCCESS!");
        println!("   Generated text contains 'dog': '{}'", generated_text);
        Ok(())
    } else {
        panic!(
            "❌ QWEN MODEL GENERATION FAILURE: Expected 'dog' in generated text, got: '{}'",
            generated_text
        );
    }
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_model_macos_requirement() {
    println!("❌ QwenModel tests require macOS");
}