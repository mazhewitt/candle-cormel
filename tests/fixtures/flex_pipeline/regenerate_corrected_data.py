#!/usr/bin/env python3
"""
Regenerate all test data files with corrected settings matching typo_fixer_complete.py exactly.
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

def tokenize_input_corrected(tokenizer, text, max_length=64):
    """Tokenize input text exactly like typo_fixer_complete.py."""
    inputs = tokenizer(
        text, 
        return_tensors="np", 
        add_special_tokens=True, 
        truncation=True, 
        max_length=max_length
    )
    return inputs['input_ids'].astype(np.int32)

def make_causal_mask(length, start):
    """Create causal attention mask."""
    mask = np.full((1, 1, length, length), -np.inf, dtype=np.float16)
    row_indices = np.arange(length).reshape(length, 1)
    col_indices = np.arange(length).reshape(1, length)
    mask[:, :, col_indices <= (row_indices + start)] = 0
    return mask

def main():
    """Regenerate all test data with corrected settings."""
    print("🔄 REGENERATING TEST DATA - Corrected Pipeline")
    print("=" * 80)
    
    # Exact same settings as typo_fixer_complete.py
    test_sentence = "This setence has multple typos in it"
    tokenizer_path = "mazhewitt/qwen-typo-fixer"
    model_dir = "/Users/mazdahewitt/projects/train-typo-fixer/models/qwen-typo-fixer-ane-flex"
    
    # Load tokenizer
    tokenizer = AutoTokenizer.from_pretrained(tokenizer_path, trust_remote_code=True)
    
    # Load models
    embeddings_path = os.path.join(model_dir, "qwen-typo-fixer_embeddings.mlpackage")
    prefill_path = os.path.join(model_dir, "qwen-typo-fixer_prefill_chunk_01of01.mlpackage")
    infer_path = os.path.join(model_dir, "qwen-typo-fixer_FFN_chunk_01of01.mlpackage")
    lm_head_path = os.path.join(model_dir, "qwen-typo-fixer_lm_head.mlpackage")
    
    embeddings_model = ct.models.MLModel(embeddings_path)
    prefill_model = ct.models.MLModel(prefill_path)
    infer_model = ct.models.MLModel(infer_path)
    lm_head_model = ct.models.MLModel(lm_head_path)
    
    print("✅ Models loaded")
    
    # === STEP 1: CORRECTED TOKENIZATION ===
    print("\n📄 Step 1: Corrected tokenization...")
    prompt = create_basic_prompt(test_sentence)
    input_ids = tokenize_input_corrected(tokenizer, prompt, max_length=64)
    context_pos = input_ids.shape[1]  # This follows typo_fixer_complete.py exactly
    
    step1_data = {
        "metadata": {
            "step": "1_tokenize_corrected",
            "input_text": test_sentence,
            "prompt": prompt,
            "tokenizer_path": tokenizer_path,
            "max_length": 64,
            "use_basic": True,
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected tokenization matching typo_fixer_complete.py exactly"
        },
        "data": {
            "input_ids": input_ids.tolist(),
            "context_pos": int(context_pos),
            "tensor_shape": list(input_ids.shape),
            "dtype": str(input_ids.dtype)
        }
    }
    
    with open("test_generation/corrected_step_1_tokens.json", 'w') as f:
        json.dump(step1_data, f, indent=2)
    print(f"   ✅ Saved: context_pos={context_pos}, shape={input_ids.shape}")
    
    # === STEP 2: CAUSAL MASK ===
    print("\n📄 Step 2: Causal mask...")
    context_length = 256
    causal_mask = make_causal_mask(context_length, 0)
    
    step2_data = {
        "metadata": {
            "step": "2_causal_mask_corrected",
            "input_file": "corrected_step_1_tokens.json",
            "context_length": context_length,
            "context_pos": context_pos,
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected causal attention mask"
        },
        "data": {
            "input_ids": input_ids.tolist(),
            "input_ids_shape": list(input_ids.shape),
            "causal_mask": causal_mask.tolist(),
            "causal_mask_shape": list(causal_mask.shape),
            "context_pos": context_pos,
            "context_length": context_length
        }
    }
    
    with open("test_generation/corrected_step_2_causal_mask.json", 'w') as f:
        json.dump(step2_data, f, indent=2)
    print(f"   ✅ Saved: causal_mask shape={causal_mask.shape}")
    
    # === STEP 3: PREFILL PREPARATION ===
    print("\n📄 Step 3: Prefill preparation...")
    batch_size = 128
    batch_pos = 0
    batch_end = min(batch_pos + batch_size, context_pos)
    current_batch_size = batch_end - batch_pos
    
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
    hidden_states = embeddings_model.predict({"input_ids": batch_input})['hidden_states']
    
    step3_data = {
        "metadata": {
            "step": "3_prefill_input_corrected",
            "input_file": "corrected_step_2_causal_mask.json", 
            "batch_size": batch_size,
            "batch_pos": batch_pos,
            "batch_end": batch_end,
            "current_batch_size": current_batch_size,
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected prefill input preparation"
        },
        "data": {
            "batch_input": batch_input.tolist(),
            "hidden_states": hidden_states.tolist(),
            "hidden_states_shape": list(hidden_states.shape),
            "position_ids": position_ids.tolist(),
            "causal_mask": batch_causal_mask.tolist(),
            "causal_mask_shape": list(batch_causal_mask.shape),
            "context_pos": context_pos
        }
    }
    
    with open("test_generation/corrected_step_3_prefill_input.json", 'w') as f:
        json.dump(step3_data, f, indent=2)
    print(f"   ✅ Saved: hidden_states shape={hidden_states.shape}")
    
    # === STEP 4: PREFILL EXECUTION ===
    print("\n📄 Step 4: Prefill execution...")
    
    # Initialize KV state
    kv_state = prefill_model.make_state()
    
    # Run prefill
    prefill_inputs = {
        'hidden_states': hidden_states.astype(np.float16),
        'position_ids': position_ids,
        'causal_mask': batch_causal_mask,
        'current_pos': np.array([batch_pos], dtype=np.int32)
    }
    
    prefill_output = prefill_model.predict(prefill_inputs, kv_state)
    
    step4_data = {
        "metadata": {
            "step": "4_prefill_output_corrected",
            "input_file": "corrected_step_3_prefill_input.json",
            "kv_state_initialized": True,
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected prefill execution with proper KV state"
        },
        "data": {
            "prefill_completed": True,
            "kv_state_ready": True,
            "next_token_pos": context_pos,
            "context_pos": context_pos,
            "prefill_output_keys": list(prefill_output.keys()),
            "prefill_output_hidden_states": prefill_output['output_hidden_states'].tolist(),
            "prefill_output_shape": list(prefill_output['output_hidden_states'].shape),
            "prefill_inputs": {
                "hidden_states": prefill_inputs['hidden_states'].tolist(),
                "hidden_states_shape": list(prefill_inputs['hidden_states'].shape),
                "position_ids": prefill_inputs['position_ids'].tolist(),
                "position_ids_shape": list(prefill_inputs['position_ids'].shape),
                "causal_mask": prefill_inputs['causal_mask'].tolist(),
                "causal_mask_shape": list(prefill_inputs['causal_mask'].shape),
                "current_pos": prefill_inputs['current_pos'].tolist(),
                "current_pos_shape": list(prefill_inputs['current_pos'].shape)
            }
        }
    }
    
    with open("test_generation/corrected_step_4_prefill_output.json", 'w') as f:
        json.dump(step4_data, f, indent=2)
    print(f"   ✅ Saved: KV state ready for position {context_pos}")
    
    # === STEP 5: INFER AND LOGITS ===
    print("\n📄 Step 5: Infer and logits...")
    
    # Get current token for infer (last token from context)
    current_token = input_ids[:, context_pos-1:context_pos]
    
    # Run embeddings on current token
    current_hidden_states = embeddings_model.predict({"input_ids": current_token})['hidden_states']
    
    # Prepare infer inputs
    infer_position_ids = np.array([context_pos-1], dtype=np.int32)
    single_causal_mask = causal_mask[:, :, context_pos-1:context_pos, :].astype(np.float16)
    
    infer_inputs = {
        'hidden_states': current_hidden_states.astype(np.float16),
        'position_ids': infer_position_ids,
        'causal_mask': single_causal_mask,
        'current_pos': infer_position_ids
    }
    
    # Run infer with same KV state
    infer_output = infer_model.predict(infer_inputs, kv_state)
    output_hidden_states = infer_output['output_hidden_states']
    
    # Run LM head
    if output_hidden_states.shape[1] > 1:
        output_hidden_states = output_hidden_states[:, -1:, :]
    
    lm_result = lm_head_model.predict({"hidden_states": output_hidden_states.astype(np.float16)})
    
    # Combine logits
    all_logits = []
    for i in range(1, 17):
        key = f"logits{i}"
        if key in lm_result:
            all_logits.append(lm_result[key])
    
    combined_logits = np.concatenate(all_logits, axis=-1)
    
    # Get top predictions
    top_5_indices = np.argsort(combined_logits[0, 0])[-5:][::-1]
    top_5_logits = combined_logits[0, 0][top_5_indices]
    
    step5_data = {
        "metadata": {
            "step": "5_infer_and_logits_corrected",
            "input_file": "corrected_step_4_prefill_output.json",
            "context_pos": context_pos,
            "timestamp": datetime.now().isoformat(),
            "description": "Corrected infer and logits with proper KV state continuity"
        },
        "data": {
            "current_token": current_token.tolist(),
            "current_token_text": tokenizer.decode(current_token[0], skip_special_tokens=False),
            "infer_input_hidden_states": current_hidden_states.tolist(),
            "infer_input_shape": list(current_hidden_states.shape),
            "infer_output_hidden_states": output_hidden_states.tolist(),
            "infer_output_shape": list(output_hidden_states.shape),
            "final_logits": {
                "logits": combined_logits.tolist(),
                "shape": list(combined_logits.shape)
            },
            "top_predictions": {
                "indices": top_5_indices.tolist(),
                "logits": top_5_logits.tolist(),
                "tokens": [tokenizer.decode([idx], skip_special_tokens=False) for idx in top_5_indices]
            },
            "first_token_matches_original": int(top_5_indices[0]) == 13
        }
    }
    
    with open("test_generation/corrected_step_5_infer_and_logits.json", 'w') as f:
        json.dump(step5_data, f, indent=2)
    
    first_token_id = int(top_5_indices[0])
    first_token_text = tokenizer.decode([first_token_id], skip_special_tokens=False)
    
    print(f"   ✅ Saved: First token {first_token_id} ('{first_token_text}')")
    
    # === SUMMARY ===
    print("\n" + "=" * 80)
    print("📊 CORRECTED TEST DATA REGENERATION SUMMARY")
    print("=" * 80)
    print(f"✅ All corrected files generated")
    print(f"✅ First token: {first_token_id} ('{first_token_text}')")
    print(f"✅ Matches typo_fixer_complete.py: {first_token_id == 13}")
    print(f"✅ Context position: {context_pos}")
    print(f"✅ Proper KV state continuity maintained")
    
    files_created = [
        "corrected_step_1_tokens.json",
        "corrected_step_2_causal_mask.json", 
        "corrected_step_3_prefill_input.json",
        "corrected_step_4_prefill_output.json",
        "corrected_step_5_infer_and_logits.json"
    ]
    
    print(f"\n📁 Files created:")
    for filename in files_created:
        filepath = f"test_generation/{filename}"
        if os.path.exists(filepath):
            size = os.path.getsize(filepath)
            print(f"   ✅ {filename} ({size:,} bytes)")
    
    print("=" * 80)

if __name__ == "__main__":
    main()