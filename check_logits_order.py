#!/usr/bin/env python3
"""
check_logits_order.py
Check if the order of logits concatenation matters
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
LM_HEAD_PATH = MODEL_DIR / "qwen_lm_head_lut6.mlmodelc"

def check_logits_order():
    try:
        lm_head_model = ct.models.CompiledMLModel(str(LM_HEAD_PATH))
        
        # Test with dummy input
        hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
        result = lm_head_model.predict({"hidden_states": hidden_states})
        
        print(f"🔍 LM Head outputs: {len(result)} outputs")
        
        # Check the order and ranges of each chunk
        for i in range(1, 17):
            key = f"logits{i}"
            if key in result:
                chunk = result[key][0, 0, :]  # Shape: (9496,)
                print(f"   {key}: shape {chunk.shape}, range [{chunk.min():.3f}, {chunk.max():.3f}], mean {chunk.mean():.3f}")
        
        # Test different concatenation orders
        print(f"\n🧪 Testing different concatenation orders:")
        
        # Order 1: logits1, logits2, ..., logits16 (current Rust implementation)
        order1_logits = []
        for i in range(1, 17):
            key = f"logits{i}"
            if key in result:
                chunk = result[key][0, 0, :]
                order1_logits.extend(chunk)
        order1_logits = np.array(order1_logits)
        
        # Order 2: Try sorted order by key name
        sorted_keys = sorted([k for k in result.keys() if k.startswith('logits')], 
                           key=lambda x: int(x.replace('logits', '')))
        order2_logits = []
        for key in sorted_keys:
            chunk = result[key][0, 0, :]
            order2_logits.extend(chunk)
        order2_logits = np.array(order2_logits)
        
        print(f"   Order 1 (1,2,3...16): top token {np.argmax(order1_logits)} with value {order1_logits.max():.3f}")
        print(f"   Order 2 (sorted): top token {np.argmax(order2_logits)} with value {order2_logits.max():.3f}")
        
        # Check if orders are the same
        if np.array_equal(order1_logits, order2_logits):
            print(f"   ✅ Both orders are identical")
        else:
            print(f"   ❌ Orders are different!")
            
        # Check specific token ranges
        # Token 9906 should be in a specific chunk
        target_token = 9906
        chunk_size = 9496
        expected_chunk = target_token // chunk_size + 1
        expected_idx_in_chunk = target_token % chunk_size
        
        print(f"\n🎯 Checking token {target_token}:")
        print(f"   Should be in logits{expected_chunk} at index {expected_idx_in_chunk}")
        
        if f"logits{expected_chunk}" in result:
            chunk = result[f"logits{expected_chunk}"][0, 0, :]
            actual_value = chunk[expected_idx_in_chunk]
            print(f"   Actual value in chunk: {actual_value:.3f}")
            
            # Check in concatenated array
            global_idx = (expected_chunk - 1) * chunk_size + expected_idx_in_chunk
            concat_value = order1_logits[global_idx]
            print(f"   Value in concatenated array: {concat_value:.3f}")
            
            if abs(actual_value - concat_value) < 1e-6:
                print(f"   ✅ Concatenation is correct")
            else:
                print(f"   ❌ Concatenation is wrong!")
        
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    check_logits_order()