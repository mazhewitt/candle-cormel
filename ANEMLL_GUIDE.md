# ANEMLL Integration Guide

Complete guide to using [ANEMLL](https://github.com/Anemll/Anemll) (Apple Neural Engine Machine Learning Library) models with candle-coreml.

## What is ANEMLL?

**ANEMLL** (pronounced "animal") is an open-source project that provides optimized Large Language Models specifically designed for Apple's Neural Engine (ANE). Unlike generic CoreML conversions, ANEMLL models are:

- **ANE-First Design**: Built specifically to maximize Apple Neural Engine utilization
- **Multi-Component Architecture**: Models split into specialized components for optimal memory usage
- **Production-Tested**: Used in real iOS/macOS applications available on TestFlight
- **Quantization Optimized**: Custom LUT4/LUT6 quantization for ANE constraints

## Why Multi-Component Architecture?

Traditional single-file models often fall back to GPU/CPU because they exceed ANE constraints. ANEMLL solves this by splitting models into components that each fit perfectly within ANE limits:

```
🚫 Traditional: [Large Monolithic Model] → GPU/CPU Fallback
✅ ANEMLL:      [Embeddings] + [FFN] + [LM Head] → Pure ANE Acceleration
```

### Benefits:
- **🚀 True ANE Speed**: Each component runs natively on Neural Engine
- **💾 Lower Memory**: Peak memory reduced through component staging
- **⚡ Better Latency**: ANE is faster than GPU/CPU for these workloads
- **🔋 Power Efficient**: ANE uses significantly less power

## Supported Models

| Model Family | Sizes | Context Length | Components | Status |
|--------------|-------|----------------|------------|--------|
| **Qwen 3** | 0.5B, 1.5B, 3B, 7B | 512-32K | 3-part | ✅ Full Support |
| **Qwen 2.5** | 0.5B, 1.5B, 3B, 7B | 512-32K | 3-part | ✅ Full Support |

### Model Variants Available:

```bash
# Qwen 3 Series (Recommended)
anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4
anemll/anemll-Qwen-Qwen3-1.5B-ctx512_0.3.4
anemll/anemll-Qwen-Qwen3-3B-ctx512_0.3.4

# Qwen 2.5 Series
anemll/anemll-Qwen-Qwen2.5-0.5B-ctx512_0.3.4
anemll/anemll-Qwen-Qwen2.5-1.5B-ctx512_0.3.4

# Browse all: https://huggingface.co/anemll
```

## 🚀 Getting Started with ANEMLL Models

### Modern API (Recommended)

The easiest way to use ANEMLL models is through the UnifiedModelLoader:

```rust
use candle_coreml::UnifiedModelLoader;

// Create loader with automatic caching and config generation
let loader = UnifiedModelLoader::new()?;

// Load any ANEMLL model - automatically downloads and sets up components
let mut model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;

// High-level text completion (recommended)  
let response = model.complete_text(
    "Explain quantum computing in simple terms:",
    100,  // max tokens
    0.8   // temperature
)?;

// Advanced generation with top-k sampling
let tokens = model.generate_tokens_topk_temp(
    "Hello, world!",
    50,       // max tokens
    0.7,      // temperature  
    Some(40)  // top_k
)?;

// Single token prediction
let next_token = model.forward_text("The weather is")?;
```

### Key Benefits of UnifiedModelLoader:

- **🎯 Zero Configuration**: Automatic model downloading and config generation
- **📦 Intelligent Caching**: Models and configs cached locally for fast subsequent loads  
- **🔍 Automatic Validation**: Built-in model validation and error checking
- **🧠 Multi-Component Support**: Handles complex ANEMLL architectures seamlessly
- **⚡ Optimized Performance**: Efficient component orchestration and memory management

### Running Examples

```bash
# Interactive chat with ANEMLL Qwen models
cargo run --example qwen_chat

# Recommended API demonstration
cargo run --example recommended_api_demo

# Model quality testing
cargo run --example proper_quality_test
```

## Architecture Deep Dive

### Component Pipeline

ANEMLL splits transformer models into three specialized components:

```
Input: [Token IDs] 
    ↓
[1. Embeddings Model]
    ↓ Hidden States [batch, seq_len, hidden_dim]
[2. FFN Transformer] ← Causal Mask
    ↓ Processed States [batch, seq_len, hidden_dim]  
[3. LM Head Model]
    ↓
Output: [Vocabulary Logits]
```

#### 1. Embeddings Component (`qwen_embeddings.mlmodelc`)
- **Purpose**: Convert token IDs to dense representations
- **Input**: Token IDs `[batch_size, sequence_length]` (Int32)
- **Output**: Hidden states `[batch_size, sequence_length, hidden_dim]` (Float32)
- **ANE Optimization**: Embedding lookup optimized for ANE memory patterns

#### 2. FFN Transformer (`qwen_FFN_PF_lut8_chunk_01of01.mlmodelc`)
- **Purpose**: Core transformer processing with attention and feed-forward
- **Inputs**: 
  - Hidden states `[batch_size, sequence_length, hidden_dim]` (Float32)
  - Causal mask `[1, 1, 1, sequence_length]` (Float32)
- **Output**: Processed hidden states `[batch_size, sequence_length, hidden_dim]` (Float32)
- **ANE Optimization**: Attention and FFN layers quantized to LUT8 for ANE

#### 3. LM Head (`qwen_lm_head_lut8.mlmodelc`)
- **Purpose**: Convert final hidden state to vocabulary probabilities
- **Input**: Last position hidden state `[batch_size, 1, hidden_dim]` (Float32)
- **Output**: Vocabulary logits `[batch_size, 1, vocab_size]` (Float32)
- **ANE Optimization**: Final linear layer quantized for maximum ANE utilization

### Causal Masking

The FFN component requires proper causal masking for autoregressive generation:

```rust
// Causal mask prevents looking at future tokens
// Shape: [1, 1, 1, sequence_length]
// Values: 0.0 for allowed positions, -inf for masked positions

let mut mask_data = vec![f32::NEG_INFINITY; sequence_length];
for i in 0..=current_position {
    mask_data[i] = 0.0;  // Allow access to current and previous tokens
}
let causal_mask = Tensor::from_vec(mask_data, (1, 1, 1, sequence_length), device)?;
```

## Integration with candle-coreml

### High-Level API (Recommended)

Our `QwenModel` provides a complete abstraction over ANEMLL's multi-component architecture:

```rust
use candle_coreml::QwenModel;

// Load model with automatic component discovery
let model = QwenModel::load_from_hub(
    "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4"
)?;

// Generate text with streaming support
let response = model.generate(
    "The future of AI is",  // prompt
    100,                    // max tokens
    0.8,                    // temperature
    None,                   // top_p (None = use defaults)
)?;

println!("Generated: {}", response);
```

### Component-Level API (Advanced)

For fine-grained control, load and orchestrate components manually:

```rust
use candle_coreml::{CoreMLModel, Config};
use candle_core::{Device, Tensor};

// Configure each component
let device = Device::Cpu;

let embed_config = Config {
    input_names: vec!["input_ids".to_string()],
    output_name: "hidden_states".to_string(),
    max_sequence_length: 512,
    vocab_size: 151936,
    model_type: "qwen-embeddings".to_string(),
};

let ffn_config = Config {
    input_names: vec!["hidden_states".to_string(), "causal_mask".to_string()],
    output_name: "processed_states".to_string(),
    max_sequence_length: 512,
    vocab_size: 151936,
    model_type: "qwen-ffn".to_string(),
};

let head_config = Config {
    input_names: vec!["hidden_states".to_string()],
    output_name: "logits".to_string(),
    max_sequence_length: 1,
    vocab_size: 151936,
    model_type: "qwen-head".to_string(),
};

// Load components
let embeddings = CoreMLModel::load_from_file("qwen_embeddings.mlmodelc", &embed_config)?;
let ffn = CoreMLModel::load_from_file("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc", &ffn_config)?;
let lm_head = CoreMLModel::load_from_file("qwen_lm_head_lut8.mlmodelc", &head_config)?;

// Manual pipeline orchestration
fn run_pipeline(
    input_ids: &Tensor,
    causal_mask: &Tensor,
    embeddings: &CoreMLModel,
    ffn: &CoreMLModel,
    lm_head: &CoreMLModel,
) -> Result<Tensor> {
    // Step 1: Convert tokens to embeddings
    let hidden_states = embeddings.forward(&[input_ids])?;
    
    // Step 2: Process through transformer with masking
    let processed_states = ffn.forward(&[&hidden_states, causal_mask])?;
    
    // Step 3: Get logits for last position only
    let last_hidden = processed_states.i((.., -1.., ..))?;  // [batch, 1, hidden_dim]
    let logits = lm_head.forward(&[&last_hidden])?;
    
    Ok(logits)
}
```

### Streaming Generation

For real-time applications, implement token-by-token generation:

```rust
use tokenizers::Tokenizer;

fn generate_streaming(
    model: &QwenModel,
    tokenizer: &Tokenizer,
    prompt: &str,
    max_tokens: usize,
    temperature: f32,
) -> Result<String> {
    let mut generated_text = prompt.to_string();
    let mut token_count = 0;
    
    while token_count < max_tokens {
        // Tokenize current text
        let encoding = tokenizer.encode(&generated_text, false)?;
        let tokens: Vec<i64> = encoding.get_ids().iter().map(|&id| id as i64).collect();
        
        // Get next token logits
        let logits = model.forward_tokens(&tokens)?;
        
        // Sample next token with temperature
        let next_token_id = sample_with_temperature(&logits, temperature)?;
        
        // Convert back to text
        let next_token = tokenizer.decode(&[next_token_id as u32], false)?;
        generated_text.push_str(&next_token);
        
        // Check for end of sequence
        if next_token_id == tokenizer.get_vocab().get("</s>").copied().unwrap_or(2) as i64 {
            break;
        }
        
        token_count += 1;
        
        // Optional: Print streaming output
        print!("{}", next_token);
        io::stdout().flush()?;
    }
    
    Ok(generated_text)
}
```

## Performance Optimization

### Context Length Recommendations

ANEMLL models support various context lengths but perform optimally within certain ranges:

| Context Length | Performance | Use Case |
|----------------|-------------|----------|
| **512 tokens** | ⭐⭐⭐⭐⭐ Optimal | Chat, Q&A, Short generation |
| **1024 tokens** | ⭐⭐⭐⭐ Excellent | Document summarization |
| **2048 tokens** | ⭐⭐⭐ Good | Long-form content |
| **4096+ tokens** | ⭐⭐ Fair | May fall back to GPU/CPU |

### Memory Usage

Multi-component architecture provides several memory advantages:

```rust
// Traditional single model: Peak memory = Full model size
// ANEMLL: Peak memory = Largest component + intermediate tensors

// Example for Qwen 0.6B:
// - Single model:     ~600MB peak
// - Multi-component:  ~200MB peak (embeddings) + ~150MB (processing)
```

### Quantization Levels

ANEMLL uses specialized quantization for ANE:

- **LUT4**: 4-bit lookup table quantization (highest compression)
- **LUT6**: 6-bit lookup table quantization (balanced)
- **LUT8**: 8-bit lookup table quantization (highest quality)

Model filenames indicate quantization level:
```
qwen_FFN_PF_lut8_chunk_01of01.mlmodelc  # 8-bit quantization
qwen_lm_head_lut6.mlmodelc              # 6-bit quantization
```

## Model Download and Caching

### Automatic Download

candle-coreml automatically downloads ANEMLL models from HuggingFace:

```rust
// First run downloads all components (~2GB for Qwen 0.6B)
let model = QwenModel::load_from_hub("anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")?;

// Subsequent runs use cached models (instant loading)
```

### Cache Location

Models are cached in platform-appropriate directories:

```bash
# macOS/Linux
~/.cache/candle-coreml/anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4/

# Windows  
%LOCALAPPDATA%\candle-coreml\anemll\anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4\
```

### Manual Download

For offline use or CI environments:

```bash
# Install HuggingFace CLI
pip install huggingface_hub[cli]

# Download specific model
huggingface-cli download anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4 --local-dir ./qwen-model

# Use local path in code
let model = QwenModel::load_from_directory("./qwen-model")?;
```

## Examples and Demos

Our repository includes comprehensive examples:

### 1. Integration Patterns Demo
```bash
# Shows multi-component coordination (works without downloads)
cargo run --example qwen_demo_patterns
```

### 2. Full Multi-Component Chat
```bash
# Real ANEMLL model chat interface (downloads models)
cargo run --example qwen_multi_component

# With options
cargo run --example qwen_multi_component -- --temperature 0.8 --max-tokens 100
```

### 3. Performance Benchmarks
```bash
# Compare ANE vs GPU vs CPU performance
cargo run --example qwen_benchmark

# Specific model and settings
cargo run --example qwen_benchmark -- --model anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4 --sequences 10
```

### 4. Component-Level Example
```bash
# Manual component loading and orchestration
cargo run --example qwen_chat --help
```

## Production Deployment

### iOS/macOS Apps

ANEMLL provides reference implementations:

1. **TestFlight Beta**: [Join here](https://testflight.apple.com/join/jrQq1D1C)
   - Complete iOS/macOS chat app
   - Shows real-world integration patterns
   - Demonstrates offline operation

2. **Model Formats**:
   - **macOS**: Supports both `.zip` and unzipped `.mlmodelc` files
   - **iOS**: Requires unzipped `.mlmodelc` files for app bundle inclusion

### Bundle Size Considerations

```bash
# Model sizes (uncompressed):
# Qwen 0.6B: ~2GB total
# - qwen_embeddings.mlmodelc: ~400MB
# - qwen_FFN_PF_lut8_chunk_01of01.mlmodelc: ~1.2GB  
# - qwen_lm_head_lut8.mlmodelc: ~400MB

# For iOS apps, consider:
# - On-device vs on-demand download
# - Component-wise loading based on features
# - Progressive model loading
```

### Error Handling and Fallbacks

```rust
use candle_coreml::{QwenModel, CoreMLError};

fn robust_model_loading(model_id: &str) -> Result<QwenModel> {
    match QwenModel::load_from_hub(model_id) {
        Ok(model) => Ok(model),
        Err(CoreMLError::ModelNotFound(_)) => {
            // Fallback to smaller model
            QwenModel::load_from_hub("anemll/anemll-Qwen-Qwen3-0.5B-ctx512_0.3.4")
        },
        Err(CoreMLError::IncompatibleDevice) => {
            // Non-macOS platform - return appropriate error
            Err(CoreMLError::IncompatibleDevice)
        },
        Err(e) => Err(e),
    }
}
```

## Troubleshooting

### Common Issues

**1. Model Download Fails**
```rust
// Solution: Check network connectivity and HuggingFace access
// Alternative: Download manually and use local path
```

**2. ANE Not Utilized**
```bash
# Check in Console.app for CoreML logs:
# "Using ANE" vs "Using GPU" vs "Using CPU"

# Ensure:
# - Model files are valid .mlmodelc format
# - Context length within optimal range (≤2048)
# - macOS with Apple Silicon (M1/M2/M3)
```

**3. High Memory Usage**
```rust
// Solution: Process in smaller batches
let chunk_size = 256;  // Reduce from 512
for chunk in input_tokens.chunks(chunk_size) {
    let output = model.forward_tokens(chunk)?;
    // Process output...
}
```

**4. Slow Performance**
```bash
# Check Activity Monitor for:
# - ANE utilization (should be >0%)
# - Memory pressure (should be green)
# - Thermal state (avoid throttling)
```

### Debugging Tools

```rust
// Enable verbose logging
std::env::set_var("RUST_LOG", "candle_coreml=debug");

// Check component loading
let model = QwenModel::load_from_hub(model_id)?;
println!("Components loaded: {:?}", model.component_info());

// Verify ANE usage (check Console.app logs)
let output = model.forward_tokens(&tokens)?;
```

## Community and Support

- **ANEMLL GitHub**: [https://github.com/Anemll/Anemll](https://github.com/Anemll/Anemll)
- **ANEMLL HuggingFace**: [https://huggingface.co/anemll](https://huggingface.co/anemll)
- **ANEMLL Twitter**: [@anemll](https://x.com/anemll)
- **candle-coreml Issues**: [GitHub Issues](https://github.com/mazhewitt/candle-cormel/issues)

## License and Attribution

- **ANEMLL Models**: Check individual model cards for licensing (typically Apache 2.0 or MIT)
- **Original Models**: Qwen models require Alibaba's license for commercial use
- **candle-coreml**: MIT OR Apache-2.0 license

When using ANEMLL models in your projects:

```rust
// Give credit to ANEMLL in your documentation:
// "This project uses ANEMLL (https://github.com/Anemll/Anemll) 
//  for Apple Neural Engine optimized language models."
```

---

This integration makes ANEMLL's cutting-edge ANE optimizations accessible to the entire Rust and Candle ecosystem, enabling developers to build fast, efficient, on-device AI applications.