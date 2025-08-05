//! CoreML Low-Level Operations Benchmarks
//!
//! This benchmark suite focuses on the lowest-level CoreML operations
//! to identify exactly where time is being spent in the inference pipeline.

#![allow(unused_variables)]

use candle_core::{DType, Device, Tensor};
use candle_coreml::{Config, CoreMLModel};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;

#[cfg(target_os = "macos")]
use candle_coreml::conversion::{create_multi_feature_provider, tensor_to_mlmultiarray};

fn get_test_model() -> Option<(CoreMLModel, Device)> {
    #[cfg(target_os = "macos")]
    {
        // Try to get a Qwen component model for testing
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";

        if let Ok(model_dir) = candle_coreml::ensure_model_downloaded(model_id, false) {
            let embeddings_path = model_dir.join("embeddings.mlmodelc");

            if embeddings_path.exists() {
                let config = Config {
                    input_names: vec!["input_ids".to_string()],
                    output_name: "embeddings".to_string(),
                    max_sequence_length: 512,
                    vocab_size: 32000,
                    model_type: "embeddings".to_string(),
                };

                let device = Device::Cpu;
                if let Ok(model) = CoreMLModel::load_from_file(&embeddings_path, &config) {
                    return Some((model, device));
                }
            }
        }
    }

    None
}

fn bench_tensor_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("tensor_conversion");

    #[cfg(target_os = "macos")]
    {
        let device = Device::Cpu;
        let sizes = [
            (1, 10),  // Small single token
            (1, 50),  // Medium sequence
            (1, 128), // Typical sequence
            (1, 512), // Max sequence
        ];

        for (batch, seq_len) in sizes {
            let total_elements = batch * seq_len;
            group.throughput(Throughput::Elements(total_elements as u64));

            // Benchmark I64 tensor conversion (token IDs)
            let i64_tensor = Tensor::zeros((batch, seq_len), DType::I64, &device).unwrap();
            group.bench_with_input(
                BenchmarkId::new("i64_to_mlarray", format!("{}x{}", batch, seq_len)),
                &i64_tensor,
                |b, tensor| {
                    b.iter(|| {
                        let ml_array = tensor_to_mlmultiarray(black_box(tensor));
                        black_box(ml_array)
                    })
                },
            );

            // Benchmark F32 tensor conversion (embeddings/logits)
            let f32_tensor = Tensor::zeros((batch, seq_len), DType::F32, &device).unwrap();
            group.bench_with_input(
                BenchmarkId::new("f32_to_mlarray", format!("{}x{}", batch, seq_len)),
                &f32_tensor,
                |b, tensor| {
                    b.iter(|| {
                        let ml_array = tensor_to_mlmultiarray(black_box(tensor));
                        black_box(ml_array)
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_feature_provider_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("feature_provider");

    #[cfg(target_os = "macos")]
    {
        let device = Device::Cpu;
        let input_names = vec!["input_ids".to_string()];

        let sizes = [(1, 1), (1, 10), (1, 50), (1, 128)];

        for (batch, seq_len) in sizes {
            group.throughput(Throughput::Elements((batch * seq_len) as u64));

            let tensor = Tensor::zeros((batch, seq_len), DType::I64, &device).unwrap();
            let ml_array = tensor_to_mlmultiarray(&tensor).unwrap();
            let ml_arrays = vec![ml_array];

            group.bench_with_input(
                BenchmarkId::new("create_provider", format!("{}x{}", batch, seq_len)),
                &(&input_names, &ml_arrays),
                |b, (names, arrays)| {
                    b.iter(|| {
                        let provider =
                            create_multi_feature_provider(black_box(names), black_box(arrays));
                        black_box(provider)
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_coreml_prediction(c: &mut Criterion) {
    let mut group = c.benchmark_group("coreml_prediction");
    group.sample_size(20); // Fewer samples for expensive operations

    if let Some((model, device)) = get_test_model() {
        let sizes = [(1, 1), (1, 5), (1, 10), (1, 25)];

        for (batch, seq_len) in sizes {
            group.throughput(Throughput::Elements((batch * seq_len) as u64));

            // Create test input tensor
            let input_data: Vec<i64> = (0..seq_len).map(|i| (i % 1000) as i64).collect();
            let input_tensor = Tensor::from_vec(input_data, (batch, seq_len), &device).unwrap();

            group.bench_with_input(
                BenchmarkId::new("forward_pass", format!("{}x{}", batch, seq_len)),
                &input_tensor,
                |b, tensor| {
                    b.iter(|| {
                        let result = model.forward(&[black_box(tensor)]);
                        black_box(result)
                    })
                },
            );
        }
    }

    group.finish();
}

fn bench_stateful_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("stateful_operations");
    group.sample_size(15);

    if let Some((model, device)) = get_test_model() {
        // Benchmark state creation
        group.bench_function("state_creation", |b| {
            b.iter(|| {
                let state = model.make_state();
                black_box(state)
            })
        });

        // Benchmark stateful prediction
        if let Ok(mut state) = model.make_state() {
            let single_token = Tensor::from_vec(vec![100i64], (1, 1), &device).unwrap();

            group.bench_function("stateful_prediction", |b| {
                b.iter(|| {
                    let result = model
                        .predict_with_state(&[black_box(&single_token)], black_box(&mut state));
                    black_box(result)
                })
            });
        }
    }

    group.finish();
}

fn bench_memory_allocation_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    let device = Device::Cpu;

    // Test allocation patterns that might be happening in the hot path
    group.bench_function("repeated_small_allocations", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for i in 0..100 {
                let tensor = Tensor::from_vec(vec![i as f32], (1,), &device).unwrap();
                results.push(tensor);
            }
            black_box(results)
        })
    });

    group.bench_function("single_large_allocation", |b| {
        b.iter(|| {
            let data: Vec<f32> = (0..10000).map(|i| i as f32).collect();
            let tensor = Tensor::from_vec(data, (10000,), &device).unwrap();
            black_box(tensor)
        })
    });

    // Test cloning patterns (might be expensive in hot path)
    let test_tensor = Tensor::randn(0.0, 1.0, (128, 768), &device).unwrap();
    group.bench_function("tensor_cloning", |b| {
        b.iter(|| {
            let cloned = black_box(&test_tensor).clone();
            black_box(cloned)
        })
    });

    group.finish();
}

fn bench_data_type_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("dtype_conversions");

    let device = Device::Cpu;
    let sizes = [100, 1000, 10000];

    for size in sizes {
        group.throughput(Throughput::Elements(size));

        // I64 to I32 conversion (happens in CoreML conversion)
        let i64_data: Vec<i64> = (0..size).map(|i| i as i64).collect();
        group.bench_with_input(
            BenchmarkId::new("i64_to_i32", size),
            &i64_data,
            |b, data| {
                b.iter(|| {
                    let i32_data: Vec<i32> = data.iter().map(|&x| x as i32).collect();
                    black_box(i32_data)
                })
            },
        );

        // F32 vector operations
        let f32_data: Vec<f32> = (0..size).map(|i| i as f32).collect();
        group.bench_with_input(BenchmarkId::new("f32_copy", size), &f32_data, |b, data| {
            b.iter(|| {
                let copied: Vec<f32> = data.clone();
                black_box(copied)
            })
        });
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = {
        let mut config = Criterion::default();
        config = config
            .warm_up_time(Duration::from_secs(3))
            .measurement_time(Duration::from_secs(20))
            .sample_size(100);

        config
    };
    targets =
        bench_tensor_conversion,
        bench_feature_provider_creation,
        bench_coreml_prediction,
        bench_stateful_operations,
        bench_memory_allocation_patterns,
        bench_data_type_conversions
);

criterion_main!(benches);
