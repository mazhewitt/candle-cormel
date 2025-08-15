//! Comprehensive Qwen Inference Benchmarks
//!
//! This benchmark suite breaks down Qwen inference into individual components
//! to identify performance bottlenecks and compare against the Python baseline.

#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

use candle_core::{Device, Tensor};
use candle_coreml::{ensure_model_downloaded, qwen::QwenModel};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

const BASELINE_PYTHON_TPS: f64 = 87.0; // tokens/second from chat.py

// Test prompt that should generate "dog" token
const TEST_PROMPT: &str = "The quick brown fox jumps over the lazy";
const EXPECTED_TOKEN: i64 = 5562; // "dog" token ID

fn get_qwen_model() -> Option<(QwenModel, Device)> {
    // Only run on macOS with the model available
    #[cfg(target_os = "macos")]
    {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        if let Ok(model_dir) = ensure_model_downloaded(model_id, false) {
            let device = Device::Cpu;
            if let Ok(model) = QwenModel::load_from_directory(&model_dir, None) {
                return Some((model, device));
            }
        }
    }

    None
}

fn bench_model_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("model_loading");
    group.sample_size(10); // Fewer samples for expensive operations

    #[cfg(target_os = "macos")]
    {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        if let Ok(model_dir) = ensure_model_downloaded(model_id, false) {
            let device = Device::Cpu;

            group.bench_function("load_qwen_model", |b| {
                b.iter(|| {
                    let model = QwenModel::load_from_directory(black_box(&model_dir), None);
                    black_box(model)
                })
            });
        }
    }

    group.finish();
}

fn bench_tokenization(c: &mut Criterion) {
    let mut group = c.benchmark_group("tokenization");

    if let Some((mut model, _device)) = get_qwen_model() {
        let test_prompts = [
            "Hello world",
            TEST_PROMPT,
            "What is the capital of France?",
            "Tell me about machine learning and artificial intelligence systems.",
        ];

        for prompt in &test_prompts {
            group.throughput(Throughput::Bytes(prompt.len() as u64));
            group.bench_with_input(
                BenchmarkId::new("tokenize", prompt.len()),
                prompt,
                |b, prompt| {
                    b.iter(|| {
                        let tokens = model.tokenize(black_box(prompt));
                        black_box(tokens)
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_tensor_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("tensor_operations");

    if let Some((_model, device)) = get_qwen_model() {
        // Test different tensor sizes
        let sizes = [(1, 10), (1, 50), (1, 100), (1, 500)];

        for (batch, seq_len) in sizes {
            let shape = (batch, seq_len);

            group.throughput(Throughput::Elements((batch * seq_len) as u64));

            // Benchmark tensor creation
            group.bench_with_input(
                BenchmarkId::new("tensor_creation_i64", format!("{batch}x{seq_len}")),
                &shape,
                |b, &(batch, seq_len)| {
                    b.iter(|| {
                        let tensor = Tensor::zeros(
                            (batch, seq_len),
                            candle_core::DType::I64,
                            black_box(&device),
                        );
                        black_box(tensor)
                    })
                },
            );

            // Benchmark tensor conversion to CoreML
            let tensor = Tensor::zeros(shape, candle_core::DType::I64, &device).unwrap();
            group.bench_with_input(
                BenchmarkId::new("tensor_to_coreml", format!("{batch}x{seq_len}")),
                &tensor,
                |b, tensor| {
                    b.iter(|| {
                        // This benchmarks the conversion process
                        #[cfg(target_os = "macos")]
                        {
                            let ml_array = candle_coreml::conversion::tensor_to_mlmultiarray(
                                black_box(tensor),
                            );
                            black_box(ml_array)
                        }
                        #[cfg(not(target_os = "macos"))]
                        {
                            black_box(tensor)
                        }
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_individual_components(c: &mut Criterion) {
    let mut group = c.benchmark_group("qwen_components");
    group.sample_size(20);

    if let Some((model, device)) = get_qwen_model() {
        // Create test tokens for the prompt
        if let Ok(tokens) = model.tokenize(TEST_PROMPT) {
            let input_tensor =
                Tensor::from_vec(tokens.clone(), (1, tokens.len()), &device).unwrap();

            // Benchmark embeddings component
            group.bench_function("embeddings_component", |b| {
                b.iter(|| {
                    let result = model.embeddings.forward(&[black_box(&input_tensor)]);
                    black_box(result)
                })
            });

            // Benchmark a single token inference
            let single_token = Tensor::from_vec(vec![tokens[0]], (1, 1), &device).unwrap();

            group.bench_function("single_token_inference", |b| {
                b.iter(|| {
                    let result = model.embeddings.forward(&[black_box(&single_token)]);
                    black_box(result)
                })
            });
        }
    }

    group.finish();
}

fn bench_end_to_end_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(60)); // Allow longer measurement time

    if let Some((mut model, _device)) = get_qwen_model() {
        let token_counts = [5, 10, 25];

        for &num_tokens in &token_counts {
            group.throughput(Throughput::Elements(num_tokens));

            group.bench_with_input(
                BenchmarkId::new("generate_tokens", num_tokens),
                &num_tokens,
                |b, &num_tokens| {
                    b.iter(|| {
                        let result = model.generate_tokens(
                            black_box(TEST_PROMPT),
                            black_box(num_tokens as usize),
                            black_box(1.0),  // temperature
                            black_box(None), // max_tokens
                        );
                        black_box(result)
                    })
                },
            );
        }

        // Add a specific benchmark comparing to Python baseline
        group.bench_function("vs_python_baseline", |b| {
            b.iter(|| {
                let start = std::time::Instant::now();
                let result = model.generate_tokens(
                    black_box(TEST_PROMPT),
                    black_box(25),
                    black_box(1.0),
                    black_box(None),
                );
                let duration = start.elapsed();

                if let Ok(ref tokens) = result {
                    let tps = 25.0 / duration.as_secs_f64();
                    let ratio = tps / BASELINE_PYTHON_TPS;

                    // This will be captured in benchmark output
                    eprintln!(
                        "🐍 Python baseline comparison: {:.1}% ({:.2}/{:.2} t/s)",
                        ratio * 100.0,
                        tps,
                        BASELINE_PYTHON_TPS
                    );
                }

                black_box(result)
            })
        });
    }

    group.finish();
}

fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");

    if let Some((_model, device)) = get_qwen_model() {
        let sizes = [100, 1000, 10000];

        for size in sizes {
            group.throughput(Throughput::Elements(size));

            // Benchmark vector allocation
            group.bench_with_input(
                BenchmarkId::new("vec_allocation", size),
                &size,
                |b, &size| {
                    b.iter(|| {
                        let vec: Vec<f32> = (0..size).map(|i| i as f32).collect();
                        black_box(vec)
                    })
                },
            );

            // Benchmark tensor from vec
            group.bench_with_input(
                BenchmarkId::new("tensor_from_vec", size),
                &size,
                |b, &size| {
                    let vec: Vec<f32> = (0..size).map(|i| i as f32).collect();
                    b.iter(|| {
                        let tensor = Tensor::from_vec(
                            black_box(vec.clone()),
                            black_box((size as usize,)),
                            black_box(&device),
                        );
                        black_box(tensor)
                    })
                },
            );
        }
    }

    group.finish();
}

// Configure criterion with custom settings for performance analysis
criterion_group!(
    name = benches;
    config = {
        let mut config = Criterion::default();
        config = config
            .warm_up_time(Duration::from_secs(5))
            .measurement_time(Duration::from_secs(30))
            .sample_size(50);

        // Enable output color
        config = config.with_output_color(true);

        config
    };
    targets =
        bench_model_loading,
        bench_tokenization,
        bench_tensor_operations,
        bench_individual_components,
        bench_end_to_end_generation,
        bench_memory_operations
);

criterion_main!(benches);
