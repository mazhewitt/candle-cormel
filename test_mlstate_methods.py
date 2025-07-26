#!/usr/bin/env python3
"""
test_mlstate_methods.py
Find the correct method to use with MLState
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def test_mlstate_methods():
    try:
        ffn_model = ct.models.CompiledMLModel(str(FFN_PATH))
        print(f"🔍 FFN Model methods:")
        methods = [attr for attr in dir(ffn_model) if not attr.startswith('_')]
        for method in methods:
            print(f"   {method}")
        
        # Create state
        state = ffn_model.make_state()
        print(f"\n🔍 MLState type: {type(state)}")
        print(f"MLState methods:")
        state_methods = [attr for attr in dir(state) if not attr.startswith('_')]
        for method in state_methods:
            print(f"   {method}")
        
        # Try to predict with state
        hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
        position_ids = np.array([0], dtype=np.int32)
        current_pos = np.array([0], dtype=np.int32)
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
        
        ffn_inputs = {
            "hidden_states": hidden_states,
            "position_ids": position_ids,
            "current_pos": current_pos,
            "causal_mask": causal_mask
        }
        
        # Check if predict accepts state parameter
        try:
            result = ffn_model.predict(ffn_inputs, state=state)
            print(f"\n✅ predict(inputs, state=state) works!")
            print(f"Output keys: {list(result.keys())}")
        except Exception as e:
            print(f"\n❌ predict(inputs, state=state) failed: {e}")
            
            # Try other variations
            try:
                ffn_inputs["model_model_kv_cache_0"] = state
                result = ffn_model.predict(ffn_inputs)
                print(f"✅ Including state as input works!")
                print(f"Output keys: {list(result.keys())}")
            except Exception as e2:
                print(f"❌ Including state as input failed: {e2}")
        
    except Exception as e:
        print(f"❌ Failed: {e}")

if __name__ == "__main__":
    test_mlstate_methods()