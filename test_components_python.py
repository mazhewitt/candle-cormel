#!/usr/bin/env python3
"""
test_components_python.py
Break down the Python inference pipeline into testable parts
"""

import coremltools as ct
from pathlib import Path
import numpy as np
import json
import torch
from transformers import AutoTokenizer

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")

def test_tokenization():
    """Test tokenization with fixed input"""
    print("=" * 60)
    print("STEP 1: TOKENIZATION")
    print("=" * 60)
    
    tokenizer = AutoTokenizer.from_pretrained(str(MODEL_DIR))
    
    # Use simple fixed input
    test_input = "Hello world"
    chat_template = f"<|im_start|>user\n{test_input}<|im_end|>\n<|im_start|>assistant\n"
    
    print(f"Input text: {repr(test_input)}")
    print(f"Chat template: {repr(chat_template)}")
    
    # Tokenize
    tokens = tokenizer.encode(chat_template, add_special_tokens=False)
    
    print(f"Tokens: {tokens}")
    print(f"Token count: {len(tokens)}")
    
    # Decode each token individually
    print("Token breakdown:")
    for i, token_id in enumerate(tokens):
        token_text = tokenizer.decode([token_id])
        print(f"  {i}: {token_id} -> {repr(token_text)}")
    
    return tokens, tokenizer

def test_embeddings(tokens):
    """Test embeddings layer with fixed tokens"""
    print("\n" + "=" * 60)
    print("STEP 2: EMBEDDINGS")
    print("=" * 60)
    
    embeddings_model = ct.models.CompiledMLModel(str(MODEL_DIR / "qwen_embeddings.mlmodelc"))
    
    # Test with single token first
    test_token = tokens[0]  # First token
    print(f"Testing embeddings with token: {test_token}")
    
    input_ids = np.array([[test_token]], dtype=np.int32)
    print(f"Input shape: {input_ids.shape}")
    
    result = embeddings_model.predict({"input_ids": input_ids})
    hidden_states = result["hidden_states"]
    
    print(f"Output shape: {hidden_states.shape}")
    print(f"Output stats: min={hidden_states.min():.6f}, max={hidden_states.max():.6f}, mean={hidden_states.mean():.6f}, std={hidden_states.std():.6f}")
    
    # Show first few values for comparison
    print(f"First 10 values: {hidden_states[0, 0, :10].tolist()}")
    
    return hidden_states

def test_ffn_single_token(hidden_states, position=0):
    """Test FFN layer with single token"""
    print("\n" + "=" * 60)
    print(f"STEP 3: FFN (position {position})")
    print("=" * 60)
    
    ffn_model = ct.models.CompiledMLModel(str(MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc"))
    
    # Create state
    state = ffn_model.make_state()
    print(f"Created MLState: {type(state)}")
    
    # Prepare inputs exactly as in original
    position_ids = np.array([position], dtype=np.int32)
    current_pos = np.array([position], dtype=np.int32)
    
    # Create causal mask - only allow access to positions up to current
    causal_mask = np.full((1, 1, 1, 512), -np.inf, dtype=np.float32)
    for i in range(position + 1):
        causal_mask[0, 0, 0, i] = 0.0
    
    print(f"position_ids: {position_ids}")
    print(f"current_pos: {current_pos}")
    print(f"causal_mask shape: {causal_mask.shape}")
    print(f"causal_mask[0,0,0,:5]: {causal_mask[0,0,0,:5]}")  # Show first 5 positions
    
    ffn_inputs = {
        "hidden_states": hidden_states,
        "position_ids": position_ids,
        "current_pos": current_pos,
        "causal_mask": causal_mask
    }
    
    # Run FFN
    result = ffn_model.predict(ffn_inputs, state=state)
    output_hidden = result["output_hidden_states"]
    
    print(f"Input hidden shape: {hidden_states.shape}")
    print(f"Output hidden shape: {output_hidden.shape}")
    print(f"Input stats: min={hidden_states.min():.6f}, max={hidden_states.max():.6f}, mean={hidden_states.mean():.6f}")
    print(f"Output stats: min={output_hidden.min():.6f}, max={output_hidden.max():.6f}, mean={output_hidden.mean():.6f}")
    
    # Show first few values for comparison
    print(f"Input first 10 values: {hidden_states[0, 0, :10].tolist()}")
    print(f"Output first 10 values: {output_hidden[0, 0, :10].tolist()}")
    
    return output_hidden, state

def test_lm_head(hidden_states):
    """Test LM head with hidden states"""
    print("\n" + "=" * 60)
    print("STEP 4: LM HEAD")
    print("=" * 60)
    
    lm_head_model = ct.models.CompiledMLModel(str(MODEL_DIR / "qwen_lm_head_lut6.mlmodelc"))
    
    print(f"Input hidden shape: {hidden_states.shape}")
    print(f"Input stats: min={hidden_states.min():.6f}, max={hidden_states.max():.6f}, mean={hidden_states.mean():.6f}")
    
    result = lm_head_model.predict({"hidden_states": hidden_states})
    
    print(f"LM head outputs: {list(result.keys())}")
    
    # Concatenate all logits chunks
    full_logits = []
    for i in range(1, 17):
        key = f"logits{i}"
        if key in result:
            chunk = result[key][0, 0, :]  # Shape: (9496,)
            full_logits.extend(chunk)
            print(f"  {key}: shape {result[key].shape}, range [{chunk.min():.3f}, {chunk.max():.3f}]")
    
    full_logits = np.array(full_logits)
    print(f"\nConcatenated logits: {len(full_logits)} total")
    print(f"Logits stats: min={full_logits.min():.6f}, max={full_logits.max():.6f}, mean={full_logits.mean():.6f}")
    
    # Find top 5 tokens
    top_indices = np.argsort(full_logits)[-5:][::-1]
    top_values = full_logits[top_indices]
    
    print("\nTop 5 tokens:")
    for i, (token_id, score) in enumerate(zip(top_indices, top_values)):
        print(f"  {i+1}. Token {token_id}: {score:.6f}")
    
    return full_logits

def test_sampling(logits, temperature=0.7):
    """Test sampling with fixed temperature"""
    print("\n" + "=" * 60)
    print(f"STEP 5: SAMPLING (temperature={temperature})")
    print("=" * 60)
    
    if temperature == 0.0:
        # Greedy sampling
        next_token = np.argmax(logits)
        print(f"Greedy sampling: token {next_token}, score {logits[next_token]:.6f}")
    else:
        # Temperature sampling
        scaled_logits = logits / temperature
        max_logit = scaled_logits.max()
        exp_logits = np.exp(scaled_logits - max_logit)
        probabilities = exp_logits / exp_logits.sum()
        
        print(f"Temperature scaling applied")
        print(f"Probability stats: min={probabilities.min():.8f}, max={probabilities.max():.8f}, sum={probabilities.sum():.8f}")
        
        # Sample with fixed seed for reproducibility
        np.random.seed(42)
        next_token = np.random.choice(len(probabilities), p=probabilities)
        
        print(f"Sampled token: {next_token}, probability {probabilities[next_token]:.8f}")
    
    return next_token

def main():
    print("🔍 Testing Python inference pipeline step by step")
    print("Fixed input: 'Hello world'")
    print("This will generate reference outputs for comparison with Rust")
    
    # Step 1: Tokenization
    tokens, tokenizer = test_tokenization()
    
    # Step 2: Embeddings (first token only)
    hidden_states = test_embeddings(tokens)
    
    # Step 3: FFN (position 0)
    processed_hidden, state = test_ffn_single_token(hidden_states, position=0)
    
    # Step 4: LM Head
    logits = test_lm_head(processed_hidden)
    
    # Step 5: Sampling
    next_token = test_sampling(logits, temperature=0.7)
    
    print("\n" + "=" * 60)
    print("SUMMARY")
    print("=" * 60)
    print(f"Input: 'Hello world'")
    print(f"First token: {tokens[0]}")
    print(f"After embeddings: mean={hidden_states.mean():.6f}")
    print(f"After FFN: mean={processed_hidden.mean():.6f}")
    print(f"Top logit: {logits.max():.6f}")
    print(f"Sampled next token: {next_token}")
    
    # Decode the next token
    next_token_text = tokenizer.decode([next_token])
    print(f"Next token text: {repr(next_token_text)}")

if __name__ == "__main__":
    main()