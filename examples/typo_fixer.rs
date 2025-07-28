//! Qwen Typo Fixer Example
//!
//! This example demonstrates how to use the Qwen typo-fixer model
//! to automatically correct typos in text.

use candle_coreml::QwenModel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Qwen Typo Fixer Example");
    println!("==========================");

    // Load the typo-fixer model from HuggingFace
    println!("📥 Loading typo-fixer model...");
    let mut model = QwenModel::load_typo_fixer(true)?;

    // Initialize states for efficient generation
    println!("🔄 Initializing model states...");
    model.initialize_states()?;

    // Test text with various typos
    let test_texts = [
        "Ths is a sentance with severl typos.",
        "I cna't beleive how mny erors are in this txt.",
        "Pleas corect these mstakes for me.",
        "The qwick brown fox jumps over the lzy dog.",
    ];

    println!("\n🔍 Correcting typos...");
    println!("====================");

    for (i, text) in test_texts.iter().enumerate() {
        println!("\n{}. Original: {}", i + 1, text);

        // Generate corrected text with low temperature for consistent corrections
        let corrected = model.generate_text(text, 50, 0.1)?;
        println!("   Corrected: {}", corrected);
    }

    println!("\n✅ Typo correction complete!");
    Ok(())
}
