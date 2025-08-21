# Using Custom ANEMLL Models with candle-coreml

This guide shows you how to use your own custom ANEMLL models with candle-coreml.

## Quick Start

The easiest way to use any ANEMLL model is with `UnifiedModelLoader`:

```rust
use candle_coreml::UnifiedModelLoader;

let loader = UnifiedModelLoader::new()?;
let model = loader.load_model("your-org/your-custom-model")?;

// Use the model immediately - no configuration needed
let result = model.forward_text("Your input text here")?;
```

This automatically:
- Downloads the model from HuggingFace
- Detects the model architecture 
- Generates the correct configuration
- Loads and initializes the model

## When You Need Manual Configuration

You may need manual configuration if:
- Your model files are not on HuggingFace
- You need to override automatic detection
- You're working with experimental model architectures

## Step 1: Generate Configuration

Use the discovery tool to analyze your model files:

```bash
python tools/discover_shapes.py \
    --model-dir /path/to/your/model \
    --output my-model-config.json
```

This creates a JSON configuration file describing your model's architecture.

## Step 2: Load Your Model

```rust
use candle_coreml::{QwenModel, QwenConfig, ModelConfig};
use std::path::Path;

// Load the generated configuration
let model_config = ModelConfig::load_from_file("my-model-config.json")?;
let qwen_config = QwenConfig::from_model_config(model_config);

// Load your model
let model_dir = Path::new("/path/to/your/model");
let model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;

// Use the model
let result = model.forward_text("Hello world")?;
```

## Understanding Model Types

### Single-Token Models
Optimized for inference-only tasks (like text correction):
- Process one token at a time
- Lower memory usage
- Faster for short outputs

### Batch Processing Models  
Standard ANEMLL models for general text generation:
- Process multiple tokens efficiently
- Higher throughput
- Better for longer text generation

The library automatically detects which type you have.

## Common Issues and Solutions

### "Shape mismatch" errors
**Problem**: Your model expects different tensor shapes than detected.

**Solution**: Check that your model files are complete and not corrupted. Re-run the discovery tool with `--verbose` to see detailed analysis.

### "Missing component" errors
**Problem**: Required model files (.mlpackage) are missing.

**Solution**: Ensure all model files are in the same directory. ANEMLL models typically need:
- Embeddings component
- FFN (feed-forward network) component  
- LM head (language model head) component

### Model loads but gives poor results
**Problem**: Model configuration may be incorrect for your use case.

**Solution**: Verify the model was trained for your intended task. Check vocabulary size and context length match your expectations.

## Advanced: Manual Configuration

If automatic detection doesn't work, you can create a configuration manually:

```rust
use candle_coreml::{ModelConfig, ShapeConfig, ModelInfo};

let model_config = ModelConfig {
    model_info: ModelInfo {
        model_id: Some("my-custom-model".to_string()),
        model_type: "qwen".to_string(),
        path: Some("/path/to/model".to_string()),
        discovered_at: None,
    },
    shapes: ShapeConfig {
        batch_size: 1,
        context_length: 256,
        hidden_size: 1024,
        vocab_size: 151936,
    },
    components: HashMap::new(), // Let the system discover components
    naming: Default::default(),
    ffn_execution: None,
};

let qwen_config = QwenConfig::from_model_config(model_config);
```

## Testing Your Integration

```rust
#[test]
fn test_my_custom_model() {
    let loader = UnifiedModelLoader::new().unwrap();
    let model = loader.load_model("my-org/my-model").unwrap();
    
    // Test with known input
    let result = model.forward_text("test input").unwrap();
    assert!(!result.is_empty());
    
    // Test configuration
    assert_eq!(model.config.batch_size(), 1);
    assert!(model.config.vocab_size() > 0);
}
```

## Best Practices

1. **Use UnifiedModelLoader first** - It handles most cases automatically
2. **Test with simple inputs** - Verify the model works before complex use cases  
3. **Check model documentation** - Understand what your model was trained for
4. **Version control configurations** - Save generated JSON files in your project
5. **Monitor performance** - Different model architectures have different characteristics

## Getting Help

If you encounter issues:

1. Run discovery tool with `--verbose` for detailed analysis
2. Check that all model files are present and valid
3. Verify your model is a supported ANEMLL architecture
4. Test with the simplest possible input first

## Example: Complete Integration

```rust
use anyhow::Result;
use candle_coreml::UnifiedModelLoader;

fn main() -> Result<()> {
    // Method 1: Automatic (recommended)
    let loader = UnifiedModelLoader::new()?;
    let mut model = loader.load_model("your-org/your-model")?;
    
    // Method 2: Manual configuration (if needed)
    // let model_config = ModelConfig::load_from_file("config.json")?;
    // let qwen_config = QwenConfig::from_model_config(model_config);
    // let mut model = QwenModel::load_from_directory("model/", Some(qwen_config))?;
    
    // Test the model
    let input = "The quick brown fox";
    let output = model.forward_text(input)?;
    
    println!("Input: {}", input);
    println!("Output: {}", output);
    
    Ok(())
}
```

This covers the most common scenarios for integrating custom models. The library is designed to work automatically in most cases, with manual configuration available when needed.