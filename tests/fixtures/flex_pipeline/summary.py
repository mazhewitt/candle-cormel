#!/usr/bin/env python3
"""
Test Generation Suite Summary
Shows all generated test files and their contents for pipeline verification.
"""

import json
from pathlib import Path

def load_test_file(filepath):
    """Load and return test file data."""
    try:
        with open(filepath, 'r') as f:
            return json.load(f)
    except Exception as e:
        return {"error": str(e)}

def main():
    """Display summary of all test generation files."""
    print("=" * 80)
    print("🧪 TYPO FIXER TEST GENERATION SUITE SUMMARY")
    print("=" * 80)
    print()
    
    test_files = [
        "corrected_step_1_tokens.json",
        "corrected_step_2_causal_mask.json", 
        "corrected_step_3_prefill_input.json",
        "corrected_step_4_prefill_output.json",
        "corrected_step_5_infer_and_logits.json"
    ]
    
    test_dir = Path(__file__).parent  # Use directory of this script
    
    for i, filename in enumerate(test_files, 1):
        filepath = test_dir / filename
        print(f"📄 STEP {i}: {filename}")
        print("-" * 60)
        
        if filepath.exists():
            data = load_test_file(filepath)
            
            if "error" in data:
                print(f"❌ Error loading file: {data['error']}")
            else:
                # Show metadata
                metadata = data.get("metadata", {})
                print(f"   Step: {metadata.get('step', 'unknown')}")
                print(f"   Description: {metadata.get('description', 'no description')}")
                print(f"   Timestamp: {metadata.get('timestamp', 'unknown')}")
                
                # Show key data shapes/info
                data_section = data.get("data", {})
                
                if filename == "step_1_tokens.json":
                    print(f"   Input text: '{metadata.get('input_text', 'unknown')}'")
                    print(f"   Tokens shape: {data_section.get('tensor_shape', 'unknown')}")
                    print(f"   Context length: {data_section.get('context_pos', 'unknown')}")
                
                elif filename == "step_2_causal_mask.json":
                    print(f"   Causal mask shape: {data_section.get('causal_mask_shape', 'unknown')}")
                    print(f"   Context position: {data_section.get('context_pos', 'unknown')}")
                
                elif filename == "step_3_prefill_input.json":
                    print(f"   Hidden states shape: {data_section.get('hidden_states_shape', 'unknown')}")
                    print(f"   Batch size: {metadata.get('batch_size', 'unknown')}")
                    print(f"   Position IDs shape: {data_section.get('position_ids_shape', 'unknown')}")
                
                elif filename == "step_4_prefill_output.json":
                    print(f"   Prefill completed: {data_section.get('ready_for_infer', 'unknown')}")
                    print(f"   Next token pos: {data_section.get('next_token_pos', 'unknown')}")
                
                elif filename == "step_5_infer_and_logits.json":
                    print(f"   Current token: {data_section.get('current_token_text', 'unknown')}")
                    logits_info = data_section.get('final_logits', {})
                    print(f"   Logits shape: {logits_info.get('shape', 'unknown')}")
                    
                    # Show top prediction
                    top_preds = data_section.get('top_predictions', {})
                    if top_preds.get('tokens'):
                        top_token = top_preds['tokens'][0]
                        print(f"   Top prediction: '{top_token}'")
                
                print(f"   File size: {filepath.stat().st_size:,} bytes")
        else:
            print("❌ File not found")
        
        print()
    
    print("=" * 80)
    print("📊 PIPELINE VERIFICATION")
    print("=" * 80)
    
    # Verify the pipeline can be reconstructed
    all_files_exist = all((test_dir / f).exists() for f in test_files)
    
    # Assert that we get the same first token as typo_fixer_complete.py
    expected_first_token_id = 13  # Token '.' from original implementation
    expected_first_token_text = '.'
    
    step5_file = test_dir / "corrected_step_5_infer_and_logits.json"
    if step5_file.exists():
        step5_data = load_test_file(step5_file)
        if "error" not in step5_data:
            top_predictions = step5_data.get("data", {}).get("top_predictions", {})
            if top_predictions.get("indices"):
                actual_first_token_id = top_predictions["indices"][0]
                actual_first_token_text = top_predictions["tokens"][0]
                
                print(f"🔍 First Token Verification:")
                print(f"   Expected: Token {expected_first_token_id} ('{expected_first_token_text}')")
                print(f"   Actual:   Token {actual_first_token_id} ('{actual_first_token_text}')")
                
                try:
                    assert actual_first_token_id == expected_first_token_id, f"First token mismatch! Expected {expected_first_token_id}, got {actual_first_token_id}"
                    print("   ✅ First token matches typo_fixer_complete.py!")
                except AssertionError as e:
                    print(f"   ❌ {e}")
                    print("   ⚠️  Pipeline may have differences from original implementation")
            else:
                print("   ❌ No top predictions found in step 5 data")
        else:
            print("   ❌ Error loading step 5 data for verification")
    else:
        print("   ❌ Step 5 file not found for verification")
    print()
    
    if all_files_exist:
        print("✅ All test files generated successfully")
        print("✅ Complete pipeline data available for testing")
        print(f"✅ Test data covers: tokenization → causal mask → prefill → infer → logits")
        print()
        print("🔧 Usage:")
        print("   • Each step can be tested independently")
        print("   • Data includes tensor shapes, dtypes, and metadata")
        print("   • Files can be loaded to verify each pipeline component")
        print("   • Use this data to validate model implementations")
    else:
        missing_files = [f for f in test_files if not (test_dir / f).exists()]
        print(f"❌ Missing files: {missing_files}")
    
    print("=" * 80)

if __name__ == "__main__":
    main()