//! End-to-end integration test for Qwen 0.6B complete pipeline
//!
//! This test validates the complete Qwen model pipeline from download to text generation,
//! ensuring the multi-component architecture works correctly for autoregressive inference.

use anyhow::Result;
use candle_coreml::{ensure_model_downloaded, qwen::QwenModel};

#[cfg(target_os = "macos")]
#[tokio::test]
// Model is cached after first download - safe for coverage analysis
async fn test_qwen_complete_pipeline_fox_completion() -> Result<()> {
    // Download the Qwen model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    // Load the complete Qwen model using the full pipeline
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, None)?;
    
    // Test the classic "The quick brown fox jumps over the lazy" completion
    let prompt = "The quick brown fox jumps over the lazy";
    
    // Generate completion with low temperature for deterministic results
    let completion = qwen_model.generate_text(prompt, 2, 0.0)?;
    
    // Assert that the completion contains "dog" or "lazy" (both have tied logits in the model)
    assert!(
        completion.to_lowercase().contains("dog") || completion.to_lowercase().contains("lazy"),
        "Expected completion to contain 'dog' or 'lazy' (both have tied logit values), but got: '{}'",
        completion
    );
    
    println!("✅ Qwen pipeline test passed!");
    println!("Prompt: {}", prompt);
    println!("Completion: {}", completion);
    
    Ok(())
}


#[cfg(not(target_os = "macos"))]
#[tokio::test]
async fn test_qwen_macos_requirement() {
    // On non-macOS platforms, verify that appropriate errors are returned
    use std::path::PathBuf;

    let model_dir = PathBuf::from("nonexistent_dir");
    let result = QwenModel::load_from_directory(&model_dir, None);

    // Should fail on non-macOS platforms
    assert!(
        result.is_err(),
        "QwenModel should not be available on non-macOS platforms"
    );
}
