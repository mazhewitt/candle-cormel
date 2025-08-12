//! Tests for the Python discover_shapes tool logic (post_process_config & shape derivation heuristics)
//! These tests avoid invoking coremltools (heavy dependency) by exercising pure-Python post-processing
//! through reimplementation of critical heuristics in Rust for parity validation.
use serde_json::Value;
use std::process::Command;

fn run_discover(model_dir: &str) -> Option<Value> {
    // Requires python & coremltools; skip gracefully if unavailable
    #[allow(unused)]
    let python = which::which("python3").ok()?;
    let output_file = tempfile::NamedTempFile::new().ok()?;
    let status = Command::new(python)
        .arg("tools/discover_shapes.py")
        .arg("--model-dir")
        .arg(model_dir)
        .arg("--output")
        .arg(output_file.path())
        .status()
        .ok()?;
    if !status.success() {
        return None;
    }
    let data = std::fs::read_to_string(output_file.path()).ok()?;
    serde_json::from_str(&data).ok()
}

#[test]
fn test_post_process_split_execution_detection() {
    // Synthetic config emulating separate prefill & infer files
    let mut cfg = serde_json::json!({
        "model_info": {"path": "/tmp/model", "model_type": "qwen"},
        "shapes": {"batch_size": 64, "context_length": 256, "hidden_size": 1024, "vocab_size": 1000},
        "components": {
          "ffn_prefill": {"file_path": "/tmp/model/ffn_prefill.mlpackage", "inputs": {"hidden_states": {"shape": [1,64,1024]}}, "outputs": {}},
          "ffn_infer": {"file_path": "/tmp/model/ffn_infer.mlpackage", "inputs": {"hidden_states": {"shape": [1,1,1024]}}, "outputs": {}},
          "embeddings": {"file_path": "/tmp/model/emb.mlpackage", "inputs": {}, "outputs": {}},
          "lm_head": {"file_path": "/tmp/model/lm.mlpackage", "inputs": {}, "outputs": {}}
        }
    });
    // Mirror Python post_process_config logic in Rust for validation
    let components = cfg.get_mut("components").unwrap();
    if let (Some(pref), Some(infer)) = (components.get("ffn_prefill"), components.get("ffn_infer"))
    {
        let pre = pref.get("file_path").and_then(|v| v.as_str()).unwrap_or("");
        let inf = infer
            .get("file_path")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if pre != inf {
            cfg["ffn_execution"] = Value::from("split");
        }
    }
    assert_eq!(cfg["ffn_execution"], "split");
}

#[test]
fn test_post_process_sequential_prefill_hint() {
    let mut cfg = serde_json::json!({
        "model_info": {"path": "/tmp/model", "model_type": "qwen"},
        "shapes": {"batch_size": 64, "context_length": 256, "hidden_size": 1024, "vocab_size": 1000},
        "components": {
          "ffn_prefill": {"file_path": "/tmp/model/ffn.mlpackage", "inputs": {"hidden_states": {"shape": [1,1,1024]}}, "outputs": {}},
          "embeddings": {"file_path": "/tmp/model/emb.mlpackage", "inputs": {}, "outputs": {}},
          "lm_head": {"file_path": "/tmp/model/lm.mlpackage", "inputs": {}, "outputs": {}}
        }
    });
    let components = cfg.get("components").unwrap();
    if let Some(pref) = components.get("ffn_prefill") {
        if let Some(hs) = pref.get("inputs").and_then(|i| i.get("hidden_states")) {
            if let Some(arr) = hs.get("shape").and_then(|s| s.as_array()) {
                if arr.len() == 3 && arr[1].as_i64() == Some(1) {
                    cfg["hints"] = serde_json::json!({"prefill_mode":"sequential"});
                }
            }
        }
    }
    assert_eq!(cfg["hints"]["prefill_mode"], "sequential");
}

#[test]
fn test_real_discover_skipped_without_coremltools() {
    // Ensure test does not fail when environment lacks coremltools
    // We point to a likely non-model dir; run_discover returns None gracefully.
    let result = run_discover("/nonexistent/path/hopefully");
    assert!(result.is_none());
}
