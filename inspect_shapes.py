#!/usr/bin/env python3
"""
inspect_shapes.py
Quick and dirty inspector for Anemll multi-component Qwen 0.6B
(c) 2025 – public domain
"""

import coremltools as ct
from pathlib import Path
import sys

# Use the downloaded model directory
MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
COMPONENTS = {
    "embeddings": MODEL_DIR / "qwen_embeddings.mlmodelc",
    "ffn":        MODEL_DIR / "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc",
    "lm_head":    MODEL_DIR / "qwen_lm_head_lut6.mlmodelc",
}

def inspect(model_path: Path, name: str):
    if not model_path.exists():
        print(f"❌ {name} – file not found: {model_path}", file=sys.stderr)
        return

    try:
        m = ct.models.CompiledMLModel(str(model_path))
    except Exception as e:
        print(f"❌ {name} – failed to load: {e}", file=sys.stderr)
        return

    print(f"\n🔍  {name.upper()}  ({model_path.name})")
    
    # Try different ways to get model info
    try:
        print("   Model type:", type(m))
        if hasattr(m, 'input_description'):
            print("   Inputs:")
            for nm, spec in m.input_description.items():
                print(f"      {nm}: {spec}")
        elif hasattr(m, '_spec'):
            print("   Model spec available")
            if hasattr(m._spec, 'description'):
                print("   Description:", m._spec.description)
        else:
            print("   Available attributes:", [attr for attr in dir(m) if not attr.startswith('_')][:10])
            
        # Try to get input names
        print("   Trying to predict with dummy data to find input requirements...")
        
    except Exception as e:
        print(f"   Error inspecting: {e}")

def brute_length_test():
    """Try feeding the embeddings model every length 1..512 to see which are legal."""
    try:
        # Try to load tokenizer from the model directory
        emb_path = COMPONENTS["embeddings"]
        if not emb_path.exists():
            return
        
        # Simple test without transformers dependency
        model = ct.models.CompiledMLModel(str(emb_path))

        print("\n📏  Brute-force length check on embeddings model …")
        ok, fail = [], []
        
        # Test common lengths first
        test_lengths = [1, 2, 4, 5, 8, 10, 13, 16, 20, 24, 32, 48, 64, 96, 128, 192, 256, 384, 512]
        
        for L in test_lengths:
            # Create dummy input_ids of length L
            import numpy as np
            ids = np.array([list(range(L))], dtype=np.int32)  # Shape: (1, L)
            try:
                result = model.predict({"input_ids": ids})
                ok.append(L)
                print(f"✅  Length {L}: SUCCESS")
            except Exception as e:
                fail.append((L, str(e)))
                # Check if it's a shape error or other error
                error_msg = str(e)
                if "MultiArray Shape" in error_msg:
                    print(f"❌  Length {L}: SHAPE REJECTED - {error_msg[:100]}...")
                else:
                    print(f"❌  Length {L}: OTHER ERROR - {error_msg[:100]}...")
                
        print(f"\n✅  Accepted lengths: {ok}")
        print(f"❌  Rejected count: {len(fail)}")
        
    except Exception as e:
        print(f"Brute test failed: {e}")

if __name__ == "__main__":
    for name, path in COMPONENTS.items():
        inspect(path, name)
    brute_length_test()