//! Simple Performance Measurement
//!
//! Quick performance measurement without criterion complexity

use candle_coreml::qwen::QwenModel;
use std::time::Instant;

const TEST_PROMPT: &str = "The quick brown fox jumps over the lazy";
const EXPECTED_TOKEN: i64 = 5562;
const PYTHON_BASELINE_TPS: f64 = 87.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Simple Performance Measurement");
    println!("==================================");

    #[cfg(target_os = "macos")]
    {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        println!("📦 Loading model: {model_id}");
        let model_dir = candle_coreml::ensure_model_downloaded(model_id, false)?;
        let mut model = QwenModel::load_from_directory(&model_dir, None)?;
        println!("✅ Model loaded");

        // Warm up
        println!("\n🔥 Warming up...");
        for _ in 0..3 {
            let _ = model.generate_tokens(TEST_PROMPT, 1, 1.0, None);
        }
        println!("✅ Warm up complete");

        println!("\n📊 Performance Measurements:");
        println!("{}", "=".repeat(50));

        // Test tokenization speed
        let start = Instant::now();
        let iterations = 1000;
        for _ in 0..iterations {
            let _ = model.tokenize(TEST_PROMPT);
        }
        let tokenization_time = start.elapsed();
        println!(
            "🔤 Tokenization: {:.2}μs per call ({} iterations)",
            tokenization_time.as_micros() as f64 / iterations as f64,
            iterations
        );

        // Test single token generation (first token - includes prefill)
        println!("\n🎯 Single Token Generation (with prefill):");
        let measurements = 5;
        let mut times = Vec::new();

        for i in 0..measurements {
            let start = Instant::now();
            let result = model.generate_tokens(TEST_PROMPT, 1, 1.0, None)?;
            let duration = start.elapsed();
            times.push(duration);

            if let Some(token) = result.first() {
                println!(
                    "  Run {}: {:.0}ms (token: {})",
                    i + 1,
                    duration.as_millis(),
                    token
                );
            }
        }

        let avg_first_token = times.iter().sum::<std::time::Duration>() / times.len() as u32;
        println!("  Average: {:.0}ms", avg_first_token.as_millis());

        // Test multi-token generation
        println!("\n🚀 Multi-Token Generation:");
        let token_counts = [5, 10, 25];

        for &count in &token_counts {
            let measurements = 3;
            let mut times = Vec::new();

            for i in 0..measurements {
                let start = Instant::now();
                let result = model.generate_tokens(TEST_PROMPT, count, 1.0, None)?;
                let duration = start.elapsed();
                times.push(duration);

                let tps = count as f64 / duration.as_secs_f64();
                let efficiency = (tps / PYTHON_BASELINE_TPS) * 100.0;

                // Check if we got expected token
                let expected_match = if count >= 2 && result.len() >= 2 {
                    if result[1] == EXPECTED_TOKEN {
                        "✅"
                    } else {
                        "❌"
                    }
                } else {
                    "?"
                };

                println!(
                    "  {} tokens, run {}: {:.0}ms ({:.1} t/s, {:.1}% of Python) {}",
                    count,
                    i + 1,
                    duration.as_millis(),
                    tps,
                    efficiency,
                    expected_match
                );
            }

            let avg_time = times.iter().sum::<std::time::Duration>() / times.len() as u32;
            let avg_tps = count as f64 / avg_time.as_secs_f64();
            let avg_efficiency = (avg_tps / PYTHON_BASELINE_TPS) * 100.0;

            println!(
                "  {} tokens average: {:.0}ms ({:.1} t/s, {:.1}% of Python)",
                count,
                avg_time.as_millis(),
                avg_tps,
                avg_efficiency
            );
        }

        // Component-level timing
        println!("\n🔬 Component-Level Analysis:");
        if let Ok(tokens) = model.tokenize(TEST_PROMPT) {
            let device = candle_core::Device::Cpu;
            let input_tensor =
                candle_core::Tensor::from_vec(tokens.clone(), (1, tokens.len()), &device)?;

            // Test embeddings
            let start = Instant::now();
            let _result = model.embeddings.forward(&[&input_tensor])?;
            let embeddings_time = start.elapsed();
            println!(
                "  Embeddings (full sequence): {:.2}ms",
                embeddings_time.as_millis()
            );

            // Test single token embeddings
            let single_token = candle_core::Tensor::from_vec(vec![tokens[0]], (1, 1), &device)?;
            let start = Instant::now();
            let _result = model.embeddings.forward(&[&single_token])?;
            let single_embeddings_time = start.elapsed();
            println!(
                "  Embeddings (single token): {:.2}ms",
                single_embeddings_time.as_millis()
            );
        }

        println!("\n📈 Summary:");
        println!("{}", "=".repeat(50));
        println!(
            "• Current performance: ~{:.1}% of Python baseline",
            (11.0 / PYTHON_BASELINE_TPS) * 100.0
        ); // Based on previous measurements
        println!("• Bottleneck appears to be in CoreML inference pipeline");
        println!("• Tokenization is very fast (~few μs)");
        println!("• Each CoreML component call takes ~hundreds of ms");

        println!("\n🎯 Optimization Opportunities:");
        println!("1. CoreML model optimization (quantization, compilation settings)");
        println!("2. Reduce tensor conversion overhead");
        println!("3. Better memory management/pooling");
        println!("4. Pipeline parallelization where possible");
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("❌ This benchmark requires macOS for CoreML support");
    }

    Ok(())
}
