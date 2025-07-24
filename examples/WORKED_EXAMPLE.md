# BERT CoreML Inference - Complete Worked Example

This guide demonstrates a complete end-to-end BERT inference pipeline using `candle-coreml`, from model acquisition to inference execution.

## 🎯 What You'll Build

A BERT fill-mask inference system that:
- Downloads or uses local CoreML models
- Handles automatic model compilation
- Performs efficient on-device inference
- Returns top-k token predictions

## 📋 Prerequisites

- **macOS**: CoreML only runs on Apple platforms
- **Xcode Command Line Tools**: For model compilation (`xcode-select --install`)
- **Rust**: With `candle-coreml` dependency

## 🚀 Step 1: Project Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
candle-core = "0.9.1"
candle-coreml = "0.1.0"
tokenizers = "0.20.3"  # For proper tokenization
hf-hub = "0.3"         # For model downloads
anyhow = "1.0"         # Error handling
```

## 🤖 Step 2: Model Acquisition

### Option A: Use HuggingFace Hub (Recommended)

The example automatically downloads optimized CoreML models:

```rust
use hf_hub::{api::sync::Api, Repo, RepoType};

// Download Apple's optimized BERT CoreML model
let repo = Repo::with_revision("apple/coremltools-models".to_string(), RepoType::Model, "main".to_string());
let api = Api::new()?;
let model_file = api.repo(repo).get("text/bert-base-uncased/bert-base-uncased.mlpackage")?;
```

### Option B: Convert Your Own Model

If you have a PyTorch BERT model, convert it using Apple's CoreML Tools:

```python
# Python conversion script
import coremltools as ct
from transformers import BertForMaskedLM
import torch

# Load your model
model = BertForMaskedLM.from_pretrained("bert-base-uncased")
model.eval()

# Create example input
batch_size, seq_len = 1, 128
input_ids = torch.randint(0, 30522, (batch_size, seq_len))
attention_mask = torch.ones((batch_size, seq_len))

# Trace the model
traced_model = torch.jit.trace(model, (input_ids, attention_mask))

# Convert to CoreML
coreml_model = ct.convert(
    traced_model,
    inputs=[
        ct.TensorType(name="input_ids", shape=(batch_size, seq_len)),
        ct.TensorType(name="attention_mask", shape=(batch_size, seq_len))
    ]
)

# Save
coreml_model.save("bert-base-uncased.mlpackage")
```

## ⚙️ Step 3: Core Implementation

```rust
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
use anyhow::Result;

fn run_bert_inference() -> Result<()> {
    // 1. Configure the model
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string(), "attention_mask".to_string()],
        output_name: "token_scores".to_string(),
        max_sequence_length: 128,
        vocab_size: 30522, // BERT vocabulary size
        model_type: "bert-base-uncased".to_string(),
    };
    
    // 2. Load the CoreML model
    let model = CoreMLModel::load_from_file("bert-base-uncased.mlmodelc", &config)?;
    
    // 3. Prepare input tensors
    let device = Device::Cpu; // or Device::Metal(0) for Metal backend
    
    // Example: "Paris is the [MASK] of France"
    let input_ids = vec![2023, 2003, 1996, 103, 1997, 2605]; // Tokenized input
    let attention_mask = vec![1, 1, 1, 1, 1, 1]; // All tokens are real
    
    let input_ids_tensor = Tensor::from_vec(
        input_ids, 
        (1, 6), 
        &device
    )?;
    
    let attention_mask_tensor = Tensor::from_vec(
        attention_mask, 
        (1, 6), 
        &device
    )?;
    
    // 4. Run inference
    let output = model.forward(&[&input_ids_tensor, &attention_mask_tensor])?;
    
    // 5. Process results
    let predictions = extract_top_predictions(&output, mask_position: 3, top_k: 5)?;
    
    for (i, (token_id, score)) in predictions.iter().enumerate() {
        println!("{}. Token {}: {:.4}", i + 1, token_id, score);
    }
    
    Ok(())
}
```

## 🏃‍♂️ Step 4: Running the Example

The repository includes a complete working example:

```bash
# Basic inference with sample text
cargo run --example bert_inference --features coreml

# Custom text (use [MASK] for the token to predict)
cargo run --example bert_inference --features coreml -- --text "The weather today is [MASK]"

# Use your own model
cargo run --example bert_inference --features coreml -- --model-path "/path/to/your/model.mlmodelc"

# Download from specific HuggingFace repository
cargo run --example bert_inference --features coreml -- --model-id "apple/coremltools-models"
```

## 🧠 Understanding CoreML Performance

### Apple Neural Engine (ANE) Considerations

**⚠️ Important**: Not all models run on the ANE! Apple's Neural Engine has strict requirements:

1. **Supported Operations**: Only a subset of ML operations are ANE-optimized
2. **Model Architecture**: Models must be specifically designed/optimized for ANE
3. **Data Types**: Primarily supports certain quantized formats
4. **Model Size**: Large models may fall back to GPU/CPU

### Performance Hierarchy

```
ANE (fastest, most efficient) > GPU (fast) > CPU (most compatible)
```

**Apple automatically chooses the best backend**, but you can influence this through:
- Model optimization during conversion
- Using Apple's pre-optimized models
- Specific CoreML compilation flags

### When to Use CoreML

✅ **Use CoreML when**:
- You have CoreML-specific models (`.mlpackage`/`.mlmodelc`)
- You need ANE acceleration for supported models
- You want Apple's automatic hardware selection
- You're deploying specifically on Apple platforms

❌ **Don't use CoreML when**:
- You can achieve the same performance with Metal/CPU backends
- Your model isn't optimized for Apple hardware
- You need cross-platform compatibility
- You're just starting with Candle (try CPU/Metal first)

## 📊 Performance Benchmarking

Compare CoreML vs other backends:

```rust
// Benchmark different backends
let backends = vec![
    ("CoreML", Device::Cpu), // Note: CoreML chooses hardware internally
    ("Metal", Device::Metal(0)),  
    ("CPU", Device::Cpu),
];

for (name, device) in backends {
    let start = std::time::Instant::now();
    let output = run_inference_on_device(&model, &input, &device)?;
    let duration = start.elapsed();
    println!("{}: {:?}", name, duration);
}
```

## 🔗 Additional Resources

- **Apple's CoreML Models**: [HuggingFace Hub - Apple](https://huggingface.co/apple)
- **CoreML Tools Documentation**: [Apple Developer](https://developer.apple.com/documentation/coreml)
- **ANE Optimization Guide**: [Apple Machine Learning Research](https://machinelearning.apple.com/)
- **BERT Paper**: [Attention Is All You Need](https://arxiv.org/abs/1706.03762)
- **Candle Framework**: [GitHub Repository](https://github.com/huggingface/candle)

## 🛠️ Troubleshooting

### Common Issues

1. **"Model not found"**: Ensure `.mlmodelc` or `.mlpackage` files exist
2. **"Compilation failed"**: Install Xcode Command Line Tools
3. **Slow performance**: Your model may not be ANE-optimized
4. **Memory errors**: Reduce batch size or sequence length

### Debug Tips

```rust
// Enable verbose logging
let config = CoreMLConfig {
    // ... other config
    debug: true, // Enable debug output
};

// Check actual model input/output names
// (They might differ from documentation)
```

## 🎉 Next Steps

1. **Try different models**: Experiment with various BERT variants
2. **Optimize for ANE**: Use Apple's conversion tools with ANE-specific flags
3. **Profile performance**: Use Instruments.app to analyze model execution
4. **Scale up**: Implement batch processing for multiple inputs

This example provides a solid foundation for building production CoreML inference systems with Candle!