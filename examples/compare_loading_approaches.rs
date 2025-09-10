use anyhow::Result;
use candle_coreml::{ensure_model_downloaded, QwenConfig, QwenModel, UnifiedModelLoader};

fn main() -> Result<()> {
    println!("🔍 Comparing model loading approaches");
    println!("====================================");

    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let prompt = "The quick brown fox jumps over the lazy";

    // Approach 1: UnifiedModelLoader (new approach causing issues)
    println!("\n🔄 Testing UnifiedModelLoader approach...");
    let loader = UnifiedModelLoader::new()?;
    let mut model_new = loader.load_model(model_id)?;
    let token_new = model_new.forward_text(prompt)?;

    if let Ok(decoded) = model_new.tokenizer().decode(&[token_new as u32], false) {
        println!("🎯 UnifiedModelLoader: Token {token_new} -> '{decoded}'");
    }

    // Approach 2: Manual loading with QwenConfig (old working approach)
    println!("\n🔄 Testing manual loading approach...");
    let model_dir = ensure_model_downloaded(model_id, false)?;
    #[allow(deprecated)]
    let config = QwenConfig::for_model_id(model_id)?;
    let mut model_old = QwenModel::load_from_directory(&model_dir, Some(config))?;
    let token_old = model_old.forward_text(prompt)?;

    if let Ok(decoded) = model_old.tokenizer().decode(&[token_old as u32], false) {
        println!("🎯 Manual loading: Token {token_old} -> '{decoded}'");
    }

    // Compare configurations
    println!("\n📊 Configuration comparison:");
    let unified_config = loader.generate_config(model_id)?;
    #[allow(deprecated)]
    let manual_config = QwenConfig::for_model_id(model_id)?;

    println!(
        "Unified config batch size: {:?}",
        unified_config.components.iter().find_map(|(name, c)| {
            if name.contains("embeddings") {
                c.inputs.get("input_ids").map(|t| &t.shape)
            } else {
                None
            }
        })
    );

    println!(
        "Manual config batch size: {:?}",
        manual_config
            .model_config
            .components
            .iter()
            .find_map(|(name, c)| {
                if name.contains("embeddings") {
                    c.inputs.get("input_ids").map(|t| &t.shape)
                } else {
                    None
                }
            })
    );

    // Test consistency
    println!("\n🔬 Testing consistency:");
    for i in 1..=3 {
        let token = model_new.forward_text(prompt)?;
        println!("  UnifiedModelLoader run {i}: {token}");
    }

    for i in 1..=3 {
        let token = model_old.forward_text(prompt)?;
        println!("  Manual loading run {i}: {token}");
    }

    Ok(())
}
