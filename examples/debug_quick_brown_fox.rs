//! Debug the "quick brown fox" prediction to understand what the model is actually doing

use anyhow::Result;
use candle_coreml::{QwenModel, QwenConfig};
use std::path::Path;

fn main() -> Result<()> {
    println!("🔍 Debugging 'The quick brown fox jumps over the lazy' prediction");
    println!("================================================================");
    
    let model_dir = Path::new("/Users/mazdahewitt/projects/candle-coreml/qwen-model");
    let config = QwenConfig::default();
    let mut model = QwenModel::load_from_directory(model_dir, Some(config))?;
    
    // Initialize states
    model.initialize_states()?;
    
    // Analyze the full phrase tokenization
    let full_phrase = "The quick brown fox jumps over the lazy";
    let full_tokens = model.tokenize(full_phrase)?;
    
    println!("📝 Full phrase: \"{}\"", full_phrase);
    println!("🔤 Full tokenization: {:?}", full_tokens);
    println!("   Token count: {}", full_tokens.len());
    
    // Analyze individual words
    let words = vec![
        "The", " quick", " brown", " fox", " jumps", " over", " the", " lazy"
    ];
    
    println!("\n📚 Word-by-word tokenization:");
    for (i, word) in words.iter().enumerate() {
        if let Ok(tokens) = model.tokenize(word) {
            println!("   {}: \"{}\" -> {:?}", i, word, tokens);
        }
    }
    
    // Check what comes after "lazy"
    println!("\n🎯 Possible completions to check:");
    let completions = vec![
        " dog", "dog", " cat", "cat", " animal", " one", " person", " man", " woman"
    ];
    
    for completion in &completions {
        if let Ok(tokens) = model.tokenize(completion) {
            if !tokens.is_empty() {
                println!("   \"{}\" -> token {}", completion, tokens[0]);
            }
        }
    }
    
    // Test the actual prediction
    println!("\n🚀 Running prediction...");
    let predicted_token = model.forward(full_phrase)?;
    println!("🎯 Predicted token: {}", predicted_token);
    
    // Try to understand what this token might represent by testing different phrases
    println!("\n🔍 Analyzing prediction context:");
    
    // Test if token 15678 appears in our tokenization
    if full_tokens.contains(&predicted_token) {
        let position = full_tokens.iter().position(|&t| t == predicted_token).unwrap();
        println!("   ✅ Token {} appears in input at position {}", predicted_token, position);
    } else {
        println!("   ➡️  Token {} is NEW (not in input)", predicted_token);
    }
    
    // Test different prefix lengths to see what the model predicts
    println!("\n🧪 Testing different prefix lengths:");
    let prefixes = vec![
        "The quick brown fox",
        "The quick brown fox jumps",
        "The quick brown fox jumps over",
        "The quick brown fox jumps over the",
        "The quick brown fox jumps over the lazy",
    ];
    
    for (i, prefix) in prefixes.iter().enumerate() {
        model.initialize_states()?; // Reset for each test
        let prediction = model.forward(prefix)?;
        println!("   {}: \"{}\" -> token {}", i + 1, prefix, prediction);
    }
    
    // Test classic completions to see if the model knows them
    println!("\n📖 Testing known completions:");
    let test_phrases = vec![
        ("The capital of France is", vec!["Paris", " Paris"]),
        ("Two plus two equals", vec!["four", " four", "4", " 4"]),
        ("The sky is", vec!["blue", " blue"]),
    ];
    
    for (phrase, expected_words) in test_phrases {
        model.initialize_states()?;
        let prediction = model.forward(phrase)?;
        
        println!("   \"{}\" -> token {}", phrase, prediction);
        
        // Check if prediction matches any expected words
        for word in expected_words {
            if let Ok(tokens) = model.tokenize(word) {
                if !tokens.is_empty() && tokens[0] == prediction {
                    println!("      ✅ Matches \"{}\"!", word);
                }
            }
        }
    }
    
    println!("\n🎯 Summary:");
    println!("   The model may be working correctly, but 'dog' might not be the most likely completion");
    println!("   Token {} could represent a different but valid word", predicted_token);
    println!("   This could indicate the model has different training than expected");
    
    Ok(())
}