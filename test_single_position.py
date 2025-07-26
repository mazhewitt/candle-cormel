#!/usr/bin/env python3
"""
test_single_position.py
Test with single position_ids value since model wants shape (1)
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_single_position():
    if not FFN_PATH.exists():
        print(f"❌ FFN model not found: {FFN_PATH}")
        return

    try:
        model = ct.models.CompiledMLModel(str(FFN_PATH))
        print(f"🔍 FFN Model: {FFN_PATH.name}")
        
        seq_len = 64
        hidden_dim = 1024
        
        # Create hidden states
        hidden_states = np.random.randn(1, seq_len, hidden_dim).astype(np.float32)
        
        # Create causal mask
        causal_mask = np.zeros((seq_len, seq_len), dtype=np.float32)
        for i in range(seq_len):
            for j in range(i + 1, seq_len):
                causal_mask[i, j] = float('-inf')
        
        # Try single position values
        for pos_val in [0, 1, 63, 64]:
            try:
                print(f"\n🧪 Testing position_ids = {pos_val}")
                
                # Single position value - shape (1,)
                position_ids = np.array([pos_val], dtype=np.int32)
                print(f"   Shape: {position_ids.shape}, dtype: {position_ids.dtype}")
                
                inputs = {
                    "hidden_states": hidden_states,
                    "position_ids": position_ids,
                    "causal_mask": causal_mask
                }
                
                result = model.predict(inputs)
                print(f"✅ SUCCESS with position_ids = {pos_val}!")
                print(f"   Outputs: {list(result.keys())}")
                for k, v in result.items():
                    print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
                return pos_val  # Return the working value
                
            except Exception as e:
                error_msg = str(e)
                print(f"❌ Error with pos={pos_val}: {error_msg[:100]}...")
        
        print("\n❌ No working position_ids value found")
        return None
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return None

if __name__ == "__main__":
    test_single_position()