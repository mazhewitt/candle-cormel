#!/usr/bin/env python3
"""
test_all_inputs.py
Test with all required inputs including current_pos
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_all_inputs():
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
        
        # Current position (likely also single value)
        current_pos = np.array([0], dtype=np.int32)
        
        # Causal mask shape: (1, 1, 1, 512)
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
        
        try:
            print(f"\n🧪 Testing with all required inputs:")
            print(f"   hidden_states: {hidden_states.shape}")
            print(f"   position_ids: {position_ids.shape}")
            print(f"   current_pos: {current_pos.shape}")
            print(f"   causal_mask: {causal_mask.shape}")
            
            inputs = {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "current_pos": current_pos,
                "causal_mask": causal_mask
            }
            
            result = model.predict(inputs)
            print(f"✅ SUCCESS! All FFN inputs working!")
            print(f"   Outputs: {list(result.keys())}")
            for k, v in result.items():
                print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
            return inputs
            
        except Exception as e:
            error_msg = str(e)
            print(f"❌ Error: {error_msg[:200]}...")
            
            # Try different current_pos values/shapes if error
            for pos_val in [1, 63, 64]:
                try:
                    current_pos = np.array([pos_val], dtype=np.int32)
                    inputs["current_pos"] = current_pos
                    result = model.predict(inputs)
                    print(f"✅ SUCCESS with current_pos = {pos_val}!")
                    return inputs
                except:
                    continue
            
            return None
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return None

if __name__ == "__main__":
    working_config = test_all_inputs()
    if working_config:
        print(f"\n🎉 Working FFN configuration discovered!")
        print("Required inputs:")
        for k, v in working_config.items():
            print(f"  {k}: shape {v.shape}, dtype {v.dtype}")