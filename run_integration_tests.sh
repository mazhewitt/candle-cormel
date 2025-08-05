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

# Function to clean up CoreML caches
cleanup_coreml_caches() {
    echo "🧹 Cleaning up CoreML temporary caches..."
    # Clean up the e5rt cache directories that accumulate
    find ~/Library/Caches -name "integration_tests-*" -type d 2>/dev/null | while read cache_dir; do
        if [[ -d "$cache_dir" ]]; then
            echo "   Removing: $(basename "$cache_dir") ($(du -sh "$cache_dir" 2>/dev/null | cut -f1 || echo "unknown size"))"
            rm -rf "$cache_dir" 2>/dev/null || true
        fi
    done
}

# Set up cleanup trap to run on script exit
trap cleanup_coreml_caches EXIT

# Set a consistent cache directory to avoid multiple caches
export TMPDIR="/tmp/candle-coreml-tests"
mkdir -p "$TMPDIR"

echo "🗂️  Using consistent cache directory: $TMPDIR"
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
echo "   • CoreML temporary caches are automatically cleaned up"