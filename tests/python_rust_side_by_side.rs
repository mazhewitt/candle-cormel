//! Side-by-side Python vs Rust comparison test
//! This will pinpoint exactly where outputs diverge

use candle_core::{Device, DType, Tensor};
use candle_coreml::{CoreMLModel, Config};
use std::process::Command;

#[test]
fn test_python_rust_output_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let device = Device::Cpu;
    
    // Test with identical input
    let test_token = 1i64;
    
    println!("🔍 PYTHON vs RUST SIDE-BY-SIDE COMPARISON");
    println!("==========================================");
    
    // === PYTHON OUTPUT ===
    let python_script = format!(r#"
import coremltools as ct
import numpy as np
import os

cache_dir = "{cache_dir}"
model = ct.models.CompiledMLModel(os.path.join(cache_dir, "qwen_embeddings.mlmodelc"))

# Exact same input as Rust
input_data = np.array([[{test_token}]], dtype=np.int32)
print("Python input:", input_data.flatten())
print("Python input shape:", input_data.shape)
print("Python input dtype:", input_data.dtype)

output = model.predict({{'input_ids': input_data}})
embeddings = output['hidden_states']
print("Python output shape:", embeddings.shape)
print("Python output dtype:", embeddings.dtype)
print("Python first 10 values:", embeddings.flatten()[:10])
print("Python max value:", embeddings.max())
print("Python min value:", embeddings.min())
print("Python mean value:", embeddings.mean())
print("Python std value:", embeddings.std())
"#);
    
    let python_output = Command::new("python3")
        .args(&["-c", &python_script])
        .output()?;
    
    println!("=== PYTHON OUTPUT ===");
    println!("{}", String::from_utf8_lossy(&python_output.stdout));
    if !python_output.stderr.is_empty() {
        println!("Python stderr: {}", String::from_utf8_lossy(&python_output.stderr));
    }
    
    // === RUST OUTPUT ===
    #[cfg(target_os = "macos")]
    {
        let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
        
        let config = Config {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            model_type: "qwen".to_string(),
            vocab_size: 151936,
        };
        
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &config)?;
        
        let input_tensor = Tensor::from_vec(vec![test_token], (1, 1), &device)?;
        
        println!("=== RUST OUTPUT ===");
        println!("Rust input: {:?}", input_tensor.to_vec2::<i64>()?);
        println!("Rust input shape: {:?}", input_tensor.dims());
        println!("Rust input dtype: {:?}", input_tensor.dtype());
        
        let rust_embeddings = embeddings.forward_single(&input_tensor)?;
        let flat_data: Vec<f32> = rust_embeddings.flatten_all()?.to_vec1()?;
        
        println!("Rust output shape: {:?}", rust_embeddings.dims());
        println!("Rust output dtype: {:?}", rust_embeddings.dtype());
        println!("Rust first 10 values: {:?}", &flat_data[..10]);
        println!("Rust max value: {}", flat_data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)));
        println!("Rust min value: {}", flat_data.iter().fold(f32::INFINITY, |a, &b| a.min(b)));
        println!("Rust mean value: {}", flat_data.iter().sum::<f32>() / flat_data.len() as f32);
        println!("Rust std value: {}", {
            let mean = flat_data.iter().sum::<f32>() / flat_data.len() as f32;
            let variance = flat_data.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / flat_data.len() as f32;
            variance.sqrt()
        });
        
        // Immediate diagnosis
        let max_abs = flat_data.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        if max_abs < 1e-6 {
            println!("❌ DIAGNOSIS: Rust outputs are essentially zero - fundamental CoreML prediction issue!");
        } else {
            println!("✅ DIAGNOSIS: Rust outputs have meaningful values");
        }
        
        // Test conversion accuracy separately
        println!("\n🔍 CONVERSION LAYER TEST:");
        test_conversion_accuracy(&input_tensor)?;
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("=== RUST OUTPUT ===");
        println!("⏭️ Skipping Rust test on non-macOS");
    }
    
    Ok(())
}

#[cfg(target_os = "macos")]
fn test_conversion_accuracy(input_tensor: &Tensor) -> Result<(), Box<dyn std::error::Error>> {
    use candle_coreml::conversion::tensor_to_mlmultiarray;
    
    // Test our conversion layer directly
    let ml_array = tensor_to_mlmultiarray(input_tensor)?;
    
    println!("MLMultiArray created successfully");
    println!("MLMultiArray data type: {:?}", unsafe { ml_array.dataType() });
    println!("MLMultiArray shape: {:?}", unsafe { ml_array.shape() });
    
    // Try to read back the data if possible
    let element_count = input_tensor.elem_count();
    println!("Expected element count: {}", element_count);
    
    Ok(())
}