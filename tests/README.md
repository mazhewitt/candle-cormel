# Test Suite for candle-coreml

This directory contains comprehensive tests for the candle-coreml library, organized by functionality and test type.

## Test Organization

### Core Integration Tests
- **`integration_tests.rs`** - Main CoreML integration tests (15 tests)
  - Model loading and validation
  - Device compatibility (CPU, Metal, CUDA rejection)
  - Stateful and stateless prediction
  - Error handling and edge cases

### Model-Specific Tests
- **`qwen_tests.rs`** - Comprehensive Qwen model testing (25+ tests)
  - Architecture validation and pipeline testing
  - Position handling and boundary conditions
  - Specific prediction validation (e.g., "dog" completion)
  - Integration testing across full pipeline
  - Extended edge case coverage
  - Organized in modules: `architecture_tests`, `position_fix_tests`, `prediction_tests`, `integration_tests`, `extended_coverage_tests`

### Utility and Component Tests
- **`builder_tests.rs`** - CoreMLModelBuilder pattern testing (18 tests)
  - Builder creation and configuration validation
  - HuggingFace integration error handling
  - Path validation and config management

- **`utils_tests.rs`** - Utility function testing (24 tests)
  - Causal mask creation and validation
  - Sampling utilities (greedy, temperature, top-k)
  - Multi-component configuration builders
  - Organized in modules: `mask_tests`, `sampling_tests`, `multi_component_tests`

- **`performance_regression_tests.rs`** - Performance benchmarks (8 tests, mostly ignored)
  - Baseline validation against reference implementations
  - Memory efficiency testing
  - Consistency validation

## Running Tests

### 🚀 Recommended: Use Test Scripts

For the best experience, use the provided test scripts that handle all the complexity:

```bash
# Fast unit tests (no models, works anywhere, ~10 seconds)
./run_unit_tests.sh

# Full integration tests (downloads models, macOS only, ~5 minutes)
./run_integration_tests.sh
```

These scripts automatically:
- ✅ Check platform compatibility and disk space
- ✅ Use proper thread safety flags (`--test-threads=1`)
- ✅ Show progress and provide helpful tips
- ✅ Handle Core ML limitations safely

### Manual Test Commands

If you prefer manual control:

#### Quick Tests (No Models Required)
```bash
# Run unit tests and utility functions
cargo test utils_tests builder_tests

# Run core integration tests (uses cached models)
cargo test integration_tests
```

### Model-Specific Tests
```bash
# Run all Qwen tests
cargo test qwen_tests

# Run specific Qwen test modules
cargo test qwen_tests::architecture_tests
cargo test qwen_tests::position_fix_tests
cargo test qwen_tests::prediction_tests
cargo test qwen_tests::integration_tests

# Run extended coverage tests (ignored by default)
cargo test qwen_tests::extended_coverage_tests -- --ignored --nocapture
```

### Performance Tests
```bash
# Run performance benchmarks (ignored by default) - MUST use --test-threads=1
cargo test performance_regression_tests -- --ignored --nocapture --test-threads=1
```

### All Tests
```bash
# Standard test suite (fast, no ignored tests)
cargo test

# Include ignored performance/integration tests (slow) - MUST use --test-threads=1
cargo test -- --ignored --nocapture --test-threads=1
```

## Test Coverage

Generate coverage reports to analyze test effectiveness:

```bash
# Generate HTML coverage report
cargo tarpaulin --workspace --all-features --out Html --output-dir coverage-report

# Open coverage report
open coverage-report/tarpaulin-report.html
```

**Current Coverage**: ~48% overall
- ✅ Conversion utilities: 76%+
- ✅ Configuration: 100%
- ✅ State management: 100%
- ✅ Utility functions: 100% (24 comprehensive tests)
- ✅ Builder patterns: Fully covered (18 tests)
- ⚠️ Qwen implementation: 36% (room for improvement)

## Model Requirements

Many tests require downloaded models, which are automatically cached:

- **Qwen Models**: `anemll/anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4` (~1.8GB)
- **OpenELM Models**: `corenet-community/coreml-OpenELM-450M-Instruct` (~1.7GB) 
- **Mistral Models**: `apple/mistral-coreml` (~17GB)

Models are downloaded automatically via `ensure_model_downloaded()` on first use.

Note: The library now requires explicit file_path configuration for multi-component models (no filename discovery/globbing). Built-in test configs already conform to this requirement.

**⏱️ Model Loading Times**: Each model requires compilation on first load:
- **Download**: ~30s for large models (cached afterward)
- **Compilation**: 30-60s per model (optimizes for your hardware: ANE/GPU/CPU)
- **Subsequent loads**: Much faster (~2-5s) as compiled models are cached

If you see a long pause after "Found [model] CoreML package", this is normal - CoreML is compiling the model for optimal performance on your hardware.

### ⚠️ Disk Space Requirements

**CRITICAL**: Running ignored tests requires **significant disk space**:

- **Minimum Required**: ~25GB free space for all models
- **Apple Mistral Model**: 17GB (largest model)
- **Model Cache Location**: `~/Library/Caches/candle-coreml/`

**Disk Space Issues**: If you encounter:
- `LLVM ERROR: IO failure on output stream: No space left on device`
- Segmentation faults during test execution
- Test hangs or crashes

Check available disk space with `df -h`. Tests will fail if insufficient space is available.

### ⚠️ Thread Safety Limitations

**CRITICAL**: Core ML models are **NOT thread-safe**. Running ignored tests in parallel will cause segmentation faults.

**Apple's Core ML Documentation**:
- "The model class is not guaranteed to be thread-safe"
- "You must assume that a single MLModel instance cannot be safely accessed from multiple threads simultaneously"
- Multiple model instances of the same .mlpackage may share internal resources

**Required for Ignored Tests**:
```bash
# ✅ SAFE: Run tests sequentially
cargo test -- --ignored --nocapture --test-threads=1

# ❌ UNSAFE: Parallel execution causes SIGSEGV
cargo test -- --ignored --nocapture  # (crashes)
```

**Why This Happens**:
- `test_mistral_baseline_completion` and `test_mistral_autoregressive_mlstate` both load the same Apple Mistral model
- Rust runs tests in parallel by default
- Concurrent access to Core ML models causes race conditions and crashes

**Solution**: Always use `--test-threads=1` when running ignored tests that load Core ML models.

## Platform Support

- **macOS**: Full test suite including CoreML functionality
- **Other Platforms**: Limited to unit tests and error handling (CoreML tests skipped)

## Test Development Guidelines

### Adding Tests
- **Unit Tests**: Add to `utils_tests.rs` or `builder_tests.rs`
- **Integration Tests**: Add to `integration_tests.rs`
- **Model Tests**: Add to appropriate modules in `qwen_tests.rs`
- **Performance Tests**: Add to `performance_regression_tests.rs`

### Test Conventions
- Use descriptive names: `test_function_specific_behavior`
- Group related tests in modules
- Use `#[ignore]` for tests requiring model downloads
- Provide graceful fallbacks for missing models
- Use conditional compilation for macOS-only features

### Performance Considerations
- Tests requiring models should be ignored by default
- Use `--nocapture` to see progress output
- First model download takes ~30s, subsequent runs use cache