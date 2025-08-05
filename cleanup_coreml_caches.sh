#!/bin/bash
# CoreML Cache Cleanup Script
# 
# This script removes accumulated CoreML cache directories that can consume
# hundreds of GB of disk space over time.

set -e

echo "🧹 CoreML Cache Cleanup Utility"
echo "==============================="
echo ""

# Function to safely calculate directory size
safe_du() {
    local dir="$1"
    if [[ -d "$dir" ]]; then
        du -sh "$dir" 2>/dev/null | cut -f1 || echo "unknown"
    else
        echo "0B"
    fi
}

# Find and display CoreML cache directories
echo "🔍 Scanning for CoreML cache directories..."
cache_dirs=($(find ~/Library/Caches -name "integration_tests-*" -type d 2>/dev/null || true))

if [[ ${#cache_dirs[@]} -eq 0 ]]; then
    echo "✅ No CoreML integration test caches found"
    echo ""
    
    # Also check for other common CoreML cache patterns
    echo "🔍 Checking for other CoreML caches..."
    other_caches=($(find ~/Library/Caches -name "*e5rt*" -type d 2>/dev/null || true))
    
    if [[ ${#other_caches[@]} -eq 0 ]]; then
        echo "✅ No other CoreML caches found"
    else
        echo "ℹ️  Found ${#other_caches[@]} other CoreML cache directories:"
        total_size=0
        for cache_dir in "${other_caches[@]}"; do
            size=$(safe_du "$cache_dir")
            echo "   $cache_dir ($size)"
        done
        echo ""
        echo "💡 These might be system CoreML caches. Only remove if you're sure."
    fi
    exit 0
fi

echo "📊 Found ${#cache_dirs[@]} integration test cache directories:"
echo ""

total_size_bytes=0
for cache_dir in "${cache_dirs[@]}"; do
    size=$(safe_du "$cache_dir")
    echo "   $cache_dir ($size)"
    
    # Try to get size in bytes for total calculation
    if [[ "$size" != "unknown" ]]; then
        size_bytes=$(du -sb "$cache_dir" 2>/dev/null | cut -f1 || echo "0")
        total_size_bytes=$((total_size_bytes + size_bytes))
    fi
done

# Convert total size to human readable
if [[ $total_size_bytes -gt 0 ]]; then
    if [[ $total_size_bytes -gt 1073741824 ]]; then
        total_size_human=$(echo "scale=1; $total_size_bytes / 1073741824" | bc -l 2>/dev/null || echo "unknown")
        total_size_human="${total_size_human}GB"
    elif [[ $total_size_bytes -gt 1048576 ]]; then
        total_size_human=$(echo "scale=1; $total_size_bytes / 1048576" | bc -l 2>/dev/null || echo "unknown")
        total_size_human="${total_size_human}MB"
    else
        total_size_human="${total_size_bytes}B"
    fi
else
    total_size_human="unknown"
fi

echo ""
echo "💾 Total estimated size: $total_size_human"
echo ""

# Confirmation prompt
read -p "🗑️  Remove all integration test caches? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Cleanup cancelled"
    exit 1
fi

echo ""
echo "🧹 Cleaning up caches..."

removed_count=0
for cache_dir in "${cache_dirs[@]}"; do
    if [[ -d "$cache_dir" ]]; then
        size=$(safe_du "$cache_dir")
        echo "   Removing: $(basename "$cache_dir") ($size)"
        rm -rf "$cache_dir" 2>/dev/null && removed_count=$((removed_count + 1)) || echo "     ⚠️  Failed to remove"
    fi
done

echo ""
echo "✅ Cleanup completed!"
echo "   Removed $removed_count cache directories"
echo "   Freed approximately $total_size_human of disk space"
echo ""
echo "💡 Tips to prevent cache buildup:"
echo "   • Use the updated ./run_integration_tests.sh (auto-cleanup enabled)"
echo "   • Run this cleanup script periodically"
echo "   • Consider using 'cargo test' with specific test names for development"