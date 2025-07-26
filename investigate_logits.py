#!/usr/bin/env python3
"""
investigate_logits.py
Investigate the 16 logits outputs to understand their purpose
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
LM_HEAD_PATH = MODEL_DIR / "qwen_lm_head_lut6.mlmodelc"

def investigate_logits():
    try:
        lm_head_model = ct.models.CompiledMLModel(str(LM_HEAD_PATH))
        print(f"🔍 LM Head Model: {LM_HEAD_PATH.name}")
        
        # Test with a few different inputs
        for test_idx in range(3):
            print(f"\n🧪 Test {test_idx + 1}:")
            
            # Create different hidden states
            hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
            result = lm_head_model.predict({"hidden_states": hidden_states})
            
            # Check if all logits are the same or different
            logits_arrays = []
            for i in range(1, 17):  # logits1 to logits16
                key = f"logits{i}"
                if key in result:
                    logits_arrays.append(result[key])
            
            if len(logits_arrays) >= 2:
                # Compare first two logits arrays
                diff = np.abs(logits_arrays[0] - logits_arrays[1]).max()
                print(f"   Max difference between logits1 and logits2: {diff}")
                
                if diff < 1e-6:
                    print("   ✅ All logits appear to be identical (copies)")
                else:
                    print("   ❓ Logits are different - might need combination")
                    
                # Check vocabulary coverage
                total_vocab = sum(arr.shape[-1] for arr in logits_arrays)
                print(f"   Total vocabulary if concatenated: {total_vocab}")
                print(f"   Individual logits vocab size: {logits_arrays[0].shape[-1]}")
                
                # Sample from first logits
                first_logits = logits_arrays[0][0, 0, :]  # Shape: (9496,)
                top_5_indices = np.argsort(first_logits)[-5:][::-1]
                top_5_values = first_logits[top_5_indices]
                
                print(f"   Top 5 logits from logits1:")
                for idx, val in zip(top_5_indices, top_5_values):
                    print(f"     Token {idx}: {val:.4f}")
                    
                break
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    investigate_logits()