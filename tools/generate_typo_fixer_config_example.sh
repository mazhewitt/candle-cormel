#!/usr/bin/env bash
# Helper script to run shape discovery for a local typo-fixer model directory.
# Edit MODEL_DIR to point to your actual model path if different.
set -euo pipefail
MODEL_DIR="${1:-/Users/mazdahewitt/projects/train-typo-fixer/models/qwen-typo-fixer-ane}"
OUT="${2:-configs/typo-fixer-discovered.json}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT"
mkdir -p "$(dirname "$OUT")"
python3 tools/discover_shapes.py --model-dir "$MODEL_DIR" --output "$OUT" --verbose

echo "\nGenerated config at $OUT"
