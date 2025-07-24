//! BERT Performance Comparison: Candle vs CoreML
//! 
//! Comprehensive benchmark comparing BERT inference performance between:
//! - Candle (CPU/Metal) using .safetensors weights
//! - CoreML runtime using .mlpackage/.mlmodelc files
//!
//! Features:
//! - Multiple sequence lengths testing
//! - Loading time measurement  
//! - Cold vs warm inference comparison
//! - Throughput calculation
//! - Memory usage analysis
//!
//! Usage:
//! ```bash
//! # Run full benchmark suite
//! cargo run --example bert_comparison --features coreml
//! 
//! # Test specific sequence length
//! cargo run --example bert_comparison --features coreml -- --sequence-lengths "128"
//! 
//! # Quick test with fewer iterations
//! cargo run --example bert_comparison --features coreml -- --warmup 1 --iterations 3
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE};
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokenizers::Tokenizer;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of warmup iterations
    #[arg(long, default_value = "3")]
    warmup: usize,

    /// Number of benchmark iterations  
    #[arg(long, default_value = "10")]
    iterations: usize,

    /// Test sequence lengths (comma-separated)
    #[arg(long, default_value = "64,128,256")]
    sequence_lengths: String,

    /// Test prompt for inference
    #[arg(long, default_value = "Paris is the capital of France.")]
    prompt: String,

    /// Enable verbose output
    #[arg(long)]
    verbose: bool,

    /// Use local model files instead of downloading
    #[arg(long)]
    local_models: bool,

    /// CoreML model repository to use on HuggingFace Hub
    #[arg(long, default_value = "google-bert/bert-base-uncased")]
    coreml_model_id: String,
    
    /// CoreML model revision (branch/tag)
    #[arg(long, default_value = "main")]
    coreml_revision: String,
}

#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    sequence_length: usize,
    loading_time: Duration,
    cold_inference: Duration,
    warm_inference: Duration,
    throughput: f64, // tokens/second
    memory_mb: Option<f64>,
}

impl BenchmarkResult {
    fn print(&self) {
        println!("\n=== {} (seq_len: {}) ===", self.name, self.sequence_length);
        println!("Loading time:     {:?}", self.loading_time);
        println!("Cold inference:   {:?}", self.cold_inference);
        println!("Warm inference:   {:?}", self.warm_inference);
        println!("Throughput:       {:.1} tokens/sec", self.throughput);
        if let Some(mem) = self.memory_mb {
            println!("Memory usage:     {:.1} MB", mem);
        }
    }
    
    fn efficiency_score(&self) -> f64 {
        // Simple efficiency metric: throughput / loading_time_seconds
        self.throughput / self.loading_time.as_secs_f64()
    }
}

fn benchmark_candle_bert(device: &Device, args: &Args, seq_len: usize) -> Result<BenchmarkResult> {
    println!("🔧 Benchmarking Candle BERT on {:?} (seq_len: {})", device, seq_len);
    
    let start = Instant::now();
    
    // Model setup - use local files if specified, otherwise download
    let (model, tokenizer) = if args.local_models {
        load_local_bert_model(device)?
    } else {
        load_huggingface_bert_model(device)?
    };
    
    let loading_time = start.elapsed();
    
    // Prepare input
    let encoding = tokenizer
        .encode(args.prompt.clone(), true)
        .map_err(E::msg)?;
    
    let mut token_ids = encoding.get_ids().to_vec();
    
    // Pad or truncate to sequence length
    token_ids.resize(seq_len, 0);
    
    let token_ids = Tensor::new(&token_ids[..], device)?.unsqueeze(0)?;
    let token_type_ids = token_ids.zeros_like()?;
    
    // Cold inference
    let start = Instant::now();
    let _embeddings = model.forward(&token_ids, &token_type_ids, None)?;
    let cold_inference = start.elapsed();
    
    // Warmup
    for _ in 0..args.warmup {
        let _ = model.forward(&token_ids, &token_type_ids, None)?;
    }
    
    // Warm inference benchmark
    let start = Instant::now();
    for _ in 0..args.iterations {
        let _ = model.forward(&token_ids, &token_type_ids, None)?;
    }
    let total_time = start.elapsed();
    let warm_inference = total_time / args.iterations as u32;
    
    // Calculate throughput
    let total_tokens = seq_len * args.iterations;
    let throughput = total_tokens as f64 / total_time.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: format!("Candle-{:?}", device),
        sequence_length: seq_len,
        loading_time,
        cold_inference,
        warm_inference,
        throughput,
        memory_mb: None, // Could add memory tracking here
    })
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn benchmark_coreml_bert(args: &Args, seq_len: usize) -> Result<BenchmarkResult> {
    use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
    
    println!("🍎 Benchmarking CoreML BERT (seq_len: {})", seq_len);
    
    let start = Instant::now();
    
    // Determine model path
    let model_path = if args.local_models {
        // Use local test model
        PathBuf::from(format!("{}/bert-model-test/coreml/fill-mask/bert-compiled.mlmodelc/float32_model.mlmodelc", 
            env!("CARGO_MANIFEST_DIR")))
    } else {
        // Download from HuggingFace Hub
        println!("🔄 Downloading CoreML BERT model from {}...", args.coreml_model_id);
        
        let repo = Repo::with_revision(args.coreml_model_id.clone(), RepoType::Model, args.coreml_revision.clone());
        let api = Api::new()?;
        let api = api.repo(repo);
        
        // Try to find available CoreML models in the repository
        let model_patterns = [
            "coreml/fill-mask/float32_model.mlpackage/Data/com.apple.CoreML/model.mlmodel",
            "coreml/bert-base-uncased.mlpackage/Data/com.apple.CoreML/model.mlmodel",
            "bert-base-uncased.mlpackage/Data/com.apple.CoreML/model.mlmodel", 
            "model.mlpackage/Data/com.apple.CoreML/model.mlmodel",
            "model.mlmodelc",
            "bert.mlmodelc",
        ];
        
        let mut found_model_file = None;
        let mut mlpackage_name = None;
        
        for pattern in &model_patterns {
            if let Ok(model_file) = api.get(pattern) {
                // Extract the .mlpackage name from the path
                let path_components: Vec<&str> = pattern.split('/').collect();
                if let Some(package_component) = path_components.first() {
                    if package_component.ends_with(".mlpackage") {
                        mlpackage_name = Some(package_component.trim_end_matches(".mlpackage"));
                    }
                }
                
                // Also download associated files for .mlpackage models
                if pattern.contains(".mlpackage") {
                    let base_path = pattern.replace("/Data/com.apple.CoreML/model.mlmodel", "");
                    let _ = api.get(&format!("{}/Data/com.apple.CoreML/weights/weight.bin", base_path));
                    let _ = api.get(&format!("{}/Manifest.json", base_path));
                }
                
                found_model_file = Some(model_file);
                break;
            }
        }
        
        let model_file = found_model_file.ok_or_else(|| {
            E::msg(format!("No CoreML BERT model found in repository {}. Try using --local-models flag.", args.coreml_model_id))
        })?;
        
        // If this is an .mlmodel file, compile it
        if model_file.extension().and_then(|s| s.to_str()) == Some("mlmodel") {
            let model_name = mlpackage_name.unwrap_or("bert");
            let compiled_model_name = format!("{}.mlmodelc", model_name);
            
            let mlpackage_dir = if model_file.to_string_lossy().contains(".mlpackage") {
                model_file.parent().unwrap().parent().unwrap().parent().unwrap()
            } else {
                model_file.parent().unwrap()
            };
            
            let cache_dir = mlpackage_dir.parent().unwrap();
            let compiled_model_path = cache_dir.join("compiled_models").join(&compiled_model_name);
            
            if !compiled_model_path.exists() {
                println!("🔨 Compiling CoreML model (this may take a moment)...");
                std::fs::create_dir_all(compiled_model_path.parent().unwrap())?;
                
                let source_path = if mlpackage_dir.extension().and_then(|s| s.to_str()) == Some("mlpackage") {
                    mlpackage_dir
                } else {
                    &model_file
                };
                
                let output = std::process::Command::new("xcrun")
                    .args([
                        "coremlc", 
                        "compile", 
                        &source_path.to_string_lossy(),
                        &compiled_model_path.to_string_lossy()
                    ])
                    .output()
                    .map_err(|e| E::msg(format!("Failed to run coremlc: {}", e)))?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(E::msg(format!("CoreML compilation failed: {}", stderr)));
                }
                
                println!("✅ CoreML model compiled successfully");
            }
            
            // Return path to the actual compiled model directory (may be nested)
            // Check for various possible nested structures
            let possible_paths = [
                compiled_model_path.join(&compiled_model_name),
                compiled_model_path.join(&compiled_model_name).join("float32_model.mlmodelc"),
                compiled_model_path.join("float32_model.mlmodelc"),
                compiled_model_path.clone(),
            ];
            
            let mut final_path = compiled_model_path.clone();
            for path in &possible_paths {
                if path.exists() {
                    final_path = path.clone();
                    break;
                }
            }
            
            final_path
        } else {
            // Already a compiled model
            model_file
        }
    };
    
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string(), "attention_mask".to_string()],
        output_name: "token_scores".to_string(),
        max_sequence_length: seq_len,
        vocab_size: 30522,
        model_type: "bert-base-uncased".to_string(),
    };

    let model = CoreMLModel::load_from_file(&model_path, &config)?;
    let loading_time = start.elapsed();
    
    // Prepare input tensors (both input_ids and attention_mask)
    let device = Device::Cpu;
    let input_data: Vec<i64> = (0..seq_len).map(|i| (i % 30522) as i64).collect();
    let attention_data: Vec<i64> = vec![1; seq_len]; // All tokens are real (not padding)
    
    let input_ids = Tensor::new(&input_data[..], &device)?.unsqueeze(0)?;
    let attention_mask = Tensor::new(&attention_data[..], &device)?.unsqueeze(0)?;
    
    // Cold inference
    let start = Instant::now();
    let _ = model.forward(&[&input_ids, &attention_mask])?;
    let cold_inference = start.elapsed();
    
    // Warmup
    for _ in 0..args.warmup {
        let _ = model.forward(&[&input_ids, &attention_mask])?;
    }
    
    // Warm inference benchmark
    let start = Instant::now();
    for _ in 0..args.iterations {
        let _ = model.forward(&[&input_ids, &attention_mask])?;
    }
    let total_time = start.elapsed();
    let warm_inference = total_time / args.iterations as u32;
    
    // Calculate throughput
    let total_tokens = seq_len * args.iterations;
    let throughput = total_tokens as f64 / total_time.as_secs_f64();
    
    Ok(BenchmarkResult {
        name: "CoreML".to_string(),
        sequence_length: seq_len,
        loading_time,
        cold_inference,
        warm_inference,
        throughput,
        memory_mb: None,
    })
}

fn load_local_bert_model(device: &Device) -> Result<(BertModel, Tokenizer)> {
    // Load from local files (for testing without internet)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let model_dir = format!("{}/bert-model-test", manifest_dir);
    
    let config_path = format!("{}/config.json", model_dir);
    let tokenizer_path = format!("{}/tokenizer.json", model_dir);
    let weights_path = format!("{}/model.safetensors", model_dir);
    
    let config: Config = serde_json::from_slice(&std::fs::read(&config_path)
        .map_err(|e| E::msg(format!("Failed to read config from {}: {}", config_path, e)))?)?;
    
    let tokenizer = Tokenizer::from_file(&tokenizer_path)
        .map_err(|e| E::msg(format!("Failed to load tokenizer from {}: {}", tokenizer_path, e)))?;
    
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_path], DTYPE, device)? };
    let model = BertModel::load(vb, &config)?;
    
    Ok((model, tokenizer))
}

fn load_huggingface_bert_model(device: &Device) -> Result<(BertModel, Tokenizer)> {
    // Download from HuggingFace Hub
    use hf_hub::{api::sync::Api, Repo, RepoType};
    
    let api = Api::new()?;
    let repo = api.repo(Repo::new("google-bert/bert-base-uncased".to_string(), RepoType::Model));
    
    let config_filename = repo.get("config.json")?;
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let weights_filename = repo.get("model.safetensors")?;
    
    let config = std::fs::read_to_string(config_filename)?;
    let config: Config = serde_json::from_str(&config)?;
    
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
    
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[&weights_filename], DTYPE, device)? };
    let model = BertModel::load(vb, &config)?;
    
    Ok((model, tokenizer))
}

fn print_comparison_summary(results: &[BenchmarkResult]) {
    println!("\n📊 BENCHMARK SUMMARY");
    println!("===================");
    
    // Group by sequence length
    let mut seq_lengths: Vec<_> = results.iter().map(|r| r.sequence_length).collect();
    seq_lengths.sort_unstable();
    seq_lengths.dedup();
    
    for seq_len in seq_lengths {
        println!("\n📏 Sequence Length: {}", seq_len);
        println!("┌─────────────┬─────────────┬─────────────┬─────────────┬─────────────┐");
        println!("│ Backend     │ Loading     │ Cold Inf.   │ Warm Inf.   │ Throughput  │");
        println!("├─────────────┼─────────────┼─────────────┼─────────────┼─────────────┤");
        
        let seq_results: Vec<_> = results.iter()
            .filter(|r| r.sequence_length == seq_len)
            .collect();
        
        for result in &seq_results {
            println!("│ {:11} │ {:9.1?} │ {:9.1?} │ {:9.1?} │ {:8.1} t/s │",
                result.name,
                result.loading_time,
                result.cold_inference,
                result.warm_inference,
                result.throughput
            );
        }
        println!("└─────────────┴─────────────┴─────────────┴─────────────┴─────────────┘");
        
        // Find best performers
        if let Some(fastest_load) = seq_results.iter().min_by_key(|r| r.loading_time) {
            println!("🚀 Fastest loading: {}", fastest_load.name);
        }
        if let Some(best_throughput) = seq_results.iter()
            .max_by(|a, b| a.throughput.partial_cmp(&b.throughput).unwrap()) {
            println!("⚡ Best throughput: {} ({:.1} tokens/sec)", 
                best_throughput.name, best_throughput.throughput);
        }
        if let Some(most_efficient) = seq_results.iter()
            .max_by(|a, b| a.efficiency_score().partial_cmp(&b.efficiency_score()).unwrap()) {
            println!("🎯 Most efficient: {} (score: {:.1})", 
                most_efficient.name, most_efficient.efficiency_score());
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("🔬 BERT Performance Benchmark");
    println!("============================");
    println!("Warmup iterations: {}", args.warmup);
    println!("Benchmark iterations: {}", args.iterations);
    println!("Test prompt: \"{}\"", args.prompt);
    
    let sequence_lengths: Vec<usize> = args.sequence_lengths
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    let mut all_results = Vec::new();
    
    for &seq_len in &sequence_lengths {
        println!("\n🔄 Testing sequence length: {}", seq_len);
        
        // Test CPU
        if let Ok(result) = benchmark_candle_bert(&Device::Cpu, &args, seq_len) {
            result.print();
            all_results.push(result);
        }
        
        // Test Metal (if available)
        if let Ok(metal_device) = Device::new_metal(0) {
            if let Ok(result) = benchmark_candle_bert(&metal_device, &args, seq_len) {
                result.print();
                all_results.push(result);
            }
        } else if args.verbose {
            println!("ℹ️  Metal device not available, skipping Metal benchmark");
        }
        
        // Test CoreML (macOS only)
        #[cfg(all(target_os = "macos", feature = "coreml"))]
        {
            match benchmark_coreml_bert(&args, seq_len) {
                Ok(result) => {
                    result.print();
                    all_results.push(result);
                },
                Err(e) => {
                    if args.verbose {
                        println!("⚠️  CoreML benchmark failed: {}", e);
                    }
                }
            }
        }
        
        #[cfg(not(all(target_os = "macos", feature = "coreml")))]
        {
            if args.verbose {
                println!("ℹ️  CoreML not available (requires macOS and 'coreml' feature)");
            }
        }
    }
    
    if !all_results.is_empty() {
        print_comparison_summary(&all_results);
    } else {
        println!("❌ No successful benchmarks completed");
    }
    
    Ok(())
}