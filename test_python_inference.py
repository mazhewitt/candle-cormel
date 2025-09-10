#!/usr/bin/env python3
"""Test inference with Python to see what token is generated"""

import os
import sys
import torch
import numpy as np
import coremltools as ct
from transformers import AutoTokenizer

# Model directory
model_dir = "/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"

# Load models
print(f"Loading models from: {model_dir}")

# Load tokenizer
tokenizer = AutoTokenizer.from_pretrained("Qwen/Qwen2.5-0.5B-Instruct", trust_remote_code=True)

# Load CoreML models
embed_model = ct.models.MLModel(os.path.join(model_dir, "qwen_embeddings.mlmodelc"))
ffn_model = ct.models.MLModel(os.path.join(model_dir, "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc"))
lm_head = ct.models.MLModel(os.path.join(model_dir, "qwen_lm_head_lut8.mlmodelc"))

print("Models loaded successfully")

# Test prompt
prompt = "The quick brown fox jumps over the lazy"
print(f"\nPrompt: {prompt}")

# Tokenize
input_ids = tokenizer.encode(prompt, return_tensors='pt')
print(f"Token IDs: {input_ids[0].tolist()}")
print(f"Token count: {len(input_ids[0])}")

# Create state
state = ffn_model.make_state()
print("State created")

# Run prefill
context_pos = len(input_ids[0])
batch_size = 64

# Pad input to batch size
padded_input = torch.nn.functional.pad(input_ids, (0, batch_size - context_pos), value=0)
print(f"Padded input shape: {padded_input.shape}")

# Run embeddings
hidden_states = embed_model.predict({'input_ids': padded_input.numpy().astype(np.int32)})['hidden_states']
print(f"Embeddings shape: {hidden_states.shape}")

# Create position IDs
position_ids = np.arange(batch_size, dtype=np.int32)

# Create causal mask (simplified - all ones for now)
causal_mask = np.ones((1, 1, batch_size, 512), dtype=np.float16)

# Run FFN prefill
current_pos = np.array([0], dtype=np.int32)
ffn_output = ffn_model.predict({
    'hidden_states': hidden_states.astype(np.float16),
    'position_ids': position_ids,
    'causal_mask': causal_mask,
    'current_pos': current_pos
}, state)
print(f"FFN prefill output shape: {ffn_output['output_hidden_states'].shape}")

# Now run inference for the last position
# Get embedding for just the last token (position context_pos - 1)
last_token = input_ids[0, -1:].numpy().astype(np.int32)
infer_hidden = embed_model.predict({'input_ids': last_token})['hidden_states']
print(f"Infer embedding shape: {infer_hidden.shape}")

# Position for inference
infer_pos = np.array([context_pos - 1], dtype=np.int32)
infer_causal_mask = causal_mask[:, :, context_pos-1:context_pos, :]

# Run FFN infer (using same model since we don't have separate infer)
infer_output = ffn_model.predict({
    'hidden_states': infer_hidden.astype(np.float16),
    'position_ids': infer_pos,
    'causal_mask': infer_causal_mask.astype(np.float16),
    'current_pos': infer_pos
}, state)
print(f"FFN infer output shape: {infer_output['output_hidden_states'].shape}")

# Run LM head
lm_output = lm_head.predict({'hidden_states': infer_output['output_hidden_states'].astype(np.float16)})

# Check what outputs we got
print(f"LM head outputs: {list(lm_output.keys())}")

# Combine logits
logits_parts = []
for i in range(1, 17):  # Try up to 16 parts
    key = f'logits{i}'
    if key in lm_output:
        logits_parts.append(torch.from_numpy(lm_output[key]))
    else:
        break

if logits_parts:
    logits = torch.cat(logits_parts, dim=-1)
    print(f"Combined logits shape: {logits.shape}")
    
    # Get the predicted token
    next_token = torch.argmax(logits[0, -1, :]).item()
    print(f"\n=== RESULT ===")
    print(f"Next token ID: {next_token}")
    print(f"Next token decoded: '{tokenizer.decode([next_token])}'")
    
    # Check if it's dog or quick
    if next_token == 5562:
        print("✅ Generated 'dog' as expected!")
    elif next_token == 3974:
        print("❌ Generated 'quick' instead of 'dog'")
    else:
        decoded = tokenizer.decode([next_token])
        print(f"⚠️ Generated unexpected token: {next_token} ('{decoded}')")
else:
    print("No logits found in output")