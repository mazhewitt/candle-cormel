#!/usr/bin/env python3
"""
test_mlstate_workflow.py
Test the complete workflow with MLState for KV-cache management
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
COMPONENTS = {
    "embeddings": MODEL_DIR / "qwen_embeddings.mlmodelc",
    "ffn": MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc",
    "lm_head": MODEL_DIR / "qwen_lm_head_lut6.mlmodelc",
}

def test_complete_workflow():
    print("🧪 Testing complete multi-component workflow with MLState")
    
    # Load all models
    try:
        embeddings_model = ct.models.CompiledMLModel(str(COMPONENTS["embeddings"]))
        ffn_model = ct.models.CompiledMLModel(str(COMPONENTS["ffn"]))
        lm_head_model = ct.models.CompiledMLModel(str(COMPONENTS["lm_head"]))
        print("✅ All models loaded successfully")
    except Exception as e:
        print(f"❌ Failed to load models: {e}")
        return False
    
    # Test 1: Embeddings with single token (length 1)
    print("\n🔍 Step 1: Testing embeddings with single token")
    try:
        input_ids = np.array([[42]], dtype=np.int32)  # Single token
        print(f"   Input: {input_ids.shape}")
        
        embeddings_result = embeddings_model.predict({"input_ids": input_ids})
        hidden_states = embeddings_result["hidden_states"]
        print(f"   ✅ Embeddings output: {hidden_states.shape}")
    except Exception as e:
        print(f"   ❌ Embeddings failed: {e}")
        return False
    
    # Test 2: Create MLState for FFN
    print("\n🔍 Step 2: Creating MLState for FFN")
    try:
        ffn_state = ffn_model.make_state()
        print(f"   ✅ FFN state created: {type(ffn_state)}")
    except Exception as e:
        print(f"   ❌ MLState creation failed: {e}")
        return False
    
    # Test 3: FFN with MLState
    print("\n🔍 Step 3: Testing FFN with MLState")
    try:
        # Prepare FFN inputs
        position_ids = np.array([0], dtype=np.int32)  # Position 0
        current_pos = np.array([0], dtype=np.int32)   # Current position
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)  # Causal mask
        
        ffn_inputs = {
            "hidden_states": hidden_states,
            "position_ids": position_ids,
            "current_pos": current_pos,
            "causal_mask": causal_mask
        }
        
        print(f"   FFN inputs:")
        for k, v in ffn_inputs.items():
            print(f"     {k}: {v.shape}")
        
        ffn_result = ffn_model.predict_with_state(ffn_inputs, ffn_state)
        processed_hidden = ffn_result["hidden_states"]
        print(f"   ✅ FFN output: {processed_hidden.shape}")
    except Exception as e:
        print(f"   ❌ FFN failed: {e}")
        return False
    
    # Test 4: LM Head
    print("\n🔍 Step 4: Testing LM Head")
    try:
        lm_head_result = lm_head_model.predict({"hidden_states": processed_hidden})
        logits = lm_head_result["logits"]
        print(f"   ✅ LM Head output: {logits.shape}")
        
        # Sample next token (argmax)
        next_token = np.argmax(logits[0, 0, :])
        print(f"   🎲 Next token: {next_token}")
        
    except Exception as e:
        print(f"   ❌ LM Head failed: {e}")
        return False
    
    print("\n🎉 Complete workflow successful!")
    print("Summary of discovered architecture:")
    print("1. Embeddings: input_ids (1,1) -> hidden_states (1,1,1024)")
    print("2. FFN: hidden_states + position_ids + current_pos + causal_mask + MLState -> hidden_states")
    print("3. LM Head: hidden_states (1,1,1024) -> logits (1,1,vocab_size)")
    print("\nThis model uses stateful inference with KV-cache for autoregressive generation!")
    
    return True

if __name__ == "__main__":
    test_complete_workflow()