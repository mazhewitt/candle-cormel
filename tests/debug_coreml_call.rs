//! Debug the exact CoreML prediction call to identify API differences

use candle_core::{Device, Tensor};
use candle_coreml::{CoreMLModel, Config};

#[test]
fn debug_coreml_prediction_call() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 DEBUGGING COREML PREDICTION CALL");
    println!("===================================");
    
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let device = Device::Cpu;
    
    #[cfg(target_os = "macos")]
    {
        use candle_coreml::conversion::tensor_to_mlmultiarray;
        use objc2_core_ml::MLMultiArrayDataType;
        
        // Load model
        let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
        let config = Config {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            model_type: "qwen".to_string(),
            vocab_size: 151936,
        };
        
        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &config)?;
        
        // Create test input
        let input_tensor = Tensor::from_vec(vec![1i64], (1, 1), &device)?;
        println!("Input tensor: {:?}", input_tensor.to_vec2::<i64>()?);
        
        // Debug conversion step by step
        println!("\n🔍 Step 1: Tensor to MLMultiArray conversion");
        let ml_array = tensor_to_mlmultiarray(&input_tensor)?;
        
        unsafe {
            let data_type = ml_array.dataType();
            let shape = ml_array.shape();
            let count = ml_array.count();
            
            println!("MLMultiArray data type: {:?}", data_type);
            println!("MLMultiArray shape: {:?}", shape);
            println!("MLMultiArray count: {}", count);
            
            // Check data type constants
            println!("Int32 constant: {:?}", MLMultiArrayDataType::Int32);
            println!("Float32 constant: {:?}", MLMultiArrayDataType::Float32);
            println!("Is Int32? {}", data_type == MLMultiArrayDataType::Int32);
            
            // Try to read the actual data from MLMultiArray using proper API
            println!("\n🔍 Step 2: Reading MLMultiArray data");
            
            use block2::StackBlock;
            use std::sync::atomic::{AtomicBool, Ordering};
            
            let data_read = AtomicBool::new(false);
            
            unsafe {
                ml_array.getBytesWithHandler(&StackBlock::new(
                    |ptr: std::ptr::NonNull<std::ffi::c_void>, len| {
                        if len >= 4 {  // At least one i32
                            let int_ptr = ptr.as_ptr() as *const i32;
                            let slice = std::slice::from_raw_parts(int_ptr, 1);
                            println!("Raw MLMultiArray data: {:?}", slice);
                            data_read.store(true, Ordering::Relaxed);
                        }
                    },
                ));
            }
            
            if !data_read.load(Ordering::Relaxed) {
                println!("❌ Could not read MLMultiArray data");
            }
        }
        
        println!("\n🔍 Step 3: CoreML prediction");
        let result = embeddings.forward_single(&input_tensor)?;
        let output_data: Vec<f32> = result.flatten_all()?.to_vec1()?;
        
        println!("Output shape: {:?}", result.dims());
        println!("First 5 outputs: {:?}", &output_data[0..5]);
        println!("Max abs value: {}", output_data.iter().map(|x| x.abs()).fold(0.0, f32::max));
        
        // Diagnostic: Compare with a different model or different inputs
        println!("\n🔍 Step 4: Testing with different inputs");
        
        for test_val in [0i64, 1i64, 2i64, 100i64] {
            let test_input = Tensor::from_vec(vec![test_val], (1, 1), &device)?;
            let test_result = embeddings.forward_single(&test_input)?;
            let test_data: Vec<f32> = test_result.flatten_all()?.to_vec1()?;
            let max_abs = test_data.iter().map(|x| x.abs()).fold(0.0, f32::max);
            println!("Token {}: max_abs = {}", test_val, max_abs);
        }
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping debug test on non-macOS");
    }
    
    Ok(())
}