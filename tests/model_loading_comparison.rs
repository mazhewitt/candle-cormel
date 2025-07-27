//! Test different model loading approaches to find the issue

use candle_core::{Device, Tensor};
use candle_coreml::{CoreMLModel, Config};

#[test]
fn test_model_loading_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let device = Device::Cpu;
    
    println!("🔧 TESTING DIFFERENT MODEL LOADING APPROACHES");
    println!("==============================================");
    
    #[cfg(target_os = "macos")]
    {
        use objc2::rc::autoreleasepool;
        use objc2_core_ml::{MLModelConfiguration, MLComputeUnits};
        use objc2_foundation::{NSString, NSURL};
        
        let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
        
        autoreleasepool(|_| -> Result<(), Box<dyn std::error::Error>> {
            // Test input
            let input_tensor = Tensor::from_vec(vec![1i64], (1, 1), &device)?;
            
            println!("\n🔍 Method 1: Current approach (default loading)");
            let config = Config {
                input_names: vec!["input_ids".to_string()],
                output_name: "hidden_states".to_string(),
                max_sequence_length: 512,
                model_type: "qwen".to_string(),
                vocab_size: 151936,
            };
            
            let model1 = CoreMLModel::load_from_file(&embeddings_path, &config)?;
            let result1 = model1.forward_single(&input_tensor)?;
            let data1: Vec<f32> = result1.flatten_all()?.to_vec1()?;
            
            println!("Method 1 - First 5 values: {:?}", &data1[0..5]);
            println!("Method 1 - Max abs: {}", data1.iter().map(|x| x.abs()).fold(0.0, f32::max));
            
            println!("\n🔍 Method 2: With MLModelConfiguration (CPU + Neural Engine)");
            
            // Create model with specific configuration like Python might
            let url = unsafe { NSURL::fileURLWithPath(&NSString::from_str(&embeddings_path)) };
            let model_config = unsafe { MLModelConfiguration::new() };
            
            // Try different compute units
            unsafe { model_config.setComputeUnits(MLComputeUnits::CPUAndNeuralEngine) };
            
            let model_with_config = unsafe {
                use objc2_core_ml::MLModel;
                MLModel::modelWithContentsOfURL_configuration_error(&url, &model_config)
                    .map_err(|e| format!("Config load failed: {:?}", e))?
            };
            
            println!("Model with config loaded successfully");
            
            // Test with this configured model using our existing infrastructure
            let model2 = CoreMLModel::from_mlmodel(model_with_config, config.clone());
            
            let result2 = model2.forward_single(&input_tensor)?;
            let data2: Vec<f32> = result2.flatten_all()?.to_vec1()?;
            
            println!("Method 2 - First 5 values: {:?}", &data2[0..5]);
            println!("Method 2 - Max abs: {}", data2.iter().map(|x| x.abs()).fold(0.0, f32::max));
            
            println!("\n🔍 Method 3: CPU only");
            
            let model_config_cpu = unsafe { MLModelConfiguration::new() };
            unsafe { model_config_cpu.setComputeUnits(MLComputeUnits::CPUOnly) };
            
            let model_cpu = unsafe {
                use objc2_core_ml::MLModel;
                MLModel::modelWithContentsOfURL_configuration_error(&url, &model_config_cpu)
                    .map_err(|e| format!("CPU load failed: {:?}", e))?
            };
            
            let model3 = CoreMLModel::from_mlmodel(model_cpu, config.clone());
            
            let result3 = model3.forward_single(&input_tensor)?;
            let data3: Vec<f32> = result3.flatten_all()?.to_vec1()?;
            
            println!("Method 3 - First 5 values: {:?}", &data3[0..5]);
            println!("Method 3 - Max abs: {}", data3.iter().map(|x| x.abs()).fold(0.0, f32::max));
            
            println!("\n🔍 Method 4: All compute units");
            
            let model_config_all = unsafe { MLModelConfiguration::new() };
            unsafe { model_config_all.setComputeUnits(MLComputeUnits::All) };
            
            let model_all = unsafe {
                use objc2_core_ml::MLModel;
                MLModel::modelWithContentsOfURL_configuration_error(&url, &model_config_all)
                    .map_err(|e| format!("All compute units load failed: {:?}", e))?
            };
            
            let model4 = CoreMLModel::from_mlmodel(model_all, config.clone());
            
            let result4 = model4.forward_single(&input_tensor)?;
            let data4: Vec<f32> = result4.flatten_all()?.to_vec1()?;
            
            println!("Method 4 - First 5 values: {:?}", &data4[0..5]);
            println!("Method 4 - Max abs: {}", data4.iter().map(|x| x.abs()).fold(0.0, f32::max));
            
            // Summary
            println!("\n📊 SUMMARY:");
            println!("Method 1 (default): max_abs = {}", data1.iter().map(|x| x.abs()).fold(0.0, f32::max));
            println!("Method 2 (CPU+NE):  max_abs = {}", data2.iter().map(|x| x.abs()).fold(0.0, f32::max));
            println!("Method 3 (CPU only): max_abs = {}", data3.iter().map(|x| x.abs()).fold(0.0, f32::max));
            println!("Method 4 (All):      max_abs = {}", data4.iter().map(|x| x.abs()).fold(0.0, f32::max));
            
            // Check if any approach produces meaningful values
            let max_values = vec![
                data1.iter().map(|x| x.abs()).fold(0.0, f32::max),
                data2.iter().map(|x| x.abs()).fold(0.0, f32::max),
                data3.iter().map(|x| x.abs()).fold(0.0, f32::max),
                data4.iter().map(|x| x.abs()).fold(0.0, f32::max),
            ];
            
            if max_values.iter().any(|&x| x > 0.01) {
                println!("✅ SUCCESS: At least one loading method produces meaningful values!");
            } else {
                println!("❌ FAILURE: All loading methods produce near-zero values");
                println!("   This suggests the issue is deeper than model configuration");
            }
            
            Ok(())
        })?;
    }
    
    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping model loading test on non-macOS");
    }
    
    Ok(())
}