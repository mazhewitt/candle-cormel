#!/usr/bin/env python3
"""
qwen-chat-test.py - Capture intermediate tensors from working Python pipeline

This script runs "The quick brown fox jumps over the lazy" through the complete
Python CoreML pipeline and saves all intermediate tensors for debugging the
Rust implementation.

Usage: python3 qwen-chat-test.py --meta meta.yaml
"""

import argparse
import numpy as np
import torch
import torch.nn.functional as F
from pathlib import Path
import os

# Import from chat.py
from chat import (
    load_models, initialize_tokenizer, create_unified_state, 
    initialize_causal_mask, run_prefill, generate_next_token,
    make_causal_mask
)

def save_tensor(tensor, name, output_dir):
    """Save a tensor to numpy format with metadata."""
    output_dir = Path(output_dir)
    output_dir.mkdir(exist_ok=True)
    
    if isinstance(tensor, torch.Tensor):
        array = tensor.numpy()
    else:
        array = np.array(tensor)
    
    filepath = output_dir / f"{name}.npy"
    np.save(filepath, array)
    print(f"Saved {name}: shape {array.shape}, dtype {array.dtype} -> {filepath}")
    
    # Save metadata
    metadata_path = output_dir / f"{name}_metadata.txt"
    with open(metadata_path, 'w') as f:
        f.write(f"name: {name}\n")
        f.write(f"shape: {array.shape}\n")
        f.write(f"dtype: {array.dtype}\n")
        if array.size < 20:  # Only show small arrays
            f.write(f"values: {array}\n")
        f.write(f"min: {array.min()}, max: {array.max()}, mean: {array.mean()}\n")

def capture_tensors_from_pipeline(model_dir, prompt="The quick brown fox jumps over the lazy"):
    """Capture all intermediate tensors from the complete pipeline."""
    
    print(f"🔍 Capturing tensors for prompt: '{prompt}'")
    output_dir = Path("test_tensors")
    output_dir.mkdir(exist_ok=True)
    
    # Set up arguments similar to chat.py main()
    class Args:
        def __init__(self):
            self.d = str(model_dir)
            self.embed = "qwen_embeddings"
            self.ffn = "qwen_FFN_PF_lut8_chunk_01of01" 
            self.lmhead = "qwen_lm_head_lut8"
            self.tokenizer = str(model_dir)
            self.context_length = 512
            self.batch_size = 64
            self.num_logits = 8
            self.split_lm_head = 16
            self.eval = True  # Suppress verbose output
    
    args = Args()
    
    # Update paths to be absolute
    model_dir = Path(args.d).resolve()
    args.embed = str(model_dir / args.embed)
    args.ffn = str(model_dir / args.ffn)
    args.lmhead = str(model_dir / args.lmhead)
    
    print(f"📂 Using model directory: {model_dir}")
    
    try:
        # Load models
        metadata = {}
        embed_model, ffn_models, lmhead_model, metadata = load_models(args, metadata)
        metadata['context_length'] = args.context_length
        metadata['split_lm_head'] = args.split_lm_head
        
        # Load tokenizer
        tokenizer = initialize_tokenizer(args.tokenizer, eval_mode=True)
        
        # Create state and causal mask
        state = create_unified_state(ffn_models, metadata['context_length'], eval_mode=True)
        causal_mask = initialize_causal_mask(metadata['context_length'], eval_mode=True)
        
        print(f"✅ Models loaded successfully")
        
        # Step 1: Tokenize input
        print(f"\n📝 Step 1: Tokenization")
        input_ids = tokenizer(
            prompt,
            return_tensors="pt",
            add_special_tokens=True
        ).input_ids.to(torch.int32)
        
        print(f"Input text: '{prompt}'")
        print(f"Token IDs: {input_ids.tolist()[0]}")
        
        # Decode tokens to verify
        decoded_tokens = [tokenizer.decode([token_id]) for token_id in input_ids[0]]
        print(f"Decoded tokens: {decoded_tokens}")
        
        save_tensor(input_ids, "01_input_tokens", output_dir)
        
        context_pos = input_ids.size(1)
        context_length = metadata['context_length']
        batch_size = args.batch_size
        
        print(f"Context position: {context_pos}, Context length: {context_length}")
        
        # Step 2: Run embeddings on full input sequence
        print(f"\n🔤 Step 2: Embeddings")
        
        # For prefill, we need to process in batches, but let's capture the full embeddings first
        input_for_embeddings = input_ids
        
        # Always pad to batch size for prefill
        if context_pos < batch_size:
            input_for_embeddings = F.pad(
                input_ids,
                (0, batch_size - context_pos),
                value=0
            )
        
        print(f"Embeddings input shape: {input_for_embeddings.shape}")
        embeddings_output = torch.from_numpy(
            embed_model.predict({
                'input_ids': input_for_embeddings.numpy().astype(np.int32)
            })['hidden_states']
        )
        
        print(f"Embeddings output shape: {embeddings_output.shape}")
        save_tensor(input_for_embeddings, "02_embeddings_input", output_dir)
        save_tensor(embeddings_output, "02_embeddings_output", output_dir)
        
        # Step 3: Run prefill phase
        print(f"\n⚡ Step 3: FFN Prefill Phase")
        
        # We'll manually step through the prefill to capture intermediate states
        batch_pos = 0
        prefill_outputs = []
        
        while batch_pos < context_pos:
            batch_end = min(batch_pos + batch_size, context_pos)
            current_batch_size = batch_end - batch_pos
            
            print(f"  Processing prefill batch: {batch_pos} to {batch_end}")
            
            # Get current batch of embeddings
            batch_embeddings = embeddings_output[:, batch_pos:batch_end, :]
            
            # Always pad to full batch size for prefill
            if current_batch_size < batch_size:
                padding_size = batch_size - current_batch_size
                batch_embeddings = F.pad(
                    batch_embeddings,
                    (0, 0, 0, padding_size),  # Pad sequence dimension
                    value=0
                )
            
            # Generate position IDs for full batch size
            position_ids = torch.arange(batch_pos, batch_pos + batch_size, dtype=torch.int32)
            batch_causal_mask = causal_mask[:, :, batch_pos:batch_pos + batch_size, :]
            
            # Run through FFN chunks with state
            hidden_states = batch_embeddings
            for chunk_idx, ffn_model in enumerate(ffn_models):
                if isinstance(ffn_model, dict):
                    inputs = {
                        'hidden_states': hidden_states.numpy().astype(np.float32),  # Use float32
                        'position_ids': position_ids.numpy().astype(np.int32),
                        'causal_mask': batch_causal_mask.numpy().astype(np.float32),  # Use float32
                        'current_pos': np.array([batch_pos], dtype=np.int32)
                    }
                    
                    # Capture inputs to first FFN chunk
                    if chunk_idx == 0 and batch_pos == 0:
                        print(f"🔍 Prefill inputs for chunk {chunk_idx}:")
                        print(f"  hidden_states: shape={inputs['hidden_states'].shape}, sample={inputs['hidden_states'][0,0,:3]}")
                        print(f"  position_ids: shape={inputs['position_ids'].shape}, sample={inputs['position_ids'][:5]}")
                        print(f"  current_pos: {inputs['current_pos']}")
                        
                        save_tensor(inputs['hidden_states'], "03_ffn_prefill_hidden_input", output_dir)
                        save_tensor(inputs['position_ids'], "03_ffn_prefill_position_ids", output_dir)
                        save_tensor(inputs['causal_mask'], "03_ffn_prefill_causal_mask", output_dir)
                        save_tensor(inputs['current_pos'], "03_ffn_prefill_current_pos", output_dir)
                    
                    output = ffn_model['prefill'].predict(inputs, state)
                    hidden_states = torch.from_numpy(output['output_hidden_states']).float()  # Ensure float32
                    
                    # Capture output from first FFN chunk, first batch
                    if chunk_idx == 0 and batch_pos == 0:
                        save_tensor(hidden_states, "03_ffn_prefill_output", output_dir)
                        print(f"🔍 Prefill output for chunk {chunk_idx}: shape={hidden_states.shape}, sample={hidden_states[0,0,:5].tolist()}")
            
            prefill_outputs.append(hidden_states)
            batch_pos = batch_end
        
        print(f"✅ Prefill phase complete")
        
        # Step 4: Generate next token using infer phase
        print(f"\n🎯 Step 4: FFN Infer Phase (Generate next token)")
        
        # Get current token (last token from input)
        current_token = input_ids[:, context_pos-1:context_pos]  # [1, 1]
        
        print(f"Current token: {current_token.item()} ('{tokenizer.decode(current_token[0])}')")
        
        # Run embeddings on single token - CRITICAL: Use float32 not float16
        single_token_embeddings = torch.from_numpy(
            embed_model.predict({'input_ids': current_token.numpy().astype(np.int32)})['hidden_states']
        ).float()  # Ensure float32
        
        save_tensor(current_token, "04_infer_input_token", output_dir)
        save_tensor(single_token_embeddings, "04_infer_token_embeddings", output_dir)
        
        print(f"🔍 Single token embeddings: shape={single_token_embeddings.shape}, dtype={single_token_embeddings.dtype}")
        print(f"🔍 Sample values: {single_token_embeddings[0, 0, :5].tolist()}")
        
        # Create masks for infer phase - CRITICAL: Fix position calculation
        pos = context_pos  # Position for next token (7 for our test case)
        update_mask = torch.zeros((1, 1, context_length, 1), dtype=torch.float32)  # Use float32
        update_mask[0, 0, pos, 0] = 1.0  # Update at position pos, not pos-1
        position_ids = torch.tensor([pos], dtype=torch.int32)  # Position pos, not pos-1
        single_causal_mask = causal_mask[:, :, pos:pos+1, :].float()  # Use float32 and correct slice
        
        save_tensor(update_mask, "04_infer_update_mask", output_dir)
        save_tensor(position_ids, "04_infer_position_ids", output_dir)
        save_tensor(single_causal_mask, "04_infer_causal_mask", output_dir)
        
        # Run through FFN chunks with state (infer mode)
        hidden_states = single_token_embeddings
        for chunk_idx, ffn_model in enumerate(ffn_models):
            if isinstance(ffn_model, dict):
                # CRITICAL: Use float32 precision throughout to avoid corruption
                inputs = {
                    'hidden_states': hidden_states.numpy().astype(np.float32),  # Use float32
                    'update_mask': update_mask.numpy().astype(np.float32),      # Use float32
                    'position_ids': position_ids.numpy().astype(np.int32),
                    'causal_mask': single_causal_mask.numpy().astype(np.float32),  # Use float32
                    'current_pos': position_ids.numpy().astype(np.int32)
                }
                
                # Debug: Print input values to verify they're reasonable
                if chunk_idx == 0:
                    print(f"🔍 Infer inputs for chunk {chunk_idx}:")
                    print(f"  hidden_states: shape={inputs['hidden_states'].shape}, sample={inputs['hidden_states'][0,0,:3]}")
                    print(f"  update_mask: shape={inputs['update_mask'].shape}, nonzero_count={np.count_nonzero(inputs['update_mask'])}")
                    print(f"  position_ids: {inputs['position_ids']}")
                    print(f"  causal_mask: shape={inputs['causal_mask'].shape}, sample={inputs['causal_mask'][0,0,0,:5]}")
                
                # Capture first chunk inputs
                if chunk_idx == 0:
                    save_tensor(inputs['hidden_states'], "04_infer_ffn_hidden_input", output_dir)
                
                output = ffn_model['infer'].predict(inputs, state)
                hidden_states = torch.from_numpy(output['output_hidden_states']).float()  # Ensure float32
                
                # Capture first chunk output
                if chunk_idx == 0:
                    save_tensor(hidden_states, "04_infer_ffn_output", output_dir)
                    print(f"🔍 Infer output for chunk {chunk_idx}: shape={hidden_states.shape}, sample={hidden_states[0,0,:5].tolist()}")
        
        print(f"✅ Infer phase complete, hidden states shape: {hidden_states.shape}")
        
        # Step 5: LM Head processing
        print(f"\n🧠 Step 5: LM Head")
        
        lm_input = hidden_states.numpy().astype(np.float16)
        save_tensor(lm_input, "05_lmhead_input", output_dir)
        
        lm_output = lmhead_model.predict({'hidden_states': lm_input})
        print(f"LM Head output keys: {list(lm_output.keys())}")
        
        # Save individual logits chunks
        num_logits = metadata.get('split_lm_head', 16)
        logits_parts = []
        
        for i in range(1, num_logits + 1):
            key = f'logits{i}'
            if key in lm_output:
                chunk = lm_output[key]
                logits_parts.append(torch.from_numpy(chunk))
                save_tensor(chunk, f"05_lmhead_logits{i}", output_dir)
        
        # Combine logits
        if logits_parts:
            combined_logits = torch.cat(logits_parts, dim=-1)
            save_tensor(combined_logits, "05_lmhead_combined_logits", output_dir)
            
            print(f"Combined logits shape: {combined_logits.shape}")
            
            # Step 6: Token prediction
            print(f"\n🎲 Step 6: Token Prediction")
            
            # Get the logits for the last position
            final_logits = combined_logits[0, -1, :]  # [vocab_size]
            save_tensor(final_logits, "06_final_logits", output_dir)
            
            # Get top-k predictions
            top_k = 10
            top_values, top_indices = torch.topk(final_logits, top_k)
            
            print(f"\nTop {top_k} predictions:")
            for i, (idx, score) in enumerate(zip(top_indices, top_values)):
                token_text = tokenizer.decode([idx.item()])
                print(f"  {i+1}. Token {idx.item()} ('{token_text}'): {score.item():.4f}")
            
            save_tensor(top_indices.numpy(), "06_top_indices", output_dir)
            save_tensor(top_values.numpy(), "06_top_values", output_dir)
            
            # Check if "dog" is the top prediction
            predicted_token = top_indices[0].item()
            predicted_text = tokenizer.decode([predicted_token])
            
            print(f"\n🏆 PREDICTION RESULT:")
            print(f"Top prediction: Token {predicted_token} = '{predicted_text}'")
            
            # Check if "dog" appears in top predictions
            dog_found = False
            dog_position = -1
            for i, idx in enumerate(top_indices):
                token_text = tokenizer.decode([idx.item()]).strip().lower()
                if "dog" in token_text:
                    dog_found = True
                    dog_position = i + 1
                    print(f"✅ 'dog' found at position {dog_position}: '{tokenizer.decode([idx.item()])}'")
                    break
            
            if not dog_found:
                print(f"❌ 'dog' not found in top {top_k} predictions")
                
            # Save prediction summary
            summary_path = output_dir / "prediction_summary.txt"
            with open(summary_path, 'w') as f:
                f.write(f"Prompt: {prompt}\n")
                f.write(f"Top prediction: {predicted_text}\n")
                f.write(f"Dog found: {dog_found}\n")
                f.write(f"Dog position: {dog_position}\n")
                f.write(f"\nTop {top_k} predictions:\n")
                for i, (idx, score) in enumerate(zip(top_indices, top_values)):
                    token_text = tokenizer.decode([idx.item()])
                    f.write(f"  {i+1}. Token {idx.item()} ('{token_text}'): {score.item():.4f}\n")
        
        print(f"\n✅ All tensors captured successfully!")
        print(f"📁 Output directory: {output_dir.absolute()}")
        
        return dog_found, dog_position
        
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        return False, -1

def main():
    parser = argparse.ArgumentParser(description='Capture tensors from Qwen pipeline')
    parser.add_argument('--meta', type=str, required=True, help='Path to meta.yaml file')
    parser.add_argument('--prompt', type=str, default="The quick brown fox jumps over the lazy",
                       help='Prompt to test (default: "The quick brown fox jumps over the lazy")')
    
    args = parser.parse_args()
    
    # Get model directory from meta.yaml path
    model_dir = Path(args.meta).parent
    
    print(f"🚀 Starting tensor capture for Qwen pipeline")
    print(f"📂 Model directory: {model_dir}")
    print(f"📝 Prompt: '{args.prompt}'")
    
    dog_found, dog_position = capture_tensors_from_pipeline(model_dir, args.prompt)
    
    if dog_found:
        print(f"\n🎉 SUCCESS: 'dog' found at position {dog_position}")
        return 0
    else:
        print(f"\n⚠️  WARNING: 'dog' not found in top predictions")
        return 1

if __name__ == "__main__":
    exit(main())