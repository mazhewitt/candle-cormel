#!/usr/bin/env python3
"""Direct CoreML test to see what token is predicted"""

import os
import sys
import numpy as np

# Since coremltools has issues, let's try using the CoreML framework directly
try:
    import coremltools as ct
    print("Using coremltools")
    USE_COREMLTOOLS = True
except:
    print("coremltools not available")
    USE_COREMLTOOLS = False

# Try direct approach - just check if models exist
model_dir = "/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"

print(f"Model directory: {model_dir}")
print(f"Models present:")
for f in os.listdir(model_dir):
    if 'mlmodelc' in f or 'mlpackage' in f:
        path = os.path.join(model_dir, f)
        print(f"  - {f} ({os.path.getsize(path) if os.path.isfile(path) else 'directory'})")

# Let's check what Rust sees vs what Python sees
print("\n" + "="*50)
print("Key observations:")
print("="*50)
print("1. Rust loads from: /Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4")
print("2. Rust loads these models:")
print("   - qwen_embeddings.mlmodelc")
print("   - qwen_FFN_PF_lut8_chunk_01of01.mlmodelc")  
print("   - qwen_lm_head_lut8.mlmodelc")
print("3. Rust predicts: token 3974 (' quick')")
print()
print("The issue is that Python's coremltools expects .mlpackage format")
print("but these models are in .mlmodelc format (compiled).")
print()
print("This is why chat.py might not be working - it needs .mlpackage format")
print("while Rust can load .mlmodelc directly.")