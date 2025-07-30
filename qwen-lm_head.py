#!/usr/bin/env python3
"""
qwen-lm_head.py - Test LM head in isolation

This script loads the FFN output tensor from Phase 1 and feeds it through 
the LM head in isolation to prove we get "dog" as the top prediction.

Usage: python3 qwen-lm_head.py --meta meta.yaml
"""

import argparse
import numpy as np
import torch
from pathlib import Path

# Import from chat.py
from chat import load_models, initialize_tokenizer

def test_lm_head_isolation(model_dir, test_tensors_dir="test_tensors"):
    """Test LM head in isolation using saved FFN output."""
    
    print(f"🧠 Testing LM Head in isolation")
    
    test_tensors_dir = Path(test_tensors_dir)
    
    # Load the saved FFN output (this is the input to LM head)
    lm_input_path = test_tensors_dir / "05_lmhead_input.npy"
    if not lm_input_path.exists():
        print(f"❌ Error: {lm_input_path} not found. Run qwen-chat-test.py first.")
        return False
    
    lm_input = np.load(lm_input_path)
    print(f"📥 Loaded LM head input: shape {lm_input.shape}, dtype {lm_input.dtype}")
    
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
    
    try:
        # Load only the LM head model (we don't need embed/ffn for this test)
        print(f"📂 Loading LM head model from: {args.lmhead}")
        
        from chat import load_model, parse_model_path
        lmhead_path = parse_model_path(args.lmhead)
        print(f"📁 Resolved LM head path: {lmhead_path}")
        lmhead_model = load_model(lmhead_path)
        
        # Load tokenizer for token decoding
        tokenizer = initialize_tokenizer(args.tokenizer, eval_mode=True)
        
        print(f"✅ LM head model loaded successfully")
        
        # Run LM head on the saved input
        print(f"🔄 Running LM head inference...")
        lm_output = lmhead_model.predict({'hidden_states': lm_input})
        
        print(f"📤 LM head output keys: {list(lm_output.keys())}")
        
        # Combine logits (same logic as in qwen-chat-test.py)
        num_logits = args.split_lm_head
        logits_parts = []
        
        for i in range(1, num_logits + 1):
            key = f'logits{i}'
            if key in lm_output:
                chunk = lm_output[key]
                logits_parts.append(torch.from_numpy(chunk))
                print(f"  {key}: shape {chunk.shape}")
        
        if not logits_parts:
            print(f"❌ Error: No logits chunks found in output")
            return False
        
        # Combine logits
        combined_logits = torch.cat(logits_parts, dim=-1)
        print(f"📊 Combined logits shape: {combined_logits.shape}")
        
        # Get the logits for the last position
        final_logits = combined_logits[0, -1, :]  # [vocab_size]
        print(f"📈 Final logits shape: {final_logits.shape}")
        
        # Get top-k predictions
        top_k = 10
        top_values, top_indices = torch.topk(final_logits, top_k)
        
        print(f"\n🏆 Top {top_k} predictions from isolated LM head:")
        dog_found = False
        dog_position = -1
        
        for i, (idx, score) in enumerate(zip(top_indices, top_values)):
            token_text = tokenizer.decode([idx.item()])
            print(f"  {i+1}. Token {idx.item()} ('{token_text}'): {score.item():.4f}")
            
            # Check if this is "dog"
            if token_text.strip().lower() == "dog":
                dog_found = True
                dog_position = i + 1
        
        # Check the top prediction specifically
        predicted_token = top_indices[0].item()
        predicted_text = tokenizer.decode([predicted_token])
        
        print(f"\n🎯 ISOLATED LM HEAD RESULT:")
        print(f"Top prediction: Token {predicted_token} = '{predicted_text}'")
        
        if predicted_text.strip().lower() == "dog":
            print(f"✅ SUCCESS: LM head correctly predicts 'dog' as top token!")
            return True
        elif dog_found:
            print(f"⚠️  WARNING: 'dog' found at position {dog_position}, but not top prediction")
            return False
        else:
            print(f"❌ FAILURE: 'dog' not found in top {top_k} predictions")
            return False
        
        # Load reference logits to compare
        reference_logits_path = test_tensors_dir / "05_lmhead_combined_logits.npy"
        if reference_logits_path.exists():
            reference_logits = np.load(reference_logits_path)
            reference_final = torch.from_numpy(reference_logits[0, -1, :])
            
            # Compare with reference
            diff = torch.abs(final_logits - reference_final)
            max_diff = diff.max().item()
            mean_diff = diff.mean().item()
            
            print(f"\n📊 Comparison with reference:")
            print(f"Max difference: {max_diff:.6f}")
            print(f"Mean difference: {mean_diff:.6f}")
            
            if max_diff < 1e-3:
                print(f"✅ Logits match reference (max diff: {max_diff:.6f})")
            else:
                print(f"⚠️  Logits differ from reference (max diff: {max_diff:.6f})")
        
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc()
        return False

def main():
    parser = argparse.ArgumentParser(description='Test Qwen LM head in isolation')
    parser.add_argument('--meta', type=str, required=True, help='Path to meta.yaml file')
    parser.add_argument('--test-tensors', type=str, default="test_tensors",
                       help='Directory containing test tensors (default: test_tensors)')
    
    args = parser.parse_args()
    
    # Get model directory from meta.yaml path
    model_dir = Path(args.meta).parent
    
    print(f"🚀 Starting LM head isolation test")
    print(f"📂 Model directory: {model_dir}")
    print(f"📁 Test tensors directory: {args.test_tensors}")
    
    success = test_lm_head_isolation(model_dir, args.test_tensors)
    
    if success:
        print(f"\n🎉 LM HEAD ISOLATION TEST PASSED!")
        return 0
    else:
        print(f"\n❌ LM HEAD ISOLATION TEST FAILED!")
        return 1

if __name__ == "__main__":
    exit(main())