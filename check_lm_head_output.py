#!/usr/bin/env python3
"""
check_lm_head_output.py
Check the actual output name for LM head model
"""

import coremltools as ct
from pathlib import Path
import numpy as np

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")
LM_HEAD_PATH = MODEL_DIR / "qwen_lm_head_lut6.mlmodelc"

def check_lm_head():
    try:
        lm_head_model = ct.models.CompiledMLModel(str(LM_HEAD_PATH))
        print(f"🔍 LM Head Model: {LM_HEAD_PATH.name}")
        
        # Test with correct input shape
        hidden_states = np.random.randn(1, 1, 1024).astype(np.float32)
        
        print(f"Input shape: {hidden_states.shape}")
        
        result = lm_head_model.predict({"hidden_states": hidden_states})
        print(f"✅ LM Head outputs: {list(result.keys())}")
        
        for k, v in result.items():
            print(f"   {k}: shape {v.shape}, dtype {v.dtype}")
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    check_lm_head()