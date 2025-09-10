//! Qwen Performance Benchmark
//!
//! This example benchmarks forward_text implementations using the new
//! UnifiedModelLoader with automatic config generation.

use anyhow::Result;
use candle_coreml::UnifiedModelLoader;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🦙 Qwen Performance Benchmark");
    println!("============================");

    // Load the model with UnifiedModelLoader
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    println!("📥 Loading model with UnifiedModelLoader: {model_id}");
    println!("🤖 Automatic config generation and shape detection");

    let loader = UnifiedModelLoader::new()?;
    let mut qwen_model = loader.load_model(model_id)?;
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
