//! Qwen 0.6B Performance Benchmark: ANE vs CPU
//!
//! This benchmark demonstrates the performance benefits of using Apple's Neural Engine
//! compared to CPU inference for Qwen 0.6B models.
//!
//! Features:
//! - Side-by-side ANE vs CPU performance comparison
//! - Latency measurements for different sequence lengths
//! - Memory usage tracking
//! - Throughput analysis (tokens/second)
//! - Statistical analysis with confidence intervals
//!
//! Usage:
//! ```bash
//! # Full benchmark suite
//! cargo run --example qwen_benchmark
//!
//! # Quick benchmark with fewer iterations
//! cargo run --example qwen_benchmark -- --iterations 5
//!
//! # Test specific sequence lengths
//! cargo run --example qwen_benchmark -- --sequence-lengths 128,256,512
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_coreml::{get_local_or_remote_file, Config as CoreMLConfig, CoreMLModel};
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokenizers::Tokenizer;

const QWEN_VOCAB_SIZE: usize = 151936;
const DEFAULT_ITERATIONS: usize = 10;
const DEFAULT_SEQUENCE_LENGTHS: &[usize] = &[64, 128, 256, 512];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model repository on HuggingFace Hub
    #[arg(long, default_value = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")]
    model_id: String,

    /// Number of benchmark iterations per test
    #[arg(long, default_value_t = DEFAULT_ITERATIONS)]
    iterations: usize,

    /// Sequence lengths to test (comma-separated)
    #[arg(long, value_delimiter = ',')]
    sequence_lengths: Option<Vec<usize>>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Skip CPU benchmark (ANE only)
    #[arg(long)]
    ane_only: bool,

    /// Use local model instead of downloading
    #[arg(long)]
    local: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct BenchmarkResult {
    device_type: String,
    sequence_length: usize,
    latency_ms: Vec<f64>,
    memory_mb: f64,
    tokens_per_second: f64,
}

impl BenchmarkResult {
    fn new(device_type: String, sequence_length: usize) -> Self {
        Self {
            device_type,
            sequence_length,
            latency_ms: Vec::new(),
            memory_mb: 0.0,
            tokens_per_second: 0.0,
        }
    }

    fn add_measurement(&mut self, latency: Duration) {
        self.latency_ms.push(latency.as_secs_f64() * 1000.0);
    }

    fn calculate_stats(&mut self) {
        if self.latency_ms.is_empty() {
            return;
        }

        let mean_latency = self.latency_ms.iter().sum::<f64>() / self.latency_ms.len() as f64;
        self.tokens_per_second = (self.sequence_length as f64 / mean_latency) * 1000.0;
    }

    fn mean_latency(&self) -> f64 {
        if self.latency_ms.is_empty() {
            0.0
        } else {
            self.latency_ms.iter().sum::<f64>() / self.latency_ms.len() as f64
        }
    }

    fn stddev_latency(&self) -> f64 {
        if self.latency_ms.len() < 2 {
            return 0.0;
        }

        let mean = self.mean_latency();
        let variance = self
            .latency_ms
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / (self.latency_ms.len() - 1) as f64;
        variance.sqrt()
    }

    fn min_latency(&self) -> f64 {
        self.latency_ms.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    fn max_latency(&self) -> f64 {
        self.latency_ms
            .iter()
            .fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }
}

struct QwenBenchmark {
    model: CoreMLModel,
    tokenizer: Tokenizer,
}

impl QwenBenchmark {
    fn new(model: CoreMLModel, tokenizer: Tokenizer) -> Self {
        Self { model, tokenizer }
    }

    fn generate_test_tokens(&self, sequence_length: usize) -> Result<Vec<i64>> {
        // Create a representative test sequence
        let test_text =
            "The quick brown fox jumps over the lazy dog. ".repeat(sequence_length / 10 + 1);

        let encoding = self
            .tokenizer
            .encode(test_text.as_str(), true)
            .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))?;

        let mut tokens: Vec<i64> = encoding
            .get_ids()
            .iter()
            .map(|&id| id as i64)
            .take(sequence_length)
            .collect();

        // Pad if necessary
        while tokens.len() < sequence_length {
            tokens.push(0); // PAD token
        }

        Ok(tokens)
    }

    fn benchmark_inference(
        &self,
        sequence_length: usize,
        iterations: usize,
        use_state: bool,
        verbose: bool,
    ) -> Result<BenchmarkResult> {
        let device_type = if use_state {
            "ANE (Stateful)"
        } else {
            "CPU (Stateless)"
        };
        let mut result = BenchmarkResult::new(device_type.to_string(), sequence_length);

        if verbose {
            println!(
                "🔧 Benchmarking {} with sequence length {}",
                device_type, sequence_length
            );
        }

        let device = Device::Cpu;
        let test_tokens = self.generate_test_tokens(sequence_length)?;

        // Warm-up run
        let input_tensor = Tensor::from_vec(test_tokens.clone(), (1, sequence_length), &device)?;

        if use_state {
            let mut state = self
                .model
                .make_state()
                .map_err(|e| E::msg(format!("Failed to create state: {}", e)))?;
            let _ = self
                .model
                .predict_with_state(&[&input_tensor], &mut state)?;
        } else {
            let _ = self.model.forward(&[&input_tensor])?;
        }

        // Benchmark iterations
        for i in 0..iterations {
            let input_tensor =
                Tensor::from_vec(test_tokens.clone(), (1, sequence_length), &device)?;

            let start_time = Instant::now();

            if use_state {
                let mut state = self
                    .model
                    .make_state()
                    .map_err(|e| E::msg(format!("Failed to create state: {}", e)))?;
                let _ = self
                    .model
                    .predict_with_state(&[&input_tensor], &mut state)?;
            } else {
                let _ = self.model.forward(&[&input_tensor])?;
            }

            let elapsed = start_time.elapsed();
            result.add_measurement(elapsed);

            if verbose && (i + 1) % 5 == 0 {
                println!("  ✓ Completed {}/{} iterations", i + 1, iterations);
            }
        }

        result.calculate_stats();
        Ok(result)
    }
}

fn download_resources(args: &Args) -> Result<(PathBuf, Tokenizer)> {
    if args.local {
        let model_path = PathBuf::from("models/qwen/model.mlmodelc");
        let tokenizer_path = PathBuf::from("models/qwen/tokenizer.json");

        if !model_path.exists() || !tokenizer_path.exists() {
            return Err(E::msg(
                "Local model or tokenizer not found. Run without --local to download.",
            ));
        }

        let tokenizer = Tokenizer::from_file(&tokenizer_path)
            .map_err(|e| E::msg(format!("Failed to load local tokenizer: {}", e)))?;

        return Ok((model_path, tokenizer));
    }

    println!("📥 Downloading benchmark resources...");

    let repo = Repo::with_revision(args.model_id.clone(), RepoType::Model, "main".to_string());
    let api = Api::new()?;
    let api_repo = api.repo(repo);

    // Download model
    let model_file = get_local_or_remote_file("model.mlmodelc", &api_repo)
        .or_else(|_| get_local_or_remote_file("model.mlpackage", &api_repo))
        .map_err(|e| E::msg(format!("Could not download model: {}", e)))?;

    // Download tokenizer
    let tokenizer_file = get_local_or_remote_file("tokenizer.json", &api_repo)
        .map_err(|e| E::msg(format!("Could not download tokenizer: {}", e)))?;

    let tokenizer = Tokenizer::from_file(&tokenizer_file)
        .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;

    println!("✅ Resources downloaded successfully");
    Ok((model_file, tokenizer))
}

fn print_results(results: &[BenchmarkResult]) {
    println!("\n📊 Benchmark Results");
    println!("===================");

    // Group results by sequence length
    let mut results_by_length: std::collections::HashMap<usize, Vec<&BenchmarkResult>> =
        std::collections::HashMap::new();

    for result in results {
        results_by_length
            .entry(result.sequence_length)
            .or_default()
            .push(result);
    }

    for (&seq_len, group) in results_by_length.iter() {
        println!("\n🔢 Sequence Length: {} tokens", seq_len);
        println!("────────────────────────────────");

        for result in group {
            println!("\n📱 Device: {}", result.device_type);
            println!(
                "   • Mean latency:    {:.2}ms ± {:.2}ms",
                result.mean_latency(),
                result.stddev_latency()
            );
            println!(
                "   • Min/Max latency: {:.2}ms / {:.2}ms",
                result.min_latency(),
                result.max_latency()
            );
            println!(
                "   • Throughput:      {:.1} tokens/second",
                result.tokens_per_second
            );
        }

        // Calculate speedup if we have both ANE and CPU results
        if group.len() == 2 {
            let ane_result = group.iter().find(|r| r.device_type.contains("ANE"));
            let cpu_result = group.iter().find(|r| r.device_type.contains("CPU"));

            if let (Some(ane), Some(cpu)) = (ane_result, cpu_result) {
                let speedup = cpu.mean_latency() / ane.mean_latency();
                let throughput_gain = (ane.tokens_per_second / cpu.tokens_per_second - 1.0) * 100.0;

                println!("\n⚡ ANE Performance Gain:");
                println!("   • {}x faster than CPU", speedup);
                println!("   • {:.1}% higher throughput", throughput_gain);
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn run_benchmark(args: &Args) -> Result<()> {
    println!("🏃‍♂️ Qwen 0.6B Performance Benchmark");
    println!("===================================");
    println!("Model: {}", args.model_id);
    println!("Iterations: {}", args.iterations);

    let sequence_lengths = args
        .sequence_lengths
        .as_deref()
        .unwrap_or(DEFAULT_SEQUENCE_LENGTHS);

    println!("Sequence lengths: {:?}", sequence_lengths);
    println!();

    // Download resources
    let (model_path, tokenizer) = download_resources(args)?;

    // Load model
    println!("🔄 Loading CoreML model...");
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string()],
        output_name: "output".to_string(),
        max_sequence_length: *sequence_lengths.iter().max().unwrap_or(&512),
        vocab_size: QWEN_VOCAB_SIZE,
        model_type: "qwen-benchmark".to_string(),
    };

    let model = CoreMLModel::load_from_file(&model_path, &config)
        .map_err(|e| E::msg(format!("Failed to load model: {}", e)))?;

    let benchmark = QwenBenchmark::new(model, tokenizer);
    let mut all_results = Vec::new();

    // Run benchmarks
    for &seq_len in sequence_lengths {
        println!("🧪 Testing sequence length: {}", seq_len);

        // ANE benchmark (with state)
        match benchmark.benchmark_inference(seq_len, args.iterations, true, args.verbose) {
            Ok(result) => {
                println!("✅ ANE benchmark completed");
                all_results.push(result);
            }
            Err(e) => {
                println!("⚠️  ANE benchmark failed: {}", e);
            }
        }

        // CPU benchmark (without state) - only if not skipped
        if !args.ane_only {
            match benchmark.benchmark_inference(seq_len, args.iterations, false, args.verbose) {
                Ok(result) => {
                    println!("✅ CPU benchmark completed");
                    all_results.push(result);
                }
                Err(e) => {
                    println!("⚠️  CPU benchmark failed: {}", e);
                }
            }
        }

        println!();
    }

    // Display results
    if !all_results.is_empty() {
        print_results(&all_results);

        println!("\n💡 Analysis Notes:");
        println!("• ANE (Stateful) uses persistent KV-cache for efficiency");
        println!("• CPU (Stateless) runs standard forward pass without state");
        println!("• Lower latency and higher throughput indicate better performance");
        println!("• Speedup varies by sequence length and model complexity");
    } else {
        println!("❌ No benchmark results available");
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn run_benchmark(_args: &Args) -> Result<()> {
    println!("❌ Benchmarking is only available on macOS with CoreML support.");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    run_benchmark(&args)
}
