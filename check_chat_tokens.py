#!/usr/bin/env python3
"""
check_chat_tokens.py
Check what tokens the chat template produces
"""

import json
from pathlib import Path

MODEL_DIR = Path("/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4")

def check_chat_tokens():
    # Load tokenizer
    tokenizer_path = MODEL_DIR / "tokenizer.json"
    with open(tokenizer_path) as f:
        data = json.load(f)
    
    vocab = data["model"]["vocab"]
    reverse_vocab = {v: k for k, v in vocab.items()}
    
    # Test the chat template used in Rust
    chat_prompt = "<|im_start|>user\nWhat is 2+2?<|im_end|>\n<|im_start|>assistant\n"
    
    print(f"🔍 Chat prompt: {repr(chat_prompt)}")
    
    # Manual tokenization to see what tokens we get
    # This is simplified - real tokenizer is more complex
    special_tokens = {
        "<|im_start|>": vocab.get("<|im_start|>", None),
        "<|im_end|>": vocab.get("<|im_end|>", None),
        "user": vocab.get("user", None),
        "assistant": vocab.get("assistant", None),
    }
    
    print(f"\n📝 Special tokens:")
    for token, id in special_tokens.items():
        if id is not None:
            print(f"   {token}: {id}")
        else:
            print(f"   {token}: NOT FOUND")
    
    # Check the tokens that were actually generated in Rust
    rust_tokens = [151644, 872, 198, 3838, 374, 220, 17, 10, 17, 30, 151645, 198, 151644, 77091, 198]
    
    print(f"\n📊 Rust tokenization result:")
    for i, token_id in enumerate(rust_tokens):
        token_text = reverse_vocab.get(token_id, f"<unk_{token_id}>")
        print(f"   {i}: {token_id} -> {repr(token_text)}")
    
    # Check if these tokens make sense
    full_text = "".join(reverse_vocab.get(token_id, f"<unk_{token_id}>") for token_id in rust_tokens)
    print(f"\n🔗 Reconstructed text: {repr(full_text)}")

if __name__ == "__main__":
    check_chat_tokens()