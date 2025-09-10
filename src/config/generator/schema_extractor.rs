//! Schema extraction utilities for CoreML manifest files
//!
//! Handles parsing of input/output tensor schemas from various CoreML manifest formats

use crate::config::model::TensorConfig;
use anyhow::{Error as E, Result};
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

/// Component role detected from tensor signatures
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentRole {
    Embeddings, // input_ids -> hidden_states
    FfnPrefill, // hidden_states + causal_mask -> output_hidden_states (no update_mask)
    FfnInfer,   // hidden_states + causal_mask + update_mask -> output_hidden_states
    FfnUnified, // hidden_states -> output_hidden_states (combined prefill/infer)
    LmHead,     // hidden_states -> logits (single or multiple chunks)
    Unknown,    // Unable to determine role from tensor signatures
}

pub struct SchemaExtractor;

impl Default for SchemaExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract input tensor configurations from manifest
    pub fn extract_inputs(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        let mut inputs = HashMap::new();

        // Handle both .mlpackage (Manifest.json) and .mlmodelc (metadata.json) formats
        if let Some(input_schema) = manifest
            .get(0)
            .and_then(|m| m.get("inputSchema").and_then(|s| s.as_array()))
        {
            debug!("📖 Parsing input schema with {} inputs", input_schema.len());

            // Try function-specific schemas first
            if let Some(function_inputs) = self.try_extract_function_inputs(manifest)? {
                return Ok(function_inputs);
            }

            // Parse top-level input schema
            inputs = self.parse_tensor_configs(input_schema)?;
        } else if manifest
            .get("itemInfoEntries")
            .and_then(|v| v.as_object())
            .is_some()
        {
            debug!("📖 Parsing .mlpackage manifest (limited). Using minimal fallback");
            // For generic models, we can't assume specific tensor names
        } else {
            debug!("📖 Unknown manifest format, no inputs extracted");
        }

        debug!("📖 Extracted {} input tensors", inputs.len());
        Ok(inputs)
    }

    /// Extract output tensor configurations from manifest  
    pub fn extract_outputs(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        let mut outputs = HashMap::new();

        if let Some(output_schema) = manifest
            .get(0)
            .and_then(|m| m.get("outputSchema").and_then(|s| s.as_array()))
        {
            debug!(
                "📖 Parsing output schema with {} outputs",
                output_schema.len()
            );

            outputs = self.parse_tensor_configs(output_schema)?;

            // Backfill empty shapes from function schemas if needed
            if self.has_empty_shapes(&outputs) {
                self.backfill_from_function_schemas(manifest, &mut outputs)?;
            }
        } else if let Some(functions) = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()))
        {
            debug!(
                "📖 Parsing outputs from functions schema ({} functions)",
                functions.len()
            );
            outputs = self.extract_outputs_from_functions(functions)?;
        } else {
            debug!("📖 Unknown manifest format, no outputs extracted");
        }

        debug!("📖 Extracted {} output tensors", outputs.len());
        Ok(outputs)
    }

    /// Parse tensor configurations from a schema array
    pub fn parse_tensor_configs(&self, schema: &[Value]) -> Result<HashMap<String, TensorConfig>> {
        let mut configs = HashMap::new();

        for tensor_def in schema {
            if let Some(tensor_config) = self.parse_tensor_definition(tensor_def)? {
                configs.insert(tensor_config.name.clone(), tensor_config);
            }
        }

        debug!("📖 Extracted {} tensor configs", configs.len());
        Ok(configs)
    }

    // Private helper methods

    fn try_extract_function_inputs(
        &self,
        manifest: &Value,
    ) -> Result<Option<HashMap<String, TensorConfig>>> {
        let functions = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()));

        if let Some(funcs) = functions {
            // Prefer prefill function shapes if available
            for prefer in ["prefill", "infer"] {
                for function in funcs {
                    if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                        if func_name == prefer {
                            if let Some(input_schema) =
                                function.get("inputSchema").and_then(|s| s.as_array())
                            {
                                debug!(
                                    "📖 Using {} function input schema with {} inputs",
                                    prefer,
                                    input_schema.len()
                                );
                                return Ok(Some(self.parse_tensor_configs(input_schema)?));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn has_empty_shapes(&self, outputs: &HashMap<String, TensorConfig>) -> bool {
        outputs.values().any(|tensor| tensor.shape.is_empty())
    }

    fn backfill_from_function_schemas(
        &self,
        manifest: &Value,
        outputs: &mut HashMap<String, TensorConfig>,
    ) -> Result<()> {
        let functions = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()));

        if let Some(funcs) = functions {
            for prefer in ["prefill", "infer"] {
                for function in funcs {
                    if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                        if func_name == prefer {
                            if let Some(func_output_schema) =
                                function.get("outputSchema").and_then(|s| s.as_array())
                            {
                                self.merge_function_outputs(func_output_schema, outputs)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn merge_function_outputs(
        &self,
        func_output_schema: &[Value],
        outputs: &mut HashMap<String, TensorConfig>,
    ) -> Result<()> {
        let function_outputs = self.parse_tensor_configs(func_output_schema)?;

        for (name, tensor_config) in function_outputs {
            let entry = outputs.entry(name.clone()).or_insert(TensorConfig {
                name: name.clone(),
                shape: vec![],
                data_type: tensor_config.data_type.clone(),
            });

            if entry.shape.is_empty() {
                entry.shape = tensor_config.shape;
                entry.data_type = tensor_config.data_type;
            }
        }

        Ok(())
    }

    fn extract_outputs_from_functions(
        &self,
        functions: &[Value],
    ) -> Result<HashMap<String, TensorConfig>> {
        let mut outputs = HashMap::new();

        for function in functions {
            if let Some(func_output_schema) =
                function.get("outputSchema").and_then(|s| s.as_array())
            {
                let function_outputs = self.parse_tensor_configs(func_output_schema)?;
                outputs.extend(function_outputs);
            }
        }

        Ok(outputs)
    }

    fn parse_tensor_definition(&self, tensor_def: &Value) -> Result<Option<TensorConfig>> {
        let (name, shape_str, data_type) = match (
            tensor_def.get("name").and_then(|n| n.as_str()),
            tensor_def.get("shape").and_then(|s| s.as_str()),
            tensor_def.get("dataType").and_then(|d| d.as_str()),
        ) {
            (Some(n), Some(s), Some(d)) => (n, s, d),
            _ => return Ok(None),
        };

        let shape = self.parse_shape_string(shape_str)?;
        debug!("   Tensor: {} -> {:?} ({})", name, shape, data_type);

        Ok(Some(TensorConfig {
            name: name.to_string(),
            shape,
            data_type: data_type.to_uppercase(),
        }))
    }

    #[allow(dead_code)]
    fn extract_tensor_shape(&self, input: &Value) -> Result<Vec<usize>> {
        // Handle enumeratedShapes when available - pick largest by default
        if let Some(enum_val) = input.get("enumeratedShapes") {
            if let Some(shapes) = self.parse_enumerated_shapes(enum_val)? {
                return Ok(shapes);
            }
        }

        // Fallback to single shape string
        if let Some(shape_str) = input.get("shape").and_then(|s| s.as_str()) {
            return self.parse_shape_string(shape_str);
        }

        Ok(vec![])
    }

    #[allow(dead_code)]
    fn parse_enumerated_shapes(&self, enum_val: &Value) -> Result<Option<Vec<usize>>> {
        if let Some(enum_str) = enum_val.as_str() {
            // enumeratedShapes is a JSON-like string, e.g. "[[1, 1], [1, 64]]"
            match serde_json::from_str::<Vec<Vec<usize>>>(enum_str) {
                Ok(mut shapes) => {
                    // Generic policy: pick the largest shape by total size
                    shapes.sort_by(|a, b| {
                        let a_size: usize = a.iter().product();
                        let b_size: usize = b.iter().product();
                        a_size.cmp(&b_size)
                    });
                    return Ok(shapes.last().cloned());
                }
                Err(err) => {
                    debug!("⚠️ Failed to parse enumeratedShapes: {}", err);
                }
            }
        } else if let Some(enum_arr) = enum_val.as_array() {
            let candidates = self.parse_enumerated_array(enum_arr)?;
            if !candidates.is_empty() {
                let mut sorted_candidates = candidates;
                sorted_candidates.sort_by(|a, b| {
                    let a_size: usize = a.iter().product();
                    let b_size: usize = b.iter().product();
                    a_size.cmp(&b_size)
                });
                return Ok(sorted_candidates.last().cloned());
            }
        }

        Ok(None)
    }

    #[allow(dead_code)]
    fn parse_enumerated_array(&self, enum_arr: &[Value]) -> Result<Vec<Vec<usize>>> {
        let mut candidates = Vec::new();

        for item in enum_arr {
            if let Some(s) = item.as_str() {
                if let Ok(v) = serde_json::from_str::<Vec<usize>>(s) {
                    candidates.push(v);
                }
            } else if let Some(arr) = item.as_array() {
                let mut v = Vec::new();
                for d in arr {
                    if let Some(u) = d.as_u64() {
                        v.push(u as usize);
                    }
                }
                if !v.is_empty() {
                    candidates.push(v);
                }
            }
        }

        Ok(candidates)
    }

    fn parse_shape_string(&self, shape_str: &str) -> Result<Vec<usize>> {
        let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
        if trimmed.is_empty() {
            return Ok(vec![]);
        }

        let mut dims = Vec::new();
        for dim_str in trimmed.split(',') {
            match dim_str.trim().parse::<usize>() {
                Ok(dim) => dims.push(dim),
                Err(_) => return Err(E::msg("Failed to parse tensor dimension")),
            }
        }

        Ok(dims)
    }

    /// Detect component role based on tensor signatures (metadata-driven approach)
    /// This replaces filename-based detection with pure metadata analysis
    pub fn detect_component_role(
        &self,
        inputs: &HashMap<String, TensorConfig>,
        outputs: &HashMap<String, TensorConfig>,
    ) -> ComponentRole {
        debug!("🔍 Analyzing tensor signatures for component role detection");
        debug!("   Inputs: {:?}", inputs.keys().collect::<Vec<_>>());
        debug!("   Outputs: {:?}", outputs.keys().collect::<Vec<_>>());

        // Special case: If no tensor information extracted, try filename-based fallback
        if inputs.is_empty() && outputs.is_empty() {
            debug!("⚠️ No tensor information available for component role detection");
            return ComponentRole::Unknown;
        }

        // 1. Check for embeddings component: input_ids -> hidden_states
        if inputs.contains_key("input_ids") && outputs.contains_key("hidden_states") {
            debug!("✅ Detected EMBEDDINGS component (input_ids -> hidden_states)");
            return ComponentRole::Embeddings;
        }

        // 2. Check for lm_head component: multiple logits outputs or single logits output
        if outputs.keys().any(|k| k.starts_with("logits")) {
            let logit_count = outputs.keys().filter(|k| k.starts_with("logits")).count();
            debug!(
                "✅ Detected LM_HEAD component ({} logits outputs)",
                logit_count
            );
            return ComponentRole::LmHead;
        }

        // 3. Differentiate between FFN prefill and infer based on input signatures
        if inputs.contains_key("hidden_states") && outputs.contains_key("output_hidden_states") {
            // Check for key differentiating tensors
            let has_causal_mask = inputs.contains_key("causal_mask");
            let has_update_mask = inputs.contains_key("update_mask");

            if has_update_mask && has_causal_mask {
                debug!("✅ Detected FFN_INFER component (has update_mask + causal_mask)");
                return ComponentRole::FfnInfer;
            } else if has_causal_mask && !has_update_mask {
                debug!("✅ Detected FFN component with causal_mask (prefill/infer ambiguous - needs filename fallback)");
                // Return Unknown so filename fallback can differentiate prefill vs infer
                return ComponentRole::Unknown;
            } else {
                debug!("✅ Detected FFN_UNIFIED component (no distinctive masks)");
                return ComponentRole::FfnUnified;
            }
        }

        debug!("❓ Could not determine component role from tensor signatures");
        ComponentRole::Unknown
    }

    /// Detect component role from filename as fallback when tensor signatures fail
    /// This handles cases where manifest parsing doesn't extract tensor info properly
    /// Follows ANEMLL naming convention: <https://github.com/Anemll/Anemll/blob/main/docs/compile_models.md#L61>
    pub fn detect_component_role_from_filename(&self, filename: &str) -> ComponentRole {
        let filename_lower = filename.to_lowercase();

        if filename_lower.contains("embedding") {
            debug!("🏷️ Filename-based detection: EMBEDDINGS ({})", filename);
            return ComponentRole::Embeddings;
        }

        if filename_lower.contains("lm_head") || filename_lower.contains("lmhead") {
            debug!("🏷️ Filename-based detection: LM_HEAD ({})", filename);
            return ComponentRole::LmHead;
        }

        // ANEMLL FFN patterns - order matters!

        // 1. Unified FFN pattern: prefix_FFN_PF_lut{N}_chunk_{X}of{Y}.mlpackage
        // This indicates a single model that handles both prefill and infer
        if filename_lower.contains("ffn_pf") {
            debug!(
                "🏷️ Filename-based detection: FFN_UNIFIED (ANEMLL FFN_PF pattern: {})",
                filename
            );
            return ComponentRole::FfnUnified;
        }

        // 2. Split FFN patterns (custom/typo-fixer models)
        // Pattern: "prefix_prefill_chunk_01of01.mlpackage" for prefill
        if filename_lower.contains("prefill") && !filename_lower.contains("ffn_pf") {
            debug!(
                "🏷️ Filename-based detection: FFN_PREFILL (split architecture: {})",
                filename
            );
            return ComponentRole::FfnPrefill;
        }

        // Pattern: "prefix_infer_chunk_01of01.mlpackage" or "prefix_FFN_chunk_01of01.mlpackage" for infer
        // Note: FFN without PF suffix indicates infer component in split architecture
        if filename_lower.contains("infer")
            || (filename_lower.contains("ffn_chunk")
                && !filename_lower.contains("ffn_pf")
                && !filename_lower.contains("prefill"))
        {
            debug!(
                "🏷️ Filename-based detection: FFN_INFER (split architecture: {})",
                filename
            );
            return ComponentRole::FfnInfer;
        }

        debug!("🏷️ Filename-based detection: UNKNOWN ({})", filename);
        ComponentRole::Unknown
    }

    /// Calculate total vocabulary size from multiple logits outputs
    /// This handles the chunked logits pattern in typo-fixer models (logits1...logits16)
    pub fn calculate_vocab_size_from_logits(
        &self,
        outputs: &HashMap<String, TensorConfig>,
    ) -> Option<usize> {
        let mut total_vocab_size = 0;
        let mut logits_found = false;

        // Check for single logits output first (standard case)
        if let Some(logits_tensor) = outputs.get("logits") {
            if let Some(&vocab_dim) = logits_tensor.shape.last() {
                debug!("📊 Single logits tensor found, vocab_size: {}", vocab_dim);
                return Some(vocab_dim);
            }
        }

        // Handle chunked logits (logits1, logits2, ... logitsN)
        for (name, tensor) in outputs {
            if name.starts_with("logits") && name != "logits" {
                if let Some(&chunk_size) = tensor.shape.last() {
                    total_vocab_size += chunk_size;
                    logits_found = true;
                    debug!("📊 Found logits chunk {}: size {}", name, chunk_size);
                }
            }
        }

        if logits_found {
            debug!(
                "📊 Total vocab size from {} chunks: {}",
                outputs
                    .keys()
                    .filter(|k| k.starts_with("logits") && *k != "logits")
                    .count(),
                total_vocab_size
            );
            Some(total_vocab_size)
        } else {
            None
        }
    }
}
