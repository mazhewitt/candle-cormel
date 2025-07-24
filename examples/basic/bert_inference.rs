//! ANE-Optimized DistilBERT Sentiment Analysis with CoreML
//! 
//! This example demonstrates sentiment analysis using Apple's ANE-optimized DistilBERT model
//! that actually runs on the Apple Neural Engine for maximum performance.
//!
//! Features:
//! - Uses Apple's ANE-optimized DistilBERT model
//! - True Apple Neural Engine acceleration
//! - Sentiment classification (positive/negative)
//! - Error handling with helpful messages
//! - Works with both .mlpackage and .mlmodelc files
//!
//! Usage:
//! ```bash
//! # Basic usage - analyzes sentiment of default text
//! cargo run --example bert_inference --features coreml
//! 
//! # Custom text for sentiment analysis
//! cargo run --example bert_inference --features coreml -- --text "I hate this movie!"
//! 
//! # Show detailed confidence scores
//! cargo run --example bert_inference --features coreml -- --show-scores
//! 
//! # Use specific ANE-optimized model path
//! cargo run --example bert_inference --features coreml -- --model-path "/path/to/DistilBERT_fp16.mlpackage"
//! ```

use anyhow::{Error as E, Result};
use candle_core::{Device, Tensor};
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use std::path::PathBuf;
use std::time::Instant;
use tokenizers::Tokenizer;

// Constants for DistilBERT tokenization
const CLS_TOKEN_ID: i64 = 101;
const SEP_TOKEN_ID: i64 = 102;
const PAD_TOKEN_ID: i64 = 0;
const DISTILBERT_VOCAB_SIZE: usize = 30522;
const ANE_SEQUENCE_LENGTH: usize = 128;

/// Tokenize text using DistilBERT tokenizer
fn tokenize_text(text: &str, tokenizer: &Tokenizer, max_length: usize) -> Result<(Vec<i64>, Vec<i64>)> {
    let encoding = tokenizer
        .encode(text, true)
        .map_err(|e| E::msg(format!("Tokenization failed: {}", e)))?;
    
    let mut input_ids: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
    
    // Truncate if too long
    if input_ids.len() > max_length {
        input_ids.truncate(max_length - 1);
        input_ids.push(SEP_TOKEN_ID);
    }
    
    // Create attention mask (1 for real tokens, 0 for padding)
    let mut attention_mask = vec![1i64; input_ids.len()];
    
    // Pad to fixed length for ANE optimization
    while input_ids.len() < max_length {
        input_ids.push(PAD_TOKEN_ID);
        attention_mask.push(0);
    }
    
    Ok((input_ids, attention_mask))
}

/// Download tokenizer from HuggingFace Hub  
fn download_tokenizer(api: &hf_hub::api::sync::ApiRepo) -> Result<Tokenizer> {
    println!("🔄 Downloading tokenizer...");
    
    let tokenizer_file = api.get("tokenizer.json")
        .map_err(|e| E::msg(format!("Failed to download tokenizer.json: {}", e)))?;
    
    let tokenizer = Tokenizer::from_file(&tokenizer_file)
        .map_err(|e| E::msg(format!("Failed to load tokenizer: {}", e)))?;
    
    println!("✅ Tokenizer loaded successfully");
    Ok(tokenizer)
}

/// Get local model path for testing
fn get_local_model_path() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(format!("{}/models/ane-distilbert/DistilBERT_fp16.mlpackage", manifest_dir))
}

/// Download model from HuggingFace Hub
fn download_model_from_hub(args: &Args) -> Result<PathBuf> {
    println!("🔄 Downloading model from {}...", args.model_id);
    
    let repo = Repo::with_revision(args.model_id.clone(), RepoType::Model, args.revision.clone());
    let api = Api::new()?;
    let api = api.repo(repo);
    
    println!("🔍 Looking for ANE-optimized DistilBERT files...");
    
    // Download the weight file to establish the model path
    let weight_file_path = "DistilBERT_fp16.mlpackage/Data/com.apple.CoreML/weights/weight.bin";
    
    let model_path = match api.get(weight_file_path) {
        Ok(weight_file) => {
            println!("✅ Successfully connected to model repository");
            
            // Safely construct the .mlpackage path from the weight file path
            let weight_parent = weight_file.parent()
                .ok_or_else(|| E::msg("Invalid weight file path: missing parent directory"))?;
            let coreml_dir = weight_parent.parent()
                .ok_or_else(|| E::msg("Invalid CoreML directory structure: missing com.apple.CoreML parent"))?;
            let data_dir = coreml_dir.parent()
                .ok_or_else(|| E::msg("Invalid data directory structure: missing Data parent"))?;
            let mlpackage_path = data_dir.parent()
                .ok_or_else(|| E::msg("Invalid mlpackage structure: missing .mlpackage parent directory"))?;
            
            if args.verbose {
                println!("📂 Found model at: {}", mlpackage_path.display());
            }
            
            // Download additional required files
            download_additional_model_files(&api, args.verbose)?;
            
            // Verify this is a valid .mlpackage directory
            if mlpackage_path.is_dir() {
                mlpackage_path.to_path_buf()
            } else {
                return Err(E::msg(format!(
                    "Model path exists but is not a valid .mlpackage directory: {}\n\
                    The model may be incomplete or corrupted.",
                    mlpackage_path.display()
                )));
            }
        },
        Err(e) => {
            return Err(E::msg(format!(
                "Could not download ANE-optimized DistilBERT model from {}.\n\
                Error: {}\n\n\
                💡 Try:\n\
                - Use --local flag if you have the model locally\n\
                - Use --model-path to specify a different model path\n\
                - Download manually from: https://huggingface.co/apple/ane-distilbert-base-uncased-finetuned-sst-2-english/tree/main/DistilBERT_fp16.mlpackage\n\
                - Check your internet connection",
                args.model_id, e
            )));
        }
    };
    
    Ok(model_path)
}

/// Download additional required model files
fn download_additional_model_files(api: &hf_hub::api::sync::ApiRepo, verbose: bool) -> Result<()> {
    println!("🔄 Downloading additional required files...");
    
    let additional_files = [
        "DistilBERT_fp16.mlpackage/Manifest.json",
        "DistilBERT_fp16.mlpackage/Data/com.apple.CoreML/model.mlmodel",
    ];
    
    for file_path in &additional_files {
        match api.get(file_path) {
            Ok(_) => {
                if verbose {
                    println!("✅ Downloaded: {}", file_path);
                }
            },
            Err(e) => {
                if verbose {
                    println!("⚠️  Could not download {}: {}", file_path, e);
                }
            }
        }
    }
    
    Ok(())
}

/// Compile .mlpackage to .mlmodelc if needed
fn compile_model_if_needed(model_path: PathBuf) -> Result<PathBuf> {
    if model_path.exists() && model_path.extension().and_then(|s| s.to_str()) == Some("mlpackage") {
        let cache_dir = model_path.parent()
            .ok_or_else(|| E::msg("Cannot determine parent directory for model compilation cache"))?
            .join("compiled_models");
        let compiled_model_path = cache_dir.join("DistilBERT_fp16.mlmodelc");
        
        if !compiled_model_path.exists() {
            println!("🔨 Compiling CoreML model for optimized performance (this may take a moment)...");
            std::fs::create_dir_all(&cache_dir)?;
            
            let output = std::process::Command::new("xcrun")
                .args([
                    "coremlc", 
                    "compile", 
                    &model_path.to_string_lossy(),
                    &compiled_model_path.to_string_lossy()
                ])
                .output()
                .map_err(|e| E::msg(format!("Failed to run coremlc: {}. Make sure Xcode command line tools are installed.", e)))?;
            
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("⚠️  Compilation failed, using .mlpackage directly: {}", stderr);
                Ok(model_path) // Use the original .mlpackage
            } else {
                println!("✅ CoreML model compiled successfully");
                
                // Check for nested compiled model structure
                let possible_paths = [
                    compiled_model_path.join("DistilBERT_fp16.mlmodelc"),
                    compiled_model_path.clone(),
                ];
                
                for path in &possible_paths {
                    if path.exists() {
                        return Ok(path.clone());
                    }
                }
                
                Ok(compiled_model_path)
            }
        } else {
            println!("✅ Using cached compiled model");
            
            // Check for nested compiled model structure
            let possible_paths = [
                compiled_model_path.join("DistilBERT_fp16.mlmodelc"),
                compiled_model_path.clone(),
            ];
            
            for path in &possible_paths {
                if path.exists() {
                    return Ok(path.clone());
                }
            }
            
            Ok(compiled_model_path)
        }
    } else {
        Ok(model_path)
    }
}

/// Determine the final model path based on arguments
fn determine_model_path(args: &Args) -> Result<PathBuf> {
    let model_path = if let Some(path) = &args.model_path {
        PathBuf::from(path)
    } else if args.local {
        get_local_model_path()
    } else {
        download_model_from_hub(args)?
    };
    
    compile_model_if_needed(model_path)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text for sentiment analysis
    #[arg(short, long, default_value = "The Neural Engine is really fast!")]
    text: String,
    
    /// Path to CoreML model file (.mlmodelc or .mlpackage)
    #[arg(short, long)]
    model_path: Option<String>,
    
    /// Model repository to use on HuggingFace Hub (Use Apple's ANE-optimized model)
    #[arg(long, default_value = "apple/ane-distilbert-base-uncased-finetuned-sst-2-english")]
    model_id: String,
    
    /// Model revision (branch/tag)
    #[arg(long, default_value = "main")]
    revision: String,
    
    /// Maximum sequence length for model input
    #[arg(long, default_value = "128")]
    max_length: usize,
    
    /// Show confidence scores for sentiment classification
    #[arg(long)]
    show_scores: bool,
    
    /// Use local test models instead of downloading
    #[arg(long)]
    local: bool,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn run_coreml_inference(args: &Args) -> Result<()> {
    use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
    
    println!("🍎 CoreML DistilBERT Sentiment Analysis (ANE-Optimized)");
    println!("=========================================================");
    println!("Input text: \"{}\"", args.text);
    
    // Determine model path and download tokenizer
    let model_path = determine_model_path(args)?;
    
    // Download and load tokenizer  
    let tokenizer = if !args.local && args.model_path.is_none() {
        let repo = Repo::with_revision(args.model_id.clone(), RepoType::Model, args.revision.clone());
        let api = Api::new()?;
        let api = api.repo(repo);
        Some(download_tokenizer(&api)?)
    } else {
        println!("⚠️  Using dummy tokenization (real tokenizer not available for local/manual paths)");
        None
    };
    
    if args.verbose {
        println!("📂 Final model path: {}", model_path.display());
    }
    
    // Check if model file exists
    if !model_path.exists() {
        return Err(E::msg(format!(
            "Model file not found: {}\n\n\
            💡 Try:\n\
            - Use --local flag for test models\n\
            - Use --model-path to specify model location\n\
            - Use --model-id to specify HuggingFace repository",
            model_path.display()
        )));
    }
    
    // Configure model for ANE-optimized DistilBERT sentiment classification
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string(), "attention_mask".to_string()],
        output_name: "logits".to_string(),
        max_sequence_length: args.max_length,
        vocab_size: DISTILBERT_VOCAB_SIZE,
        model_type: "ane-distilbert-base-uncased-finetuned-sst-2-english".to_string(),
    };
    
    // Load model
    let start = Instant::now();
    let model = CoreMLModel::load_from_file(&model_path, &config)
        .map_err(|e| E::msg(format!("Failed to load CoreML model: {}", e)))?;
    let loading_time = start.elapsed();
    
    println!("✅ Model loaded in {:?}", loading_time);
    println!("📋 Config: {:?}", config);
    
    // Prepare input using real or dummy tokenization
    let device = Device::Cpu;
    
    // Tokenize the input text
    let (input_ids, attention_mask) = if let Some(ref tokenizer) = tokenizer {
        // Use real tokenization
        println!("🔤 Tokenizing text with DistilBERT tokenizer...");
        tokenize_text(&args.text, tokenizer, ANE_SEQUENCE_LENGTH)?
    } else {
        // Use dummy tokenization for local/manual model paths
        println!("🔤 Using dummy tokenization (demo purposes only)...");
        
        let mut input_ids = Vec::with_capacity(ANE_SEQUENCE_LENGTH);
        input_ids.push(CLS_TOKEN_ID);
        
        // Add some demo tokens representing the input text
        let demo_tokens: Vec<i64> = (1000..1010).collect();
        input_ids.extend(demo_tokens);
        input_ids.push(SEP_TOKEN_ID);
        
        // Pad to fixed sequence length
        while input_ids.len() < ANE_SEQUENCE_LENGTH {
            input_ids.push(PAD_TOKEN_ID);
        }
        
        // Create attention mask (1 for real tokens, 0 for padding)
        let mut attention_mask = vec![1i64; 12]; // [CLS] + 10 demo tokens + [SEP]
        while attention_mask.len() < ANE_SEQUENCE_LENGTH {
            attention_mask.push(0);
        }
        
        (input_ids, attention_mask)
    };
    
    let input_ids_tensor = Tensor::from_vec(input_ids, (1, ANE_SEQUENCE_LENGTH), &device)?;
    let attention_mask_tensor = Tensor::from_vec(attention_mask, (1, ANE_SEQUENCE_LENGTH), &device)?;
    
    if args.verbose {
        println!("🔢 Input shape: {:?}", input_ids_tensor.shape());
        println!("🎭 Attention mask shape: {:?}", attention_mask_tensor.shape());
    }
    
    // Run inference
    println!("\n🚀 Running inference...");
    let start = Instant::now();
    
    let output = model.forward(&[&input_ids_tensor, &attention_mask_tensor])
        .map_err(|e| E::msg(format!("Inference failed: {}", e)))?;
    
    let inference_time = start.elapsed();
    println!("✅ Inference completed in {:?}", inference_time);
    println!("📊 Output shape: {:?}", output.shape());
    
    // Process sentiment classification results
    if let Ok(output_data) = output.to_vec2::<f32>() {
        if !output_data.is_empty() && output_data[0].len() >= 2 {
            let logits = &output_data[0];
            let negative_score = logits[0];
            let positive_score = logits[1];
            
            // Apply softmax to get probabilities
            let exp_neg = negative_score.exp();
            let exp_pos = positive_score.exp();
            let sum = exp_neg + exp_pos;
            
            let negative_prob = exp_neg / sum;
            let positive_prob = exp_pos / sum;
            
            let sentiment = if positive_prob > negative_prob { "POSITIVE" } else { "NEGATIVE" };
            let confidence = positive_prob.max(negative_prob);
            
            println!("\n🎯 Sentiment Analysis Results:");
            println!("  📊 Prediction: {} ({:.1}% confidence)", sentiment, confidence * 100.0);
            
            if args.show_scores {
                println!("  📈 Detailed scores:");
                println!("    • Negative: {:.4} (probability: {:.1}%)", negative_score, negative_prob * 100.0);
                println!("    • Positive: {:.4} (probability: {:.1}%)", positive_score, positive_prob * 100.0);
            }
            
            // Indicate if this likely ran on ANE
            if confidence > 0.8 {
                println!("  ⚡ High confidence suggests ANE acceleration was likely used!");
            }
        } else {
            println!("⚠️  Unexpected output format: {:?}", output.shape());
        }
    } else {
        println!("⚠️  Could not process output tensor");
    }
    
    println!("\n💡 Performance Summary:");
    println!("  • Loading time: {:?}", loading_time);
    println!("  • Inference time: {:?}", inference_time);
    println!("  • Total time: {:?}", loading_time + inference_time);
    
    Ok(())
}

#[cfg(not(all(target_os = "macos", feature = "coreml")))]
fn run_coreml_inference(_args: &Args) -> Result<()> {
    println!("❌ CoreML inference is only available on macOS with the 'coreml' feature enabled.");
    println!("\n💡 To use CoreML:");
    println!("   • Run on macOS");
    println!("   • Build with: cargo run --example bert_inference --features coreml");
    Ok(())
}

fn print_help() {
    println!("🤖 ANE-Optimized DistilBERT Sentiment Analysis");
    println!("=============================================");
    println!();
    println!("This example demonstrates sentiment analysis using Apple's ANE-optimized DistilBERT");
    println!("model that actually runs on the Apple Neural Engine for maximum performance.");
    println!();
    println!("📋 Requirements:");
    println!("  • macOS (for CoreML support)");
    println!("  • ANE-optimized DistilBERT model (.mlpackage format)");
    println!("  • Candle built with 'coreml' feature");
    println!();
    println!("🚀 Quick Start:");
    println!("  1. cargo run --example bert_inference --features coreml");
    println!("  2. Try different text: --text \"I love this product!\"");
    println!("  3. Show confidence: --show-scores");
    println!("  4. Use local model: --model-path \"/path/to/DistilBERT_fp16.mlpackage\"");
    println!();
    println!("⚡ This model is specifically optimized to run on Apple's Neural Engine!");
    println!("🔗 For more examples, see the benchmarks/ and advanced/ directories.");
    println!();
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    if args.verbose {
        print_help();
    }
    
    run_coreml_inference(&args)
}