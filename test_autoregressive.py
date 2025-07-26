#!/usr/bin/env python3
"""
test_autoregressive.py
Test proper autoregressive generation with state management
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

def test_autoregressive_generation():
    print("🔍 Testing proper autoregressive generation")
    
    # Load models
    try:
        embeddings_model = ct.models.CompiledMLModel(str(COMPONENTS["embeddings"]))
        ffn_model = ct.models.CompiledMLModel(str(COMPONENTS["ffn"]))
        lm_head_model = ct.models.CompiledMLModel(str(COMPONENTS["lm_head"]))
        print("✅ All models loaded")
    except Exception as e:
        print(f"❌ Failed to load models: {e}")
        return
    
    # Create FFN state ONCE
    ffn_state = ffn_model.make_state()
    print("✅ FFN state created")
    
    # Test sequence: "Hello world" (simplified to just a few tokens)
    # Let's use actual tokens that should form coherent text
    test_tokens = [9906, 1917]  # Roughly "Hello world" 
    
    print(f"\n🎯 Testing with sequence: {test_tokens}")
    
    def process_token(token, position, state):
        """Process a single token through the pipeline"""
        print(f"\n  🔸 Processing token {token} at position {position}")
        
        # Step 1: Embeddings
        input_ids = np.array([[token]], dtype=np.int32)
        embeddings_result = embeddings_model.predict({"input_ids": input_ids})
        hidden_states = embeddings_result["hidden_states"]
        
        # Step 2: FFN with state
        position_ids = np.array([position], dtype=np.int32)
        current_pos = np.array([position], dtype=np.int32)
        causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
        
        ffn_inputs = {
            "hidden_states": hidden_states,
            "position_ids": position_ids,
            "current_pos": current_pos,
            "causal_mask": causal_mask
        }
        
        ffn_result = ffn_model.predict(ffn_inputs, state=state)
        processed_hidden = ffn_result["output_hidden_states"]
        
        print(f"     Hidden stats: min={processed_hidden.min():.3f}, max={processed_hidden.max():.3f}, mean={processed_hidden.mean():.3f}")
        
        return processed_hidden
    
    def get_next_token(hidden_states):
        """Get next token from hidden states"""
        lm_head_result = lm_head_model.predict({"hidden_states": hidden_states})
        
        # Concatenate all logits
        full_logits = []
        for i in range(1, 17):
            key = f"logits{i}"
            if key in lm_head_result:
                chunk = lm_head_result[key][0, 0, :]
                full_logits.extend(chunk)
        
        full_logits = np.array(full_logits)
        
        # Use greedy sampling for predictable results
        next_token = np.argmax(full_logits)
        confidence = full_logits[next_token]
        
        print(f"     Next token: {next_token} (confidence: {confidence:.3f})")
        return next_token
    
    # Process the input tokens (prefill phase)
    print("\n📝 Prefill phase:")
    last_hidden = None
    for pos, token in enumerate(test_tokens):
        last_hidden = process_token(token, pos, ffn_state)
    
    # Generation phase
    print("\n🎲 Generation phase:")
    generated_tokens = []
    current_pos = len(test_tokens)
    
    for step in range(5):  # Generate 5 tokens
        # Get next token
        next_token = get_next_token(last_hidden)
        generated_tokens.append(next_token)
        
        # Process the new token
        last_hidden = process_token(next_token, current_pos, ffn_state)
        current_pos += 1
        
        # Stop if we hit a reasonable stopping point
        if next_token in [151645, 151643]:  # Common EOS tokens
            break
    
    print(f"\n📊 Results:")
    print(f"   Input tokens: {test_tokens}")
    print(f"   Generated tokens: {generated_tokens}")
    
    # Check if tokens are reasonable
    if len(set(generated_tokens)) == 1:
        print("   ❌ All generated tokens are the same (likely stuck)")
    elif any(t > 151936 for t in generated_tokens):
        print("   ❌ Generated tokens are out of vocabulary range")
    else:
        print("   ✅ Generated tokens look reasonable")

if __name__ == "__main__":
    test_autoregressive_generation()