//! Direct comparison between Python and Rust CoreML usage
//!
//! This test replicates the exact Python chat.py logic to identify
//! discrepancies in our Rust implementation.

use candle_core::{Device, DType, Tensor};
use candle_coreml::{CoreMLModel, Config};

#[test]
fn test_python_rust_comparison() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model";
    let device = Device::Cpu;

    println!("🔄 Python-to-Rust Comparison Test");
    println!("=================================");

    // Load models exactly like Python does
    let embeddings_path = format!("{}/qwen_embeddings.mlmodelc", cache_dir);
    let ffn_path = format!("{}/qwen_FFN_PF_lut8_chunk_01of01.mlmodelc", cache_dir);
    let lm_head_path = format!("{}/qwen_lm_head_lut8.mlmodelc", cache_dir);

    println!("📂 Loading models...");

    let config = Config {
        input_names: vec!["input_ids".to_string()],
        output_name: "hidden_states".to_string(),
        max_sequence_length: 512,
        model_type: "qwen".to_string(),
        vocab_size: 151936,
    };

    let embeddings = CoreMLModel::load_from_file(&embeddings_path, &config)?;
    
    // Load FFN with prefill function (like Python)
    let ffn_config = Config {
        input_names: vec![
            "hidden_states".to_string(),
            "position_ids".to_string(), 
            "causal_mask".to_string(),
            "current_pos".to_string(),
        ],
        output_name: "output_hidden_states".to_string(),
        max_sequence_length: 512,
        model_type: "qwen".to_string(),
        vocab_size: 151936,
    };
    
    let ffn_prefill = CoreMLModel::load_with_function(&ffn_path, &ffn_config, "prefill")?;
    
    let lm_head_config = Config {
        input_names: vec!["hidden_states".to_string()],
        output_name: "logits1".to_string(), // Python concatenates all logits1-16
        max_sequence_length: 512,
        model_type: "qwen".to_string(),
        vocab_size: 151936,
    };
    
    let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

    println!("✅ Models loaded successfully");

    // Test 1: Embeddings with batch size 64 (Python style)
    println!("\n🧪 Test 1: Embeddings (batch size 64)");
    
    // Use I64 (since Candle doesn't have I32) - conversion layer handles I64->I32
    let input_batch = Tensor::ones((1, 64), DType::I64, &device)?;
    println!("Input shape: {:?}", input_batch.dims());
    
    // Print actual input values to compare with Python
    let input_vec: Vec<i64> = input_batch.flatten_all()?.to_vec1()?;
    println!("Input values sample: {:?}", &input_vec[0..5]);
    
    let embed_output = embeddings.forward_single(&input_batch)?;
    println!("Embeddings shape: {:?}, dtype: {:?}", embed_output.dims(), embed_output.dtype());
    
    // Print sample values to compare with Python
    let embed_flat = embed_output.flatten_all()?;
    let embed_vec: Vec<f32> = embed_flat.to_vec1()?;
    println!("Embeddings sample: {:?}", &embed_vec[0..5]);

    // Test 2: FFN Prefill with exact Python inputs
    println!("\n🧪 Test 2: FFN Prefill");
    
    // Keep as F32 (Rust doesn't support F16 conversion yet)
    let hidden_states_f32 = embed_output;
    
    // Create position_ids like Python: torch.arange(0, 64) - use I64
    let position_ids = Tensor::arange(0u32, 64u32, &device)?.to_dtype(DType::I64)?;
    
    // Create causal mask like Python: [1, 1, 64, 512] - use F32 for Rust
    let causal_mask = Tensor::zeros((1, 1, 64, 512), DType::F32, &device)?;
    
    // Create current_pos like Python: [0] - use I64
    let current_pos = Tensor::new(&[0i64], &device)?;
    
    println!("FFN inputs:");
    println!("  hidden_states: {:?} dtype: {:?}", hidden_states_f32.dims(), hidden_states_f32.dtype());
    println!("  position_ids: {:?} dtype: {:?}", position_ids.dims(), position_ids.dtype());
    println!("  causal_mask: {:?} dtype: {:?}", causal_mask.dims(), causal_mask.dtype());
    println!("  current_pos: {:?} dtype: {:?}", current_pos.dims(), current_pos.dtype());

    // Run FFN prefill with state (like Python: state = ffn_prefill.make_state())
    let mut ffn_state = ffn_prefill.make_state()?;
    let ffn_inputs = [&hidden_states_f32, &position_ids, &causal_mask, &current_pos];
    let ffn_output = ffn_prefill.predict_with_state(&ffn_inputs, &mut ffn_state)?;
    
    println!("FFN output shape: {:?}, dtype: {:?}", ffn_output.dims(), ffn_output.dtype());
    
    // Print sample values to compare with Python
    let ffn_flat = ffn_output.flatten_all()?;
    let ffn_vec: Vec<f32> = ffn_flat.to_vec1()?;
    println!("FFN output sample: {:?}", &ffn_vec[0..5]);

    // Test 3: LM Head
    println!("\n🧪 Test 3: LM Head");
    
    let lm_input = ffn_output; // Keep as F32
    println!("LM head input shape: {:?}, dtype: {:?}", lm_input.dims(), lm_input.dtype());
    
    // Get all outputs from LM head (Python concatenates logits1-16)
    let lm_outputs = lm_head.forward_all(&[&lm_input])?;
    
    println!("LM head outputs: {:?}", lm_outputs.keys().collect::<Vec<_>>());
    
    if let Some(logits1) = lm_outputs.get("logits1") {
        println!("Logits1 shape: {:?}, dtype: {:?}", logits1.dims(), logits1.dtype());
        
        // Check for meaningful values
        let logits_flat = logits1.flatten_all()?;
        let logits_vec: Vec<f32> = logits_flat.to_vec1()?;
        println!("Logits1 sample: {:?}", &logits_vec[0..5]);
        
        let max_val = logits_vec.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min_val = logits_vec.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        println!("Logits1 max: {:.1}, min: {:.1}", max_val, min_val);
        
        if max_val > 1e-6 || min_val < -1e-6 {
            println!("✅ Rust produces meaningful logits!");
        } else {
            println!("❌ Rust produces zero logits");
        }
    }

    Ok(())
}