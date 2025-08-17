#!/bin/bash
# Enhanced CoreML Cache Cleanup Script
# 
# This script removes accumulated CoreML cache directories that can consume
# hundreds of GB of disk space over time. Updated to handle all candle-coreml
# related cache patterns discovered through investigation.

set -e

echo "🧹 Enhanced CoreML Cache Cleanup Utility"
echo "========================================"
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

# Function to get size in bytes for calculations
get_size_bytes() {
    local dir="$1"
    if [[ -d "$dir" ]]; then
        du -sb "$dir" 2>/dev/null | cut -f1 || echo "0"
    else
        echo "0"
    fi
}

# Function to convert bytes to human readable
bytes_to_human() {
    local bytes="$1"
    if [[ $bytes -gt 1073741824 ]]; then
        echo "scale=1; $bytes / 1073741824" | bc -l 2>/dev/null | awk '{printf "%.1fGB\n", $1}'
    elif [[ $bytes -gt 1048576 ]]; then
        echo "scale=1; $bytes / 1048576" | bc -l 2>/dev/null | awk '{printf "%.1fMB\n", $1}'
    elif [[ $bytes -gt 1024 ]]; then
        echo "scale=1; $bytes / 1024" | bc -l 2>/dev/null | awk '{printf "%.1fKB\n", $1}'
    else
        echo "${bytes}B"
    fi
}

# Check if cache directory exists
if [[ ! -d ~/Library/Caches ]]; then
    echo "❌ Cache directory ~/Library/Caches not found"
    exit 1
fi

echo "🔍 Scanning for candle-coreml related cache directories..."

# Define patterns to search for based on our investigation
declare -a cache_patterns=(
    "candle_coreml-*"           # Main library caches
    "candle-coreml-*"           # Alternative naming
    "integration_tests-*"       # Integration test caches
    "performance_regression_tests-*"  # Performance test caches
    "qwen_tests-*"             # Qwen test caches
    "typo_fixer_test*"         # Typo fixer test caches
    "typo_fixer_tests-*"       # Typo fixer integration tests
    "flex_pipeline_tests-*"    # Flex pipeline test caches
    "builder_tests-*"          # Builder test caches
    "tensor_regression_tests-*" # Tensor test caches
    "utils_tests-*"            # Utils test caches
    "bundle_id_*"              # Bundle ID test caches
)

# Find all matching cache directories
declare -a found_caches=()
total_size_bytes=0

for pattern in "${cache_patterns[@]}"; do
    while IFS= read -r -d '' cache_dir; do
        # Check if it contains CoreML-specific files
        if [[ -d "$cache_dir/com.apple.e5rt.e5bundlecache" ]] || [[ -f "$cache_dir/.coreml_cache" ]] || [[ "$cache_dir" == *"coreml"* ]]; then
            found_caches+=("$cache_dir")
            size_bytes=$(get_size_bytes "$cache_dir")
            total_size_bytes=$((total_size_bytes + size_bytes))
        fi
    done < <(find ~/Library/Caches -maxdepth 1 -name "$pattern" -type d -print0 2>/dev/null || true)
done

# Also find standalone e5rt caches
while IFS= read -r -d '' cache_dir; do
    # Only include if not already found
    if [[ ! " ${found_caches[*]} " =~ " $cache_dir " ]]; then
        found_caches+=("$cache_dir")
        size_bytes=$(get_size_bytes "$cache_dir")
        total_size_bytes=$((total_size_bytes + size_bytes))
    fi
done < <(find ~/Library/Caches -maxdepth 1 -name "*e5rt*" -type d -print0 2>/dev/null || true)

# Check if we found any caches
if [[ ${#found_caches[@]} -eq 0 ]]; then
    echo "✅ No candle-coreml related cache directories found"
    echo ""
    echo "🔍 Checking for any remaining CoreML-related caches..."
    
    # Do a broader search for any directories containing e5rt
    broader_caches=($(find ~/Library/Caches -name "*e5rt*" -type d 2>/dev/null || true))
    
    if [[ ${#broader_caches[@]} -eq 0 ]]; then
        echo "✅ No CoreML caches found at all"
    else
        echo "ℹ️  Found ${#broader_caches[@]} other potential CoreML cache directories:"
        for cache_dir in "${broader_caches[@]}"; do
            size=$(safe_du "$cache_dir")
            echo "   $(basename "$cache_dir") ($size)"
        done
        echo ""
        echo "💡 These might be system CoreML caches or from other applications."
        echo "   Only remove if you're sure they're not needed."
    fi
    exit 0
fi

echo "📊 Found ${#found_caches[@]} candle-coreml related cache directories:"
echo ""

# Display found caches with sizes
for cache_dir in "${found_caches[@]}"; do
    size=$(safe_du "$cache_dir")
    basename_dir=$(basename "$cache_dir")
    
    # Check what type of cache it contains
    cache_type=""
    if [[ -d "$cache_dir/com.apple.e5rt.e5bundlecache" ]]; then
        cache_type=" [CoreML e5rt]"
    fi
    
    echo "   $basename_dir ($size)$cache_type"
done

echo ""
total_size_human=$(bytes_to_human $total_size_bytes)
echo "💾 Total estimated size: $total_size_human"
echo ""

# Show what will be preserved
echo "🔒 Preserved cache directories:"
echo "   • candle-coreml (model downloads)"
echo "   • typo-fixer-cli (application cache)"
echo "   • System CoreML caches (Apple managed)"
echo ""

# Confirmation prompt with more detailed options
echo "Choose cleanup action:"
echo "1) Remove all found caches (recommended)"
echo "2) Interactive selection"
echo "3) Dry run (show what would be deleted)"
echo "4) Cancel"
echo ""
read -p "Select option (1-4): " -n 1 -r option
echo

case $option in
    1)
        echo ""
        echo "🧹 Removing all found caches..."
        removed_count=0
        freed_bytes=0
        
        for cache_dir in "${found_caches[@]}"; do
            if [[ -d "$cache_dir" ]]; then
                size_bytes=$(get_size_bytes "$cache_dir")
                size_human=$(safe_du "$cache_dir")
                echo "   Removing: $(basename "$cache_dir") ($size_human)"
                
                if rm -rf "$cache_dir" 2>/dev/null; then
                    removed_count=$((removed_count + 1))
                    freed_bytes=$((freed_bytes + size_bytes))
                else
                    echo "     ⚠️  Failed to remove"
                fi
            fi
        done
        
        echo ""
        echo "✅ Cleanup completed!"
        echo "   Removed $removed_count cache directories"
        echo "   Freed approximately $(bytes_to_human $freed_bytes) of disk space"
        ;;
        
    2)
        echo ""
        echo "🎯 Interactive cache removal:"
        removed_count=0
        freed_bytes=0
        
        for cache_dir in "${found_caches[@]}"; do
            size_human=$(safe_du "$cache_dir")
            basename_dir=$(basename "$cache_dir")
            
            read -p "Remove $basename_dir ($size_human)? (y/N): " -n 1 -r
            echo
            
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                size_bytes=$(get_size_bytes "$cache_dir")
                echo "   Removing: $basename_dir"
                
                if rm -rf "$cache_dir" 2>/dev/null; then
                    removed_count=$((removed_count + 1))
                    freed_bytes=$((freed_bytes + size_bytes))
                    echo "   ✅ Removed"
                else
                    echo "   ⚠️  Failed to remove"
                fi
            else
                echo "   ⏭️  Skipped"
            fi
            echo
        done
        
        echo "✅ Interactive cleanup completed!"
        echo "   Removed $removed_count cache directories"
        echo "   Freed approximately $(bytes_to_human $freed_bytes) of disk space"
        ;;
        
    3)
        echo ""
        echo "🔍 Dry run - would remove:"
        for cache_dir in "${found_caches[@]}"; do
            size_human=$(safe_du "$cache_dir")
            echo "   $(basename "$cache_dir") ($size_human)"
        done
        echo ""
        echo "Total would free: $total_size_human"
        echo "Run with option 1 to actually remove these caches."
        ;;
        
    *)
        echo "❌ Cleanup cancelled"
        exit 1
        ;;
esac

echo ""
echo "💡 Tips to prevent cache buildup:"
echo "   • Use 'cargo test --test specific_test' for targeted testing"
echo "   • Run this cleanup script periodically"
echo "   • Consider setting up automatic cleanup in CI/CD"
echo "   • Use the CacheManager API for programmatic cleanup"