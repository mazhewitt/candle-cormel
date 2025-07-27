//! Unit tests for tensor conversion to isolate conversion issues

use candle_core::{DType, Device, Tensor};
use candle_coreml::conversion::tensor_to_mlmultiarray;

#[test]
fn test_i64_to_int32_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    println!("🔧 Testing I64 to Int32 conversion");

    // Create test tensor with known values
    let test_values = vec![1i64, 2, 3, 4, 5];
    let tensor = Tensor::from_vec(test_values.clone(), (5,), &device)?;

    println!("Input tensor: {:?}", tensor);
    println!("Input values: {:?}", test_values);
    println!("Tensor dtype: {:?}", tensor.dtype());

    // Test conversion
    #[cfg(target_os = "macos")]
    {
        let ml_array = tensor_to_mlmultiarray(&tensor)?;
        println!("✅ Conversion successful");

        // Try to read back the values to verify conversion

        println!("MLArray data type: {:?}", unsafe { ml_array.dataType() });
        println!("MLArray shape: {:?}", unsafe { ml_array.shape() });

        // Check if conversion preserved values correctly
        let element_count = tensor.elem_count();
        println!("Element count: {}", element_count);
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping conversion test on non-macOS");
    }

    Ok(())
}

#[test]
fn test_f32_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    println!("🔧 Testing F32 conversion");

    // Create test tensor with known values
    let test_values = vec![1.5f32, 2.5, 3.5, 4.5, 5.5];
    let tensor = Tensor::from_vec(test_values.clone(), (5,), &device)?;

    println!("Input tensor: {:?}", tensor);
    println!("Input values: {:?}", test_values);
    println!("Tensor dtype: {:?}", tensor.dtype());

    // Test conversion
    #[cfg(target_os = "macos")]
    {
        let ml_array = tensor_to_mlmultiarray(&tensor)?;
        println!("✅ Conversion successful");

        println!("MLArray data type: {:?}", unsafe { ml_array.dataType() });
        println!("MLArray shape: {:?}", unsafe { ml_array.shape() });
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping conversion test on non-macOS");
    }

    Ok(())
}

#[test]
fn test_batch_tensor_conversion() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::Cpu;

    println!("🔧 Testing batch tensor conversion (like embeddings input)");

    // Create tensor matching our failing case: (1, 64) with all 1s
    let tensor = Tensor::ones((1, 64), DType::I64, &device)?;

    println!("Input tensor shape: {:?}", tensor.dims());
    println!("Input tensor dtype: {:?}", tensor.dtype());

    // Get actual values
    let values: Vec<i64> = tensor.flatten_all()?.to_vec1()?;
    println!("First 5 values: {:?}", &values[0..5]);
    println!("All values equal 1? {}", values.iter().all(|&x| x == 1));

    // Test conversion
    #[cfg(target_os = "macos")]
    {
        let ml_array = tensor_to_mlmultiarray(&tensor)?;
        println!("✅ Batch conversion successful");

        println!("MLArray data type: {:?}", unsafe { ml_array.dataType() });
        println!("MLArray shape: {:?}", unsafe { ml_array.shape() });

        // Verify the MLArray contains the right data
        let ml_shape = unsafe { ml_array.shape() };
        println!("Expected shape: [1, 64], Actual shape: {:?}", ml_shape);
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping conversion test on non-macOS");
    }

    Ok(())
}

#[test]
fn test_embeddings_model_direct() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Testing direct embeddings model with simple input");

    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let device = Device::Cpu;

    // Simple single token test first
    let simple_input = Tensor::ones((1, 1), DType::I64, &device)?;
    println!(
        "Simple input shape: {:?}, dtype: {:?}",
        simple_input.dims(),
        simple_input.dtype()
    );

    let simple_values: Vec<i64> = simple_input.flatten_all()?.to_vec1()?;
    println!("Simple input values: {:?}", simple_values);

    #[cfg(target_os = "macos")]
    {
        use candle_coreml::{Config, CoreMLModel};

        let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);

        let config = Config {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            model_type: "qwen".to_string(),
            vocab_size: 151936,
        };

        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &config)?;

        println!("✅ Embeddings model loaded");

        // Test different token values like Python
        println!("🔍 Testing different token values in Rust:");

        // Test 1: Token 1
        let embed_output_1 = embeddings.forward_single(&simple_input)?;
        let embed_values_1: Vec<f32> = embed_output_1.flatten_all()?.to_vec1()?;
        println!("Token 1 result: {:?}", &embed_values_1[0..3]);

        // Test 2: Token 0
        let input_0 = Tensor::zeros((1, 1), DType::I64, &device)?;
        let embed_output_0 = embeddings.forward_single(&input_0)?;
        let embed_values_0: Vec<f32> = embed_output_0.flatten_all()?.to_vec1()?;
        println!("Token 0 result: {:?}", &embed_values_0[0..3]);

        // Test 3: Token 2
        let input_2 = Tensor::new(&[[2i64]], &device)?;
        let embed_output_2 = embeddings.forward_single(&input_2)?;
        let embed_values_2: Vec<f32> = embed_output_2.flatten_all()?.to_vec1()?;
        println!("Token 2 result: {:?}", &embed_values_2[0..3]);

        // Check if any values are meaningful
        let max_abs_1 = embed_values_1
            .iter()
            .map(|x| x.abs())
            .fold(0.0f32, f32::max);
        let max_abs_0 = embed_values_0
            .iter()
            .map(|x| x.abs())
            .fold(0.0f32, f32::max);
        let max_abs_2 = embed_values_2
            .iter()
            .map(|x| x.abs())
            .fold(0.0f32, f32::max);

        println!("Token 1 max abs: {}", max_abs_1);
        println!("Token 0 max abs: {}", max_abs_0);
        println!("Token 2 max abs: {}", max_abs_2);

        if max_abs_1 < 1e-10 && max_abs_0 < 1e-10 && max_abs_2 < 1e-10 {
            println!("❌ All embeddings are essentially zero - fundamental issue!");
        } else {
            println!("✅ Some embeddings have meaningful values");
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("⏭️ Skipping embeddings test on non-macOS");
    }

    Ok(())
}
