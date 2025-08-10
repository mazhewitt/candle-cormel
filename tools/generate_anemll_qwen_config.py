#!/usr/bin/env python3
"""Unified configuration generator for ANEMLL Qwen CoreML models.

Goals:
  * Accept a model directory containing .mlpackage / .mlmodelc parts.
  * Auto-detect embeddings, FFN (prefill & infer), LM head (chunked logits) using discover_shapes logic.
  * Produce a single candle-coreml ModelConfig JSON (shapes + components + naming=NULL) suitable for direct loading.
  * Handle typo-fixer style (separate prefill/infer) and unified FFN models.
  * Warn about anomalies (position_ids mismatch, missing functions, multipart logits).

Usage:
  python3 tools/generate_anemll_qwen_config.py --model-dir <path> --output configs/<name>.json

Optional:
  --model-id some/id (embed model_id into model_info)
  --force-overwrite (allow overwriting existing output file)
"""
import argparse
import json
import sys
from pathlib import Path
from datetime import datetime
from typing import Any, Dict, Optional

# Reuse core logic by importing discover_shapes if available, else fail gracefully
try:
    from discover_shapes import ModelShapeDiscovery  # type: ignore
except Exception:
    print("Error: Could not import discover_shapes.ModelShapeDiscovery. Ensure tools/discover_shapes.py is present.", file=sys.stderr)
    sys.exit(1)

def build_model_config(raw: Dict[str, Any], model_id: Optional[str]) -> Dict[str, Any]:
    # Prune absolute file paths to just basenames for portability
    components: Dict[str, Any] = {}
    for name, comp in raw.get("components", {}).items():
        file_path = comp.get("file_path") or comp.get("path") or comp.get("file")
        if file_path:
            rel = Path(file_path).name
        else:
            rel = None
        components[name] = {
            "file_path": rel,
            "inputs": comp.get("inputs", {}),
            "outputs": comp.get("outputs", {}),
            "functions": comp.get("functions", []),
        }

    config = {
        "model_info": {
            "model_id": model_id,
            "path": str(raw.get("model_info", {}).get("path", "")),
            "model_type": raw.get("model_info", {}).get("model_type", "qwen"),
            "discovered_at": datetime.now().isoformat(),
        },
        "shapes": raw.get("shapes", {}),
        "components": components,
        "naming": {  # explicit patterns removed
            "embeddings_pattern": None,
            "ffn_prefill_pattern": None,
            "ffn_infer_pattern": None,
            "lm_head_pattern": None,
        },
    }
    return config

def validate_minimum(config: Dict[str, Any]) -> None:
    missing = []
    for required in ("embeddings", "lm_head"):
        if required not in config["components"]:
            missing.append(required)
    if missing:
        print(f"⚠️  Warning: missing required components: {missing}", file=sys.stderr)

    # Basic wiring hints
    emb_out = config["components"].get("embeddings", {}).get("outputs", {}).get("hidden_states")
    ffn_in = config["components"].get("ffn_prefill", {}).get("inputs", {}).get("hidden_states")
    if emb_out and ffn_in and emb_out.get("shape") != ffn_in.get("shape"):
        print("⚠️  Warning: embeddings.hidden_states != ffn_prefill.hidden_states", file=sys.stderr)

    # Infer path shape check
    if "ffn_infer" in config["components"]:
        infer_in = config["components"]["ffn_infer"].get("inputs", {}).get("hidden_states")
        infer_out = config["components"]["ffn_infer"].get("outputs", {}).get("output_hidden_states")
        lm_in = config["components"].get("lm_head", {}).get("inputs", {}).get("hidden_states")
        if infer_out and lm_in and infer_out.get("shape") != lm_in.get("shape"):
            print("⚠️  Warning: ffn_infer.output_hidden_states != lm_head.hidden_states", file=sys.stderr)
        if infer_in and infer_in.get("shape", [None, None])[1] == 1:
            print("ℹ️  Detected single-token infer hidden state shape.")


def main():
    ap = argparse.ArgumentParser(description="Generate candle-coreml ModelConfig for ANEMLL Qwen model")
    ap.add_argument("--model-dir", required=True, type=Path)
    ap.add_argument("--output", required=True, type=Path)
    ap.add_argument("--model-id", required=False)
    ap.add_argument("--force-overwrite", action="store_true")
    ap.add_argument("--verbose", action="store_true")
    args = ap.parse_args()

    if args.output.exists() and not args.force_overwrite:
        print(f"Error: {args.output} exists (use --force-overwrite to overwrite)", file=sys.stderr)
        sys.exit(1)

    discovery = ModelShapeDiscovery(verbose=args.verbose)
    raw = discovery.discover_model_shapes(args.model_dir)

    config = build_model_config(raw, args.model_id)
    validate_minimum(config)

    args.output.parent.mkdir(parents=True, exist_ok=True)
    with open(args.output, "w") as f:
        json.dump(config, f, indent=2)

    print(f"✅ Generated ModelConfig: {args.output}")
    print("Summary:")
    print(f"  model_id: {config['model_info'].get('model_id')}")
    s = config['shapes']
    print(f"  shapes: batch_size={s.get('batch_size')} context_length={s.get('context_length')} hidden_size={s.get('hidden_size')} vocab_size={s.get('vocab_size')}")
    print(f"  components: {', '.join(config['components'].keys())}")

if __name__ == "__main__":
    main()
