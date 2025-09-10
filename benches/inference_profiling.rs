//! Inference Pipeline Profiling
//!
//! This benchmark focuses on the actual inference pipeline bottlenecks

#![allow(unused_variables)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::get_first)]

use candle_coreml::qwen::QwenModel;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::{Duration, Instant};

const TEST_PROMPT: &str = "The quick brown fox jumps over the lazy";
const EXPECTED_TOKEN: i64 = 5562;

fn get_qwen_model() -> Option<QwenModel> {
    #[cfg(target_os = "macos")]
    {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        if let Ok(model_dir) = candle_coreml::ensure_model_downloaded(model_id, false) {
            if let Ok(model) = QwenModel::load_from_directory(&model_dir, None) {
                return Some(model);
            }
        }
    }

    None
}

fn bench_tokenization_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenization");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    if let Some(model) = get_qwen_model() {
        group.bench_function("tokenize_prompt", |b| {
            b.iter(|| {
                let tokens = model.tokenize(black_box(TEST_PROMPT));
                black_box(tokens)
            })
        });

        let long_prompt = "The quick brown fox jumps over the lazy dog and runs through the forest, chasing rabbits and birds while the sun shines brightly overhead, casting long shadows across the meadow where wildflowers bloom in vibrant colors during the warm summer afternoon.";
        group.bench_function("tokenize_long_prompt", |b| {
            b.iter(|| {
                let tokens = model.tokenize(black_box(long_prompt));
                black_box(tokens)
            })
        });
    }

    group.finish();
}

fn bench_single_token_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_token");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(2));
    group.measurement_time(Duration::from_secs(10));

    if let Some(mut model) = get_qwen_model() {
        // Test the first token generation (includes prefill)
        group.bench_function("first_token_generation", |b| {
            b.iter(|| {
                let start = Instant::now();
                let result = model.generate_tokens_topk_temp(
                    black_box(TEST_PROMPT),
                    black_box(1),
                    black_box(1.0),
                    black_box(None),
                );
                let duration = start.elapsed();

                if let Ok(tokens) = &result {
                    eprintln!(
                        "First token took: {:.2}ms, got token: {:?}",
                        duration.as_millis(),
                        tokens.first()
                    );
                }

                black_box(result)
            })
        });

        // Test subsequent token generation (should use cache)
        group.bench_function("subsequent_token_batch", |b| {
            b.iter(|| {
                let start = Instant::now();
                let result = model.generate_tokens_topk_temp(
                    black_box(TEST_PROMPT),
                    black_box(5),
                    black_box(1.0),
                    black_box(None),
                );
                let duration = start.elapsed();

                if let Ok(tokens) = &result {
                    let tps = 5.0 / duration.as_secs_f64();
                    eprintln!(
                        "5 tokens took: {:.2}ms ({:.1} t/s)",
                        duration.as_millis(),
                        tps
                    );
                }

                black_box(result)
            })
        });
    }

    group.finish();
}

fn bench_pipeline_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline_components");
    group.sample_size(15);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(8));

    if let Some(model) = get_qwen_model() {
        if let Ok(tokens) = model.tokenize(TEST_PROMPT) {
            let device = candle_core::Device::Cpu;
            let input_tensor =
                candle_core::Tensor::from_vec(tokens.clone(), (1, tokens.len()), &device).unwrap();

            // Benchmark embeddings
            group.bench_function("embeddings_forward", |b| {
                b.iter(|| {
                    let start = Instant::now();
                    let result = model.embeddings.forward(&[black_box(&input_tensor)]);
                    let duration = start.elapsed();

                    if result.is_ok() {
                        eprintln!("Embeddings took: {:.2}ms", duration.as_millis());
                    }

                    black_box(result)
                })
            });

            // Test single token for inference phase
            let single_token =
                candle_core::Tensor::from_vec(vec![tokens[0]], (1, 1), &device).unwrap();

            group.bench_function("embeddings_single_token", |b| {
                b.iter(|| {
                    let start = Instant::now();
                    let result = model.embeddings.forward(&[black_box(&single_token)]);
                    let duration = start.elapsed();

                    if result.is_ok() {
                        eprintln!("Single token embeddings: {:.2}ms", duration.as_millis());
                    }

                    black_box(result)
                })
            });
        }
    }

    group.finish();
}

fn bench_end_to_end_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    group.sample_size(5);
    group.warm_up_time(Duration::from_secs(3));
    group.measurement_time(Duration::from_secs(15));

    if let Some(mut model) = get_qwen_model() {
        // Python baseline comparison
        const PYTHON_BASELINE_TPS: f64 = 87.0;

        group.bench_function("vs_python_25_tokens", |b| {
            b.iter(|| {
                let start = Instant::now();
                let result = model.generate_tokens_topk_temp(
                    black_box(TEST_PROMPT),
                    black_box(25),
                    black_box(1.0),
                    black_box(None),
                );
                let total_duration = start.elapsed();

                if let Ok(tokens) = &result {
                    let rust_tps = 25.0 / total_duration.as_secs_f64();
                    let efficiency = (rust_tps / PYTHON_BASELINE_TPS) * 100.0;

                    eprintln!(
                        "🦀 Rust: {rust_tps:.1} t/s | 🐍 Python: {PYTHON_BASELINE_TPS:.1} t/s | Efficiency: {efficiency:.1}%"
                    );

                    // Check if we got the expected token
                    if tokens.len() >= 2 && tokens[1] == EXPECTED_TOKEN {
                        eprintln!("✅ Got expected 'dog' token at position 1");
                    } else {
                        eprintln!(
                            "❌ Expected token {} not found at position 1, got: {:?}",
                            EXPECTED_TOKEN,
                            tokens.get(1)
                        );
                    }
                }

                black_box(result)
            })
        });

        // Test different batch sizes to see scaling
        for &num_tokens in &[1, 5, 10, 25] {
            group.bench_function(&format!("generate_{num_tokens}_tokens"), |b| {
                b.iter(|| {
                    let start = Instant::now();
                    let result = model.generate_tokens_topk_temp(
                        black_box(TEST_PROMPT),
                        black_box(num_tokens),
                        black_box(1.0),
                        black_box(None),
                    );
                    let duration = start.elapsed();

                    if let Ok(_tokens) = &result {
                        let tps = num_tokens as f64 / duration.as_secs_f64();
                        eprintln!(
                            "{} tokens: {:.1} t/s ({:.0}ms total)",
                            num_tokens,
                            tps,
                            duration.as_millis()
                        );
                    }

                    black_box(result)
                })
            });
        }
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = {
        Criterion::default()
            .warm_up_time(Duration::from_secs(2))
            .measurement_time(Duration::from_secs(10))
            .sample_size(10)
    };
    targets =
        bench_tokenization_speed,
        bench_single_token_generation,
        bench_pipeline_components,
        bench_end_to_end_comparison
);

criterion_main!(benches);
