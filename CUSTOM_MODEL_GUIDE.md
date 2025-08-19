# Custom Qwen ANEMLL Model Configuration Guide

This guide explains how to configure your own Qwen ANEMLL model with custom shapes using the Python shape discovery tool and integrate it with the candle-coreml library.
## Prerequisites

- Python 3.7+ with `coremltools` installed
- Your custom ANEMLL model files (.mlpackage or .mlmodelc)

### Installing Dependencies

```bash
pip install coremltools
```
## Overview

The candle-coreml library supports any ANEMLL model architecture through a configuration system that defines:

1. **Model shapes** (batch_size, context_length, hidden_size, vocab_size)
2. **Component mappings** (embeddings, FFN, LM head)
3. **Input/output tensor specifications**
4. **Explicit file paths** (no globbing or discovery)

## Step 1: Discover Model Shapes

Use the Python discovery tool to extract shape information from your CoreML model files.

### Basic Usage

For a single model directory:

```bash
python tools/discover_shapes.py \
  --model-dir /path/to/your/custom-qwen-model \
  --output configs/custom-qwen.json \
  --verbose
```
For multiple models in a directory:

```bash
python tools/discover_shapes.py \
  --scan-directory /path/to/models/directory \
  --output-dir configs/ \
  --verbose
```
### Example Output

The tool will generate a configuration file like this:

```json
{
  "model_info": {
    "path": "/path/to/your/custom-qwen-model",
    "model_type": "qwen",
    "discovered_at": "2025-01-15T10:30:00.123456"
  },
  "shapes": {
    "batch_size": 1,
    "context_length": 256,
    "hidden_size": 1024,
    "vocab_size": 151669
  },
  "components": {
    "embeddings": {
      "file_path": "/path/to/your/custom-qwen-model/custom_embeddings.mlpackage",
      "inputs": {
        "input_ids": {
          "name": "input_ids",
          "shape": [1, 1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "ffn_infer": {
      "file_path": "/path/to/your/custom-qwen-model/custom_FFN_lut4_chunk_01of01.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [1],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 1, 256],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "lm_head": {
      "file_path": "/path/to/your/custom-qwen-model/custom_lm_head_lut6.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "outputs": {
        "logits1": {
          "name": "logits1",
          "shape": [1, 1, 9480],
          "data_type": "FLOAT32"
        },
        "logits2": {
          "name": "logits2",
          "shape": [1, 1, 9480],
          "data_type": "FLOAT32"
        },
        "logits16": {
          "name": "logits16",
          "shape": [1, 1, 9479],
          "data_type": "FLOAT32"
        }
      },
      "functions": []
    }
  },
  "naming": {
    "embeddings_pattern": null,
    "ffn_infer_pattern": null,
    "lm_head_pattern": null
  }
}
```
## Step 2: Understanding the Configuration

### Model Shapes

...

### Component Types

...

### Multipart Logits

...

## Step 3: Integration with candle-coreml

### Runtime Configuration (Recommended)

Load your model configuration at runtime:

```rust
use candle_coreml::{QwenModel, QwenConfig, ModelConfig};
use std::path::Path;

// Load configuration from JSON file
let config_path = "configs/custom-qwen.json";
let model_config = ModelConfig::load_from_file(config_path)?;

// Create QwenConfig from the loaded configuration
let qwen_config = QwenConfig::from_model_config(model_config);

// Load the model
let model_dir = Path::new("/path/to/your/custom-qwen-model");
let mut qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;

// Use the model
let result = qwen_model.forward_text("Your input text here")?;
```
### Automatic Configuration (Recommended)

The easiest way to use your custom model is with the UnifiedModelLoader, which automatically generates configurations:

```rust
use candle_coreml::UnifiedModelLoader;

let loader = UnifiedModelLoader::new()?;
let model = loader.load_model("your-org/custom-qwen-model")?;
// Configuration is automatically generated from the downloaded .mlpackage files
```

### Manual Configuration (Advanced)

If you need more control, you can create a configuration file manually:

```rust
let model_config = ModelConfig {
    model_info: ModelInfo {
        model_id: Some("your-org/custom-qwen-model".to_string()),
        model_type: "qwen".to_string(),
        ..Default::default()
    },
    shapes: ShapeConfig {
        batch_size: 1,
        context_length: 256,
        hidden_size: 1024,
        vocab_size: 151669,
    },
    // ... rest of your configuration
    ..Default::default()
};

let qwen_config = QwenConfig::from_model_config(model_config);
let mut qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;
```
## Step 4: Model-Specific Considerations

### Single-Token vs Batch Processing Models

...

### FFN Component Variations

...

## Step 5: Validation and Testing

### Validate Configuration

...

### Test Integration

...

## Step 6: Common Model Patterns

### Pattern 1: Fine-tuned Single-Token Model

...

### Pattern 2: Standard ANEMLL Model

...

### Pattern 3: Custom Context Length

...

## Troubleshooting

### Common Issues

...

### Shape Mismatch: "MultiArray shape (1 x 64 x 1024) does not match (1 x 1 x 1024)"

Root cause:
You are passing the full sequence hidden_states tensor produced by embeddings/prefill (shape like [1, 64, hidden]) directly into an ffn_infer CoreML function that was compiled to accept only the last token (shape [1, 1, hidden]). Some fine‑tuned single‑token generation exports (e.g. typo‑fixer) separate prefill and infer: prefill consumes the full sequence; infer expects a single step.

Fixes:
1. Configuration: Ensure the ffn_infer.hidden_states input shape in your JSON is [1, 1, hidden_size].
2. Runtime safeguard: The library auto‑slices the last token if it detects ffn_infer expects seq len 1.
3. If building your own model export, compile infer graph with a dynamic or single‑token sequence dimension consistent with your config.

Also verify position_ids: prefill may expect shape [1] (last position) instead of [sequence_length]. Update the config's ffn_prefill.position_ids shape accordingly; do not confuse model batch_size with the number of tokens in the prompt.

### Debug Mode

...

### Manual Configuration Fixes

...

## Example Integration

Here's a complete example showing how to integrate a custom model:

```rust
use anyhow::Result;
use candle_coreml::{QwenModel, QwenConfig, ModelConfig};
use std::path::Path;

fn load_custom_model() -> Result<QwenModel> {
  // Load model configuration
  let config_path = "configs/my-custom-qwen.json";
  let model_config = ModelConfig::load_from_file(config_path)?;
    
  // Create QwenConfig
  let qwen_config = QwenConfig::from_model_config(model_config);
    
  // Verify configuration
  println!("Model Configuration:");
  println!("  Batch Size: {}", qwen_config.batch_size());
  println!("  Context Length: {}", qwen_config.context_length());
  println!("  Hidden Size: {}", qwen_config.hidden_size());
  println!("  Vocab Size: {}", qwen_config.vocab_size());
  println!("  Multipart Logits: {}", qwen_config.has_multipart_logits());
    
  // Load model
  let model_dir = Path::new("models/my-custom-qwen");
  let qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;
    
  Ok(qwen_model)
}

fn main() -> Result<()> {
  let mut model = load_custom_model()?;
    
  // Test the model
  let input_text = "The quick brown fox";
  let result = model.forward_text(input_text)?;
  println!("Generated token: {}", result);
    
  Ok(())
}
```
## Best Practices

1. **Always use the discovery tool first** - It provides the most accurate configuration
2. **Validate configurations** - Test with small inputs before production use
3. **Version control configurations** - Keep JSON files in your repository
4. **Document custom models** - Include model purpose, training details, and expected behavior
5. **Test edge cases** - Verify behavior with empty inputs, maximum context length, etc.
6. **Monitor performance** - Custom shapes may have different performance characteristics

## Contributing

If you create configurations for commonly used models, consider contributing them to the built-in registry via pull request. This helps the community and ensures your models work out-of-the-box for other users.
# Custom Qwen ANEMLL Model Configuration Guide

This guide explains how to configure your own Qwen ANEMLL model with custom shapes using the Python shape discovery tool and integrate it with the candle-coreml library.

## Prerequisites

- Python 3.7+ with `coremltools` installed
- Your custom ANEMLL model files (.mlpackage or .mlmodelc)
- Rust/candle-coreml development environment

### Installing Dependencies

```bash
pip install coremltools
```

## Overview

The candle-coreml library supports any ANEMLL model architecture through a configuration system that defines:

1. **Model shapes** (batch_size, context_length, hidden_size, vocab_size)
2. **Component mappings** (embeddings, FFN, LM head)
3. **Input/output tensor specifications**
4. **File naming patterns**

## Step 1: Discover Model Shapes

Use the Python discovery tool to automatically extract shape information from your CoreML model files.

### Basic Usage

For a single model directory:

```bash
python tools/discover_shapes.py \
    --model-dir /path/to/your/custom-qwen-model \
    --output configs/custom-qwen.json \
    --verbose
```

For multiple models in a directory:

```bash
python tools/discover_shapes.py \
    --scan-directory /path/to/models/directory \
    --output-dir configs/ \
    --verbose
```

### Example Output

The tool will generate a configuration file like this:

```json
{
  "model_info": {
    "path": "/path/to/your/custom-qwen-model",
    "model_type": "qwen",
    "discovered_at": "2025-01-15T10:30:00.123456"
  },
  "shapes": {
    "batch_size": 1,
    "context_length": 256,
    "hidden_size": 1024,
    "vocab_size": 151669
  },
  "components": {
    "embeddings": {
      "file_path": "/path/to/your/custom-qwen-model/custom_embeddings.mlpackage",
      "inputs": {
        "input_ids": {
          "name": "input_ids",
          "shape": [1, 1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "ffn_infer": {
      "file_path": "/path/to/your/custom-qwen-model/custom_FFN_lut4_chunk_01of01.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        },
        "position_ids": {
          "name": "position_ids",
          "shape": [1],
          "data_type": "INT32"
        },
        "causal_mask": {
          "name": "causal_mask",
          "shape": [1, 1, 1, 256],
          "data_type": "FLOAT16"
        },
        "current_pos": {
          "name": "current_pos",
          "shape": [1],
          "data_type": "INT32"
        }
      },
      "outputs": {
        "output_hidden_states": {
          "name": "output_hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "functions": []
    },
    "lm_head": {
      "file_path": "/path/to/your/custom-qwen-model/custom_lm_head_lut6.mlpackage",
      "inputs": {
        "hidden_states": {
          "name": "hidden_states",
          "shape": [1, 1, 1024],
          "data_type": "FLOAT16"
        }
      },
      "outputs": {
        "logits1": {
          "name": "logits1",
          "shape": [1, 1, 9480],
          "data_type": "FLOAT32"
        },
        "logits2": {
          "name": "logits2",
          "shape": [1, 1, 9480],
          "data_type": "FLOAT32"
        },
        "logits16": {
          "name": "logits16",
          "shape": [1, 1, 9479],
          "data_type": "FLOAT32"
        }
      },
      "functions": []
    }
  },
  "naming": {
    "embeddings_pattern": "custom_embeddings.mlpackage",
    "ffn_infer_pattern": "custom_FFN_lut*_chunk_*.mlpackage",
    "lm_head_pattern": "custom_lm_head_lut*.mlpackage"
  }
}
```

## Step 2: Understanding the Configuration

### Model Shapes

- **batch_size**: Number of sequences processed simultaneously (often 1 for inference, 64 for standard ANEMLL)
- **context_length**: Maximum sequence length the model can handle
- **hidden_size**: Dimensionality of hidden states (typically 1024, 2048, 4096, etc.)
- **vocab_size**: Total vocabulary size (sum of all logits parts for multipart models)

### Component Types

- **embeddings**: Converts token IDs to hidden states
- **ffn_prefill**: Feed-forward network for prefill phase (processes multiple tokens)
- **ffn_infer**: Feed-forward network for inference phase (processes single token)
- **lm_head**: Language model head that produces logits for next token prediction

### Multipart Logits

Some models split the vocabulary across multiple outputs (e.g., 16 parts with ~9,480 tokens each):

```json
"outputs": {
  "logits1": {"shape": [1, 1, 9480]},
  "logits2": {"shape": [1, 1, 9480]},
  "logits16": {"shape": [1, 1, 9479]}
}
```

The discovery tool automatically detects this and calculates the total vocabulary size.

## Step 3: Integration with candle-coreml

### Option 1: Runtime Configuration (Recommended)

Load your model configuration at runtime:

```rust
use candle_coreml::{QwenModel, QwenConfig, ModelConfig};
use std::path::Path;

// Load configuration from JSON file
let config_path = "configs/custom-qwen.json";
let model_config = ModelConfig::load_from_file(config_path)?;

// Create QwenConfig from the loaded configuration
let qwen_config = QwenConfig::from_model_config(model_config);

// Load the model
let model_dir = Path::new("/path/to/your/custom-qwen-model");
let mut qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;

// Use the model
let result = qwen_model.forward_text("Your input text here")?;
```

### Option 2: Automatic Configuration with UnifiedModelLoader

The recommended approach is to use UnifiedModelLoader for automatic configuration:

```rust
use candle_coreml::UnifiedModelLoader;

let loader = UnifiedModelLoader::new()?;
let model = loader.load_model("your-org/custom-qwen-model")?;
// Configuration is automatically generated from the downloaded .mlpackage files
```

This approach provides:
- Automatic model downloading from HuggingFace
- Dynamic configuration generation
- No need for manual shape configuration
- Support for any ANEMLL model architecture

## Step 4: Model-Specific Considerations

### Single-Token vs Batch Processing Models

**Single-Token Models** (batch_size=1, embeddings input [1,1]):
- Optimized for inference-only use cases
- Lower memory usage
- Typical for fine-tuned models

```json
"shapes": {
  "batch_size": 1,
  "context_length": 256
},
"components": {
  "embeddings": {
    "inputs": {
      "input_ids": {"shape": [1, 1]}
    }
  }
}
```

**Batch Processing Models** (batch_size=64, embeddings input [1,64]):
- Standard ANEMLL configuration
- Supports both prefill and inference phases
- Higher throughput for multi-token processing

```json
"shapes": {
  "batch_size": 64,
  "context_length": 512
},
"components": {
  "embeddings": {
    "inputs": {
      "input_ids": {"shape": [1, 64]}
    }
  }
}
```

### FFN Component Variations

**Separate Prefill/Infer Components**:
```json
"components": {
  "ffn_prefill": {
    "inputs": {
      "hidden_states": {"shape": [1, 64, 1024]},
      "causal_mask": {"shape": [1, 1, 64, 512]}
    }
  },
  "ffn_infer": {
    "inputs": {
      "hidden_states": {"shape": [1, 1, 1024]},
      "causal_mask": {"shape": [1, 1, 1, 512]}
    }
  }
}
```

**Single FFN Component**:
```json
"components": {
  "ffn_prefill": {
    "functions": ["prefill", "infer"],
    "inputs": {
      "hidden_states": {"shape": [1, 64, 1024]}
    }
  }
}
```

## Step 5: Validation and Testing

### Validate Configuration

```bash
# Run shape discovery with validation
python tools/discover_shapes.py \
    --model-dir /path/to/your/model \
    --output config.json \
    --verbose
```

The tool will warn about:
- Missing required components
- Unusual shape values
- Inconsistent tensor shapes

### Test Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_model_loading() {
        let config_path = "configs/custom-qwen.json";
        let model_config = ModelConfig::load_from_file(config_path).unwrap();
        let qwen_config = QwenConfig::from_model_config(model_config);
        
        // Verify expected shapes
        assert_eq!(qwen_config.batch_size(), 1);
        assert_eq!(qwen_config.context_length(), 256);
        assert_eq!(qwen_config.vocab_size(), 151669);
        
        // Test model loading (if model files available)
        // let model_dir = Path::new("/path/to/your/model");
        // let qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config));
        // assert!(qwen_model.is_ok());
    }
}
```

## Step 6: Common Model Patterns

### Pattern 1: Fine-tuned Single-Token Model

Typical for task-specific models (e.g., text correction, classification):

```json
{
  "shapes": {
    "batch_size": 1,
    "context_length": 128,
    "vocab_size": 151669
  },
  "components": {
    "embeddings": {
      "inputs": {"input_ids": {"shape": [1, 1]}}
    },
    "lm_head": {
      "outputs": {
        "logits1": {"shape": [1, 1, 9480]},
        "logits16": {"shape": [1, 1, 9479]}
      }
    }
  }
}
```

### Pattern 2: Standard ANEMLL Model

General-purpose models for text generation:

```json
{
  "shapes": {
    "batch_size": 64,
    "context_length": 512,
    "vocab_size": 151936
  },
  "components": {
    "embeddings": {
      "inputs": {"input_ids": {"shape": [1, 64]}}
    },
    "lm_head": {
      "outputs": {"logits": {"shape": [1, 1, 151936]}}
    }
  }
}
```

### Pattern 3: Custom Context Length

Models with non-standard context windows:

```json
{
  "shapes": {
    "batch_size": 32,
    "context_length": 1024,
    "vocab_size": 200000
  },
  "components": {
    "ffn_prefill": {
      "inputs": {
        "causal_mask": {"shape": [1, 1, 32, 1024]}
      }
    }
  }
}
```

## Troubleshooting

### Common Issues

**Shape Mismatch Errors**:
```
Error: CoreML stateful prediction error: MultiArray shape (64) does not match the shape (1) specified in the model description
```

Solution: Verify the embeddings input shape in your configuration matches your model's actual input requirements.

**Missing Components**:
```
Warning: Missing required components: ['lm_head']
```

Solution: Ensure all required model files are present and correctly named. Check the file naming patterns.

**Incorrect Vocabulary Size**:
```
Warning: Unusual vocab_size: 1234567
```

Solution: For multipart logits, verify the discovery tool correctly summed all logits outputs. Manually check if needed.

### Debug Mode

Run with verbose output to see detailed analysis:

```bash
python tools/discover_shapes.py \
    --model-dir /path/to/your/model \
    --output config.json \
    --verbose
```

This will show:
- Component detection process
- Shape extraction for each tensor
- Validation warnings
- Inferred model parameters

### Manual Configuration Fixes

If the discovery tool makes incorrect assumptions, manually edit the generated JSON:

```json
{
  "shapes": {
    "batch_size": 1,           // ← Manually corrected
    "context_length": 256,     // ← Manually corrected
    "hidden_size": 1024,
    "vocab_size": 151669
  }
}
```

## Example Integration

Here's a complete example showing how to integrate a custom model:

```rust
use anyhow::Result;
use candle_coreml::{QwenModel, QwenConfig, ModelConfig};
use std::path::Path;

fn load_custom_model() -> Result<QwenModel> {
    // Load model configuration
    let config_path = "configs/my-custom-qwen.json";
    let model_config = ModelConfig::load_from_file(config_path)?;
    
    // Create QwenConfig
    let qwen_config = QwenConfig::from_model_config(model_config);
    
    // Verify configuration
    println!("Model Configuration:");
    println!("  Batch Size: {}", qwen_config.batch_size());
    println!("  Context Length: {}", qwen_config.context_length());
    println!("  Hidden Size: {}", qwen_config.hidden_size());
    println!("  Vocab Size: {}", qwen_config.vocab_size());
    println!("  Multipart Logits: {}", qwen_config.has_multipart_logits());
    
    // Load model
    let model_dir = Path::new("models/my-custom-qwen");
    let qwen_model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;
    
    Ok(qwen_model)
}

fn main() -> Result<()> {
    let mut model = load_custom_model()?;
    
    // Test the model
    let input_text = "The quick brown fox";
    let result = model.forward_text(input_text)?;
    println!("Generated token: {}", result);
    
    Ok(())
}
```

## Best Practices

1. **Always use the discovery tool first** - It provides the most accurate configuration
2. **Validate configurations** - Test with small inputs before production use
3. **Version control configurations** - Keep JSON files in your repository
4. **Document custom models** - Include model purpose, training details, and expected behavior
5. **Test edge cases** - Verify behavior with empty inputs, maximum context length, etc.
6. **Monitor performance** - Custom shapes may have different performance characteristics

## Contributing

If you create configurations for commonly used models, consider contributing them to the built-in registry via pull request. This helps the community and ensures your models work out-of-the-box for other users.