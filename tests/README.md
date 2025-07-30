# Integration Tests for candle-coreml

This directory contains comprehensive integration tests for the candle-coreml library, specifically focusing on the F16-compatible typo correction pipeline.

## Test Files

### `typo_fixer_integration_tests.rs` - Main Automation Tests ⭐

**Purpose**: Comprehensive integration tests for the Qwen typo-fixer model with F16 CoreML support

**Key Features Tested**:
- ✅ **F16 Compatibility**: Verifies F16 data type handling doesn't crash
- ✅ **Token Generation**: Ensures diverse tokens (not repetitive 151668)  
- ✅ **Performance**: Validates reasonable inference times
- ✅ **Pipeline Integrity**: End-to-end model loading and prediction

### Running Individual Tests

Since each test loads the model (which takes ~30s), run tests individually for faster iteration:

```bash
# Test model loading with F16 support (essential)
cargo test --test typo_fixer_integration_tests test_typo_fixer_model_loading -- --nocapture

# Test single token generation (F16 regression test)
cargo test --test typo_fixer_integration_tests test_typo_fixer_single_token_generation -- --nocapture

# Test performance baseline
cargo test --test typo_fixer_integration_tests test_typo_fixer_performance_baseline -- --nocapture

# Test tokenization works correctly
cargo test --test typo_fixer_integration_tests test_typo_fixer_tokenization -- --nocapture

# Test text generation doesn't crash
cargo test --test typo_fixer_integration_tests test_typo_fixer_text_generation -- --nocapture

# Test model validation
cargo test --test typo_fixer_integration_tests test_typo_fixer_model_validation -- --nocapture

# Test specific corrections (ignored by default - run manually)
cargo test --test typo_fixer_integration_tests test_typo_fixer_specific_corrections -- --ignored --nocapture
```

### Running All Tests (Warning: Slow)

```bash
# Run all tests (takes ~3-5 minutes due to model loading)
cargo test --test typo_fixer_integration_tests

# Run all tests including ignored ones
cargo test --test typo_fixer_integration_tests -- --ignored
```

### CI/CD Usage

For automated testing in CI/CD pipelines:

```bash
# Quick regression test (essential for F16 compatibility)
cargo test --test typo_fixer_integration_tests test_typo_fixer_single_token_generation

# Full validation suite (if models are available)
cargo test --test typo_fixer_integration_tests
```

## Other Test Files

### `qwen_integration_tests.rs`
Generic Qwen model tests (downloads models from HuggingFace)

### `integration_tests.rs`  
General CoreML integration tests

### `conversion_tests.rs`
Tensor conversion performance tests

### `prediction_options_test.rs`
Prediction configuration tests

## Test Requirements

### Model Files Required
Tests require the typo-fixer model at:
```
/Users/mazdahewitt/projects/train-typo-fixer/models/qwen-typo-fixer-ane/
```

If models are not available, tests will skip gracefully with informative messages.

### Platform Requirements
- **macOS only**: CoreML is macOS-specific
- **Non-macOS**: Tests verify proper error handling

## Key Test Validations

### F16 Regression Tests ⚡
These tests specifically prevent regression of the F16 data type fix:

1. **Token Diversity**: `assert_ne!(next_token, 151668)` 
   - Before F16 fix: Generated only token 151668 repeatedly
   - After F16 fix: Generates diverse tokens

2. **No F16 Crashes**: `assert!(!error_msg.contains("MLMultiArrayDataType"))`
   - Before F16 fix: Crashed with data type errors  
   - After F16 fix: Handles F16 gracefully

3. **Performance**: Inference completes within reasonable time
   - Before F16 fix: Often timed out or failed
   - After F16 fix: Fast, reliable inference

### Performance Baselines

| Test | Expected Time | Pre-F16 Fix | Post-F16 Fix |
|------|---------------|-------------|--------------|
| Model Loading | < 35s | ❌ Often failed | ✅ ~30s |
| Single Token | < 2s | ❌ 420ms+ (just LM head) | ✅ ~580ms (full pipeline) |
| Full Generation | < 5s | ❌ Often infinite loop | ✅ Works correctly |

## Expected Test Output

### Success Example:
```
✅ Typo-fixer model loaded successfully with F16 support
✅ Generated token: 115752 -> '哪一个'
✅ F16 pipeline working correctly (no repetitive generation)
✅ Single token inference time: 583ms
✅ F16 pipeline performance: 583ms (pre-fix: often failed completely)
```

### Graceful Handling (No Models):
```
⚠️ Skipping integration test: Models not found at /path/to/models
```

## Troubleshooting

### Test Failures

1. **"Models not found"**: Install typo-fixer models or set correct path
2. **"Token 151668 generated"**: F16 fix regression - check data type handling
3. **"MLMultiArrayDataType error"**: F16 conversion issue - check conversion.rs
4. **Performance timeout**: Normal on first run, models need warming up

### Development Workflow

1. **After F16 changes**: Run `test_typo_fixer_single_token_generation`
2. **Before commits**: Run essential tests to prevent regression
3. **Full validation**: Run all tests before releases

## Adding New Tests

When adding tests:

1. **Use descriptive names**: `test_typo_fixer_specific_feature`
2. **Check for models**: Skip gracefully if not available
3. **Test F16 handling**: Include F16-specific assertions
4. **Document purpose**: Explain what regression it prevents

## Integration with CI/CD

Recommended CI/CD test strategy:
```yaml
# Essential F16 regression test (fast)
- name: F16 Regression Test
  run: cargo test --test typo_fixer_integration_tests test_typo_fixer_single_token_generation

# Full validation (if models available)  
- name: Full Integration Tests
  run: cargo test --test typo_fixer_integration_tests
  continue-on-error: true  # Models may not be available in CI
```

---

**The typo_fixer integration tests provide comprehensive validation that the F16 data type fix works correctly and prevents regression of the critical token generation issues.** 🚀