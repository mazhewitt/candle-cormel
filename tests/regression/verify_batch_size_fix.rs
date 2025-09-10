//! Test to verify the batch size inference fix for split FFN architectures

#[cfg(test)]
mod batch_size_fix_tests {
    use candle_coreml::config_generator::shape_inference::ShapeInference;
    use candle_coreml::model_config::{ComponentConfig, TensorConfig};
    use std::collections::HashMap;

    #[test]
    fn test_batch_size_inference_with_split_ffn() {
        println!("Testing batch size inference fix for split FFN architecture");
        
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
        
        // Mock Embeddings component
        let mut embeddings_outputs = HashMap::new();
        embeddings_outputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![128, 12, 1024],  // Should match prefill batch size
            data_type: "FLOAT16".to_string(),
        });
        
        let embeddings = ComponentConfig {
            file_path: Some("embeddings.mlpackage".to_string()),
            inputs: HashMap::new(),
            outputs: embeddings_outputs,
            functions: vec![],
            input_order: None,
        };
        components.insert("embeddings".to_string(), embeddings);
        
        // Test the shape inference
        let shape_inference = ShapeInference::new();
        
        let shapes = shape_inference.infer_shapes(&components)
            .expect("Shape inference should succeed");
        
        println!("Inferred shapes:");
        println!("  Batch size: {}", shapes.batch_size);
        println!("  Context length: {}", shapes.context_length);
        println!("  Hidden size: {}", shapes.hidden_size);
        
        // The fix should ensure batch_size is 128 (from ffn_prefill), not 1
        assert_eq!(shapes.batch_size, 128, 
            "Batch size should be 128 from ffn_prefill, not 1 from ffn_infer");
        
        // Verify other dimensions
        assert_eq!(shapes.hidden_size, 1024, "Hidden size should be 1024");
        assert_eq!(shapes.context_length, 12, "Context length should be 12");
    }
    
    #[test]
    fn test_batch_size_without_split_ffn() {
        // Test that normal models without split FFN still work correctly
        let mut components = HashMap::new();
        
        // Single FFN component
        let mut ffn_inputs = HashMap::new();
        ffn_inputs.insert("hidden_states".to_string(), TensorConfig {
            name: "hidden_states".to_string(),
            shape: vec![64, 256, 1024],
            data_type: "FLOAT16".to_string(),
        });
        
        let ffn = ComponentConfig {
            file_path: Some("ffn.mlpackage".to_string()),
            inputs: ffn_inputs,
            outputs: HashMap::new(),
            functions: vec![],
            input_order: None,
        };
        components.insert("ffn".to_string(), ffn);
        
        let shape_inference = ShapeInference::new();
        let shapes = shape_inference.infer_shapes(&components)
            .expect("Shape inference should succeed");
        
        assert_eq!(shapes.batch_size, 64, "Batch size should be 64 for unified FFN");
    }
}