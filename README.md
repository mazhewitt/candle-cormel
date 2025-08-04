# candle-coreml

CoreML inference engine for Candle tensors - providing Apple CoreML integration for Rust machine learning applications.

## Overview

`candle-coreml` is a standalone crate that bridges [Candle](https://github.com/huggingface/candle) tensors with Apple's CoreML framework, enabling efficient on-device inference on macOS and iOS. Unlike generic CoreML bindings, this crate provides:

- **Candle-specific integration** - Direct tensor conversion and device validation
- **Inference engine approach** - CoreML as an inference backend, not a device type
- **Apple Silicon optimization** - Leverages unified memory architecture
- **Production ready** - Comprehensive error handling and testing

## Key Features

- ✅ **Direct Candle tensor support** - CPU and Metal tensor inference
- ✅ **Device validation** - Automatic device compatibility checking  
- ✅ **Unified memory** - Efficient tensor conversion using M1/M2 architecture
- ✅ **Error handling** - Candle-compatible error types and messages
- ✅ **Comprehensive testing** - Unit tests, integration tests, and real model testing
- ✅ **Cross-platform builds** - Compiles on all platforms, runs on macOS

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
candle-coreml = "0.2.1"
candle-core = "0.9.1"
```

Basic usage:

```rust
use candle_core::{Device, Tensor};
use candle_coreml::{Config, CoreMLModel};

// Create config for your model
let config = Config {
    input_names: vec!["input_ids".to_string()],
    output_name: "logits".to_string(),
    max_sequence_length: 128,
    vocab_size: 32000,
    model_type: "YourModel".to_string(),
};

// Load CoreML model (no device parameter needed)
let model = CoreMLModel::load_from_file("model.mlmodelc", &config)?;

// Create input tensor on CPU or Metal
let device = Device::Cpu;
let input = Tensor::ones((1, 128), candle_core::DType::F32, &device)?;

// Run inference (device validation happens automatically)
let output = model.forward(&input)?;

// Output tensor uses same device as input
assert_eq!(output.device(), input.device());
```

## 🔥 ANEMLL Models: Multi-Component ANE Architecture

**[ANEMLL](https://github.com/Anemll/Anemll)** (pronounced "animal") provides state-of-the-art Apple Neural Engine optimizations for large language models. Our crate provides comprehensive support for ANEMLL's multi-component architecture.

### Why ANEMLL?

ANEMLL converts large models into **multiple specialized components** that maximize Apple Neural Engine utilization:

- **🚀 True ANE Acceleration**: Models specifically optimized for Apple's Neural Engine
- **💾 Memory Efficiency**: Component splitting reduces peak memory usage
- **⚡ Optimized Performance**: Custom quantization (LUT4/LUT6) for ANE constraints
- **🔧 Production Ready**: Used in real iOS/macOS apps via TestFlight

### Supported Models

| Model | Size | Context | Components | Status |
|-------|------|---------|------------|--------|
| Qwen 3 | 0.5B-7B | 512-32K | 3-part split | ✅ Fully Supported |
| Qwen 2.5 | 0.5B-7B | 512-32K | 3-part split | ✅ Fully Supported |

### Multi-Component Architecture

ANEMLL splits models into specialized components for optimal ANE performance:

```
Input Tokens → [Embeddings] → [FFN Transformer] → [LM Head] → Output Logits
               ↓              ↓                   ↓
               embeddings.    FFN_chunk_01.      lm_head.
               mlmodelc       mlmodelc           mlmodelc
```

#### Component Details:
1. **Embeddings Model** (`qwen_embeddings.mlmodelc`)
   - Converts token IDs to hidden representations
   - Output: `[batch, seq_len, hidden_dim]`

2. **FFN Model** (`qwen_FFN_PF_lut8_chunk_01of01.mlmodelc`) 
   - Transformer feed-forward network with attention
   - Includes causal masking for autoregressive generation
   - Output: `[batch, seq_len, hidden_dim]`

3. **LM Head Model** (`qwen_lm_head_lut8.mlmodelc`)
   - Final linear layer producing vocabulary logits
   - Input: Last position hidden state `[batch, 1, hidden_dim]`
   - Output: `[batch, 1, vocab_size]`

### Quick Start with ANEMLL Models

```rust
use candle_coreml::QwenModel;

// Load complete multi-component model
let model = QwenModel::load_from_hub(
    "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4"
)?;

// Generate text using all components
let response = model.generate(
    "Hello, how are you?",
    50,  // max tokens
    0.8  // temperature
)?;
```

### Manual Component Loading

For advanced use cases, load components individually:

```rust
use candle_coreml::{CoreMLModel, Config};

// Load each component with specific configs
let embeddings = CoreMLModel::load_from_file("qwen_embeddings.mlmodelc", &embed_config)?;
let ffn = CoreMLModel::load_from_file("qwen_FFN_PF_lut8_chunk_01of01.mlmodelc", &ffn_config)?;
let lm_head = CoreMLModel::load_from_file("qwen_lm_head_lut8.mlmodelc", &head_config)?;

// Orchestrate pipeline manually
let hidden = embeddings.forward(&[&input_ids])?;
let processed = ffn.forward(&[&hidden, &causal_mask])?;
let logits = lm_head.forward(&[&processed.i((.., -1.., ..))?])?;
```

### Examples and Demos

```bash
# 🌟 Integration patterns demo (works immediately)
cargo run --example qwen_demo_patterns

# Full multi-component chat (downloads ~2GB models)
cargo run --example qwen_multi_component

# Performance benchmarks
cargo run --example qwen_benchmark
```

### Model Download and Setup

ANEMLL models are hosted on HuggingFace and downloaded automatically:

```bash
# Models are cached in ~/.cache/candle-coreml/
# First run downloads all components (~2GB for Qwen 0.6B)

# Available models:
# - anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4
# - anemll/anemll-Qwen-Qwen2.5-0.5B-ctx512_0.3.4
# - More models available at: https://huggingface.co/anemll
```

### Performance Characteristics

- **Context Length**: Optimized for 512-2048 tokens (up to 32K supported)
- **Quantization**: LUT4/LUT6 optimizations for ANE constraints  
- **Memory**: Component splitting reduces peak usage vs monolithic models
- **Speed**: True ANE acceleration vs GPU/CPU fallback

### Reference Implementation

ANEMLL provides reference apps showing production usage:
- **TestFlight App**: [Join Beta](https://testflight.apple.com/join/jrQq1D1C)
- **iOS/macOS Support**: Complete mobile deployment examples
- **GitHub**: [ANEMLL Repository](https://github.com/Anemll/Anemll)

### Integration with candle-coreml

Our crate provides the missing piece for Rust developers wanting to use ANEMLL's optimized models:

- ✅ **Automatic component discovery and loading**
- ✅ **Pipeline orchestration with proper data flow**  
- ✅ **Causal masking for transformer architectures**
- ✅ **HuggingFace integration for seamless model access**
- ✅ **Streaming generation with multi-component coordination**

This makes ANEMLL's advanced ANE optimizations accessible to the entire Candle ecosystem.

**📚 [Complete ANEMLL Integration Guide](ANEMLL_GUIDE.md)** - Comprehensive documentation covering architecture, usage patterns, and production deployment.

## Architecture

This crate follows the **inference engine** pattern rather than treating CoreML as a device backend:

- **Accepts**: CPU and Metal tensors via Candle's unified memory
- **Rejects**: CUDA tensors with clear error messages  
- **Output**: Tensors on the same device as input
- **Conversion**: Automatic F32/I64→I32 tensor conversion as needed

## Comparison with coreml-rs

| Feature | coreml-rs | candle-coreml |
|---------|-----------|---------------|
| Bindings | swift-bridge | objc2 direct |
| Purpose | Generic CoreML | Candle tensor integration |
| API | Raw CoreML interface | Candle patterns (T5-like) |
| Error Handling | Generic | Candle error types |
| Device Support | Generic | CPU/Metal validation |

## 🔥 Complete Worked Example

**👉 [BERT CoreML Inference - Step-by-Step Guide](https://github.com/mazhewitt/candle-cormel/blob/main/examples/WORKED_EXAMPLE.md)**

A comprehensive tutorial covering:
- Model download and compilation
- End-to-end inference pipeline  
- Performance optimization tips
- ANE vs GPU vs CPU comparison
- Production deployment guidance

## ⚠️ When to Use CoreML (Important!)

### ✅ Use CoreML When:
- You have **CoreML-specific models** (`.mlpackage`/`.mlmodelc` files)
- You want **Apple Neural Engine (ANE)** acceleration for supported models
- You need Apple's automatic **hardware selection** (ANE → GPU → CPU)
- You're deploying specifically on **Apple platforms**

### ❌ Don't Use CoreML When:
- You can achieve the same performance with **Metal/CPU backends**
- Your model **isn't optimized** for Apple hardware  
- You need **cross-platform compatibility**
- You're **just starting** with Candle (try CPU/Metal first)

### 🧠 Apple Neural Engine (ANE) Reality Check

**Not all models run on the ANE!** Apple's Neural Engine has strict requirements:

- **Supported Operations**: Only a subset of ML operations are ANE-optimized
- **Model Architecture**: Models must be specifically designed/optimized for ANE  
- **Data Types**: Primarily supports certain quantized formats
- **Model Size**: Large models may fall back to GPU/CPU

**Recommendation**: Use Apple's pre-optimized models (like their optimized BERT) for guaranteed ANE acceleration, or stick with Metal/CPU backends for general use.

### 📊 Performance Hierarchy

```
ANE (fastest, most efficient) > GPU/Metal (fast) > CPU (most compatible)
```

Apple automatically chooses the best available backend, but your model must be ANE-compatible to benefit from the fastest option.

## Examples

See the `examples/` directory for:
- **[WORKED_EXAMPLE.md](examples/WORKED_EXAMPLE.md)** - Complete BERT inference tutorial
- **Basic inference** - Simple model loading and inference  
- **Benchmarks** - Performance comparisons vs Metal/CPU
- **Advanced usage** - Complex model configurations
- **🦙 Qwen Chat** - Real-world ANE-accelerated chat with Qwen 0.6B

### 🚀 New: Qwen 0.6B Multi-Component ANE Implementation

Complete implementation of Anemll's multi-component architecture:

```bash
# 🌟 Integration patterns demo (WORKS IMMEDIATELY)
cargo run --example qwen_demo_patterns

# Multi-component Qwen chat with real models 
cargo run --example qwen_multi_component

# Performance benchmarks and single-model examples
cargo run --example qwen_benchmark
cargo run --example qwen_chat
```

Features:
- **✅ Multi-Component Architecture**: Separate embeddings, FFN, and LM head models
- **✅ Pipeline Orchestration**: Proper data flow between model components  
- **✅ True ANE acceleration** using Anemll's optimized model components
- **✅ Causal Masking**: Correct transformer-style attention patterns
- **✅ HuggingFace integration** with automatic component download
- **✅ Comprehensive testing** of full pipeline

This demonstrates how to integrate complex, multi-file CoreML models with Candle, providing a foundation for advanced ANE-optimized architectures.

See [examples/qwen/README.md](examples/qwen/README.md) for detailed documentation.

## Platform Support

- **macOS**: Full CoreML runtime support
- **iOS**: Full CoreML runtime support (when targeting iOS)
- **Other platforms**: Builds successfully, runtime features disabled

## Contributing

This is an independent project providing CoreML integration for the Candle ecosystem. Contributions welcome!

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.