//! Qwen Performance Benchmark
//!
//! This example benchmarks the old vs optimized forward_text implementations
//! to demonstrate the performance improvements from architectural fixes.

use anyhow::Result;
use candle_coreml::{
    ensure_model_downloaded,
    qwen::{QwenConfig, QwenModel},
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦙 Qwen Performance Benchmark");
    println!("============================");

    // Load the model
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    println!("📥 Loading model: {model_id}");
    let model_dir = ensure_model_downloaded(model_id, true)?;

    let config = QwenConfig::default();
    let mut qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    println!("✅ Model loaded successfully");

    // Benchmark prompt
    let test_prompt = "The quick brown fox jumps over the lazy";
    let iterations = 3; // Small number for quick testing

    println!("\n🏁 Running performance benchmark...");
    println!("Prompt: '{test_prompt}'");
    println!("Iterations: {iterations}");

    // Run the benchmark
    qwen_model.benchmark_implementations(test_prompt, iterations)?;

    println!("\n✅ Benchmark complete!");
    Ok(())
}
