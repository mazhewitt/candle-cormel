#!/usr/bin/env python3
"""
inspect_io_names.py
Inspect actual input/output names for each component
"""

import coremltools as ct
from pathlib import Path
import sys

# Use the downloaded model directory
MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
COMPONENTS = {
    "embeddings": MODEL_DIR / "qwen_embeddings.mlmodelc",
    "ffn":        MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc", 
    "lm_head":    MODEL_DIR / "qwen_lm_head_lut6.mlmodelc",
}

def inspect_io(model_path: Path, name: str):
    if not model_path.exists():
        print(f"❌ {name} – file not found: {model_path}", file=sys.stderr)
        return

    try:
        model = ct.models.CompiledMLModel(str(model_path))
        print(f"\n🔍  {name.upper()}  ({model_path.name})")
        
        # Try to predict with dummy data to discover actual input/output names
        import numpy as np
        
        if name == "embeddings":
            # Try different input names and sequence lengths
            for input_name in ["input_ids", "inputIds", "input", "tokens"]:
                for seq_len in [1, 64]:
                    try:
                        ids = np.array([list(range(seq_len))], dtype=np.int32)
                        result = model.predict({input_name: ids})
                        print(f"✅  INPUT: '{input_name}' with shape (1, {seq_len}) -> SUCCESS")
                        print(f"✅  OUTPUTS: {list(result.keys())}")
                        for k, v in result.items():
                            print(f"    '{k}': shape {v.shape}, dtype {v.dtype}")
                        return
                    except Exception as e:
                        if "not found" in str(e) or "key" in str(e).lower():
                            print(f"❌  INPUT: '{input_name}' -> NOT FOUND")
                        elif "shape" in str(e).lower():
                            print(f"❌  INPUT: '{input_name}' with shape (1, {seq_len}) -> SHAPE ERROR")
                        else:
                            print(f"❌  INPUT: '{input_name}' -> {str(e)[:100]}...")
                            
        elif name == "ffn":
            # Try FFN with different input names
            for hidden_name in ["hidden_states", "hiddenStates", "hidden", "input"]:
                for mask_name in ["causal_mask", "causalMask", "mask", "attention_mask"]:
                    try:
                        hidden = np.random.randn(1, 64, 896).astype(np.float32)  # Likely shape
                        mask = np.zeros((64, 64), dtype=np.float32)  # Causal mask
                        result = model.predict({hidden_name: hidden, mask_name: mask})
                        print(f"✅  INPUTS: '{hidden_name}' + '{mask_name}' -> SUCCESS")
                        print(f"✅  OUTPUTS: {list(result.keys())}")
                        for k, v in result.items():
                            print(f"    '{k}': shape {v.shape}, dtype {v.dtype}")
                        return
                    except Exception as e:
                        print(f"❌  INPUTS: '{hidden_name}' + '{mask_name}' -> {str(e)[:100]}...")
                        
        elif name == "lm_head":
            # Try LM head with different input names
            for hidden_name in ["hidden_states", "hiddenStates", "hidden", "input"]:
                try:
                    hidden = np.random.randn(1, 1, 896).astype(np.float32)  # Last position only
                    result = model.predict({hidden_name: hidden})
                    print(f"✅  INPUT: '{hidden_name}' -> SUCCESS")
                    print(f"✅  OUTPUTS: {list(result.keys())}")
                    for k, v in result.items():
                        print(f"    '{k}': shape {v.shape}, dtype {v.dtype}")
                    return
                except Exception as e:
                    print(f"❌  INPUT: '{hidden_name}' -> {str(e)[:100]}...")
                    
    except Exception as e:
        print(f"❌ Failed to load {name}: {e}")

if __name__ == "__main__":
    for name, path in COMPONENTS.items():
        inspect_io(model_path=path, name=name)