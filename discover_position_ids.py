#!/usr/bin/env python3
"""
discover_position_ids.py
Try different position_ids configurations
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_position_ids():
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
        
        # Try different position_ids configurations
        position_ids_configs = [
            # Different data types
            ("int32 range", np.arange(seq_len, dtype=np.int32).reshape(1, seq_len)),
            ("int64 range", np.arange(seq_len, dtype=np.int64).reshape(1, seq_len)),
            ("float32 range", np.arange(seq_len, dtype=np.float32).reshape(1, seq_len)),
            
            # Different ranges
            ("0-based int32", np.arange(0, seq_len, dtype=np.int32).reshape(1, seq_len)),
            ("1-based int32", np.arange(1, seq_len + 1, dtype=np.int32).reshape(1, seq_len)),
            
            # Different shapes
            ("2D int32", np.arange(seq_len, dtype=np.int32).reshape(seq_len, 1)),
            ("1D int32", np.arange(seq_len, dtype=np.int32)),
            
            # Special values
            ("zeros int32", np.zeros(seq_len, dtype=np.int32).reshape(1, seq_len)),
            ("ones int32", np.ones(seq_len, dtype=np.int32).reshape(1, seq_len)),
        ]
        
        for desc, position_ids in position_ids_configs:
            try:
                print(f"\n🧪 Testing position_ids: {desc}")
                print(f"   Shape: {position_ids.shape}, dtype: {position_ids.dtype}")
                print(f"   Values: {position_ids.flatten()[:10]}...")
                
                inputs = {
                    "hidden_states": hidden_states,
                    "position_ids": position_ids,
                    "causal_mask": causal_mask
                }
                
                result = model.predict(inputs)
                print(f"✅ SUCCESS with {desc}!")
                print(f"   Outputs: {list(result.keys())}")
                for k, v in result.items():
                    print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
                return position_ids  # Return the working configuration
                
            except Exception as e:
                error_msg = str(e)
                if "range" in error_msg.lower():
                    print(f"❌ Range error: {error_msg[:150]}...")
                elif "shape" in error_msg.lower():
                    print(f"❌ Shape error: {error_msg[:150]}...")
                elif "type" in error_msg.lower():
                    print(f"❌ Type error: {error_msg[:150]}...")
                else:
                    print(f"❌ Error: {error_msg[:150]}...")
        
        print("\n❌ No working position_ids configuration found")
        return None
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return None

if __name__ == "__main__":
    test_position_ids()