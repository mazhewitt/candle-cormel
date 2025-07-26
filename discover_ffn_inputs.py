#!/usr/bin/env python3
"""
discover_ffn_inputs.py
Try to figure out what the FFN model actually needs for inputs
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
FFN_PATH = MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"

def discover_ffn_inputs():
    if not FFN_PATH.exists():
        print(f"❌ FFN model not found: {FFN_PATH}")
        return

    try:
        model = ct.models.CompiledMLModel(str(FFN_PATH))
        print(f"🔍 FFN Model: {FFN_PATH.name}")
        
        # Based on the error, we know it needs position_ids
        # Let's try different combinations of inputs
        
        seq_len = 64  # We know this works
        hidden_dim = 1024  # From embeddings output
        
        # Create position_ids (likely just 0, 1, 2, ..., seq_len-1)
        position_ids = np.arange(seq_len, dtype=np.int32).reshape(1, seq_len)
        
        # Create hidden states (from embeddings)
        hidden_states = np.random.randn(1, seq_len, hidden_dim).astype(np.float32)
        
        # Try different mask configurations
        causal_mask = np.zeros((seq_len, seq_len), dtype=np.float32)
        # Fill upper triangle with -inf for causal masking
        for i in range(seq_len):
            for j in range(i + 1, seq_len):
                causal_mask[i, j] = float('-inf')
                
        attention_mask = np.ones((1, seq_len), dtype=np.float32)  # All tokens are valid
        
        # Try different input combinations
        test_cases = [
            # Case 1: All three inputs
            {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "causal_mask": causal_mask
            },
            # Case 2: With attention mask instead of causal mask
            {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "attention_mask": attention_mask
            },
            # Case 3: Try different mask shapes
            {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "causal_mask": causal_mask.reshape(1, seq_len, seq_len)  # Add batch dim
            },
            # Case 4: Try attention_mask with causal_mask
            {
                "hidden_states": hidden_states,
                "position_ids": position_ids,
                "attention_mask": attention_mask,
                "causal_mask": causal_mask
            },
        ]
        
        for i, inputs in enumerate(test_cases, 1):
            try:
                print(f"\n🧪 Test case {i}:")
                print(f"   Inputs: {list(inputs.keys())}")
                for k, v in inputs.items():
                    print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
                
                result = model.predict(inputs)
                print(f"✅ SUCCESS!")
                print(f"   Outputs: {list(result.keys())}")
                for k, v in result.items():
                    print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
                return inputs  # Return the working configuration
                
            except Exception as e:
                error_msg = str(e)
                if "required but not specified" in error_msg:
                    missing = error_msg.split("Feature ")[1].split(" is required")[0]
                    print(f"❌ Missing input: {missing}")
                elif "shape" in error_msg.lower():
                    print(f"❌ Shape error: {error_msg[:100]}...")
                else:
                    print(f"❌ Error: {error_msg[:100]}...")
        
        print("\n❌ No working configuration found")
        return None
        
    except Exception as e:
        print(f"❌ Failed to load FFN model: {e}")
        return None

if __name__ == "__main__":
    discover_ffn_inputs()