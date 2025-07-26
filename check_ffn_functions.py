#!/usr/bin/env python3
"""
check_ffn_functions.py
Check what functions/configurations are available in the FFN model
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def check_ffn_functions():
    try:
        ffn_model = ct.models.CompiledMLModel(str(FFN_PATH))
        print(f"🔍 FFN Model: {FFN_PATH.name}")
        
        # Check available attributes
        attrs = [attr for attr in dir(ffn_model) if not attr.startswith('_')]
        print(f"Available attributes: {attrs}")
        
        # Check if there are different function names
        if hasattr(ffn_model, 'function_name'):
            print(f"Function name: {ffn_model.function_name}")
        
        # Test with different sequence lengths to understand the model's expectations
        print(f"\n🧪 Testing different input configurations:")
        
        # Test 1: Single token (what we've been doing)
        print(f"\n1. Single token test:")
        try:
            state = ffn_model.make_state()
            hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
            position_ids = np.array([0], dtype=np.int32)
            current_pos = np.array([0], dtype=np.int32)
            causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
            
            inputs = {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "current_pos": current_pos,
                "causal_mask": causal_mask
            }
            
            result = ffn_model.predict(inputs, state=state)
            print(f"   ✅ Single token works: {list(result.keys())}")
        except Exception as e:
            print(f"   ❌ Single token failed: {e}")
        
        # Test 2: Multi-token sequence (prefill style)
        print(f"\n2. Multi-token test (length 4):")
        try:
            state = ffn_model.make_state()
            hidden_states = np.random.randn(1, 4, 1024).astype(np.float32)
            position_ids = np.array([0, 1, 2, 3], dtype=np.int32)
            current_pos = np.array([0], dtype=np.int32)  # Start position
            causal_mask = np.zeros((1, 1, 4, 512), dtype=np.float32)
            
            # Fill causal mask properly
            for i in range(4):
                for j in range(i + 1, 4):
                    causal_mask[0, 0, i, j] = -np.inf
            
            inputs = {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "current_pos": current_pos,
                "causal_mask": causal_mask
            }
            
            result = ffn_model.predict(inputs, state=state)
            print(f"   ✅ Multi-token works: output shape {result['output_hidden_states'].shape}")
        except Exception as e:
            print(f"   ❌ Multi-token failed: {e}")
            
        # Test 3: Check if we need to use different current_pos values
        print(f"\n3. Testing current_pos values:")
        try:
            state = ffn_model.make_state()
            hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
            
            for pos in [0, 1, 2, 5]:
                position_ids = np.array([pos], dtype=np.int32)
                current_pos = np.array([pos], dtype=np.int32)
                causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
                
                inputs = {
                    "hidden_states": hidden_states,
                    "position_ids": position_ids,
                    "current_pos": current_pos,
                    "causal_mask": causal_mask
                }
                
                result = ffn_model.predict(inputs, state=state)
                output_stats = result['output_hidden_states']
                print(f"   Position {pos}: mean={output_stats.mean():.3f}, std={output_stats.std():.3f}")
                
        except Exception as e:
            print(f"   ❌ Position test failed: {e}")
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    check_ffn_functions()