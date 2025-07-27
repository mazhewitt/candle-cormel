#!/usr/bin/env python3
import coremltools as ct
import os

cache_dir = "/Users/mazdahewitt/projects/candle-coreml/qwen-model"

# Check what Python is actually using
print("🔍 Python chat.py model inspection:")
print("==================================")

# Check the exact files
files_to_check = [
    "qwen_embeddings.mlmodelc",
    "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc", 
    "qwen_lm_head_lut8.mlmodelc"
]

for file in files_to_check:
    full_path = os.path.join(cache_dir, file)
    if os.path.exists(full_path):
        print(f"\n📁 {file}")
        try:
            model = ct.models.CompiledMLModel(full_path)
            desc = model.get_spec().description
            
            # Check functions
            if hasattr(desc, 'functions') and desc.functions:
                print(f"   Functions: {list(desc.functions.keys())}")
            else:
                print("   Functions: None")
            
            # Check inputs
            input_info = []
            for i in desc.input:
                type_name = i.type.WhichOneof("type")
                input_info.append(f"{i.name}: {type_name}")
            print(f"   Inputs: {input_info}")
            
            # Check outputs  
            output_info = []
            for o in desc.output:
                type_name = o.type.WhichOneof("type")
                output_info.append(f"{o.name}: {type_name}")
            print(f"   Outputs: {output_info}")
            
            # Check if it has functions
            if hasattr(desc, 'functions') and desc.functions:
                for func_name, func in desc.functions.items():
                    print(f"   Function '{func_name}':")
                    func_inputs = [i.name for i in func.input]
                    func_outputs = [o.name for o in func.output]
                    print(f"     Inputs: {func_inputs}")
                    print(f"     Outputs: {func_outputs}")
                    
        except Exception as e:
            print(f"   Error: {e}")

print("\n" + "="*50)
print("🧪 Testing function loading:")

# Test function loading specifically
ffn_path = os.path.join(cache_dir, "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc")
if os.path.exists(ffn_path):
    print(f"\n📋 Testing FFN model functions:")
    try:
        prefill_model = ct.models.CompiledMLModel(ffn_path, function_name='prefill')
        print("   ✅ Prefill function loaded successfully")
    except Exception as e:
        print(f"   ❌ Prefill function failed: {e}")
        
    try:
        infer_model = ct.models.CompiledMLModel(ffn_path, function_name='infer')
        print("   ✅ Infer function loaded successfully")
    except Exception as e:
        print(f"   ❌ Infer function failed: {e}")
        
    try:
        default_model = ct.models.CompiledMLModel(ffn_path)
        print("   ✅ Default function loaded successfully")
    except Exception as e:
        print(f"   ❌ Default function failed: {e}")

print("\n" + "="*50)
print("🎯 Simple Inference Test:")

# Test a simple forward pass
try:
    import numpy as np
    
    # Load models like Python chat.py does
    embed_model = ct.models.CompiledMLModel(os.path.join(cache_dir, "qwen_embeddings.mlmodelc"))
    ffn_prefill = ct.models.CompiledMLModel(ffn_path, function_name='prefill')
    ffn_infer = ct.models.CompiledMLModel(ffn_path, function_name='infer')
    lmhead_model = ct.models.CompiledMLModel(os.path.join(cache_dir, "qwen_lm_head_lut8.mlmodelc"))
    
    print("\n📊 Testing with simple input:")
    
    # Test with different token values
    print("\n🔍 Testing different token values:")
    
    # Test 1: All 1s (current test)
    test_input_batch = np.ones((1, 64), dtype=np.int32)  
    print(f"Test 1 - All 1s: {test_input_batch.flatten()[:5]}")
    embed_output_1s = embed_model.predict({'input_ids': test_input_batch})
    hidden_states_1s = embed_output_1s['hidden_states']
    print(f"  Result: {hidden_states_1s.flatten()[:3]}")
    
    # Test 2: Token 0 (often padding)
    test_input_zeros = np.zeros((1, 64), dtype=np.int32)
    print(f"Test 2 - All 0s: {test_input_zeros.flatten()[:5]}")
    embed_output_0s = embed_model.predict({'input_ids': test_input_zeros})
    hidden_states_0s = embed_output_0s['hidden_states']
    print(f"  Result: {hidden_states_0s.flatten()[:3]}")
    
    # Test 3: Common tokens like 2, 3, 4
    test_input_seq = np.array([[2, 3, 4, 5, 6] + [1]*59], dtype=np.int32)
    print(f"Test 3 - Sequence: {test_input_seq.flatten()[:5]}")
    embed_output_seq = embed_model.predict({'input_ids': test_input_seq})
    hidden_states_seq = embed_output_seq['hidden_states']
    print(f"  Result: {hidden_states_seq.flatten()[:3]}")
    
    # Use the sequence version for the rest of the test
    test_input_batch = test_input_seq
    hidden_states = hidden_states_seq
    print(f"Embeddings shape: {hidden_states.shape}, dtype: {hidden_states.dtype}")
    print(f"Embeddings sample: {hidden_states.flatten()[:5]}")
    
    # Test FFN prefill - exactly match Python chat.py
    hidden_states_fp16 = hidden_states.astype(np.float16)
    position_ids = np.arange(0, 64, dtype=np.int32)  # Python: torch.arange(batch_pos, batch_pos+64)
    causal_mask = np.zeros((1, 1, 64, 512), dtype=np.float16)  # Python: [1, 1, 64, context_length]
    current_pos = np.array([0], dtype=np.int32)  # Python: [batch_pos]
    
    prefill_inputs = {
        'hidden_states': hidden_states_fp16,
        'position_ids': position_ids,
        'causal_mask': causal_mask,
        'current_pos': current_pos
    }
    
    # Create state for stateful processing
    state = ffn_prefill.make_state()
    
    ffn_output = ffn_prefill.predict(prefill_inputs, state)
    processed_hidden = ffn_output['output_hidden_states']
    print(f"FFN prefill shape: {processed_hidden.shape}, dtype: {processed_hidden.dtype}")
    print(f"FFN prefill sample: {processed_hidden.flatten()[:5]}")
    
    # Test LM head
    lm_output = lmhead_model.predict({'hidden_states': processed_hidden})
    print(f"LM head outputs: {list(lm_output.keys())}")
    
    if 'logits1' in lm_output:
        logits1 = lm_output['logits1']
        print(f"Logits1 shape: {logits1.shape}, dtype: {logits1.dtype}")
        print(f"Logits1 sample: {logits1.flatten()[:5]}")
        print(f"Logits1 max: {np.max(logits1)}, min: {np.min(logits1)}")
        
        # Check if we have meaningful values
        if np.max(logits1) > 1e-6 or np.min(logits1) < -1e-6:
            print("✅ Python produces meaningful logits!")
        else:
            print("❌ Python also produces zero logits")
    
except Exception as e:
    print(f"❌ Simple inference test failed: {e}")
    import traceback
    traceback.print_exc()