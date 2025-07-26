#!/usr/bin/env python3
"""
test_rank4_mask.py
Test with rank 4 causal mask
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_rank4_mask():
    if not FFN_PATH.exists():
        print(f"❌ FFN model not found: {FFN_PATH}")
        return

    try:
        model = ct.models.CompiledMLModel(str(FFN_PATH))
        print(f"🔍 FFN Model: {FFN_PATH.name}")
        
        hidden_dim = 1024
        
        # Create hidden states for single token
        hidden_states = np.random.randn(1, 1, hidden_dim).astype(np.float32)
        
        # Position for current token
        position_ids = np.array([0], dtype=np.int32)
        
        # Try different rank 4 causal mask shapes
        mask_configs = [
            ("(1, 1, 1, 1)", np.zeros((1, 1, 1, 1), dtype=np.float32)),
            ("(1, 1, 1, 64)", np.zeros((1, 1, 1, 64), dtype=np.float32)),
            ("(1, 1, 64, 64)", np.zeros((1, 1, 64, 64), dtype=np.float32)),
            ("(1, 12, 1, 64)", np.zeros((1, 12, 1, 64), dtype=np.float32)),  # 12 attention heads?
            ("(1, 16, 1, 64)", np.zeros((1, 16, 1, 64), dtype=np.float32)),  # 16 attention heads?
        ]
        
        for desc, causal_mask in mask_configs:
            try:
                print(f"\n🧪 Testing causal_mask shape: {desc}")
                print(f"   hidden_states: {hidden_states.shape}")
                print(f"   position_ids: {position_ids.shape}")
                print(f"   causal_mask: {causal_mask.shape}")
                
                inputs = {
                    "hidden_states": hidden_states,
                    "position_ids": position_ids,
                    "causal_mask": causal_mask
                }
                
                result = model.predict(inputs)
                print(f"✅ SUCCESS with causal_mask shape {desc}!")
                print(f"   Outputs: {list(result.keys())}")
                for k, v in result.items():
                    print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
                return causal_mask.shape
                
            except Exception as e:
                error_msg = str(e)
                if "shape" in error_msg.lower():
                    print(f"❌ Shape error: {error_msg[:150]}...")
                else:
                    print(f"❌ Error: {error_msg[:150]}...")
        
        print("\n❌ No working causal_mask shape found")
        return None
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return None

if __name__ == "__main__":
    test_rank4_mask()