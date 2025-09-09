use candle_coreml::config_generator::shape_inference::ShapeInference;
use candle_coreml::model_config::{ComponentConfig, TensorConfig};
use std::collections::HashMap;

fn main() {
    println!("Testing batch size inference fix for split FFN architecture");
    println!("=" .repeat(60));
    
    // Create mock components simulating typo-fixer model structure
    let mut components = HashMap::new();
    
    // Mock FFN Prefill component with batch_size = 128
    let mut ffn_prefill_inputs = HashMap::new();
    ffn_prefill_inputs.insert("hidden_states".to_string(), TensorConfig {
        name: "hidden_states".to_string(),
        shape: vec![128, 12, 1024],  // batch=128, seq=12, hidden=1024
        data_type: "FLOAT16".to_string(),
    });
    
    let ffn_prefill = ComponentConfig {
        file_path: Some("ffn_prefill.mlpackage".to_string()),
        inputs: ffn_prefill_inputs,
        outputs: HashMap::new(),
        functions: vec!["prefill".to_string()],
        input_order: None,
    };
    components.insert("ffn_prefill".to_string(), ffn_prefill);
    
    // Mock FFN Infer component with batch_size = 1
    let mut ffn_infer_inputs = HashMap::new();
    ffn_infer_inputs.insert("hidden_states".to_string(), TensorConfig {
        name: "hidden_states".to_string(),
        shape: vec![1, 1, 1024],  // batch=1, seq=1, hidden=1024
        data_type: "FLOAT16".to_string(),
    });
    
    let ffn_infer = ComponentConfig {
        file_path: Some("ffn_infer.mlpackage".to_string()),
        inputs: ffn_infer_inputs,
        outputs: HashMap::new(),
        functions: vec!["infer".to_string()],
        input_order: None,
    };
    components.insert("ffn_infer".to_string(), ffn_infer);
    
    // Mock Embeddings component with batch_size = 1
    let mut embeddings_inputs = HashMap::new();
    embeddings_inputs.insert("input_ids".to_string(), TensorConfig {
        name: "input_ids".to_string(),
        shape: vec![1, 12],  // batch=1, seq=12
        data_type: "INT32".to_string(),
    });
    
    let embeddings = ComponentConfig {
        file_path: Some("embeddings.mlpackage".to_string()),
        inputs: embeddings_inputs,
        outputs: HashMap::new(),
        functions: vec![],
        input_order: None,
    };
    components.insert("embeddings".to_string(), embeddings);
    
    // Test the shape inference
    let shape_inference = ShapeInference::new();
    
    match shape_inference.infer_shapes(&components) {
        Ok(shapes) => {
            println!("✅ Shape inference succeeded!");
            println!("\nInferred shapes:");
            println!("  Batch size: {}", shapes.batch_size);
            println!("  Context length: {}", shapes.context_length);
            println!("  Hidden size: {}", shapes.hidden_size);
            println!("  Vocab size: {}", shapes.vocab_size);
            
            // Verify the fix worked
            if shapes.batch_size == 128 {
                println!("\n🎉 SUCCESS: Batch size correctly inferred as 128 (from ffn_prefill)");
                println!("The fix is working! The batch size is no longer incorrectly set to 1.");
            } else if shapes.batch_size == 1 {
                println!("\n❌ FAILURE: Batch size is still 1 (old behavior)");
                println!("The fix did not work as expected. The minimum was still used.");
            } else {
                println!("\n⚠️ UNEXPECTED: Batch size is {} (unexpected value)", shapes.batch_size);
            }
        }
        Err(e) => {
            println!("❌ Shape inference failed: {}", e);
        }
    }
}