//! Direct test for CoreML metadata extraction

use candle_coreml::config_generator::CoreMLMetadataExtractor;
use std::path::Path;

#[test]
fn test_direct_metadata_extraction() {
    let model_path = Path::new("/tmp/qwen-typo-fixer-debug/qwen-typo-fixer_embeddings.mlpackage/Data/com.apple.CoreML/model.mlmodel");
    
    if !model_path.exists() {
        println!("⚠️ Skipping test - model not available");
        return;
    }

    let extractor = CoreMLMetadataExtractor::new();
    let result = extractor.extract_tensor_signatures(model_path);

    match result {
        Ok((inputs, outputs)) => {
            println!("✅ Successfully extracted metadata!");
            println!("Inputs: {:?}", inputs.keys().collect::<Vec<_>>());
            println!("Outputs: {:?}", outputs.keys().collect::<Vec<_>>());
            
            // Verify we got the expected embeddings component structure
            assert!(inputs.contains_key("input_ids"), "Expected input_ids input");
            assert!(outputs.contains_key("hidden_states"), "Expected hidden_states output");
        }
        Err(e) => {
            panic!("❌ Failed to extract metadata: {}", e);
        }
    }
}

#[test]
fn test_all_typo_fixer_components() {
    let model_dir = Path::new("/tmp/qwen-typo-fixer-debug");
    
    if !model_dir.exists() {
        println!("⚠️ Skipping test - model directory not available");
        return;
    }

    let components = [
        ("embeddings", "qwen-typo-fixer_embeddings.mlpackage"),
        ("ffn_infer", "qwen-typo-fixer_FFN_chunk_01of01.mlpackage"),
        ("ffn_prefill", "qwen-typo-fixer_prefill_chunk_01of01.mlpackage"),
        ("lm_head", "qwen-typo-fixer_lm_head.mlpackage"),
    ];

    let extractor = CoreMLMetadataExtractor::new();

    for (component_name, package_name) in &components {
        let model_path = model_dir.join(package_name).join("Data/com.apple.CoreML/model.mlmodel");
        
        if !model_path.exists() {
            println!("⚠️ Skipping {} - model file not found", component_name);
            continue;
        }

        println!("🔍 Testing {}: {}", component_name, package_name);
        
        let result = extractor.extract_tensor_signatures(&model_path);
        
        match result {
            Ok((inputs, outputs)) => {
                println!("  ✅ Inputs: {:?}", inputs.keys().collect::<Vec<_>>());
                println!("  ✅ Outputs: {:?}", outputs.keys().collect::<Vec<_>>());
            }
            Err(e) => {
                println!("  ❌ Failed: {}", e);
            }
        }
        println!();
    }
}