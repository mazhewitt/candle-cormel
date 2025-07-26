#!/usr/bin/env python3
"""
test_final_ffn.py
Test with the exact shapes the FFN model expects
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_final_ffn():
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
        
        # Exact shape expected: (1, 1, 1, 512)
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
        
        try:
            print(f"\n🧪 Testing with exact expected shapes:")
            print(f"   hidden_states: {hidden_states.shape}")
            print(f"   position_ids: {position_ids.shape}")
            print(f"   causal_mask: {causal_mask.shape}")
            
            inputs = {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "causal_mask": causal_mask
            }
            
            result = model.predict(inputs)
            print(f"✅ SUCCESS! FFN model working!")
            print(f"   Outputs: {list(result.keys())}")
            for k, v in result.items():
                print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
            return True
            
        except Exception as e:
            error_msg = str(e)
            print(f"❌ Error: {error_msg[:200]}...")
            return False
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return False

if __name__ == "__main__":
    test_final_ffn()