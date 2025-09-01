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

### Example Structure
```
examples/qwen/
├── qwen_chat.rs                    # ✅ Interactive chat with UnifiedModelLoader
├── qwen_multi_component.rs         # 🌟 Full multi-component implementation 
├── qwen_benchmark.rs               # ✅ Performance benchmarks with UnifiedModelLoader
├── qwen_performance_benchmark.rs   # ✅ Forward text benchmarking
├── qwen_demo_patterns.rs           # 📚 Educational patterns demo
└── README.md                       # This documentation
```

**Updated with UnifiedModelLoader System:**
- ✅ **qwen_chat.rs**: Now uses automatic config generation 
- ✅ **qwen_benchmark.rs**: Updated to use UnifiedModelLoader
- ✅ **qwen_performance_benchmark.rs**: Updated to use UnifiedModelLoader
- 📚 **qwen_demo_patterns.rs**: Educational (no changes needed)
- 🔧 **qwen_multi_component.rs**: Low-level implementation (preserved for learning)

## 🔧 Usage Examples

### Multi-Component Qwen Chat (With Real Models)
```bash
# 🔧 Full Anemll multi-component implementation (requires model download)
cargo run --example qwen_multi_component

# With verbose logging to see component interactions
cargo run --example qwen_multi_component -- --verbose --temperature 0.8

# Help and options
cargo run --example qwen_multi_component -- --help
```

*Note: The multi-component chat requires downloading large model files. If you encounter download issues, ensure your ModelConfig uses explicit file_path values for each component and see CUSTOM_MODEL_GUIDE.md for setup details.*

### Single-Model Interface (Reference)
```bash
# Shows integration patterns for single models
cargo run --example qwen_chat --help

# Performance benchmarking framework
cargo run --example qwen_benchmark --help
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