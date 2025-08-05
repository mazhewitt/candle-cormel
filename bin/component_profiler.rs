//! Component-Level Performance Profiler
//!
//! Profiles each component of the Qwen pipeline individually

use candle_core::{Device, Tensor};
use candle_coreml::qwen::QwenModel;
use std::time::Instant;

const TEST_PROMPT: &str = "The quick brown fox jumps over the lazy";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Component-Level Performance Profiler");
    println!("=======================================");

    #[cfg(target_os = "macos")]
    {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        // Load model
        println!("📦 Loading model...");
        let model_dir = candle_coreml::ensure_model_downloaded(model_id, false)?;
        let mut model = QwenModel::load_from_directory(&model_dir, None)?;
        println!("✅ Model loaded");

        // Prepare test data
        let device = Device::Cpu;
        let tokens = model.tokenize(TEST_PROMPT)?;
        println!("🔤 Test tokens: {:?} (length: {})", tokens, tokens.len());

        // Warm up
        println!("\n🔥 Warming up...");
        for _ in 0..3 {
            let _ = model.generate_tokens(TEST_PROMPT, 1, 1.0, None);
        }

        println!("\n⏱️  Individual Component Profiling:");
        println!("{}", "=".repeat(60));

        // Test full sequence (prefill phase)
        let full_input = Tensor::from_vec(tokens.clone(), (1, tokens.len()), &device)?;

        // 1. Embeddings - full sequence
        let iterations = 100;
        let start = Instant::now();
        let mut embeddings_output = None;
        for _ in 0..iterations {
            let result = model.embeddings.forward(&[&full_input])?;
            embeddings_output = Some(result);
        }
        let embeddings_full_avg = start.elapsed().as_micros() as f64 / iterations as f64;
        println!(
            "🎯 Embeddings (full seq, {} tokens): {:.1}µs",
            tokens.len(),
            embeddings_full_avg
        );

        // 2. FFN - using embeddings output
        if let Some(ref emb_output) = embeddings_output {
            let iterations = 10;
            let start = Instant::now();
            let mut ffn_output = None;
            for _ in 0..iterations {
                let result = model.ffn_prefill.forward(&[emb_output])?;
                ffn_output = Some(result);
            }
            let ffn_full_avg = start.elapsed().as_millis() as f64 / iterations as f64;
            println!(
                "🧠 FFN (full seq, {} tokens): {:.1}ms",
                tokens.len(),
                ffn_full_avg
            );

            // 3. LM Head - using FFN output
            if let Some(ref ffn_out) = ffn_output {
                let iterations = 10;
                let start = Instant::now();
                for _ in 0..iterations {
                    let _result = model.lm_head.forward(&[ffn_out])?;
                }
                let lm_head_full_avg = start.elapsed().as_millis() as f64 / iterations as f64;
                println!(
                    "📝 LM Head (full seq, {} tokens): {:.1}ms",
                    tokens.len(),
                    lm_head_full_avg
                );
            }
        }

        println!("\n⏱️  Single Token Component Profiling (Autoregressive):");
        println!("{}", "=".repeat(60));

        // Test single token (inference phase)
        let single_input = Tensor::from_vec(vec![tokens[0]], (1, 1), &device)?;

        // 1. Embeddings - single token
        let iterations = 100;
        let start = Instant::now();
        let mut single_embeddings_output = None;
        for _ in 0..iterations {
            let result = model.embeddings.forward(&[&single_input])?;
            single_embeddings_output = Some(result);
        }
        let embeddings_single_avg = start.elapsed().as_micros() as f64 / iterations as f64;
        println!(
            "🎯 Embeddings (single token): {:.1}µs",
            embeddings_single_avg
        );

        // 2. FFN - single token
        if let Some(ref single_emb_output) = single_embeddings_output {
            let iterations = 10;
            let start = Instant::now();
            let mut single_ffn_output = None;
            for _ in 0..iterations {
                let result = model.ffn_infer.forward(&[single_emb_output])?;
                single_ffn_output = Some(result);
            }
            let ffn_single_avg = start.elapsed().as_millis() as f64 / iterations as f64;
            println!("🧠 FFN (single token): {:.1}ms", ffn_single_avg);

            // 3. LM Head - single token
            if let Some(ref single_ffn_out) = single_ffn_output {
                let iterations = 10;
                let start = Instant::now();
                for _ in 0..iterations {
                    let _result = model.lm_head.forward(&[single_ffn_out])?;
                }
                let lm_head_single_avg = start.elapsed().as_millis() as f64 / iterations as f64;
                println!("📝 LM Head (single token): {:.1}ms", lm_head_single_avg);

                // Total single token time
                let total_single =
                    (embeddings_single_avg / 1000.0) + ffn_single_avg + lm_head_single_avg;
                println!("⚡ Total single token (computed): {:.1}ms", total_single);
            }
        }

        println!("\n📊 Performance Analysis:");
        println!("{}", "=".repeat(60));

        // Compare efficiency ratios
        println!("🔍 Efficiency Analysis:");
        println!(
            "• Embeddings scaling: {:.1}x more efficient for full sequence",
            (embeddings_single_avg * tokens.len() as f64) / embeddings_full_avg
        );

        // Test tensor conversion overhead specifically
        println!("\n🔄 Tensor Conversion Overhead Analysis:");
        println!("{}", "=".repeat(60));

        // Test MLMultiArray conversion overhead
        let iterations = 1000;
        let start = Instant::now();
        for _ in 0..iterations {
            #[cfg(target_os = "macos")]
            {
                let _ml_array = candle_coreml::conversion::tensor_to_mlmultiarray(&single_input);
            }
        }
        let conversion_overhead = start.elapsed().as_nanos() as f64 / iterations as f64;
        println!(
            "🔄 Tensor→MLMultiArray conversion: {:.1}ns per call",
            conversion_overhead
        );

        // Test feature provider creation
        let iterations = 1000;
        let start = Instant::now();
        #[cfg(target_os = "macos")]
        {
            let input_names = vec!["input_ids".to_string()];

            for _ in 0..iterations {
                let ml_array = candle_coreml::conversion::tensor_to_mlmultiarray(&single_input)?;
                let _provider = candle_coreml::conversion::create_multi_feature_provider(
                    &input_names,
                    &[ml_array],
                );
            }
        }
        let provider_overhead = start.elapsed().as_nanos() as f64 / iterations as f64;
        println!(
            "🎭 Feature provider creation: {:.1}ns per call",
            provider_overhead
        );

        println!("\n🎯 Bottleneck Conclusions:");
        println!("{}", "=".repeat(60));

        println!(
            "• Tensor conversions are very fast (~{:.1}µs total)",
            (conversion_overhead + provider_overhead) / 1000.0
        );
        println!("• Main bottlenecks are likely FFN and LM Head CoreML model calls");
        println!("• The issue is not Rust bindings but CoreML model inference speed");

        println!("\n💡 Optimization Recommendations:");
        println!("1. Profile CoreML model compilation settings");
        println!("2. Check if models are using ANE vs CPU/GPU");
        println!("3. Investigate model quantization options");
        println!("4. Consider caching strategies for repeated patterns");
    }

    #[cfg(not(target_os = "macos"))]
    {
        println!("❌ This profiler requires macOS for CoreML support");
    }

    Ok(())
}
