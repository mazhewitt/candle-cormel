# Examples for candle-coreml

This directory contains examples demonstrating the candle-coreml library with ANEMLL models and the modern UnifiedModelLoader API.

## 🌟 Recommended Starting Points

### `recommended_api_demo.rs` - **START HERE**

**Purpose**: Demonstrates the modern `UnifiedModelLoader` API - the recommended way to use candle-coreml  
**Key concepts**: Automatic model downloading, config generation, caching, modern text generation methods

```bash
# Run the recommended API demo
cargo run --example recommended_api_demo

# With custom model
RUST_LOG=info cargo run --example recommended_api_demo
```

This example shows:
- `UnifiedModelLoader::new()` and `load_model()`
- `complete_text()` - the recommended high-level API
- `generate_tokens_topk_temp()` - advanced generation control
- Automatic HuggingFace model downloading and caching

### `qwen_chat.rs` - Interactive Chat

**Purpose**: Interactive chat interface using ANEMLL Qwen models  
**Key concepts**: Real-time text generation, user interaction, model quality assessment

```bash
# Interactive chat with Qwen model (downloads ~2GB on first run)
cargo run --example qwen_chat

# With debug logging
RUST_LOG=debug cargo run --example qwen_chat
```

## 🔧 Advanced Examples

### `compare_loading_approaches.rs` - API Comparison

**Purpose**: Compare the new UnifiedModelLoader with legacy loading approaches  
**Key concepts**: Migration patterns, performance comparison, backward compatibility

```bash
cargo run --example compare_loading_approaches
```

### `test_thinking_behavior.rs` - Model Quality Testing

**Purpose**: Test model reasoning and generation capabilities  
**Key concepts**: Quality assessment, reasoning evaluation, generation testing

```bash
cargo run --example test_thinking_behavior
```

### `proper_quality_test.rs` - Comprehensive Quality Assessment

**Purpose**: Comprehensive model quality testing framework  
**Key concepts**: Multi-prompt testing, coherence evaluation, statistical analysis

```bash
cargo run --example proper_quality_test
```

### `debug_token_mismatch.rs` - Debugging Tool

**Purpose**: Debug token generation issues and model behavior  
**Key concepts**: Token analysis, generation debugging, troubleshooting

```bash
cargo run --example debug_token_mismatch
```

## 🏗️ Architecture Examples

These examples demonstrate different aspects of the multi-component ANEMLL architecture:

- **Embeddings Component**: Token ID to hidden state conversion
- **FFN Component**: Transformer processing with attention
- **LM Head Component**: Hidden state to logits conversion

All examples use models from the [ANEMLL collection](https://huggingface.co/anemll) which are optimized for Apple Neural Engine.

## 📖 API Usage Patterns

### Modern Approach (Recommended)

```rust
use candle_coreml::UnifiedModelLoader;

// Automatic setup - downloads, caches, validates
let loader = UnifiedModelLoader::new()?;
let mut model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;

// High-level text completion
let response = model.complete_text("Hello, world!", 50, 0.8)?;

// Advanced generation with top-k sampling
let tokens = model.generate_tokens_topk_temp("Hello!", 20, 0.7, Some(40))?;

// Single token prediction
let next_token = model.forward_text("Hello")?;
```

### Legacy Manual Approach (For Advanced Users)

```rust
use candle_coreml::{QwenModel, QwenConfig};

// Manual setup with explicit paths
let model_dir = "/path/to/downloaded/model";
let mut model = QwenModel::load_from_directory(&model_dir, None)?;

// Legacy generation methods (still supported)
let response = model.complete_text("Hello!", 20, 0.7)?;
```

## 🚀 Getting Started

1. **Start with `recommended_api_demo.rs`** to understand the modern API
2. **Try `qwen_chat.rs`** for an interactive experience
3. **Use `proper_quality_test.rs`** to evaluate model performance
4. **Reference other examples** for specific use cases

## ⚙️ Configuration

### Environment Variables

| Variable | Purpose | Example |
|----------|---------|---------|
| `RUST_LOG` | Enable debug logging | `debug`, `info`, `trace` |

### Model Caching

Models are automatically cached in:
- **Models**: `~/.cache/candle-coreml/models/`
- **Configs**: `~/.cache/candle-coreml/configs/`

First run downloads models (~2GB for Qwen 0.6B), subsequent runs are fast.

## 🔧 Troubleshooting

### "Model file not found"

**Solution**: Use the UnifiedModelLoader which handles downloads automatically:

```rust
let loader = UnifiedModelLoader::new()?;
let model = loader.load_model("anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")?;
```

### "Shape mismatch errors"

**Solution**: The UnifiedModelLoader automatically generates correct configurations. For manual setup, ensure model configs match the downloaded model files.

### "Deprecated method warnings"

**Solution**: Use modern methods:
- ✅ `complete_text()` - recommended for text generation
- ✅ `generate_tokens_topk_temp()` - for advanced control
- ❌ ~~`generate_tokens()`~~ - deprecated

## 🎯 Platform Support

- **macOS**: Full CoreML runtime support with Apple Neural Engine acceleration
- **Other platforms**: Examples compile but CoreML features are disabled

## 📚 Additional Documentation

- **[Main README](../README.md)**: Complete library documentation
- **[ANEMLL Guide](../ANEMLL_GUIDE.md)**: Deep dive into multi-component architecture
- **[Custom Model Guide](../CUSTOM_MODEL_GUIDE.md)**: Advanced model configuration

## 🤝 Contributing

When adding new examples:

1. Use the modern `UnifiedModelLoader` API
2. Include comprehensive error handling
3. Add usage documentation
4. Test on macOS with CoreML enabled
5. Use descriptive names and documentation

---

**Start with `cargo run --example recommended_api_demo` to see the modern API in action!** 🚀