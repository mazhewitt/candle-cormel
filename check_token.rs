use candle_coreml::{ensure_model_downloaded, qwen::{QwenModel, QwenConfig}};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4";
    let model_dir = ensure_model_downloaded(model_id, true)?;
    
    let config = QwenConfig::default();
    let qwen_model = QwenModel::load_from_directory(&model_dir, Some(config))?;
    
    // Check what token 15678 decodes to
    println!("Token 15678 decodes to: '{}'", qwen_model.tokenizer().decode(&[15678], false)?);
    println!("Token 5562 decodes to: '{}'", qwen_model.tokenizer().decode(&[5562], false)?);
    
    Ok(())
}