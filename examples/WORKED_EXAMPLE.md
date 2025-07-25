# ANE-Optimized DistilBERT Sentiment Analysis - Complete Worked Example

This guide demonstrates a complete end-to-end sentiment analysis pipeline using `candle-coreml` with Apple's ANE-optimized DistilBERT model that actually runs on the Apple Neural Engine.

## 🎯 What You'll Build

A DistilBERT sentiment analysis system that:
- Uses Apple's ANE-optimized model for true Neural Engine acceleration
- Downloads or uses local CoreML models (.mlpackage format)
- Handles automatic model compilation
- Performs efficient on-device sentiment classification
- Returns confidence scores for positive/negative sentiment

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

### Option A: Use Apple's ANE-Optimized Model (Recommended)

The example automatically downloads Apple's ANE-optimized DistilBERT:

```rust
use hf_hub::{api::sync::Api, Repo, RepoType};

// Download Apple's ANE-optimized DistilBERT model
let repo = Repo::with_revision("apple/ane-distilbert-base-uncased-finetuned-sst-2-english".to_string(), RepoType::Model, "main".to_string());
let api = Api::new()?;
let model_file = api.repo(repo).get("DistilBERT_fp16.mlpackage")?;
```

**⚡ This model is specifically optimized to run on Apple's Neural Engine!**

### Option B: Convert Your Own Model (Advanced)

If you want to convert your own model for ANE optimization, use Apple's ml-ane-transformers:

```python
# Follow Apple's ANE optimization guide
# GitHub: https://github.com/apple/ml-ane-transformers

import coremltools as ct
from ane_transformers.distilbert import distilbert_base_uncased_finetuned_sst_2_english
import torch

# Load ANE-optimized architecture
model = distilbert_base_uncased_finetuned_sst_2_english()
model.eval()

# Create example input (fixed sequence length for ANE optimization)
batch_size, seq_len = 1, 128
input_ids = torch.randint(0, 30522, (batch_size, seq_len))
attention_mask = torch.ones((batch_size, seq_len))

# Trace the model
traced_model = torch.jit.trace(model, (input_ids, attention_mask))

# Convert to CoreML with ANE-specific optimizations
coreml_model = ct.convert(
    traced_model,
    inputs=[
        ct.TensorType(name="input_ids", shape=(batch_size, seq_len)),
        ct.TensorType(name="attention_mask", shape=(batch_size, seq_len))
    ],
    compute_precision=ct.precision.FLOAT16,  # Required for ANE
    compute_units=ct.ComputeUnit.ALL  # Let CoreML choose best backend
)

# Save
coreml_model.save("DistilBERT_fp16.mlpackage")
```

**⚠️ Note**: Standard BERT models won't run on ANE - you need Apple's specialized architecture!

## ⚙️ Step 3: Core Implementation

```rust
use candle_core::{Device, Tensor};
use candle_coreml::{Config as CoreMLConfig, CoreMLModel};
use anyhow::Result;

fn run_sentiment_analysis() -> Result<()> {
    // 1. Configure the ANE-optimized DistilBERT model
    let config = CoreMLConfig {
        input_names: vec!["input_ids".to_string(), "attention_mask".to_string()],
        output_name: "logits".to_string(), // Sentiment classification outputs
        max_sequence_length: 128,
        vocab_size: 30522, // DistilBERT vocabulary size
        model_type: "ane-distilbert-base-uncased-finetuned-sst-2-english".to_string(),
    };
    
    // 2. Load the CoreML model
    let model = CoreMLModel::load_from_file("DistilBERT_fp16.mlpackage", &config)?;
    
    // 3. Prepare input tensors
    let device = Device::Cpu; // or Device::Metal(0) for Metal backend
    
    // Example: "The Neural Engine is really fast!"
    // In production, use proper DistilBERT tokenizer
    let input_ids = vec![101, 1996, 15756, 3194, 2003, 2428, 3435, 999, 102]; // Tokenized input
    let attention_mask = vec![1, 1, 1, 1, 1, 1, 1, 1, 1]; // All tokens are real
    
    let input_ids_tensor = Tensor::from_vec(
        input_ids, 
        (1, 9), 
        &device
    )?;
    
    let attention_mask_tensor = Tensor::from_vec(
        attention_mask, 
        (1, 9), 
        &device
    )?;
    
    // 4. Run inference (ANE acceleration happens automatically!)
    let output = model.forward(&[&input_ids_tensor, &attention_mask_tensor])?;
    
    // 5. Process sentiment results
    if let Ok(output_data) = output.to_vec2::<f32>() {
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
        
        println!("Sentiment: {} ({:.1}% confidence)", sentiment, confidence * 100.0);
    }
    
    Ok(())
}
```

## 🏃‍♂️ Step 4: Running the Example

The repository includes a complete working example:

```bash
# Basic inference with sample text
cargo run --example bert_inference

# Custom text (use [MASK] for the token to predict)
cargo run --example bert_inference -- --text "The weather today is [MASK]"

# Use your own model
cargo run --example bert_inference -- --model-path "/path/to/your/model.mlmodelc"

# Download from specific HuggingFace repository
cargo run --example bert_inference -- --model-id "apple/coremltools-models"
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