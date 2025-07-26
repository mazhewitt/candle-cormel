//! Integration tests for multi-component Qwen architecture
//!
//! These tests validate the complete pipeline for Anemll's multi-component approach:
//! - Embeddings model loading and inference
//! - FFN model processing with causal masking
//! - LM head model for token prediction
//! - Component orchestration and data flow

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel, download_model};
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const HIDDEN_SIZE: usize = 896;
const TEST_SEQUENCE_LENGTH: usize = 8;

struct MultiComponentTestSetup {
    embeddings: CoreMLModel,
    ffn: CoreMLModel,
    lm_head: CoreMLModel,
    tokenizer: Tokenizer,
    device: Device,
}

impl MultiComponentTestSetup {
    async fn new() -> Result<Self> {
        let device = Device::Cpu;
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        
        // Download the complete model using the clean git2+LFS approach
        println!("📥 Downloading multi-component Qwen model with clean strategy...");
        let cache_dir = download_model(model_id, true)?;
        
        // Set up component paths
        let tokenizer_path = cache_dir.join("tokenizer.json");
        let embeddings_path = cache_dir.join("qwen_embeddings.mlmodelc");
        let ffn_path = cache_dir.join("qwen_FFN_PF_lut6_chunk_01of01.mlmodelc");
        let lm_head_path = cache_dir.join("qwen_lm_head_lut6.mlmodelc");

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| anyhow::Error::msg(format!("Failed to load tokenizer: {}", e)))?;

        // Configure and load embeddings model
        let embeddings_config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "embeddings".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings-test".to_string(),
        };

        let embeddings = CoreMLModel::load_from_file(&embeddings_path, &embeddings_config)?;

        // Configure and load FFN model
        let ffn_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string(), "causal_mask".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: HIDDEN_SIZE,
            model_type: "qwen-ffn-test".to_string(),
        };

        let ffn = CoreMLModel::load_from_file(&ffn_path, &ffn_config)?;

        // Configure and load LM head model
        let lm_head_config = CoreMLConfig {
            input_names: vec!["hidden_states".to_string()],
            output_name: "logits".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-lm-head-test".to_string(),
        };

        let lm_head = CoreMLModel::load_from_file(&lm_head_path, &lm_head_config)?;

        Ok(Self {
            embeddings,
            ffn,
            lm_head,
            tokenizer,
            device,
        })
    }

    fn tokenize_text(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self.tokenizer.encode(text, true)
            .map_err(|e| anyhow::Error::msg(format!("Tokenization failed: {}", e)))?;
        let tokens: Vec<i64> = encoding
            .get_ids()
            .iter()
            .map(|&id| id as i64)
            .take(TEST_SEQUENCE_LENGTH)
            .collect();
        Ok(tokens)
    }

    fn create_causal_mask(&self, seq_len: usize) -> Result<Tensor> {
        let mut mask_data = vec![0.0f32; seq_len * seq_len];
        
        // Fill upper triangle with -inf for causal masking
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask_data[i * seq_len + j] = f32::NEG_INFINITY;
            }
        }

        Tensor::from_vec(
            mask_data,
            (seq_len, seq_len),
            &self.device,
        ).map_err(Into::into)
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_multi_component_model_loading() -> Result<()> {
    // Test that all model components can be loaded successfully
    let _setup = MultiComponentTestSetup::new().await?;
    
    // If we get here, all models loaded successfully
    assert!(true, "All model components loaded successfully");
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_embeddings_component() -> Result<()> {
    let setup = MultiComponentTestSetup::new().await?;
    
    let test_text = "Hello world";
    let tokens = setup.tokenize_text(test_text)?;
    
    // Create input tensor
    let input_tensor = Tensor::from_vec(
        tokens.clone(),
        (1, tokens.len()),
        &setup.device,
    )?;

    // Run embeddings model
    let embeddings = setup.embeddings.forward(&[&input_tensor])?;
    
    // Verify output shape: [batch, seq_len, hidden_size]
    let shape = embeddings.shape();
    assert_eq!(shape.dims().len(), 3, "Embeddings should be 3D");
    assert_eq!(shape.dims()[0], 1, "Batch size should be 1");
    assert_eq!(shape.dims()[1], tokens.len(), "Sequence length should match input");
    assert_eq!(shape.dims()[2], HIDDEN_SIZE, "Hidden size should match expected");
    
    println!("✅ Embeddings: {:?} -> {:?}", input_tensor.shape(), shape);
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_ffn_component() -> Result<()> {
    let setup = MultiComponentTestSetup::new().await?;
    
    let test_text = "Test FFN";
    let tokens = setup.tokenize_text(test_text)?;
    
    // Get embeddings first
    let input_tensor = Tensor::from_vec(
        tokens.clone(),
        (1, tokens.len()),
        &setup.device,
    )?;
    let embeddings = setup.embeddings.forward(&[&input_tensor])?;
    
    // Create causal mask
    let causal_mask = setup.create_causal_mask(tokens.len())?;
    
    // Run FFN model
    let hidden_states = setup.ffn.forward(&[&embeddings, &causal_mask])?;
    
    // Verify output shape matches input embeddings shape
    let input_shape = embeddings.shape();
    let output_shape = hidden_states.shape();
    
    assert_eq!(input_shape.dims(), output_shape.dims(), 
        "FFN output shape should match input shape");
    
    println!("✅ FFN: {:?} -> {:?}", input_shape, output_shape);
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_lm_head_component() -> Result<()> {
    let setup = MultiComponentTestSetup::new().await?;
    
    let test_text = "Test LM head";
    let tokens = setup.tokenize_text(test_text)?;
    
    // Get hidden states from embeddings and FFN
    let input_tensor = Tensor::from_vec(
        tokens.clone(),
        (1, tokens.len()),
        &setup.device,
    )?;
    let embeddings = setup.embeddings.forward(&[&input_tensor])?;
    
    let causal_mask = setup.create_causal_mask(tokens.len())?;
    let hidden_states = setup.ffn.forward(&[&embeddings, &causal_mask])?;
    
    // Extract last position for LM head
    let seq_len = tokens.len();
    let last_hidden = hidden_states.narrow(1, seq_len - 1, 1)?;
    
    // Run LM head model
    let logits = setup.lm_head.forward(&[&last_hidden])?;
    
    // Verify output shape: [batch, 1, vocab_size]
    let shape = logits.shape();
    assert_eq!(shape.dims().len(), 3, "Logits should be 3D");
    assert_eq!(shape.dims()[0], 1, "Batch size should be 1");
    assert_eq!(shape.dims()[1], 1, "Should have single position");
    assert_eq!(shape.dims()[2], QWEN_VOCAB_SIZE, "Vocab size should match");
    
    println!("✅ LM Head: {:?} -> {:?}", last_hidden.shape(), shape);
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_full_pipeline() -> Result<()> {
    let setup = MultiComponentTestSetup::new().await?;
    
    let test_text = "The capital of France is";
    let tokens = setup.tokenize_text(test_text)?;
    
    println!("🧪 Testing full pipeline with: \"{}\"", test_text);
    println!("📝 Tokens: {:?}", tokens);
    
    // Step 1: Embeddings
    let input_tensor = Tensor::from_vec(
        tokens.clone(),
        (1, tokens.len()),
        &setup.device,
    )?;
    
    let start_time = std::time::Instant::now();
    let embeddings = setup.embeddings.forward(&[&input_tensor])?;
    let embeddings_time = start_time.elapsed();
    
    // Step 2: FFN with causal mask
    let causal_mask = setup.create_causal_mask(tokens.len())?;
    
    let ffn_start = std::time::Instant::now();
    let hidden_states = setup.ffn.forward(&[&embeddings, &causal_mask])?;
    let ffn_time = ffn_start.elapsed();
    
    // Step 3: LM Head for next token prediction
    let seq_len = tokens.len();
    let last_hidden = hidden_states.narrow(1, seq_len - 1, 1)?;
    
    let lm_head_start = std::time::Instant::now();
    let logits = setup.lm_head.forward(&[&last_hidden])?;
    let lm_head_time = lm_head_start.elapsed();
    
    // Verify we got valid logits
    let logits_data = logits.to_vec3::<f32>()?;
    assert!(!logits_data.is_empty(), "Should have logits data");
    assert_eq!(logits_data[0][0].len(), QWEN_VOCAB_SIZE, "Should have full vocab logits");
    
    // Find most likely next token
    let logits_vec = &logits_data[0][0];
    let (best_idx, best_score) = logits_vec
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap();
    
    println!("✅ Pipeline completed successfully!");
    println!("⏱️  Timing breakdown:");
    println!("   • Embeddings: {:?}", embeddings_time);
    println!("   • FFN: {:?}", ffn_time);
    println!("   • LM Head: {:?}", lm_head_time);
    println!("   • Total: {:?}", embeddings_time + ffn_time + lm_head_time);
    println!("🎯 Next token prediction: {} (score: {:.4})", best_idx, best_score);
    
    // Performance check: should complete within reasonable time
    let total_time = embeddings_time + ffn_time + lm_head_time;
    assert!(total_time.as_millis() < 5000, 
        "Pipeline should complete within 5 seconds, took: {:?}", total_time);
    
    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
async fn test_causal_mask_creation() -> Result<()> {
    let setup = MultiComponentTestSetup::new().await?;
    
    let seq_len = 4;
    let mask = setup.create_causal_mask(seq_len)?;
    
    // Verify mask shape
    let shape = mask.shape();
    assert_eq!(shape.dims(), &[seq_len, seq_len], "Mask should be square");
    
    // Verify mask values
    let mask_data = mask.to_vec2::<f32>()?;
    
    // Check diagonal and lower triangle are 0.0
    for i in 0..seq_len {
        for j in 0..=i {
            assert_eq!(mask_data[i][j], 0.0, 
                "Lower triangle and diagonal should be 0.0 at ({}, {})", i, j);
        }
    }
    
    // Check upper triangle is -inf
    for i in 0..seq_len {
        for j in (i + 1)..seq_len {
            assert_eq!(mask_data[i][j], f32::NEG_INFINITY, 
                "Upper triangle should be -inf at ({}, {})", i, j);
        }
    }
    
    println!("✅ Causal mask created correctly:");
    for row in &mask_data {
        println!("   {:?}", row);
    }
    
    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_multi_component_macos_requirement() {
    // On non-macOS platforms, verify that appropriate errors are returned
    use candle_coreml::CoreMLModel;
    use std::path::PathBuf;
    
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string()],
        output_name: "embeddings".to_string(),
        max_sequence_length: 512,
        vocab_size: QWEN_VOCAB_SIZE,
        model_type: "qwen-test".to_string(),
    };
    
    let model_path = PathBuf::from("nonexistent.mlmodelc");
    let result = CoreMLModel::load_from_file(&model_path, &config);
    
    // Should fail on non-macOS platforms
    assert!(result.is_err(), "CoreML should not be available on non-macOS platforms");
}