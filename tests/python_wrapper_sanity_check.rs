//! Final sanity check: Replace objc2-core-ml with Python wrapper
//! This will definitively prove whether the issue is with objc2-core-ml bindings

use candle_core::{Device, Tensor};
use candle_coreml::{CoreMLModel, Config};
use std::process::Command;

fn predict_with_python(model_path: &str, token: i32) -> Vec<f32> {
    let output = Command::new("python3")
        .arg("-c")
        .arg(&format!(r#"
import coremltools as ct
import numpy as np

model = ct.models.CompiledMLModel('{}')
input_data = np.array([[{}]], dtype=np.int32)
result = model.predict({{'input_ids': input_data}})
print(' '.join(map(str, result['hidden_states'].flatten()[:10])))
"#, model_path, token))
        .output()
        .expect("Python prediction failed");
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[test]
fn test_python_wrapper_vs_objc2_final_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 FINAL SANITY CHECK: Python Wrapper vs objc2-core-ml");
    println!("=====================================================");
    
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
    let test_token = 1i32;
    
    // Test 1: Python wrapper approach
    println!("\n🐍 Python Wrapper Prediction:");
    let python_values = predict_with_python(&embeddings_path, test_token);
    
    if python_values.len() >= 10 {
        println!("Python values: {:?}", &python_values[..10]);
        let python_max_abs = python_values.iter().map(|x| x.abs()).fold(0.0, f32::max);
        println!("Python max abs: {}", python_max_abs);
        
        if python_max_abs > 0.01 {
            println!("✅ SUCCESS: Python wrapper produces meaningful values!");
        } else {
            println!("❌ UNEXPECTED: Python wrapper also produces near-zero values");
        }
    } else {
        println!("❌ ERROR: Python wrapper returned insufficient data");
        return Ok(());
    }
    
    // Test 2: objc2-core-ml approach
    println!("\n🦀 Rust objc2-core-ml Prediction:");
    
    #[cfg(target_os = "macos")]
    {
        let device = Device::Cpu;
        let config = Config {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            model_type: "qwen".to_string(),
            vocab_size: 151936,
        };
        
        let model = CoreMLModel::load_from_file(&embeddings_path, &config)?;
        let input_tensor = Tensor::from_vec(vec![test_token as i64], (1, 1), &device)?;
        let result = model.forward_single(&input_tensor)?;
        let rust_values: Vec<f32> = result.flatten_all()?.to_vec1()?;
        
        println!("Rust values: {:?}", &rust_values[..10]);
        let rust_max_abs = rust_values.iter().map(|x| x.abs()).fold(0.0, f32::max);
        println!("Rust max abs: {}", rust_max_abs);
        
        if rust_max_abs > 0.01 {
            println!("✅ SUCCESS: Rust objc2-core-ml produces meaningful values!");
        } else {
            println!("❌ CONFIRMED: Rust objc2-core-ml produces near-zero values");
        }
        
        // Final comparison
        println!("\n📊 FINAL COMPARISON:");
        let python_max_abs = python_values.iter().map(|x| x.abs()).fold(0.0, f32::max);
        println!("Python max abs:     {}", python_max_abs);
        println!("Rust objc2 max abs: {}", rust_max_abs);
        
        let ratio = if rust_max_abs > 0.0 { python_max_abs / rust_max_abs } else { f32::INFINITY };
        println!("Ratio (Python/Rust): {}", ratio);
        
        if ratio > 1000.0 {
            println!("🚨 DEFINITIVE PROOF: objc2-core-ml binding issue confirmed!");
            println!("   Python produces {}x larger values than Rust objc2-core-ml", ratio);
            println!("   This conclusively demonstrates the CoreML binding incompatibility");
        } else {
            println!("🤔 UNEXPECTED: Values are similar between Python and Rust");
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping Rust objc2-core-ml test on non-macOS");
    }
    
    Ok(())
}

#[test]
fn test_multiple_tokens_python_wrapper() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔢 TESTING MULTIPLE TOKENS WITH PYTHON WRAPPER");
    println!("==============================================");
    
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
    
    // Test various token values
    let test_tokens = [0, 1, 2, 10, 100, 1000];
    
    for &token in &test_tokens {
        println!("\n🎯 Testing token: {}", token);
        let values = predict_with_python(&embeddings_path, token);
        
        if values.len() >= 10 {
            let max_abs = values.iter().map(|x| x.abs()).fold(0.0, f32::max);
            println!("Token {} - First 5 values: {:?}", token, &values[..5]);
            println!("Token {} - Max abs: {}", token, max_abs);
            
            if max_abs > 0.01 {
                println!("✅ Token {} produces meaningful values", token);
            } else {
                println!("❌ Token {} produces near-zero values", token);
            }
        } else {
            println!("❌ Token {} failed to produce sufficient output", token);
        }
    }
    
    Ok(())
}

#[test] 
fn test_python_wrapper_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("🛡️ TESTING PYTHON WRAPPER ERROR HANDLING");
    println!("=========================================");
    
    // Test with non-existent model path
    let fake_path = "/nonexistent/model.mlmodelc";
    
    let output = Command::new("python3")
        .arg("-c")
        .arg(&format!(r#"
import coremltools as ct
import numpy as np
import sys

try:
    model = ct.models.CompiledMLModel('{}')
    input_data = np.array([[1]], dtype=np.int32)
    result = model.predict({{'input_ids': input_data}})
    print(' '.join(map(str, result['hidden_states'].flatten()[:10])))
except Exception as e:
    print(f"ERROR: {{e}}", file=sys.stderr)
    sys.exit(1)
"#, fake_path))
        .output()
        .expect("Python command failed");
    
    if output.status.success() {
        println!("❌ UNEXPECTED: Python succeeded with fake path");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("✅ Python wrapper correctly handles errors:");
        println!("   {}", stderr.trim());
    }
    
    Ok(())
}