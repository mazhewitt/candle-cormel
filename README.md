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

## Examples

See the `examples/` directory for:
- **Basic inference** - Simple model loading and inference
- **Benchmarks** - Performance comparisons
- **Advanced usage** - Complex model configurations

## Platform Support

- **macOS**: Full CoreML runtime support
- **iOS**: Full CoreML runtime support (when targeting iOS)
- **Other platforms**: Builds successfully, runtime features disabled

## Contributing

This is an independent project providing CoreML integration for the Candle ecosystem. Contributions welcome!

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.