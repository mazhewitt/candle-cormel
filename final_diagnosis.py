#!/usr/bin/env python3
"""
final_diagnosis.py
Compare working vs broken inference patterns
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

def load_tokenizer():
    tokenizer_path = MODEL_DIR / "tokenizer.json"
    with open(tokenizer_path) as f:
        data = json.load(f)
    
    # Create simple vocab lookup
    vocab = {v: k for k, v in data["model"]["vocab"].items()}
    return vocab

def working_generation():
    """Test with a very simple, known working pattern"""
    
    print("🔍 Testing simple known working patterns")
    
    # Load models
    embeddings_model = ct.models.CompiledMLModel(str(COMPONENTS["embeddings"]))
    ffn_model = ct.models.CompiledMLModel(str(COMPONENTS["ffn"]))
    lm_head_model = ct.models.CompiledMLModel(str(COMPONENTS["lm_head"]))
    vocab = load_tokenizer()
    
    # Test extremely simple case: single token "Hello" -> what should come next?
    hello_token = 9906  # "Hello"
    
    print(f"\n🧪 Testing: What comes after 'Hello' (token {hello_token})?")
    
    # Create fresh state
    state = ffn_model.make_state()
    
    # Process "Hello" at position 0
    print(f"\n1. Processing 'Hello' at position 0")
    input_ids = np.array([[hello_token]], dtype=np.int32)
    embeddings_result = embeddings_model.predict({"input_ids": input_ids})
    hidden_states = embeddings_result["hidden_states"]
    
    # FFN with proper setup
    position_ids = np.array([0], dtype=np.int32)
    current_pos = np.array([0], dtype=np.int32)
    causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
    # Only position 0 should be visible
    causal_mask[:, :, :, 1:] = -np.inf
    
    ffn_inputs = {
        "hidden_states": hidden_states,
        "position_ids": position_ids,
        "current_pos": current_pos,
        "causal_mask": causal_mask
    }
    
    ffn_result = ffn_model.predict(ffn_inputs, state=state)
    hello_hidden = ffn_result["output_hidden_states"]
    print(f"   Hello hidden stats: mean={hello_hidden.mean():.3f}, std={hello_hidden.std():.3f}")
    
    # Get next token prediction
    lm_result = lm_head_model.predict({"hidden_states": hello_hidden})
    
    # Concatenate logits and find top tokens
    full_logits = []
    for i in range(1, 17):
        key = f"logits{i}"
        if key in lm_result:
            chunk = lm_result[key][0, 0, :]
            full_logits.extend(chunk)
    
    full_logits = np.array(full_logits)
    
    # Find top 10 tokens
    top_indices = np.argsort(full_logits)[-10:][::-1]
    top_values = full_logits[top_indices]
    
    print(f"\n📊 Top tokens after 'Hello':")
    for i, (token_id, score) in enumerate(zip(top_indices, top_values)):
        token_text = vocab.get(token_id, f"<unk_{token_id}>")
        print(f"   {i+1}. Token {token_id} ({token_text}): {score:.3f}")
    
    # Test what happens if we pick the top token and continue
    next_token = top_indices[0]
    print(f"\n2. Continuing with token {next_token} at position 1")
    
    # Process next token
    input_ids = np.array([[next_token]], dtype=np.int32)
    embeddings_result = embeddings_model.predict({"input_ids": input_ids})
    next_hidden_states = embeddings_result["hidden_states"]
    
    # FFN for position 1 (can see positions 0 and 1)
    position_ids = np.array([1], dtype=np.int32)
    current_pos = np.array([1], dtype=np.int32)
    causal_mask = np.zeros((1, 1, 1, 512), dtype=np.float32)
    # Positions 0 and 1 should be visible
    causal_mask[:, :, :, 2:] = -np.inf
    
    ffn_inputs = {
        "hidden_states": next_hidden_states,
        "position_ids": position_ids,
        "current_pos": current_pos,
        "causal_mask": causal_mask
    }
    
    ffn_result = ffn_model.predict(ffn_inputs, state=state)  # Same state!
    next_processed_hidden = ffn_result["output_hidden_states"]
    print(f"   Next hidden stats: mean={next_processed_hidden.mean():.3f}, std={next_processed_hidden.std():.3f}")
    
    # Check if the hidden states changed significantly
    if np.allclose(hello_hidden, next_processed_hidden, atol=0.1):
        print("   ❌ Hidden states barely changed - state not working!")
    else:
        print("   ✅ Hidden states changed significantly - state is working!")
    
    # Get prediction for third token
    lm_result = lm_head_model.predict({"hidden_states": next_processed_hidden})
    
    full_logits = []
    for i in range(1, 17):
        key = f"logits{i}"
        if key in lm_result:
            chunk = lm_result[key][0, 0, :]
            full_logits.extend(chunk)
    
    full_logits = np.array(full_logits)
    third_token = np.argmax(full_logits)
    third_score = full_logits[third_token]
    third_text = vocab.get(third_token, f"<unk_{third_token}>")
    
    print(f"\n3. Third token prediction: {third_token} ({third_text}): {third_score:.3f}")
    
    # Check if we're getting the same token repeatedly
    if next_token == third_token:
        print("   ❌ Getting same token again - model is stuck!")
    else:
        print("   ✅ Different token - model is progressing!")

if __name__ == "__main__":
    working_generation()