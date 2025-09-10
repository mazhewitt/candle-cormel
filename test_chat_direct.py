#!/usr/bin/env python3
"""Direct test of chat.py logic with the same model Rust uses"""

import os
import sys
import torch
import numpy as np
import coremltools as ct
from transformers import AutoTokenizer
import torch.nn.functional as F

# Model directory - same as what Rust uses
model_dir = "/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"

print(f"Loading models from: {model_dir}")

# Load tokenizer
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2.5-0.5B-Instruct", trust_remote_code=True)

# Load CoreML models - these are what Rust loads
embed_model = ct.models.MLModel(os.path.join(model_dir, "qwen_embeddings.mlmodelc"))
ffn_model = ct.models.MLModel(os.path.join(model_dir, "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc"))
lm_head = ct.models.MLModel(os.path.join(model_dir, "qwen_lm_head_lut8.mlmodelc"))

print("Models loaded successfully")

# Test prompt - exact same as Rust test
prompt = "The quick brown fox jumps over the lazy"
print(f"\nPrompt: '{prompt}'")

# Tokenize
input_ids = tokenizer.encode(prompt, return_tensors='pt')
print(f"Token IDs: {input_ids[0].tolist()}")
print(f"Token count: {len(input_ids[0])}")

# Create state for stateful model
state = ffn_model.make_state()

# Parameters
context_pos = len(input_ids[0])
batch_size = 64
context_length = 512

# ===== PREFILL PHASE (like chat.py) =====
print("\n=== PREFILL PHASE ===")

# Pad input to batch size
padded_input = F.pad(input_ids, (0, batch_size - context_pos), value=0)
print(f"Padded input shape: {padded_input.shape}")

# Run embeddings
embed_output = embed_model.predict({'input_ids': padded_input.numpy().astype(np.int32)})
hidden_states = torch.from_numpy(embed_output['hidden_states'])
print(f"Embeddings shape: {hidden_states.shape}")

# Create position IDs (0 to batch_size-1)
position_ids = torch.arange(batch_size, dtype=torch.int32)

# Create causal mask
def make_causal_mask(context_length, prefetch):
    mask = np.ones((1, 1, context_length, context_length), dtype=np.float16)
    for i in range(context_length):
        for j in range(i + 1 + prefetch, context_length):
            mask[0, 0, i, j] = -65500.0
    return mask

causal_mask = torch.tensor(make_causal_mask(context_length, 0), dtype=torch.float16)
batch_causal_mask = causal_mask[:, :, :batch_size, :]

# Run FFN prefill
current_pos = torch.tensor([0], dtype=torch.int32)
ffn_output = ffn_model.predict({
    'hidden_states': hidden_states.numpy().astype(np.float16),
    'position_ids': position_ids.numpy().astype(np.int32),
    'causal_mask': batch_causal_mask.numpy().astype(np.float16),
    'current_pos': current_pos.numpy().astype(np.int32)
}, state)

print(f"FFN prefill complete")

# ===== INFERENCE PHASE (generate next token) =====
print("\n=== INFERENCE PHASE ===")

# Get embedding for last token
pos = context_pos
current_token = input_ids[:, pos-1:pos]
print(f"Current token for inference: {current_token[0].tolist()}")

# Run embeddings for single token
hidden_states = torch.from_numpy(
    embed_model.predict({'input_ids': current_token.numpy().astype(np.int32)})['hidden_states']
)
print(f"Infer embedding shape: {hidden_states.shape}")

# Create masks for inference
update_mask = torch.zeros((1, 1, context_length, 1), dtype=torch.float16)
update_mask[0, 0, pos-1, 0] = 1.0
position_ids = torch.tensor([pos-1], dtype=torch.int32)
single_causal_mask = causal_mask[:, :, pos-1:pos, :]

# Run FFN inference
ffn_inputs = {
    'hidden_states': hidden_states.numpy().astype(np.float16),
    'update_mask': update_mask.numpy().astype(np.float16),
    'position_ids': position_ids.numpy().astype(np.int32),
    'causal_mask': single_causal_mask.numpy().astype(np.float16),
    'current_pos': position_ids.numpy().astype(np.int32)
}

# Note: The model might have a unified interface or separate prefill/infer
# Try with update_mask first (infer mode)
try:
    ffn_output = ffn_model.predict(ffn_inputs, state)
    hidden_states = torch.from_numpy(ffn_output['output_hidden_states'])
    print(f"FFN infer output shape: {hidden_states.shape}")
except:
    # If that fails, try without update_mask (prefill mode)
    print("Trying without update_mask...")
    ffn_inputs.pop('update_mask')
    ffn_output = ffn_model.predict(ffn_inputs, state)
    hidden_states = torch.from_numpy(ffn_output['output_hidden_states'])
    print(f"FFN output shape: {hidden_states.shape}")

# Run LM head
lm_output = lm_head.predict({'hidden_states': hidden_states.numpy().astype(np.float16)})

# Get logits
print(f"\nLM head outputs: {list(lm_output.keys())}")

# Combine logits parts
logits_parts = []
for i in range(1, 17):  # Check up to 16 parts
    key = f'logits{i}'
    if key in lm_output:
        logits_parts.append(torch.from_numpy(lm_output[key]))

if logits_parts:
    logits = torch.cat(logits_parts, dim=-1)
    print(f"Combined logits shape: {logits.shape}")
    
    # Get the predicted token (greedy sampling)
    next_token = torch.argmax(logits[0, -1, :]).item()
    
    print(f"\n{'='*50}")
    print(f"RESULT FROM PYTHON/CHAT.PY LOGIC:")
    print(f"{'='*50}")
    print(f"Next token ID: {next_token}")
    print(f"Next token decoded: '{tokenizer.decode([next_token])}'")
    
    # Check what it is
    if next_token == 5562:
        print("✅ Python generated 'dog' (token 5562)")
    elif next_token == 3974:
        print("⚠️ Python also generated ' quick' (token 3974) - same as Rust!")
    else:
        decoded = tokenizer.decode([next_token])
        print(f"❓ Python generated unexpected token: {next_token} ('{decoded}')")
        
    # Show top 5 predictions
    logits_flat = logits[0, -1, :]
    top5 = torch.topk(logits_flat, 5)
    print(f"\nTop 5 predictions:")
    for i, (score, idx) in enumerate(zip(top5.values, top5.indices)):
        token_id = idx.item()
        decoded = tokenizer.decode([token_id])
        print(f"  {i+1}. Token {token_id} ('{decoded}'): {score:.2f}")
else:
    print("ERROR: No logits found in output")