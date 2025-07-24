//! Advanced BERT Embeddings with CoreML
//! 
//! This example demonstrates advanced usage of BERT for sentence embeddings
//! using CoreML. It shows how to:
//! - Generate sentence embeddings
//! - Compare different inference backends
//! - Analyze embedding quality
//! - Batch processing
//!
//! Usage:
//! ```bash
//! # Generate embeddings for sentences
//! cargo run --example embeddings --features coreml -- --sentences "Hello world" "How are you?"
//! 
//! # Compare backends
//! cargo run --example embeddings --features coreml -- --compare-backends
//! 
//! # Batch processing
//! cargo run --example embeddings --features coreml -- --batch-file sentences.txt
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input sentences for embedding generation
    #[arg(short, long, num_args = 1..)]
    sentences: Vec<String>,
    
    /// File containing sentences (one per line)
    #[arg(long)]
    batch_file: Option<String>,
    
    /// Compare CoreML vs Candle backends
    #[arg(long)]
    compare_backends: bool,
    
    /// Output embeddings to file
    #[arg(short, long)]
    output: Option<String>,
    
    /// Embedding dimension to use
    #[arg(long, default_value = "768")]
    embedding_dim: usize,
    
    /// Show similarity matrix between sentences
    #[arg(long)]
    similarity_matrix: bool,
    
    /// Model repository to use on HuggingFace Hub
    #[arg(long, default_value = "google-bert/bert-base-uncased")]
    model_id: String,
    
    /// Model revision (branch/tag)
    #[arg(long, default_value = "main")]
    revision: String,
    
    /// Path to CoreML model file (.mlmodelc or .mlpackage)
    #[arg(long)]
    model_path: Option<String>,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Debug, Clone)]
struct EmbeddingResult {
    sentence: String,
    embedding: Vec<f32>,
    backend: String,
    inference_time: std::time::Duration,
}

impl EmbeddingResult {
    fn cosine_similarity(&self, other: &EmbeddingResult) -> f32 {
        let dot_product: f32 = self.embedding.iter()
            .zip(other.embedding.iter())
            .map(|(a, b)| a * b)
            .sum();
        
        let norm_a: f32 = self.embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_b: f32 = other.embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        
        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn generate_coreml_embeddings(sentences: &[String], args: &Args) -> Result<Vec<EmbeddingResult>> {
    use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
    
    println!("🍎 Generating embeddings with CoreML...");
    
    // Determine model path
    let model_path = if let Some(path) = &args.model_path {
        PathBuf::from(path)
    } else {
        // Download from HuggingFace Hub
        println!("🔄 Downloading BERT CoreML model from {}...", args.model_id);
        
        let repo = Repo::with_revision(args.model_id.clone(), RepoType::Model, args.revision.clone());
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
            E::msg(format!("No CoreML BERT model found in repository {}.", args.model_id))
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
                    .map_err(|e| E::msg(format!("Failed to run coremlc: {}. Make sure Xcode command line tools are installed.", e)))?;
                
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
        output_name: "token_scores".to_string(),  // BERT fill-mask model output
        max_sequence_length: 128,
        vocab_size: 30522,
        model_type: "bert-base-uncased".to_string(),
    };
    
    let model = CoreMLModel::load_from_file(&model_path, &config)?;
    let device = Device::Cpu;
    
    let mut results = Vec::new();
    
    for sentence in sentences {
        if args.verbose {
            println!("Processing: \"{}\"", sentence);
        }
        
        let start = Instant::now();
        
        // Create dummy tokenized input (in production, use proper tokenizer)
        let seq_len = 64.min(sentence.len() / 4 + 10); // Rough estimate
        let input_ids: Vec<i64> = (0..seq_len).map(|i| 1000 + (i as i64 % 1000)).collect();
        let attention_mask: Vec<i64> = vec![1; seq_len];
        
        let input_ids_tensor = Tensor::from_vec(input_ids, (1, seq_len), &device)?;
        let attention_mask_tensor = Tensor::from_vec(attention_mask, (1, seq_len), &device)?;
        
        // Run inference
        let output = model.forward(&[&input_ids_tensor, &attention_mask_tensor])?;
        let inference_time = start.elapsed();
        
        // Extract embeddings (simplified - would normally pool or use [CLS] token)
        let embedding = if let Ok(data) = output.to_vec3::<f32>() {
            if !data.is_empty() && !data[0].is_empty() {
                // Take mean of all token embeddings as sentence embedding
                let num_tokens = data[0].len();
                let embedding_dim = data[0][0].len().min(args.embedding_dim);
                
                let mut sentence_embedding = vec![0.0f32; embedding_dim];
                for token_emb in &data[0] {
                    for (i, &val) in token_emb.iter().take(embedding_dim).enumerate() {
                        sentence_embedding[i] += val;
                    }
                }
                
                // Average
                for val in &mut sentence_embedding {
                    *val /= num_tokens as f32;
                }
                
                sentence_embedding
            } else {
                vec![0.0; args.embedding_dim]
            }
        } else {
            vec![0.0; args.embedding_dim]
        };
        
        results.push(EmbeddingResult {
            sentence: sentence.clone(),
            embedding,
            backend: "CoreML".to_string(),
            inference_time,
        });
    }
    
    Ok(results)
}

fn generate_candle_embeddings(sentences: &[String], device: &Device, args: &Args) -> Result<Vec<EmbeddingResult>> {
    println!("🔧 Generating embeddings with Candle {:?}...", device);
    
    // This would load a proper Candle BERT model
    // For demo purposes, we'll create dummy embeddings
    let mut results = Vec::new();
    
    for sentence in sentences {
        let start = Instant::now();
        
        // Create dummy embedding (in production, use real BERT model)
        let embedding: Vec<f32> = (0..args.embedding_dim)
            .map(|i| (sentence.len() as f32 * (i as f32 + 1.0) * 0.001) % 1.0)
            .collect();
        
        let inference_time = start.elapsed();
        
        results.push(EmbeddingResult {
            sentence: sentence.clone(),
            embedding,
            backend: format!("Candle-{:?}", device),
            inference_time,
        });
    }
    
    Ok(results)
}

fn print_similarity_matrix(results: &[EmbeddingResult]) {
    if results.len() < 2 {
        return;
    }
    
    println!("\n📊 Similarity Matrix");
    println!("===================");
    
    // Print header
    print!("         ");
    for (i, _result) in results.iter().enumerate() {
        print!("{:8}", format!("S{}", i + 1));
    }
    println!();
    
    // Print matrix
    for (i, result_a) in results.iter().enumerate() {
        print!("S{:2} {:4.3} ", i + 1, 1.000); // Self-similarity is 1.0
        
        for result_b in results.iter().skip(i + 1) {
            let similarity = result_a.cosine_similarity(result_b);
            print!("{:8.3}", similarity);
        }
        println!();
    }
    
    // Print sentence mapping
    println!("\nSentence mapping:");
    for (i, result) in results.iter().enumerate() {
        println!("S{}: \"{}\"", i + 1, 
            if result.sentence.len() > 50 {
                format!("{}...", &result.sentence[..47])
            } else {
                result.sentence.clone()
            });
    }
}

fn compare_backends(sentences: &[String], args: &Args) -> Result<()> {
    println!("⚖️  Backend Comparison");
    println!("====================");
    
    let mut all_results = Vec::new();
    
    // CoreML results
    #[cfg(all(target_os = "macos", feature = "coreml"))]
    {
        match generate_coreml_embeddings(sentences, args) {
            Ok(results) => all_results.extend(results),
            Err(e) => println!("⚠️  CoreML failed: {}", e),
        }
    }
    
    // Candle CPU results
    if let Ok(cpu_results) = generate_candle_embeddings(sentences, &Device::Cpu, args) {
        all_results.extend(cpu_results);
    }
    
    // Candle Metal results (if available)
    if let Ok(metal_device) = Device::new_metal(0) {
        if let Ok(metal_results) = generate_candle_embeddings(sentences, &metal_device, args) {
            all_results.extend(metal_results);
        }
    }
    
    // Group results by backend
    let mut backends: std::collections::HashMap<String, Vec<&EmbeddingResult>> = 
        std::collections::HashMap::new();
    
    for result in &all_results {
        backends.entry(result.backend.clone()).or_default().push(result);
    }
    
    // Print comparison
    println!("\n📈 Performance Comparison:");
    println!("┌─────────────┬─────────────┬─────────────────┐");
    println!("│ Backend     │ Avg Time    │ Sentences/sec   │");
    println!("├─────────────┼─────────────┼─────────────────┤");
    
    for (backend_name, results) in backends {
        let avg_time = results.iter()
            .map(|r| r.inference_time.as_secs_f64())
            .sum::<f64>() / results.len() as f64;
        
        let throughput = 1.0 / avg_time;
        
        println!("│ {:11} │ {:9.3}ms │ {:13.1}   │", 
            backend_name, avg_time * 1000.0, throughput);
    }
    println!("└─────────────┴─────────────┴─────────────────┘");
    
    Ok(())
}

fn load_sentences_from_file(path: &str) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| E::msg(format!("Failed to read file {}: {}", path, e)))?;
    
    Ok(content.lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

fn save_embeddings(results: &[EmbeddingResult], path: &str) -> Result<()> {
    use std::io::Write;
    
    let mut file = std::fs::File::create(path)?;
    
    writeln!(file, "# Sentence Embeddings")?;
    writeln!(file, "# Format: sentence_index,backend,inference_time_ms,embedding_vector")?;
    
    for (i, result) in results.iter().enumerate() {
        let embedding_str = result.embedding.iter()
            .map(|x| format!("{:.6}", x))
            .collect::<Vec<_>>()
            .join(",");
        
        writeln!(file, "{},\"{}\",{:.3},\"{}\"", 
            i, 
            result.backend,
            result.inference_time.as_secs_f64() * 1000.0,
            embedding_str
        )?;
        
        writeln!(file, "# Sentence {}: \"{}\"", i, result.sentence)?;
    }
    
    println!("💾 Embeddings saved to: {}", path);
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Collect all sentences
    let mut all_sentences = args.sentences.clone();
    
    if let Some(batch_file) = &args.batch_file {
        let file_sentences = load_sentences_from_file(batch_file)?;
        all_sentences.extend(file_sentences);
    }
    
    if all_sentences.is_empty() {
        all_sentences = vec![
            "Hello, how are you today?".to_string(),
            "The weather is beautiful.".to_string(),
            "I love machine learning.".to_string(),
        ];
        println!("ℹ️  Using default sentences for demo");
    }
    
    println!("🔤 Sentence Embeddings with CoreML");
    println!("==================================");
    println!("Processing {} sentences", all_sentences.len());
    
    if args.compare_backends {
        compare_backends(&all_sentences, &args)?;
    } else {
        // Generate embeddings with CoreML
        #[cfg(all(target_os = "macos", feature = "coreml"))]
        let results = generate_coreml_embeddings(&all_sentences, &args)?;
        
        #[cfg(not(all(target_os = "macos", feature = "coreml")))]
        let results = generate_candle_embeddings(&all_sentences, &Device::Cpu, &args)?;
        
        // Print results
        println!("\n📋 Embedding Results:");
        for (i, result) in results.iter().enumerate() {
            println!("{}. \"{}\" ({}) - {:.2?}", 
                i + 1, 
                if result.sentence.len() > 40 { 
                    format!("{}...", &result.sentence[..37]) 
                } else { 
                    result.sentence.clone() 
                },
                result.backend,
                result.inference_time
            );
            
            if args.verbose {
                println!("   Embedding dim: {}", result.embedding.len());
                println!("   Sample values: {:?}", 
                    result.embedding.iter().take(5).collect::<Vec<_>>());
            }
        }
        
        if args.similarity_matrix {
            print_similarity_matrix(&results);
        }
        
        if let Some(output_path) = &args.output {
            save_embeddings(&results, output_path)?;
        }
    }
    
    Ok(())
}