// Minimal test to isolate MLState crash
use candle_coreml::{ensure_model_downloaded, Config, CoreMLModel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing MLState creation with Apple Mistral...");

    // Download model
    let model_path = ensure_model_downloaded("apple/mistral-coreml", true)?
        .join("StatefulMistral7BInstructInt4.mlpackage");

    let config = Config {
        input_names: vec!["inputIds".to_string(), "causalMask".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: 1,
        vocab_size: 32000,
        model_type: "StatefulMistral7BInstructInt4".to_string(),
    };

    println!("Loading model...");
    let model = CoreMLModel::load_from_file(&model_path, &config)?;
    println!("✅ Model loaded");

    // Test 1: Single state creation
    println!("Creating first state...");
    let _state1 = model.make_state()?;
    println!("✅ First state created");

    // Test 2: Multiple states from same model
    println!("Creating second state from same model...");
    let _state2 = model.make_state()?;
    println!("✅ Second state created");

    // Test 3: Scope test - state should be dropped here
    {
        println!("Creating scoped state...");
        let _state3 = model.make_state()?;
        println!("✅ Scoped state created");
    }
    println!("✅ Scoped state dropped");

    // Test 4: Multiple models + states
    println!("Loading second model instance...");
    let model2 = CoreMLModel::load_from_file(&model_path, &config)?;
    let _state4 = model2.make_state()?;
    println!("✅ Second model + state created");

    println!("🎉 All tests passed - no crash detected");
    Ok(())
}
