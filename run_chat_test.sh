#!/bin/bash
# Test script to run chat.py with the exact same model Rust is using

MODEL_DIR="/Users/mazhewitt/Library/Caches/candle-coreml/clean-anemll--anemll-Qwen-Qwen3-0.6B-LUT888-ctx512_0.3.4"
PROMPT="The quick brown fox jumps over the lazy"

echo "Testing chat.py with model directory: $MODEL_DIR"
echo "Prompt: $PROMPT"
echo "----------------------------------------"

# Activate virtual environment and run chat.py
source test_venv/bin/activate

# Run with eval mode, no template, no warmup, and just 1 token
python chat.py \
    --d "$MODEL_DIR" \
    --embed "qwen_embeddings.mlmodelc" \
    --ffn "qwen_FFN_PF_lut8_chunk_01of01.mlmodelc" \
    --lmhead "qwen_lm_head_lut8.mlmodelc" \
    --prompt "$PROMPT" \
    --max-tokens 1 \
    --no-template \
    --nw \
    --eval 2>&1 | tail -1

echo ""
echo "----------------------------------------"
echo "Done"