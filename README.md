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
candle-coreml = "0.2.4"
candle-core = "0.9.1"
```

Basic usage with UnifiedModelLoader (Recommended):

```rust
use candle_coreml::UnifiedModelLoader;

// Load model directly from HuggingFace with automatic setup
let loader = UnifiedModelLoader::new()?;
let mut model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;

// Generate text using the new API
let response = model.complete_text(
    "Hello, how are you?",
    50,   // max tokens  
    0.8,  // temperature
)?;

println!("Response: {}", response);
```

Manual CoreML model loading:

```rust
use candle_coreml::{CoreMLModel, ModelConfig};

// Load model config (typically auto-generated)
let config = ModelConfig::load_from_file("model_config.json")?;

// Load CoreML model components
let model = CoreMLModel::load_from_file("model.mlpackage", &config)?;

// Create input tensor
let input = candle_core::Tensor::zeros((1, 128), candle_core::DType::I64, &candle_core::Device::Cpu)?;

// Run inference
let output = model.forward(&[input])?;
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
use candle_coreml::UnifiedModelLoader;

// Load complete multi-component model with automatic setup
let loader = UnifiedModelLoader::new()?;
let mut model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;

// Generate text using the new API methods
let response = model.complete_text(
    "Hello, how are you?",
    50,   // max tokens
    0.8,  // temperature  
)?;

// Or use the more advanced generation method
let tokens = model.generate_tokens_topk_temp(
    "Hello, how are you?",
    50,   // max tokens
    0.8,  // temperature
    Some(50), // top_k
)?;
```

### Manual Component Loading

For advanced use cases, load components individually:

```rust
use candle_coreml::{CoreMLModel, ModelConfig, QwenModel, QwenConfig};

// Option 1: Load from directory with auto-generated config  
let model_dir = "/path/to/downloaded/model";
let mut model = QwenModel::load_from_directory(&model_dir, None)?;

// Option 2: Manual component loading with ModelConfig
let config = ModelConfig::load_from_file("model_config.json")?;
let embeddings = CoreMLModel::load_from_file("embeddings.mlpackage", &config)?;
let ffn_prefill = CoreMLModel::load_from_file("ffn_prefill.mlpackage", &config)?;
let ffn_infer = CoreMLModel::load_from_file("ffn_infer.mlpackage", &config)?;
let lm_head = CoreMLModel::load_from_file("lm_head.mlpackage", &config)?;

// Use the high-level API for text generation
let response = model.complete_text("Hello!", 20, 0.7)?;
```

### Examples and Demos

```bash
# Recommended API demonstration
cargo run --example recommended_api_demo

# Multi-component chat with Qwen models (downloads ~2GB models)  
cargo run --example qwen_chat

# Test thinking behavior and quality
cargo run --example test_thinking_behavior
cargo run --example proper_quality_test

# Performance comparisons
cargo run --example compare_loading_approaches
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

- ✅ **Explicit component selection via file paths (no globbing/discovery)**
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

**👉 [BERT CoreML Inference - Step-by-Step Guide](examples/WORKED_EXAMPLE.md)**

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

## 🚀 Modern API: UnifiedModelLoader

The recommended approach for loading and using models is through the `UnifiedModelLoader`, which handles:

- **Automatic HuggingFace Downloads**: Models are downloaded and cached automatically  
- **Config Generation**: Model configurations are generated from the downloaded files
- **Validation**: Comprehensive model validation and error checking
- **Caching**: Intelligent caching of both models and configurations

### UnifiedModelLoader Examples

```rust
use candle_coreml::UnifiedModelLoader;

// Create loader (initializes cache and config generation)
let loader = UnifiedModelLoader::new()?;

// Load any ANEMLL model from HuggingFace
let mut model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;

// Available generation methods:
// 1. High-level text completion (recommended)
let response = model.complete_text("Hello, world!", 50, 0.8)?;

// 2. Advanced token generation with top-k sampling  
let tokens = model.generate_tokens_topk_temp("Hello!", 20, 0.7, Some(40))?;

// 3. Single token prediction
let next_token = model.forward_text("Hello")?;

// 4. Text generation with parameters
let result = model.generate_text_with_params("Hello!", 30, 0.9)?;
```

### QwenModel API Reference

The `QwenModel` provides several methods for text generation:

| Method | Description | Use Case |
|--------|-------------|-----------|
| `complete_text(prompt, max_tokens, temperature)` | **Recommended** - High-level text completion | General text generation |
| `generate_tokens_topk_temp(prompt, max_tokens, temp, top_k)` | Advanced generation with top-k sampling | Fine-tuned control over generation |
| `forward_text(text)` | Single token prediction | Next token prediction, embeddings |  
| `generate_text_with_params(prompt, max_tokens, temperature)` | Text generation with custom parameters | Custom generation logic |
| ~~`generate_tokens()`~~ | **Deprecated** - Use `generate_tokens_topk_temp()` instead | Legacy compatibility only |

### Cache Management

Models and configs are cached automatically:

```rust
// Models cached in: ~/.cache/candle-coreml/models/  
// Configs cached in: ~/.cache/candle-coreml/configs/

// Clear caches if needed
use candle_coreml::CacheManager;
let cache = CacheManager::new()?;
// cache.clear_model_cache()?; // if needed
```

## Model Configuration System (Advanced Usage)

Complex multi-component language models (e.g. ANEMLL Qwen variants, custom fine-tunes) are described declaratively using a `ModelConfig` JSON file. This removes hardcoded shapes and enables:

- Explicit component file paths (no globbing)
- Per-component input/output tensor shapes & dtypes
- Multipart logits combination (auto-detected part count)
- Split vs unified FFN execution (`ffn_execution` = `split` | `unified`)
- Automatic detection of prefill mode (batched vs sequential single-token)

### Minimal Example

```jsonc
{
    "model_info": { "model_type": "qwen", "path": "/path/to/model" },
    "shapes": { "batch_size": 64, "context_length": 256, "hidden_size": 1024, "vocab_size": 151669 },
    "components": {
        "embeddings": { "file_path": "embeddings.mlpackage", "inputs": { "input_ids": {"shape": [1,64], "data_type": "INT32", "name": "input_ids" } }, "outputs": { "hidden_states": {"shape": [1,64,1024], "data_type": "FLOAT16", "name": "hidden_states" } }, "functions": [] },
        "ffn_prefill": { "file_path": "ffn_prefill.mlpackage", "inputs": { "hidden_states": {"shape": [1,64,1024], "data_type": "FLOAT16","name":"hidden_states"}, "position_ids": {"shape":[64],"data_type":"INT32","name":"position_ids"}, "causal_mask": {"shape":[1,1,64,256],"data_type":"FLOAT16","name":"causal_mask"}, "current_pos": {"shape":[1],"data_type":"INT32","name":"current_pos"} }, "outputs": { "output_hidden_states": {"shape":[1,1,1024],"data_type":"FLOAT16","name":"output_hidden_states"} }, "functions":["prefill"] },
        "ffn_infer": { "file_path": "ffn_infer.mlpackage", "inputs": { "hidden_states": {"shape": [1,1,1024], "data_type": "FLOAT16","name":"hidden_states"}, "position_ids": {"shape":[1],"data_type":"INT32","name":"position_ids"}, "causal_mask": {"shape":[1,1,1,256],"data_type":"FLOAT16","name":"causal_mask"}, "current_pos": {"shape":[1],"data_type":"INT32","name":"current_pos"} }, "outputs": { "output_hidden_states": {"shape":[1,1,1024],"data_type":"FLOAT16","name":"output_hidden_states"} }, "functions":["infer"] },
        "lm_head": { "file_path": "lm_head.mlpackage", "inputs": { "hidden_states": {"shape":[1,1,1024],"data_type":"FLOAT16","name":"hidden_states" } }, "outputs": { "logits1": {"shape":[1,1,9480],"data_type":"FLOAT16","name":"logits1"}, "logits2": {"shape":[1,1,9479],"data_type":"FLOAT16","name":"logits2"} }, "functions": [] }
    },
    "ffn_execution": "split"
}
```

### Execution Modes

| Mode | When | Behavior |
|------|------|----------|
| `unified` | Single CoreML package exposes `prefill` & `infer` functions | Shared file, one state, batched prefill then token-by-token infer |
| `split` | Separate `ffn_prefill` & `ffn_infer` model files | Distinct model files; state created from prefill model and reused for infer |

If `ffn_execution` is omitted, the system infers `split` when `ffn_prefill.file_path != ffn_infer.file_path`.

### Prefill Modes

Prefill can be either batch (process full sequence in one call) or sequential (one token at a time). Sequential mode is auto-enabled when `ffn_prefill.hidden_states` shape has `seq_len == 1` (e.g. `[1,1,H]`) indicating a single-token CoreML prefill variant. This matches certain fine-tuned or distilled models exported with single-token kernels.

### Multipart Logits

The LM head may output `logits1..logitsN`. The library detects count dynamically and stitches them into a contiguous logits tensor. No manual configuration needed beyond listing outputs.

### Validation

`ModelConfig::validate()` checks basic consistency; `validate_internal_wiring()` ensures adjacent component tensor shapes align (e.g. embeddings → ffn_prefill). Warnings are logged but loading proceeds to aid iterative development.

### Custom Model Guide

See `CUSTOM_MODEL_GUIDE.md` for deep-dive shape discovery tooling and advanced customization.

### Migrating From Globs

Legacy filename pattern discovery has been removed. Always set `file_path` for each component—this avoids ambiguity and improves reproducibility.

### Troubleshooting

| Symptom | Likely Cause | Fix |
|---------|--------------|-----|
| `MultiArray shape (64) does not match shape (1)` | Prefill or infer mismatch between batch vs single-token tensors | Ensure correct `ffn_prefill` / `ffn_infer` shapes or adjust to sequential mode by setting prefill hidden_states to `[1,1,H]` |
| Missing logits concatenation | Outputs not named `logits*` | Rename outputs or manually post-process |
| Incorrect token length padding | Embeddings `input_ids` shape mismatch | Align `embeddings.inputs.input_ids.shape` with expected max prefill length |
| LM head shape mismatch | `output_hidden_states` vs `lm_head.hidden_states` differ | Regenerate config with discovery tool; fix shapes |

For detailed examples see `configs/` directory (e.g. `anemll-qwen3-0.6b.json`).


## Examples

The `examples/` directory demonstrates various usage patterns:

### 🌟 Recommended Starting Points

- **[recommended_api_demo.rs](examples/recommended_api_demo.rs)** - **START HERE** - Shows the modern UnifiedModelLoader API
- **[qwen_chat.rs](examples/qwen_chat.rs)** - Interactive chat using ANEMLL Qwen models  
- **[proper_quality_test.rs](examples/proper_quality_test.rs)** - Model quality assessment

### 🔧 Advanced Examples

- **[compare_loading_approaches.rs](examples/compare_loading_approaches.rs)** - Compare old vs new loading methods
- **[test_thinking_behavior.rs](examples/test_thinking_behavior.rs)** - Test model reasoning capabilities  
- **[debug_token_mismatch.rs](examples/debug_token_mismatch.rs)** - Debugging token generation issues

### 📚 Documentation Examples

- **[WORKED_EXAMPLE.md](examples/WORKED_EXAMPLE.md)** - Complete BERT inference tutorial (legacy)
- **[qwen/README.md](examples/qwen/README.md)** - Qwen model documentation

### Running Examples

```bash
# Start with the recommended API
cargo run --example recommended_api_demo

# Interactive Qwen chat (downloads ~2GB on first run)
cargo run --example qwen_chat

# Test model quality  
cargo run --example proper_quality_test

# Compare loading approaches
cargo run --example compare_loading_approaches
```

### ✨ Key Features Demonstrated

- **🚀 UnifiedModelLoader**: Automatic downloading, config generation, and caching
- **🧠 Multi-Component Architecture**: ANEMLL's specialized model components  
- **⚡ ANE Acceleration**: True Apple Neural Engine optimization
- **🔧 Advanced Generation**: Top-k sampling, temperature control, quality assessment
- **📦 HuggingFace Integration**: Seamless model access from HuggingFace Hub

## Platform Support

- **macOS**: Full CoreML runtime support
- **iOS**: Full CoreML runtime support (when targeting iOS)
- **Other platforms**: Builds successfully, runtime features disabled

## Contributing

This is an independent project providing CoreML integration for the Candle ecosystem. Contributions welcome!

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.