//! Integration tests for Qwen 0.6B models with ANE acceleration
//!
//! These tests validate the complete pipeline from HuggingFace model download
//! to successful inference using our CoreML inference engine.

use anyhow::Result;
use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, Config as CoreMLConfig, CoreMLModel};
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const TEST_SEQUENCE_LENGTH: usize = 64;

struct QwenTestSetup {
    model: CoreMLModel,
    tokenizer: Tokenizer,
}

impl QwenTestSetup {
    async fn new() -> Result<Self> {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        // Download model using our ensure_model_downloaded function
        let model_dir = ensure_model_downloaded(model_id, true)?;
        
        // Get paths to specific model files
        let model_path = model_dir.join("qwen_embeddings.mlmodelc");
        let tokenizer_path = model_dir.join("tokenizer.json");
        
        // Verify files exist
        if !model_path.exists() {
            return Err(anyhow::Error::msg(format!("Embeddings model not found: {}", model_path.display())));
        }
        if !tokenizer_path.exists() {
            return Err(anyhow::Error::msg(format!("Tokenizer not found: {}", tokenizer_path.display())));
        }

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| anyhow::Error::msg(format!("Failed to load tokenizer: {}", e)))?;

        // Configure and load embeddings model
        let config = CoreMLConfig {
            input_names: vec!["input_ids".to_string()],
            output_name: "hidden_states".to_string(),
            max_sequence_length: 512,
            vocab_size: QWEN_VOCAB_SIZE,
            model_type: "qwen-embeddings".to_string(),
        };

        let model = CoreMLModel::load_from_file(&model_path, &config)?;

        Ok(Self { model, tokenizer })
    }

    fn tokenize_text(&self, text: &str) -> Result<Vec<i64>> {
        let encoding = self
            .tokenizer
            .encode(text, true)
            .map_err(|e| anyhow::Error::msg(format!("Tokenization failed: {}", e)))?;
        let tokens: Vec<i64> = encoding
            .get_ids()
            .iter()
            .map(|&id| id as i64)
            .take(TEST_SEQUENCE_LENGTH)
            .collect();
        Ok(tokens)
    }

    fn create_test_tensor(&self, tokens: &[i64]) -> Result<Tensor> {
        let device = Device::Cpu;
        let mut padded_tokens = tokens.to_vec();

        // Pad to test sequence length
        while padded_tokens.len() < TEST_SEQUENCE_LENGTH {
            padded_tokens.push(0); // PAD token
        }

        Tensor::from_vec(padded_tokens, (1, TEST_SEQUENCE_LENGTH), &device).map_err(Into::into)
    }
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen model loading"]
async fn test_qwen_model_download_and_load() -> Result<()> {
    // This test verifies that we can successfully download and load a Qwen model
    let _setup = QwenTestSetup::new().await?;

    // If we get here, the model loaded successfully

    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen tokenization"]
async fn test_qwen_tokenization() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let test_text = "Hello, how are you today?";
    let tokens = setup.tokenize_text(test_text)?;

    // Verify tokenization produces reasonable output
    assert!(!tokens.is_empty(), "Tokenization should produce tokens");
    assert!(
        tokens.len() <= TEST_SEQUENCE_LENGTH,
        "Tokens should fit in sequence length"
    );

    // Verify tokens are in valid range
    for &token in &tokens {
        assert!(token >= 0, "Token IDs should be non-negative");
        assert!(
            (token as usize) < QWEN_VOCAB_SIZE,
            "Token IDs should be within vocabulary"
        );
    }

    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen stateless inference"]
async fn test_qwen_stateless_inference() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let test_text = "The weather is";
    let tokens = setup.tokenize_text(test_text)?;
    let input_tensor = setup.create_test_tensor(&tokens)?;

    // Run stateless inference
    let output = setup.model.forward(&[&input_tensor])?;

    // Verify output shape
    let output_shape = output.shape();
    assert_eq!(output_shape.dims().len(), 2, "Output should be 2D");
    assert_eq!(output_shape.dims()[0], 1, "Batch size should be 1");
    assert_eq!(
        output_shape.dims()[1],
        QWEN_VOCAB_SIZE,
        "Output should match vocab size"
    );

    // Verify output values are reasonable (logits)
    let output_data = output.to_vec2::<f32>()?;
    assert!(!output_data.is_empty(), "Output should contain data");
    assert_eq!(output_data.len(), 1, "Should have one batch");
    assert_eq!(
        output_data[0].len(),
        QWEN_VOCAB_SIZE,
        "Should have vocab size outputs"
    );

    // Check that logits are in reasonable range
    let logits = &output_data[0];
    let max_logit = logits.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let min_logit = logits.iter().fold(f32::INFINITY, |a, &b| a.min(b));

    assert!(max_logit > min_logit, "Should have variation in logits");
    assert!(max_logit < 100.0, "Logits should be in reasonable range");
    assert!(min_logit > -100.0, "Logits should be in reasonable range");

    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen stateful inference"]
async fn test_qwen_stateful_inference() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let test_text = "The capital of France is";
    let tokens = setup.tokenize_text(test_text)?;
    let input_tensor = setup.create_test_tensor(&tokens)?;

    // Create state for stateful inference
    let mut state = setup.model.make_state()?;

    // Run stateful inference
    let output = setup
        .model
        .predict_with_state(&[&input_tensor], &mut state)?;

    // Verify output (same checks as stateless)
    let output_shape = output.shape();
    assert_eq!(output_shape.dims().len(), 2, "Output should be 2D");
    assert_eq!(output_shape.dims()[0], 1, "Batch size should be 1");
    assert_eq!(
        output_shape.dims()[1],
        QWEN_VOCAB_SIZE,
        "Output should match vocab size"
    );

    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen streaming generation"]
async fn test_qwen_streaming_generation() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let test_text = "Hello";
    let initial_tokens = setup.tokenize_text(test_text)?;

    // Create state for streaming
    let mut state = setup.model.make_state()?;
    let mut generated_tokens = initial_tokens.clone();

    // Generate a few tokens
    for step in 0..3 {
        let input_len = if step == 0 { initial_tokens.len() } else { 1 };
        let start_idx = generated_tokens.len() - input_len;
        let input_tokens = &generated_tokens[start_idx..];

        let input_tensor = setup.create_test_tensor(input_tokens)?;
        let output = setup
            .model
            .predict_with_state(&[&input_tensor], &mut state)?;

        // Get next token (greedy sampling for test)
        let output_data = output.to_vec2::<f32>()?;
        let logits = &output_data[0];
        let next_token = logits
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(idx, _)| idx as i64)
            .unwrap_or(0);

        generated_tokens.push(next_token);

        // Verify token is in valid range
        assert!(next_token >= 0, "Generated token should be non-negative");
        assert!(
            (next_token as usize) < QWEN_VOCAB_SIZE,
            "Generated token should be within vocabulary"
        );
    }

    // Verify we generated new tokens
    assert!(
        generated_tokens.len() > initial_tokens.len(),
        "Should have generated new tokens"
    );

    Ok(())
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "downloads large model - run manually to verify Qwen performance baseline"]
async fn test_qwen_performance_baseline() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let test_text = "Performance test input";
    let tokens = setup.tokenize_text(test_text)?;
    let input_tensor = setup.create_test_tensor(&tokens)?;

    // Time inference
    let start_time = std::time::Instant::now();
    let _output = setup.model.forward(&[&input_tensor])?;
    let inference_time = start_time.elapsed();

    // Verify inference completes reasonably quickly (under 1 second for test)
    assert!(
        inference_time.as_millis() < 1000,
        "Inference should complete within 1 second, took: {:?}",
        inference_time
    );

    println!("Inference time: {:?}", inference_time);

    Ok(())
}

#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_macos_requirement() {
    // On non-macOS platforms, verify that appropriate errors are returned
    use candle_coreml::CoreMLModel;
    use std::path::PathBuf;

    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string()],
        output_name: "output".to_string(),
        max_sequence_length: 512,
        vocab_size: QWEN_VOCAB_SIZE,
        model_type: "qwen-test".to_string(),
    };

    let model_path = PathBuf::from("nonexistent.mlmodelc");
    let result = CoreMLModel::load_from_file(&model_path, &config);

    // Should fail on non-macOS platforms
    assert!(
        result.is_err(),
        "CoreML should not be available on non-macOS platforms"
    );
}

// Helper function for manual testing (not run automatically)
#[allow(dead_code)]
async fn manual_qwen_chat_test() -> Result<()> {
    let setup = QwenTestSetup::new().await?;

    let prompts = vec![
        "Hello, how are you?",
        "What is the capital of France?",
        "Explain artificial intelligence in simple terms.",
    ];

    for prompt in prompts {
        println!("\nPrompt: {}", prompt);

        let tokens = setup.tokenize_text(prompt)?;
        let input_tensor = setup.create_test_tensor(&tokens)?;

        let start_time = std::time::Instant::now();
        let output = setup.model.forward(&[&input_tensor])?;
        let inference_time = start_time.elapsed();

        println!("Inference time: {:?}", inference_time);
        println!("Output shape: {:?}", output.shape());

        // Could add more detailed output processing here
    }

    Ok(())
}
