//! Test different MLPredictionOptions to see if they affect output quality

use candle_core::{Device, Tensor};
use candle_coreml::{Config, CoreMLModel};

#[cfg(target_os = "macos")]
#[test]
#[ignore] // Requires external model files
fn test_prediction_options_impact() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 TESTING DIFFERENT MLPREDICTIONOPTIONS");
    println!("========================================");

    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
    let device = Device::Cpu;

    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "hidden_states".to_string(),
        max_sequence_length: 512,
        model_type: "qwen".to_string(),
        vocab_size: 151936,
    };

    let model = CoreMLModel::load_from_file(&embeddings_path, &config)?;
    let input_tensor = Tensor::from_vec(vec![1i64], (1, 1), &device)?;

    // Test 1: Current default approach (no options)
    println!("\n🔍 Test 1: Default prediction (no options)");
    let result1 = model.forward_single(&input_tensor)?;
    let data1: Vec<f32> = result1.flatten_all()?.to_vec1()?;
    let max_abs1 = data1.iter().map(|x| x.abs()).fold(0.0, f32::max);
    println!("Default max abs: {}", max_abs1);

    // Test 2: Try with MLPredictionOptions using CPU only
    println!("\n🔍 Test 2: Testing MLPredictionOptions with usesCPUOnly");
    test_with_cpu_only_option(&model, &input_tensor)?;

    // Test 3: Manual prediction with options via inner model access
    println!("\n🔍 Test 3: Manual prediction with explicit options");
    test_manual_prediction_with_options(&model, &input_tensor)?;

    Ok(())
}

#[cfg(target_os = "macos")]
fn test_with_cpu_only_option(
    model: &CoreMLModel,
    input_tensor: &Tensor,
) -> Result<(), Box<dyn std::error::Error>> {
    use candle_coreml::conversion::{
        create_multi_feature_provider, extract_output, tensor_to_mlmultiarray,
    };
    use objc2::rc::autoreleasepool;
    use objc2::runtime::ProtocolObject;
    use objc2_core_ml::MLPredictionOptions;

    autoreleasepool(|_| -> Result<(), Box<dyn std::error::Error>> {
        // Convert input using existing infrastructure
        let ml_array = tensor_to_mlmultiarray(input_tensor)?;
        let provider = create_multi_feature_provider(&["input_ids".to_string()], &[ml_array])?;
        let protocol_provider = ProtocolObject::from_ref(&*provider);

        // Create prediction options (CPU-only is deprecated, so skip that)
        let options = unsafe { MLPredictionOptions::new() };

        // Make prediction with options using model's inner model
        let prediction = unsafe {
            model
                .inner_model()
                .predictionFromFeatures_options_error(protocol_provider, &options)
                .map_err(|e| format!("Prediction with options failed: {:?}", e))?
        };

        // Extract output
        let output_tensor = extract_output(&prediction, "hidden_states", input_tensor.device())?;
        let data: Vec<f32> = output_tensor.flatten_all()?.to_vec1()?;
        let max_abs = data.iter().map(|x| x.abs()).fold(0.0, f32::max);

        println!("With prediction options max abs: {}", max_abs);

        if max_abs > 1e-6 {
            println!("✅ SUCCESS: Prediction options produce meaningful values!");
        } else {
            println!("❌ Prediction options still produce near-zero values");
        }

        Ok(())
    })
}

#[cfg(target_os = "macos")]
fn test_manual_prediction_with_options(
    model: &CoreMLModel,
    input_tensor: &Tensor,
) -> Result<(), Box<dyn std::error::Error>> {
    use candle_coreml::conversion::{
        create_multi_feature_provider, extract_output, tensor_to_mlmultiarray,
    };
    use objc2::rc::autoreleasepool;
    use objc2::runtime::ProtocolObject;
    use objc2_core_ml::MLPredictionOptions;

    autoreleasepool(|_| -> Result<(), Box<dyn std::error::Error>> {
        // Convert input using existing infrastructure
        let ml_array = tensor_to_mlmultiarray(input_tensor)?;
        let provider = create_multi_feature_provider(&["input_ids".to_string()], &[ml_array])?;
        let protocol_provider = ProtocolObject::from_ref(&*provider);

        // Test just the basic MLPredictionOptions approach
        let options = unsafe { MLPredictionOptions::new() };

        let prediction = unsafe {
            model
                .inner_model()
                .predictionFromFeatures_options_error(protocol_provider, &options)
                .map_err(|e| format!("Manual prediction with options failed: {:?}", e))?
        };

        let output_tensor = extract_output(&prediction, "hidden_states", input_tensor.device())?;
        let data: Vec<f32> = output_tensor.flatten_all()?.to_vec1()?;
        let max_abs = data.iter().map(|x| x.abs()).fold(0.0, f32::max);

        println!("Manual prediction with options: max abs = {}", max_abs);

        if max_abs > 1e-6 {
            println!("✅ Manual prediction with options produces meaningful values!");
        } else {
            println!("❌ Manual prediction with options still near-zero");
        }

        Ok(())
    })
}

#[cfg(not(target_os = "macos"))]
#[test]
fn test_prediction_options_impact() -> Result<(), Box<dyn std::error::Error>> {
    println!("⏭️ Skipping prediction options test on non-macOS");
    Ok(())
}
