#!/usr/bin/env python3
"""
Corrected Pipeline: Complete end-to-end test with proper KV state continuity
Uses exact same settings as typo_fixer_complete.py and maintains KV state from prefill to infer.
"""

import json
import numpy as np
import coremltools as ct
import os
from datetime import datetime
from transformers import AutoTokenizer

def create_basic_prompt(text_with_typos):
    """Create basic prompt (from typo_fixer_complete.py)."""
    return f"Fix: {text_with_typos}"

def tokenize_input(tokenizer, text, max_length=64):
    """Tokenize input text (from typo_fixer_complete.py)."""
    inputs = tokenizer(
        text, 
        return_tensors="np", 
        max_length=max_length, 
        padding="max_length", 
        truncation=True,
        add_special_tokens=True
    )
    return inputs['input_ids'].astype(np.int32)

def make_causal_mask(length, start):
    """Create causal attention mask (from typo_fixer_complete.py)."""
    mask = np.full((1, 1, length, length), -np.inf, dtype=np.float16)
    row_indices = np.arange(length).reshape(length, 1)
    col_indices = np.arange(length).reshape(1, length)
    mask[:, :, col_indices <= (row_indices + start)] = 0
    return mask

def run_embeddings(embeddings_model, input_ids):
    """Run embeddings model."""
    result = embeddings_model.predict({"input_ids": input_ids})
    return result['hidden_states']

def run_lm_head(lm_head_model, hidden_states):
    """Run LM Head model to get token predictions."""
    if hidden_states.shape[1] > 1:
        hidden_states = hidden_states[:, -1:, :]  # Take last token
    
    result = lm_head_model.predict({"hidden_states": hidden_states.astype(np.float16)})
    
    # Combine all logits parts
    all_logits = []
    for i in range(1, 17):  # 16 parts
        key = f"logits{i}"
        if key in result:
            all_logits.append(result[key])
    
    combined_logits = np.concatenate(all_logits, axis=-1)
    return combined_logits

def main():
    """Run corrected end-to-end pipeline with proper KV state continuity."""
    # Use exact same settings as typo_fixer_complete.py
    test_sentence = "This setence has multple typos in it"
    model_dir = "/Users/mazdahewitt/projects/train-typo-fixer/models/qwen-typo-fixer-ane-flex"
    tokenizer_path = "mazhewitt/qwen-typo-fixer"
    
    print("🔧 CORRECTED PIPELINE - Exact typo_fixer_complete.py replication")
    print("=" * 80)
    print(f"Test sentence: '{test_sentence}'")
    print()
    
    # Load tokenizer
    tokenizer = AutoTokenizer.from_pretrained(tokenizer_path, trust_remote_code=True)
    print("✅ Tokenizer loaded")
    
    # Load models
    embeddings_path = os.path.join(model_dir, "qwen-typo-fixer_embeddings.mlpackage")
    prefill_path = os.path.join(model_dir, "qwen-typo-fixer_prefill_chunk_01of01.mlpackage")
    infer_path = os.path.join(model_dir, "qwen-typo-fixer_FFN_chunk_01of01.mlpackage")
    lm_head_path = os.path.join(model_dir, "qwen-typo-fixer_lm_head.mlpackage")
    
    embeddings_model = ct.models.MLModel(embeddings_path)
    prefill_model = ct.models.MLModel(prefill_path)
    infer_model = ct.models.MLModel(infer_path)
    lm_head_model = ct.models.MLModel(lm_head_path)
    print("✅ All models loaded")
    print()
    
    # Step 1: Create exact same prompt and tokenize
    prompt = create_basic_prompt(test_sentence)
    print(f"📝 Basic prompt: '{prompt}'")
    
    input_ids = tokenize_input(tokenizer, prompt, max_length=64)
    
    # Find actual context length (before padding tokens)
    # The prompt has 12 actual tokens, but gets padded to 64
    tokens_no_pad = tokenizer(prompt, return_tensors="np", add_special_tokens=True)
    context_pos = tokens_no_pad['input_ids'].shape[1]  # This gives us 12
    
    print(f"🔤 Tokenized to {context_pos} actual tokens (padded shape: {input_ids.shape})")
    print()
    
    # Step 2: Create causal mask
    context_length = 256
    causal_mask = make_causal_mask(context_length, 0)
    print(f"📐 Causal mask created: {causal_mask.shape}")
    print()
    
    # Step 3: Initialize KV state from PREFILL model (critical!)
    print("🏗️  Initializing KV state from prefill model...")
    kv_state = prefill_model.make_state()
    print("✅ KV state initialized")
    print()
    
    # Step 4: Run prefill with exact same logic as typo_fixer_complete.py
    print("🏃 Running prefill phase...")
    batch_size = 128
    batch_pos = 0
    batch_end = min(batch_pos + batch_size, context_pos)
    current_batch_size = batch_end - batch_pos
    
    print(f"   📦 Prefill batch {batch_pos}-{batch_end-1} ({current_batch_size} tokens)")
    
    # Get current batch
    batch_input = input_ids[:, batch_pos:batch_end]
    
    # Always pad to full batch size for prefill
    if current_batch_size < batch_size:
        pad_size = batch_size - current_batch_size
        padding = np.zeros((1, pad_size), dtype=np.int32)
        batch_input = np.concatenate([batch_input, padding], axis=1)
    
    # Generate position IDs for full batch size
    position_ids = np.arange(batch_pos, batch_pos + batch_size, dtype=np.int32)
    batch_causal_mask = causal_mask[:, :, batch_pos:batch_pos + batch_size, :].astype(np.float16)
    
    # Run embeddings
    hidden_states = run_embeddings(embeddings_model, batch_input)
    print(f"   📥 Embeddings output: {hidden_states.shape}")
    
    # Run prefill with shared KV state
    prefill_inputs = {
        'hidden_states': hidden_states.astype(np.float16),
        'position_ids': position_ids,
        'causal_mask': batch_causal_mask,
        'current_pos': np.array([batch_pos], dtype=np.int32)
    }
    
    print(f"   🔄 Running prefill with KV state...")
    prefill_output = prefill_model.predict(prefill_inputs, kv_state)
    print(f"   ✅ Prefill completed, KV state updated")
    print()
    
    # Step 5: Run infer with SAME KV state (this is the key!)
    print("🎯 Running infer phase (first token generation)...")
    
    # Position for next token generation
    current_pos = context_pos  # Position 12 for our prompt
    
    # Get current token (last token from context)
    current_token = input_ids[:, current_pos-1:current_pos]  # [1, 1]
    token_text = tokenizer.decode(current_token[0], skip_special_tokens=False)
    print(f"   📍 Current token: {current_token[0][0]} ('{token_text}')")
    
    # Run embeddings on current token
    current_hidden_states = run_embeddings(embeddings_model, current_token)
    print(f"   📥 Current token embeddings: {current_hidden_states.shape}")
    
    # Prepare infer inputs
    infer_position_ids = np.array([current_pos-1], dtype=np.int32)
    single_causal_mask = causal_mask[:, :, current_pos-1:current_pos, :].astype(np.float16)
    
    infer_inputs = {
        'hidden_states': current_hidden_states.astype(np.float16),
        'position_ids': infer_position_ids,
        'causal_mask': single_causal_mask,
        'current_pos': infer_position_ids
    }
    
    print(f"   🔄 Running infer with SAME KV state...")
    infer_output = infer_model.predict(infer_inputs, kv_state)
    output_hidden_states = infer_output['output_hidden_states']
    print(f"   ✅ Infer completed: {output_hidden_states.shape}")
    
    # Step 6: Run LM head to get logits
    print("🎯 Running LM head...")
    logits = run_lm_head(lm_head_model, output_hidden_states)
    print(f"   ✅ Logits generated: {logits.shape}")
    
    # Get top predictions
    top_5_indices = np.argsort(logits[0, 0])[-5:][::-1]
    top_5_logits = logits[0, 0][top_5_indices]
    
    print(f"\n🏆 TOP 5 PREDICTIONS:")
    for i, (idx, logit) in enumerate(zip(top_5_indices, top_5_logits)):
        token_text = tokenizer.decode([idx], skip_special_tokens=False)
        marker = "🎯" if i == 0 else "  "
        print(f"   {marker} {i+1}. Token {idx}: '{token_text}' (logit: {logit:.4f})")
    
    first_token_id = int(top_5_indices[0])
    first_token_text = tokenizer.decode([first_token_id], skip_special_tokens=False)
    
    print(f"\n📊 RESULT:")
    print(f"   First predicted token: {first_token_id} ('{first_token_text}')")
    print(f"   Expected from typo_fixer_complete.py: 13 ('.')")
    
    if first_token_id == 13:
        print(f"   ✅ SUCCESS! Matches typo_fixer_complete.py exactly!")
    else:
        print(f"   ❌ Mismatch - need further investigation")
    
    # Save corrected pipeline data
    output_data = {
        "metadata": {
            "step": "corrected_pipeline",
            "test_sentence": test_sentence,
            "prompt": prompt,
            "settings": {
                "use_basic": True,
                "max_length": 64,
                "context_pos": int(context_pos),
                "context_length": context_length
            },
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected pipeline with proper KV state continuity"
        },
        "data": {
            "input_ids": input_ids.tolist(),
            "context_pos": int(context_pos),
            "prefill_completed": True,
            "infer_completed": True,
            "first_token_prediction": {
                "token_id": first_token_id,
                "token_text": first_token_text,
                "logit": float(top_5_logits[0])
            },
            "top_5_predictions": {
                "indices": top_5_indices.tolist(),
                "logits": top_5_logits.tolist(),
                "tokens": [tokenizer.decode([idx], skip_special_tokens=False) for idx in top_5_indices]
            },
            "matches_original": first_token_id == 13
        }
    }
    
    with open("test_generation/corrected_pipeline_result.json", 'w') as f:
        json.dump(output_data, f, indent=2)
    
    print(f"\n✅ Saved corrected pipeline results")
    print("=" * 80)

if __name__ == "__main__":
    main()