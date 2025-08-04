#!/bin/bash
# Integration Test Runner for candle-coreml
# 
# This script runs the full integration test suite with proper thread safety
# and disk space checks to prevent crashes and failures.

set -e  # Exit on any error

echo "🧪 candle-coreml Integration Test Suite"
echo "======================================="

# Check if we're on macOS (required for CoreML)
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "❌ CoreML tests require macOS. Skipping integration tests."
    echo "   Running unit tests only..."
    cargo test
    exit 0
fi

# Check available disk space (need ~25GB for all models)
echo "📊 Checking disk space requirements..."
available_gb=$(df -g . | tail -1 | awk '{print $4}')
if [ "$available_gb" -lt 30 ]; then
    echo "⚠️  WARNING: Low disk space detected (${available_gb}GB available)"
    echo "   Integration tests require ~25GB for model downloads"
    echo "   Consider freeing up space or running: ./run_unit_tests.sh"
    echo ""
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "❌ Aborted by user"
        exit 1
    fi
fi

echo "✅ Sufficient disk space available (${available_gb}GB)"
echo ""

# Show what will be tested
echo "🎯 Test Suite Coverage:"
echo "   • OpenELM baseline text completion"
echo "   • Apple Mistral baseline (stateless)"  
echo "   • Apple Mistral autoregressive (MLState)"
echo "   • Qwen extended coverage tests"
echo "   • Performance regression benchmarks"
echo "   • Memory efficiency validation"
echo ""

echo "⚠️  NOTE: First run will download ~20GB of models (cached afterward)"
echo "⏱️  Expected runtime: 3-5 minutes (after models cached)"
echo "🔧 Each model requires compilation on first load (30-60s pause is normal)"
echo ""

# Confirmation prompt
read -p "Run full integration test suite? (Y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo "❌ Aborted by user"
    exit 1
fi

echo ""
echo "🚀 Starting integration tests..."
echo "   Using --test-threads=1 for Core ML thread safety"
echo ""

# Run the tests with proper flags
# --test-threads=1: Required for Core ML thread safety (prevents SIGSEGV)
# --nocapture: Show test output for progress monitoring
# --ignored: Run the ignored tests that require model downloads
RUST_LOG=info cargo test -- --ignored --nocapture --test-threads=1

echo ""
echo "🎉 Integration test suite completed successfully!"
echo ""
echo "💡 Tips:"
echo "   • Models are cached in ~/Library/Caches/candle-coreml/"
echo "   • Run './run_unit_tests.sh' for fast tests without models"
echo "   • Use 'cargo test <test_name> -- --ignored --nocapture --test-threads=1' for individual tests"