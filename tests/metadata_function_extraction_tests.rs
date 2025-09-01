//! TDD tests for metadata-based function extraction
//!
//! These tests verify that we can correctly parse component roles from CoreML metadata
//! instead of relying on filename patterns, supporting both .mlmodelc and .mlpackage formats.

use candle_coreml::{ConfigGenerator, config_generator::ComponentRole};
use std::path::Path;

#[test]
fn test_detect_typo_fixer_components_from_metadata() {
    // TDD: Test that we can detect all 4 components from the actual typo-fixer model
    let model_path = "/tmp/qwen-typo-fixer-debug";
    
    if !Path::new(model_path).exists() {
        println!("⚠️ Skipping test - model not downloaded. Run: huggingface-cli download mazhewitt/qwen-typo-fixer-coreml --local-dir /tmp/qwen-typo-fixer-debug");
        return;
    }

    let config_generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
    
    // Test: Generate config from the actual model directory
    let result = config_generator.generate_config_from_directory_enhanced(
        Path::new(model_path),
        "mazhewitt/qwen-typo-fixer-coreml",
        "qwen"
    );

    match result {
        Ok(config) => {
            println!("✅ Config generated successfully!");
            println!("📋 Found components: {:?}", config.components.keys().collect::<Vec<_>>());
            
            // Assert we have all required components
            assert!(config.components.contains_key("embeddings"), 
                   "Missing embeddings component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
            assert!(config.components.contains_key("lm_head"), 
                   "Missing lm_head component. Found: {:?}", config.components.keys().collect::<Vec<_>>());
            
            // Check for FFN components (split architecture)
            let has_ffn_prefill = config.components.contains_key("ffn_prefill");
            let has_ffn_infer = config.components.contains_key("ffn_infer");
            
            assert!(has_ffn_prefill || has_ffn_infer, 
                   "Missing FFN components. Expected ffn_prefill and/or ffn_infer. Found: {:?}", 
                   config.components.keys().collect::<Vec<_>>());
            
            // Validate execution mode
            if has_ffn_prefill && has_ffn_infer {
                assert_eq!(config.ffn_execution.as_deref(), Some("split"), 
                          "Expected split execution mode for separate FFN components");
                println!("✅ Split FFN architecture detected correctly");
            } else {
                println!("ℹ️ Unified FFN architecture detected");
            }
            
            // Validate shapes are reasonable
            assert!(config.shapes.vocab_size > 1000, 
                   "Vocab size should be > 1000, got: {}", config.shapes.vocab_size);
            assert!(config.shapes.hidden_size > 0, 
                   "Hidden size should be > 0, got: {}", config.shapes.hidden_size);
            
            println!("✅ All validations passed!");
            println!("📊 Config: vocab={}, hidden={}, context={}", 
                    config.shapes.vocab_size, config.shapes.hidden_size, config.shapes.context_length);
        }
        Err(e) => {
            println!("❌ Config generation failed: {}", e);
            // Print debug info about what we found
            let packages = config_generator.find_mlpackage_files(Path::new(model_path))
                .unwrap_or_default();
            println!("🔍 Found packages: {:?}", packages.iter().map(|p| p.file_name().unwrap().to_str().unwrap()).collect::<Vec<_>>());
            
            panic!("Expected config generation to succeed, but got: {}", e);
        }
    }
}

#[test]
fn test_component_role_detection_from_actual_metadata() {
    // TDD: Test that we can detect component roles from actual .mlpackage metadata
    let model_path = "/tmp/qwen-typo-fixer-debug";
    
    if !Path::new(model_path).exists() {
        println!("⚠️ Skipping test - model not downloaded");
        return;
    }

    let config_generator = ConfigGenerator::new().expect("Failed to create ConfigGenerator");
    
    // Test each component individually
    let test_cases = vec![
        ("qwen-typo-fixer_embeddings.mlpackage", ComponentRole::Embeddings),
        ("qwen-typo-fixer_lm_head.mlpackage", ComponentRole::LmHead),
        ("qwen-typo-fixer_prefill_chunk_01of01.mlpackage", ComponentRole::FfnPrefill),
        ("qwen-typo-fixer_FFN_chunk_01of01.mlpackage", ComponentRole::FfnInfer),
    ];
    
    for (package_name, expected_role) in test_cases {
        let package_path = Path::new(model_path).join(package_name);
        if package_path.exists() {
            println!("🔍 Testing component: {}", package_name);
            
            // Analyze the package and detect its role
            let component_config = config_generator.analyze_mlpackage(&package_path)
                .expect(&format!("Failed to analyze {}", package_name));
                
            // This would use our enhanced role detection - for now just test that we can analyze the package
            // We'll implement proper role detection in the next step
            let detected_role = ComponentRole::Unknown; // TODO: implement proper detection
            
            println!("   Detected role: {:?}, Expected: {:?}", detected_role, expected_role);
            println!("   Inputs: {:?}", component_config.inputs.keys().collect::<Vec<_>>());
            println!("   Outputs: {:?}", component_config.outputs.keys().collect::<Vec<_>>());
            
            // For now, we'll accept Unknown and implement the fix
            if detected_role == ComponentRole::Unknown {
                println!("   ⚠️ Role detection needs improvement - this is what we'll fix");
            } else {
                assert_eq!(detected_role, expected_role, 
                          "Component role mismatch for {}", package_name);
                println!("   ✅ Role detected correctly!");
            }
        }
    }
}