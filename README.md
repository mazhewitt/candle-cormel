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
candle-coreml = "0.1.0"
candle-core = "0.9.1"
```

Basic usage:

```rust
use candle_core::{Device, Tensor};
use candle_coreml::{Config, CoreMLModel};

// Create config for your model
let config = Config {
    input_name: "input_ids".to_string(),
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