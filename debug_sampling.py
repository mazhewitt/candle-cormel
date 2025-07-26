#!/usr/bin/env python3
"""
debug_sampling.py
Debug the sampling process to see what's going wrong
"""

import coremltools as ct
from pathlib import Path
import numpy as np
import json

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
COMPONENTS = {
    "embeddings": MODEL_DIR / "qwen_embeddings.mlmodelc",
    "ffn": MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc",
    "lm_head": MODEL_DIR / "qwen_lm_head_lut6.mlmodelc",
}

def debug_full_pipeline():
    print("🔍 Debugging the full pipeline with a simple prompt")
    
    # Load tokenizer to understand tokens
    tokenizer_path = MODEL_DIR / "tokenizer.json"
    if tokenizer_path.exists():
        with open(tokenizer_path) as f:
            tokenizer_data = json.load(f)
        print(f"✅ Tokenizer loaded")
    
    # Test prompt
    test_prompt = "Hello"
    print(f"🧪 Test prompt: '{test_prompt}'")
    
    # Load models
    try:
        embeddings_model = ct.models.CompiledMLModel(str(COMPONENTS["embeddings"]))
        ffn_model = ct.models.CompiledMLModel(str(COMPONENTS["ffn"]))
        lm_head_model = ct.models.CompiledMLModel(str(COMPONENTS["lm_head"]))
        print("✅ All models loaded")
    except Exception as e:
        print(f"❌ Failed to load models: {e}")
        return
    
    # Test with simple token sequence
    # Token for "Hello" should be around 9906 based on common tokenizers
    # Let's test with a known good token
    test_token = 9906  # "Hello" in many tokenizers
    
    print(f"\n🔸 Testing with token {test_token}")
    
    # Step 1: Embeddings
    input_ids = np.array([[test_token]], dtype=np.int32)
    print(f"   Input shape: {input_ids.shape}")
    
    try:
        embeddings_result = embeddings_model.predict({"input_ids": input_ids})
        hidden_states = embeddings_result["hidden_states"]
        print(f"   ✅ Embeddings: {hidden_states.shape}")
        print(f"   Embeddings stats: min={hidden_states.min():.4f}, max={hidden_states.max():.4f}, mean={hidden_states.mean():.4f}")
    except Exception as e:
        print(f"   ❌ Embeddings failed: {e}")
        return
    
    # Step 2: FFN with state
    try:
        ffn_state = ffn_model.make_state()
        
        position_ids = np.array([0], dtype=np.int32)
        current_pos = np.array([0], dtype=np.int32)
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
        
        ffn_inputs = {
            "hidden_states": hidden_states,
            "position_ids": position_ids,
            "current_pos": current_pos,
            "causal_mask": causal_mask
        }
        
        ffn_result = ffn_model.predict(ffn_inputs, state=ffn_state)
        processed_hidden = ffn_result["output_hidden_states"]
        print(f"   ✅ FFN: {processed_hidden.shape}")
        print(f"   FFN stats: min={processed_hidden.min():.4f}, max={processed_hidden.max():.4f}, mean={processed_hidden.mean():.4f}")
    except Exception as e:
        print(f"   ❌ FFN failed: {e}")
        return
    
    # Step 3: LM Head
    try:
        lm_head_result = lm_head_model.predict({"hidden_states": processed_hidden})
        
        # Concatenate all logits
        full_logits = []
        for i in range(1, 17):
            key = f"logits{i}"
            if key in lm_head_result:
                chunk = lm_head_result[key][0, 0, :]  # Shape: (9496,)
                full_logits.extend(chunk)
        
        full_logits = np.array(full_logits)
        print(f"   ✅ LM Head: {len(full_logits)} logits")
        print(f"   Logits stats: min={full_logits.min():.4f}, max={full_logits.max():.4f}, mean={full_logits.mean():.4f}")
        
        # Check top tokens
        top_5_indices = np.argsort(full_logits)[-5:][::-1]
        top_5_values = full_logits[top_5_indices]
        
        print(f"\n📊 Top 5 tokens:")
        for idx, val in zip(top_5_indices, top_5_values):
            print(f"     Token {idx}: {val:.4f}")
            
        # Check if logits are reasonable
        if np.isnan(full_logits).any():
            print("❌ NaN values in logits!")
        elif np.isinf(full_logits).any():
            print("❌ Infinite values in logits!")
        elif full_logits.max() - full_logits.min() < 0.1:
            print("❌ Logits have very small range - might be all similar values")
        else:
            print("✅ Logits look reasonable")
            
        # Test with temperature sampling
        temperature = 0.7
        scaled_logits = full_logits / temperature
        max_logit = scaled_logits.max()
        exp_logits = np.exp(scaled_logits - max_logit)
        probabilities = exp_logits / exp_logits.sum()
        
        print(f"\n🎲 Sampling with temperature {temperature}:")
        print(f"   Probability stats: min={probabilities.min():.6f}, max={probabilities.max():.6f}")
        
        # Sample multiple times to see consistency
        samples = []
        for _ in range(5):
            sample_idx = np.random.choice(len(probabilities), p=probabilities)
            samples.append(sample_idx)
        
        print(f"   Sample tokens: {samples}")
        
    except Exception as e:
        print(f"   ❌ LM Head failed: {e}")
        return

if __name__ == "__main__":
    debug_full_pipeline()