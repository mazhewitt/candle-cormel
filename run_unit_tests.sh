#!/bin/bash
# Unit Test Runner for candle-coreml
# 
# This script runs fast unit tests without downloading models or using CoreML.
# Safe to run on any platform and in CI environments.

set -e  # Exit on any error

echo "⚡ candle-coreml Unit Test Suite"
echo "==============================="

echo "🎯 Test Coverage:"
echo "   • Utility functions (masks, sampling, config)"
echo "   • Builder patterns and validation"  
echo "   • Tensor conversion utilities"
echo "   • Error handling and edge cases"
echo "   • Cross-platform compatibility"
echo ""

echo "✅ No model downloads required"
echo "✅ Works on all platforms (macOS, Linux, Windows)"
echo "✅ Safe for CI/CD environments"
echo "⏱️  Expected runtime: ~10 seconds"
echo ""

echo "🚀 Running unit tests..."

# Run the fast, reliable unit tests
echo "🔧 Running utility function tests..."
cargo test --test utils_tests

echo ""
echo "🏗️  Running builder pattern tests..."
cargo test --test builder_tests

echo ""
echo "⚙️  Running library unit tests..."
cargo test --lib

echo ""
echo "🎉 Unit test suite completed successfully!"
echo ""
echo "💡 Next steps:"
echo "   • Run './run_integration_tests.sh' for full CoreML model testing (macOS only)"
echo "   • Run 'cargo test <test_name>' for individual test debugging"
echo "   • Check 'tests/README.md' for detailed test documentation"