//! Quick profiling benchmark to identify bottlenecks fast
//!
//! This runs fewer samples and shorter measurements to get rapid feedback

use candle_core::{DType, Device, Tensor};
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use std::time::Duration;

#[cfg(target_os = "macos")]
use candle_coreml::conversion::tensor_to_mlmultiarray;

fn quick_tensor_conversion_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("quick_tensor_conversion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(5));

    #[cfg(target_os = "macos")]
    {
        let device = Device::Cpu;

        // Test small tensor (single token)
        let small_tensor = Tensor::zeros((1, 1), DType::I64, &device).unwrap();
        group.bench_function("small_i64_tensor", |b| {
            b.iter(|| {
                let result = tensor_to_mlmultiarray(black_box(&small_tensor));
                black_box(result)
            })
        });

        // Test medium tensor (typical sequence)
        let medium_tensor = Tensor::zeros((1, 50), DType::I64, &device).unwrap();
        group.bench_function("medium_i64_tensor", |b| {
            b.iter(|| {
                let result = tensor_to_mlmultiarray(black_box(&medium_tensor));
                black_box(result)
            })
        });

        // Test f32 tensor conversion
        let f32_tensor = Tensor::zeros((1, 50), DType::F32, &device).unwrap();
        group.bench_function("medium_f32_tensor", |b| {
            b.iter(|| {
                let result = tensor_to_mlmultiarray(black_box(&f32_tensor));
                black_box(result)
            })
        });
    }

    group.finish();
}

fn quick_memory_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("quick_memory");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));

    let _device = Device::Cpu;

    // Test allocation patterns
    group.bench_function("vec_allocation_small", |b| {
        b.iter(|| {
            let vec: Vec<i64> = (0..100).collect();
            black_box(vec)
        })
    });

    group.bench_function("vec_allocation_medium", |b| {
        b.iter(|| {
            let vec: Vec<i64> = (0..1000).collect();
            black_box(vec)
        })
    });

    // Test data type conversions that happen in our pipeline
    group.bench_function("i64_to_i32_conversion", |b| {
        let data: Vec<i64> = (0..1000).collect();
        b.iter(|| {
            let converted: Vec<i32> = data.iter().map(|&x| x as i32).collect();
            black_box(converted)
        })
    });

    group.finish();
}

fn quick_tensor_ops_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("quick_tensor_ops");
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(3));

    let device = Device::Cpu;

    // Test tensor creation
    group.bench_function("tensor_from_vec_small", |b| {
        let data: Vec<i64> = (0..50).collect();
        b.iter(|| {
            let tensor = Tensor::from_vec(
                black_box(data.clone()),
                black_box((50,)),
                black_box(&device),
            );
            black_box(tensor)
        })
    });

    group.bench_function("tensor_from_vec_medium", |b| {
        let data: Vec<i64> = (0..500).collect();
        b.iter(|| {
            let tensor = Tensor::from_vec(
                black_box(data.clone()),
                black_box((500,)),
                black_box(&device),
            );
            black_box(tensor)
        })
    });

    // Test tensor cloning
    let test_tensor = Tensor::randn(0.0, 1.0, (128, 768), &device).unwrap();
    group.bench_function("tensor_clone", |b| {
        b.iter(|| {
            let cloned = black_box(&test_tensor).clone();
            black_box(cloned)
        })
    });

    group.finish();
}

criterion_group!(
    name = benches;
    config = {
        Criterion::default()
            .warm_up_time(Duration::from_secs(1))
            .measurement_time(Duration::from_secs(5))
            .sample_size(10)
    };
    targets =
        quick_tensor_conversion_bench,
        quick_memory_bench,
        quick_tensor_ops_bench
);

criterion_main!(benches);
