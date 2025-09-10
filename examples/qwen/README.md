# Qwen 0.6B CoreML ANE Demonstration

This directory contains examples showcasing our candle-coreml inference engine integration patterns with Anemll's ANE-optimized models.

## ✅ Status: Multi-Component Architecture Implemented

We've successfully implemented the full Anemll multi-component architecture! Our candle-coreml crate now supports:

- **Multi-Component Models**: Separate loading of embeddings, FFN, and LM head models
- **Pipeline Orchestration**: Proper data flow between model components
- **Causal Masking**: Correct transformer-style attention masking
- **Streaming Generation**: Token-by-token generation with component coordination

## 🎯 Complete Implementation

Our candle-coreml crate provides comprehensive support for Anemll's architecture:

### Core Features Implemented
- ✅ **Multi-Component Loading**: Separate model files (embeddings, FFN, LM head)
- ✅ **Pipeline Orchestration**: Proper data flow between components
- ✅ **Causal Masking**: Transformer-style attention patterns
- ✅ **HuggingFace Integration**: Auto-download of all model components
- ✅ **Streaming Generation**: Token-by-token output coordination
- ✅ **Temperature Sampling**: Configurable generation parameters
- ✅ **Comprehensive Testing**: Full pipeline validation

### Current Examples

All examples have been updated to use the modern UnifiedModelLoader API:

```
examples/
├── qwen_chat.rs                    # ✅ Interactive chat (recommended)
├── test_thinking_behavior.rs       # 🧠 Model reasoning evaluation  
├── proper_quality_test.rs          # 📊 Comprehensive quality testing
├── compare_loading_approaches.rs   # 📈 API comparison
├── debug_token_mismatch.rs         # 🔧 Debugging utilities
├── recommended_api_demo.rs         # ⭐ **START HERE** - Modern API demo
└── qwen/README.md                  # This documentation
```

**Modern API Features:**
- ✅ **UnifiedModelLoader**: Automatic downloading, config generation, and caching
- ✅ **`complete_text()`**: Recommended high-level text generation API
- ✅ **`generate_tokens_topk_temp()`**: Advanced generation with top-k sampling
- ✅ **Automatic Configuration**: No manual config files needed
- ✅ **Intelligent Caching**: Models and configs cached automatically

## 🔧 Usage Examples

### Interactive Qwen Chat (Recommended)
```bash
# Start with the recommended API demo
cargo run --example recommended_api_demo

# Interactive chat with Qwen models (downloads ~2GB on first run)
cargo run --example qwen_chat

# With debug logging to see model interactions
RUST_LOG=debug cargo run --example qwen_chat
```

### Model Quality and Testing
```bash
# Comprehensive model quality assessment
cargo run --example proper_quality_test

# Test model reasoning capabilities
cargo run --example test_thinking_behavior

# Debug token generation issues
cargo run --example debug_token_mismatch
```

### Advanced Usage
```bash
# Compare old vs new API approaches
cargo run --example compare_loading_approaches

# All examples use automatic model downloading and config generation
# First run downloads models (~2GB), subsequent runs are fast
```

## 🎯 Multi-Component Architecture Details

Our implementation follows the Anemll pattern exactly:

### Component Pipeline
1. **Embeddings Model** (`qwen_embeddings.mlmodelc`)
   - Input: Token IDs [batch, seq_len]
   - Output: Hidden states [batch, seq_len, hidden_dim]

2. **FFN Model** (`qwen_FFN_PF_lut8_chunk_01of01.mlmodelc`)
   - Input: Hidden states + causal mask
   - Output: Processed hidden states [batch, seq_len, hidden_dim]

3. **LM Head Model** (`qwen_lm_head_lut8.mlmodelc`)
   - Input: Last position hidden state [batch, 1, hidden_dim]
   - Output: Token logits [batch, 1, vocab_size]

### Key Features
- **Causal Masking**: Proper transformer attention patterns
- **State Management**: Hidden states flow between components
- **Memory Efficient**: Components can be loaded/unloaded as needed
- **ANE Optimized**: Each component targets Apple Neural Engine

## 💡 Architecture Insights

The Anemll approach demonstrates advanced CoreML usage:
- **Memory Optimization**: Chunking reduces peak memory usage
- **ANE Targeting**: Specific optimizations for Apple Neural Engine
- **Flexible Deployment**: Modular components for different use cases

## 🔍 Model File Components and Filenames

The core components and expected filenames are:

- `qwen_embeddings.mlmodelc` — Token embeddings 
- `qwen_FFN_PF_lut8_chunk_01of01.mlmodelc` — Feed-forward/transformer core
- `qwen_lm_head_lut8.mlmodelc` — Language model head

Built-in configs reference these exact filenames and will auto-download the components from HuggingFace on first run. For custom/local models, configure explicit file_path values per component; filename discovery/globbing is not supported.

## 🎨 Integration Patterns Demonstrated

Our examples showcase key integration patterns applicable to any multi-component CoreML model:

### Core Patterns
- **Multi-Component Loading**: Load separate model files with different configurations
- **Pipeline Orchestration**: Coordinate data flow between model components
- **State Management**: Handle hidden states and intermediate representations
- **Causal Masking**: Implement transformer attention patterns in CoreML
- **HuggingFace Integration**: Download complex model architectures automatically
- **Error Handling**: Graceful fallbacks when model files aren't available

### Performance Patterns  
- **Component Timing**: Profile each pipeline stage independently
- **Memory Management**: Load/unload components as needed
- **ANE Optimization**: Ensure each component targets Apple Neural Engine
- **Batch Processing**: Handle variable sequence lengths efficiently

## 💡 Implementation Insights

This implementation demonstrates several advanced CoreML integration techniques:

1. **Multi-Model Coordination**: Unlike single-model approaches, this shows how to orchestrate multiple CoreML models in a pipeline
2. **Dynamic Configuration**: Each model component has its own configuration and requirements
3. **State Flow Management**: Hidden states flow between components with proper shape handling
4. **Error Recovery**: Robust handling of missing files or incompatible model versions

These patterns provide a foundation for integrating any complex, multi-component ANE-optimized model with Candle.