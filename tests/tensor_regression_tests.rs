//! Tensor-based regression tests using captured fixtures
//! 
//! These tests validate tensor operations and model behavior using previously
//! captured tensor data, ensuring consistent behavior without requiring
//! live models. This provides comprehensive test coverage while being 
//! CI/CD friendly.

use candle_core::{Device, Tensor};
use candle_coreml::ModelConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TensorData {
    shape: Vec<usize>,
    data: Vec<f32>,
    dtype: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CapturedTensors {
    model_id: String,
    test_prompt: String,
    tokens: Vec<i64>,
    embeddings: TensorData,
    prefill_outputs: Vec<TensorData>,
    final_logits: TensorData,
    generated_token: i64,
    metadata: HashMap<String, serde_json::Value>,
}

impl TensorData {
    fn to_tensor(&self, device: &Device) -> candle_core::Result<Tensor> {
        Tensor::from_vec(self.data.clone(), self.shape.clone(), device)
    }
}

// Helper function to assert tensors are approximately equal
fn tensors_approx_equal(a: &Tensor, b: &Tensor, tolerance: f32) -> candle_core::Result<bool> {
    if a.dims() != b.dims() {
        return Ok(false);
    }
    
    let a_flat = a.flatten_all()?.to_vec1::<f32>()?;
    let b_flat = b.flatten_all()?.to_vec1::<f32>()?;
    
    for (av, bv) in a_flat.iter().zip(b_flat.iter()) {
        if (av - bv).abs() > tolerance {
            return Ok(false);
        }
    }
    
    Ok(true)
}

#[test]
fn test_load_captured_tensor_fixture() {
    // Test that we can successfully load and parse the captured tensor data
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data)
        .expect("Should be able to parse captured tensor fixture");
    
    assert_eq!(captured.model_id, "typo-fixer-test-data");
    assert_eq!(captured.test_prompt, "teh quick bronw");
    assert_eq!(captured.tokens.len(), 5);
    assert_eq!(captured.embeddings.shape, vec![1, 64, 1024]);
    assert_eq!(captured.final_logits.shape, vec![1, 1, 151669]);
    assert_eq!(captured.generated_token, 198);
    
    // Verify metadata
    assert!(captured.metadata.contains_key("model_type"));
    assert!(captured.metadata.contains_key("batch_size"));
    assert!(captured.metadata.contains_key("context_length"));
}

#[test]
fn test_tensor_reconstruction() {
    // Test that we can reconstruct tensors from the captured data
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    let device = Device::Cpu;
    
    // Test embeddings tensor reconstruction
    let embeddings = captured.embeddings.to_tensor(&device)
        .expect("Should be able to reconstruct embeddings tensor");
    assert_eq!(embeddings.dims(), &[1, 64, 1024]);
    
    // Test logits tensor reconstruction
    let logits = captured.final_logits.to_tensor(&device)
        .expect("Should be able to reconstruct logits tensor");
    assert_eq!(logits.dims(), &[1, 1, 151669]);
    
    // Test that we can perform operations on reconstructed tensors
    let argmax = logits.argmax(2).expect("Should be able to compute argmax");
    let token = argmax.flatten_all().unwrap().to_vec1::<u32>().unwrap()[0] as i64;
    assert_eq!(token, captured.generated_token);
}

#[test]
fn test_multipart_logits_assembly() {
    // Test logic for assembling multipart logits (common in fine-tuned models)
    // This tests the tensor concatenation logic using fixture data
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    let device = Device::Cpu;
    let full_logits = captured.final_logits.to_tensor(&device).unwrap();
    
    // Test splitting and reassembling (simulates multipart logits handling)
    let split_size = full_logits.dims()[2] / 4; // Split into 4 parts
    let mut parts = Vec::new();
    
    for i in 0..4 {
        let start = i * split_size;
        let size = if i == 3 { 
            full_logits.dims()[2] - start  // Last part gets remainder
        } else { 
            split_size 
        };
        let part = full_logits.narrow(2, start, size).unwrap();
        parts.push(part);
    }
    
    // Reassemble
    let reassembled = Tensor::cat(&parts, 2).unwrap();
    
    // Should match original
    assert_eq!(reassembled.dims(), full_logits.dims());
    
    let are_equal = tensors_approx_equal(&full_logits, &reassembled, 1e-6).unwrap();
    assert!(are_equal, "Reassembled tensor should match original");
}

#[test]
fn test_tensor_shape_validation() {
    // Test tensor shape validation logic using captured data
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    // Test batch size validation
    let batch_size = captured.metadata["batch_size"].as_u64().unwrap() as usize;
    assert_eq!(captured.embeddings.shape[0], 1); // Actual batch in tensor
    assert_eq!(batch_size, 64); // Config batch size
    
    // Test context length validation  
    let context_length = captured.metadata["context_length"].as_u64().unwrap() as usize;
    assert_eq!(captured.embeddings.shape[1], 64); // Embeddings sequence length
    assert_eq!(context_length, 256); // Model's configured context length
    
    // Test hidden size consistency
    let hidden_size = captured.metadata["hidden_size"].as_u64().unwrap() as usize;
    assert_eq!(captured.embeddings.shape[2], hidden_size);
    
    // Test that prefill outputs have correct single-token shape
    assert!(!captured.prefill_outputs.is_empty());
    let last_hidden = &captured.prefill_outputs[0];
    assert_eq!(last_hidden.shape, vec![1, 1, hidden_size]); // Single token output
}

#[test]
fn test_generic_model_config_parsing() {
    // Test that we can create generic ModelConfig structures that would
    // support the same tensor operations as the captured data
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    // Create a generic ModelConfig that matches the captured metadata
    let config_json = format!(r#"{{
        "model_info": {{
            "model_id": "test-model",
            "model_type": "qwen"
        }},
        "shapes": {{
            "batch_size": {},
            "context_length": {},
            "hidden_size": {},
            "vocab_size": {}
        }},
        "components": {{
            "embeddings": {{
                "file_path": "test_embeddings.mlpackage",
                "inputs": {{
                    "input_ids": {{ "name": "input_ids", "shape": [1, {}], "data_type": "INT32" }}
                }},
                "outputs": {{
                    "hidden_states": {{ "name": "hidden_states", "shape": [{}, {}, {}], "data_type": "FLOAT16" }}
                }},
                "functions": []
            }}
        }},
        "naming": {{
            "embeddings_pattern": null,
            "ffn_prefill_pattern": null,
            "lm_head_pattern": null
        }}
    }}"#,
        captured.metadata["batch_size"].as_u64().unwrap(),
        captured.metadata["context_length"].as_u64().unwrap(),
        captured.metadata["hidden_size"].as_u64().unwrap(),
        captured.metadata["vocab_size"].as_u64().unwrap(),
        captured.metadata["batch_size"].as_u64().unwrap(),
        captured.embeddings.shape[0],
        captured.embeddings.shape[1],
        captured.embeddings.shape[2]
    );
    
    let model_config: ModelConfig = serde_json::from_str(&config_json)
        .expect("Should be able to parse generic model config");
        
    // Validate the config matches our captured data
    assert_eq!(model_config.shapes.hidden_size, captured.embeddings.shape[2]);
    assert!(model_config.components.contains_key("embeddings"));
    
    // Test that config can be parsed (validation would require all components)
    assert_eq!(model_config.model_info.model_type, "qwen");
}

#[test]
fn test_sampling_with_captured_logits() {
    // Test sampling functions using the captured logits data
    use candle_coreml::sampling;
    
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    let device = Device::Cpu;
    let logits = captured.final_logits.to_tensor(&device).unwrap();
    
    // Test greedy sampling (should match captured token)
    let greedy_token = sampling::greedy_sample(&logits.squeeze(0).unwrap().squeeze(0).unwrap()).unwrap();
    assert_eq!(greedy_token, captured.generated_token);
    
    // Test top-k sampling with k=1 (should also match)
    let topk_token = sampling::sample_top_k(&logits.squeeze(0).unwrap().squeeze(0).unwrap(), 1, 0.0).unwrap();
    assert_eq!(topk_token, captured.generated_token);
    
    // Test that top-k with higher k returns valid token
    let topk_10 = sampling::sample_top_k(&logits.squeeze(0).unwrap().squeeze(0).unwrap(), 10, 0.7).unwrap();
    assert!(topk_10 >= 0 && topk_10 < captured.metadata["vocab_size"].as_i64().unwrap());
}

#[test]
fn test_sequential_prefill_planning() {
    // Test prefill planning logic using captured token data
    use candle_coreml::qwen::QwenModel;
    use candle_coreml::qwen::inference::PrefillStep;
    
    let fixture_data = include_str!("fixtures/typo_fixer_tensors.json");
    let captured: CapturedTensors = serde_json::from_str(fixture_data).unwrap();
    
    let token_count = captured.tokens.len();
    let embeddings_len = captured.metadata["batch_size"].as_u64().unwrap() as usize;
    
    // Test planning for the captured scenario
    let plan = QwenModel::plan_sequential_prefill_static(token_count, embeddings_len, 0);
    
    // Should plan to prefill all but last token (last token for inference)
    assert_eq!(plan.steps.len(), token_count - 1);
    assert_eq!(plan.steps[0], PrefillStep { local_idx: 0, global_pos: 0 });
    assert_eq!(plan.steps.last().unwrap().global_pos, token_count - 2);
    assert_eq!(plan.last_local_idx, token_count - 1);
}

#[test]
fn test_golden_tensor_consistency() {
    // Load the existing golden fixture and ensure it matches our new format
    let golden_fixture = include_str!("fixtures/typo_fixer_prefill_plan_golden.json");
    let golden_data: serde_json::Value = serde_json::from_str(golden_fixture).unwrap();
    
    // Verify the golden data structure
    assert!(golden_data["token_count"].is_number());
    assert!(golden_data["embeddings_len"].is_number());
    assert!(golden_data["plan"].is_object());
    
    // Test that our new tensor fixture complements the golden data
    let tensor_fixture = include_str!("fixtures/typo_fixer_tensors.json");
    let tensor_data: CapturedTensors = serde_json::from_str(tensor_fixture).unwrap();
    
    // Both fixtures should be usable together for comprehensive testing
    assert!(!tensor_data.tokens.is_empty());
    assert!(!tensor_data.embeddings.data.is_empty());
    assert!(golden_data["token_count"].as_u64().unwrap() > 0);
}