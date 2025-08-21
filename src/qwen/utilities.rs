//! Utility functions and low-level pipeline methods for Qwen models
//!
//! This module contains helper functions, debugging utilities, and granular
//! pipeline methods that expose individual steps for testing and debugging.

use crate::qwen::model::QwenModel;
use crate::utils::multi_component;
use candle_core::{Error as CandleError, Tensor};
use std::collections::HashMap;
use tracing::{debug, info, trace};

impl QwenModel {
    /// Adapt current_pos for infer: if model expects [1], ensure scalar; if expects vector, expand.
    fn adapt_current_pos_for_infer(&self, current_pos: &Tensor) -> Result<Tensor, CandleError> {
        if let Some(infer_component) = self.config.model_config.components.get("ffn_infer") {
            if let Some(cur_cfg) = infer_component.inputs.get("current_pos") {
                if cur_cfg.shape.len() == 1 {
                    let expected_len = cur_cfg.shape[0];
                    if expected_len == 1 {
                        // Ensure scalar [1]
                        let val = if let Ok(v) = current_pos.to_vec1::<i64>() {
                            *v.get(0).unwrap_or(&0)
                        } else if let Ok(vf) = current_pos.to_vec1::<f32>() {
                            vf.get(0).copied().unwrap_or(0.0) as i64
                        } else {
                            0
                        };
                        return Tensor::from_vec(vec![val], (1,), &self.config.device);
                    } else if let Ok(actual_len) = current_pos.dim(0) {
                        if actual_len != expected_len {
                            // Expand or slice to expected length
                            let val = if let Ok(v) = current_pos.to_vec1::<i64>() {
                                *v.get(0).unwrap_or(&0)
                            } else if let Ok(vf) = current_pos.to_vec1::<f32>() {
                                vf.get(0).copied().unwrap_or(0.0) as i64
                            } else {
                                0
                            };
                            let data: Vec<i64> = vec![val; expected_len];
                            return Tensor::from_vec(data, (expected_len,), &self.config.device);
                        }
                    }
                }
            }
        }
        Ok(current_pos.clone())
    }
    /// Adapt hidden_states for infer phase (slice to last token if config expects seq_len=1).
    fn adapt_hidden_states_for_infer(&self, hidden_states: &Tensor) -> Result<Tensor, CandleError> {
        if let Some(infer_component) = self.config.model_config.components.get("ffn_infer") {
            if let Some(hs_cfg) = infer_component.inputs.get("hidden_states") {
                if hs_cfg.shape.len() == 3 && hs_cfg.shape[1] == 1 {
                    if let Ok(actual_seq) = hidden_states.dim(1) {
                        if actual_seq > 1 {
                            debug!(
                                "🔧 adapt_hidden_states_for_infer: slicing seq_len {} -> 1 (last token)",
                                actual_seq
                            );
                            return hidden_states.narrow(1, actual_seq - 1, 1).map_err(|e| {
                                CandleError::Msg(format!(
                                    "Failed to narrow hidden_states for infer: {e}"
                                ))
                            });
                        }
                    }
                }
            }
        }
        Ok(hidden_states.clone())
    }

    /// Adapt position_ids for infer: if config expects [1], reduce to scalar; else expand/truncate.
    fn adapt_position_ids_for_infer(&self, position_ids: &Tensor) -> Result<Tensor, CandleError> {
        if let Some(infer_component) = self.config.model_config.components.get("ffn_infer") {
            if let Some(pos_cfg) = infer_component.inputs.get("position_ids") {
                if pos_cfg.shape.len() == 1 {
                    let expected_len = pos_cfg.shape[0];
                    if expected_len == 1 {
                        let val = if let Ok(v) = position_ids.to_vec1::<i64>() {
                            *v.last().unwrap_or(&0)
                        } else if let Ok(vf) = position_ids.to_vec1::<f32>() {
                            vf.last().copied().unwrap_or(0.0) as i64
                        } else {
                            0
                        };
                        return Tensor::from_vec(vec![val], (1,), &self.config.device);
                    } else if let Ok(actual_len) = position_ids.dim(0) {
                        if actual_len != expected_len {
                            // Expand/truncate to expected length, copy last value
                            let val = if let Ok(v) = position_ids.to_vec1::<i64>() {
                                *v.last().unwrap_or(&0)
                            } else if let Ok(vf) = position_ids.to_vec1::<f32>() {
                                vf.last().copied().unwrap_or(0.0) as i64
                            } else {
                                0
                            };
                            let data: Vec<i64> = vec![val; expected_len];
                            return Tensor::from_vec(data, (expected_len,), &self.config.device);
                        }
                    }
                }
            }
        }
        Ok(position_ids.clone())
    }

    /// Combine LM head output parts into a single logits tensor (fixed 16-way split for now).
    pub fn combine_lm_head_outputs(
        &self,
        outputs: HashMap<String, Tensor>,
    ) -> Result<Tensor, CandleError> {
        // Use dynamic part count from configuration (fallback to 1 if unknown)
        let parts = self.config.model_config.logits_part_count();
        multi_component::combine_chunked_logits(outputs, parts)
    }

    /// Run FFN prefill phase with explicit inputs.
    pub fn run_ffn_prefill_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        position_ids: &Tensor,
        causal_mask: &Tensor,
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            self.initialize_states()?;
        }
        // Optionally adapt current_pos if prefill expects [1]
        let adjusted_current_pos = if let Some(prefill_comp) =
            self.config.model_config.components.get("ffn_prefill")
        {
            if let Some(cur_cfg) = prefill_comp.inputs.get("current_pos") {
                if cur_cfg.shape.len() == 1 && cur_cfg.shape[0] == 1 {
                    // Collapse to [1]
                    let val = if let Ok(v) = current_pos.to_vec1::<i64>() {
                        *v.get(0).unwrap_or(&0)
                    } else if let Ok(vf) = current_pos.to_vec1::<f32>() {
                        vf.get(0).copied().unwrap_or(0.0) as i64
                    } else {
                        0
                    };
                    Tensor::from_vec(vec![val], (1,), &self.config.device)?
                } else {
                    current_pos.clone()
                }
            } else {
                current_pos.clone()
            }
        } else {
            current_pos.clone()
        };

        // Debug inputs for prefill call
        let prefill_input_names = vec![
            "hidden_states".to_string(),
            "position_ids".to_string(),
            "causal_mask".to_string(),
            "current_pos".to_string(),
        ];
        let prefill_inputs: Vec<&Tensor> = vec![
            hidden_states,
            position_ids,
            causal_mask,
            &adjusted_current_pos,
        ];
        debug!("🧪 FFN_PREFILL: preparing {} inputs", prefill_inputs.len());
        for (idx, (name, t)) in prefill_input_names.iter().zip(prefill_inputs.iter()).enumerate()
        {
            let dims = t.dims();
            if dims.len() == 1 && dims[0] <= 8 {
                if let Ok(v) = t.to_vec1::<i64>() {
                    debug!("    [{}] {} shape={:?} vals={:?}", idx, name, dims, v);
                    continue;
                }
            }
            debug!("    [{}] {} shape={:?}", idx, name, dims);
        }

        // Strict validation: prefill shapes must match ModelConfig exactly
        if let Some(prefill_comp) = self.config.model_config.components.get("ffn_prefill") {
            let check = |name: &str, t: &Tensor| -> Result<(), CandleError> {
                if let Some(cfg) = prefill_comp.inputs.get(name) {
                    let t_dims = t.dims();
                    let expected = &cfg.shape;
                    if t_dims.len() != expected.len() {
                        return Err(CandleError::Msg(format!(
                            "ffn_prefill input '{}' rank mismatch: got {:?}, expected {:?}",
                            name, t_dims, expected
                        )));
                    }
                    for (i, (got, exp)) in t_dims.iter().zip(expected.iter()).enumerate() {
                        if got != exp {
                            return Err(CandleError::Msg(format!(
                                "ffn_prefill input '{}' dim{} mismatch: got {:?}, expected {:?}",
                                name, i, t_dims, expected
                            )));
                        }
                    }
                }
                Ok(())
            };
            check("hidden_states", hidden_states)?;
            check("position_ids", position_ids)?;
            check("causal_mask", causal_mask)?;
            check("current_pos", &adjusted_current_pos)?;
        }

        let inputs = [hidden_states, position_ids, causal_mask, &adjusted_current_pos];
        let state = self.unified_state.as_mut().unwrap();
        let out = self.ffn_prefill.predict_with_state(&inputs, state)?;
        Ok(out)
    }

    /// Run FFN infer phase with explicit inputs (supports optional update_mask if declared in config).
    pub fn run_ffn_infer_with_inputs(
        &mut self,
        hidden_states: &Tensor,
        position_ids: &Tensor,
        causal_mask: &Tensor,
        current_pos: &Tensor,
    ) -> Result<Tensor, CandleError> {
        if self.unified_state.is_none() {
            return Err(CandleError::Msg(
                "No unified state available - prefill must be run first".to_string(),
            ));
        }

    let adjusted_hidden_states = self.adapt_hidden_states_for_infer(hidden_states)?;
    let adjusted_position_ids = self.adapt_position_ids_for_infer(position_ids)?;
        // Debug adaptation (kept via tracing debug elsewhere)
    // Do not slice position_ids down; CoreML enumerated shapes often require full-length vectors (e.g., 128)

        let expects_update_mask = self
            .config
            .model_config
            .components
            .get("ffn_infer")
            .map(|c| c.inputs.contains_key("update_mask"))
            .unwrap_or(false);

        // Adapt causal mask if model expects singleton seq dimension
        let adapted_causal_mask =
            if let Some(infer_comp) = self.config.model_config.components.get("ffn_infer") {
                if let Some(cm_cfg) = infer_comp.inputs.get("causal_mask") {
                    if cm_cfg.shape.len() == 4 && cm_cfg.shape[2] == 1 {
                        if let Ok(actual) = causal_mask.dim(2) {
                            if actual > 1 {
                                debug!(
                                "🔧 adapt_causal_mask_for_infer: slicing causal_mask dim2 {} -> 1",
                                actual
                            );
                                match causal_mask.narrow(2, actual - 1, 1) {
                                    Ok(n) => n,
                                    Err(e) => {
                                        return Err(CandleError::Msg(format!(
                                            "Failed to narrow causal_mask for infer: {e}"
                                        )))
                                    }
                                }
                            } else {
                                causal_mask.clone()
                            }
                        } else {
                            causal_mask.clone()
                        }
                    } else {
                        causal_mask.clone()
                    }
                } else {
                    causal_mask.clone()
                }
            } else {
                causal_mask.clone()
            };

        // Prepare adjusted tensors before borrowing state mutably to avoid aliasing issues
        let adjusted_current_pos_full = self.adapt_current_pos_for_infer(current_pos)?;
    let state = self.unified_state.as_mut().unwrap();
    // Helper closure to debug-print the shapes about to be sent to CoreML
        let debug_log_inputs = |label: &str, ordered_names: &[String], tensors: &[&Tensor]| {
            // Use info! so logs show up even when only info-level logging is enabled in child processes
            info!("🧪 {label}: preparing {} inputs", tensors.len());
            for (idx, (name, t)) in ordered_names.iter().zip(tensors.iter()).enumerate() {
                let dims = t.dims();
                // For small 1-D tensors, also print values (handy for position_ids/current_pos)
                if dims.len() == 1 && dims[0] <= 8 {
                    if let Ok(v) = t.to_vec1::<i64>() {
                        info!("    [{}] {} shape={:?} vals={:?}", idx, name, dims, v);
                        continue;
                    }
                }
                info!("    [{}] {} shape={:?}", idx, name, dims);
            }
        };

        // Strict validation against ModelConfig: rank and each dim must match expected
        let validate_against_config = |component: &str,
                                       ordered_names: &[String],
                                       tensors: &[&Tensor]|
         -> Result<(), CandleError> {
            if let Some(comp) = self.config.model_config.components.get(component) {
                for (name, tensor) in ordered_names.iter().zip(tensors.iter()) {
                    if let Some(cfg) = comp.inputs.get(name) {
                        let t_dims = tensor.dims();
                        let expected = &cfg.shape;
                        if t_dims.len() != expected.len() {
                            return Err(CandleError::Msg(format!(
                                "{} input '{}' rank mismatch: got {:?}, expected {:?}",
                                component, name, t_dims, expected
                            )));
                        }
                        // Compare each dimension strictly
                        for (i, (got, exp)) in t_dims.iter().zip(expected.iter()).enumerate() {
                            if got != exp {
                                return Err(CandleError::Msg(format!(
                                    "{} input '{}' dim{} mismatch: got {:?}, expected {:?}",
                                    component, name, i, t_dims, expected
                                )));
                            }
                        }
                    }
                }
            }
            Ok(())
        };

    let output = if self
            .config
            .model_config
            .components
            .contains_key("ffn_infer")
        {
            // Separate infer component path
            if expects_update_mask {
                // Create one-hot update_mask over context length
                let context_length = self.config.context_length();
                let mut data = vec![0f32; context_length];
                let pos_idx = if let Ok(vals) = current_pos.to_vec1::<f32>() {
                    vals.first().cloned().unwrap_or(0.0) as usize
                } else {
                    0
                };
                let pos_idx = pos_idx.min(context_length.saturating_sub(1));
                data[pos_idx] = 1.0;
                let update_mask =
                    Tensor::from_vec(data, (1, 1, context_length, 1), &self.config.device)?;
                let ordered_names = self
                    .config
                    .model_config
                    .components
                    .get("ffn_infer")
                    .map(|c| {
                        c.input_order.clone().unwrap_or_else(|| {
                            vec![
                                "hidden_states".to_string(),
                                "position_ids".to_string(),
                                "update_mask".to_string(),
                                "causal_mask".to_string(),
                                "current_pos".to_string(),
                            ]
                        })
                    })
                    .unwrap();
                // Map tensors by name for reordering
                let mut by_name: std::collections::HashMap<&str, &Tensor> =
                    std::collections::HashMap::new();
                by_name.insert("hidden_states", &adjusted_hidden_states);
                by_name.insert("position_ids", &adjusted_position_ids);
                by_name.insert("update_mask", &update_mask);
                by_name.insert("causal_mask", &adapted_causal_mask);
                // Keep full-length current_pos to satisfy CoreML enumerated shapes
                by_name.insert("current_pos", &adjusted_current_pos_full);
                let ordered: Vec<&Tensor> = ordered_names
                    .iter()
                    .filter_map(|n| by_name.get(n.as_str()).copied())
                    .collect();
                debug_log_inputs("FFN_INFER(update_mask)", &ordered_names, &ordered);
                // Validate shapes vs config before calling CoreML (strict)
                validate_against_config("ffn_infer", &ordered_names, &ordered)?;
                info!("Infer: using separate ffn_infer with update_mask (reordered)");
                match self.ffn_infer.predict_with_state(&ordered, state) {
                    Ok(o) => o,
                    Err(e) => {
                        // Enrich error with input shapes and expected config
                        let expected: Vec<(String, Vec<usize>)> = self
                            .config
                            .model_config
                            .components
                            .get("ffn_infer")
                            .map(|c| {
                                ordered_names
                                    .iter()
                                    .filter_map(|n| c.inputs.get(n).map(|i| (n.clone(), i.shape.clone())))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        let actual: Vec<(String, Vec<usize>)> = ordered_names
                            .iter()
                            .zip(ordered.iter())
                            .map(|(n, t)| (n.clone(), t.dims().to_vec()))
                            .collect();
                        return Err(CandleError::Msg(format!(
                            "CoreML infer failed (update_mask). Inputs actual vs expected:\n  actual: {:?}\n  expected: {:?}\n  original error: {}",
                            actual, expected, e
                        )));
                    }
                }
            } else {
                let ordered_names = self
                    .config
                    .model_config
                    .components
                    .get("ffn_infer")
                    .map(|c| {
                        c.input_order.clone().unwrap_or_else(|| {
                            vec![
                                "hidden_states".to_string(),
                                "position_ids".to_string(),
                                "causal_mask".to_string(),
                                "current_pos".to_string(),
                            ]
                        })
                    })
                    .unwrap();
                let mut by_name: std::collections::HashMap<&str, &Tensor> =
                    std::collections::HashMap::new();
                by_name.insert("hidden_states", &adjusted_hidden_states);
                by_name.insert("position_ids", &adjusted_position_ids);
                by_name.insert("causal_mask", &adapted_causal_mask);
                by_name.insert("current_pos", &adjusted_current_pos_full);
                let ordered: Vec<&Tensor> = ordered_names
                    .iter()
                    .filter_map(|n| by_name.get(n.as_str()).copied())
                    .collect();
                debug_log_inputs("FFN_INFER", &ordered_names, &ordered);
                // Validate shapes vs config before calling CoreML (strict)
                validate_against_config("ffn_infer", &ordered_names, &ordered)?;
                info!("Infer: using separate ffn_infer (no update_mask, reordered)");
                match self.ffn_infer.predict_with_state(&ordered, state) {
                    Ok(o) => o,
                    Err(e) => {
                        // Enrich error with input shapes and expected config
                        let expected: Vec<(String, Vec<usize>)> = self
                            .config
                            .model_config
                            .components
                            .get("ffn_infer")
                            .map(|c| {
                                ordered_names
                                    .iter()
                                    .filter_map(|n| c.inputs.get(n).map(|i| (n.clone(), i.shape.clone())))
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default();
                        let actual: Vec<(String, Vec<usize>)> = ordered_names
                            .iter()
                            .zip(ordered.iter())
                            .map(|(n, t)| (n.clone(), t.dims().to_vec()))
                            .collect();
                        return Err(CandleError::Msg(format!(
                            "CoreML infer failed. Inputs actual vs expected:\n  actual: {:?}\n  expected: {:?}\n  original error: {}",
                            actual, expected, e
                        )));
                    }
                }
            }
        } else {
            // Unified component fallback
            let ordered_names = vec![
                "hidden_states".to_string(),
                "position_ids".to_string(),
                "causal_mask".to_string(),
                "current_pos".to_string(),
            ];
            let inputs = [
                &adjusted_hidden_states,
                &adjusted_position_ids,
                &adapted_causal_mask,
                current_pos,
            ];
            let ordered: Vec<&Tensor> = inputs.iter().copied().collect();
            debug_log_inputs("FFN_INFER(unified-prefill)", &ordered_names, &ordered);
            // Validate against prefill config in unified mode
            let _ = validate_against_config("ffn_prefill", &ordered_names, &ordered);
            info!("Infer: using prefill component for infer phase (unified mode)");
            match self.ffn_prefill.predict_with_state(&inputs, state) {
                Ok(o) => o,
                Err(e) => {
                    let expected: Vec<(String, Vec<usize>)> = self
                        .config
                        .model_config
                        .components
                        .get("ffn_prefill")
                        .map(|c| {
                            ordered_names
                                .iter()
                                .filter_map(|n| c.inputs.get(n).map(|i| (n.clone(), i.shape.clone())))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let actual: Vec<(String, Vec<usize>)> = ordered_names
                        .iter()
                        .zip(ordered.iter())
                        .map(|(n, t)| (n.clone(), t.dims().to_vec()))
                        .collect();
                    return Err(CandleError::Msg(format!(
                        "CoreML infer (unified) failed. Inputs actual vs expected:\n  actual: {:?}\n  expected: {:?}\n  original error: {}",
                        actual, expected, e
                    )));
                }
            }
        };

        trace!("INFER OUTPUT shape={:?}", output.shape());
        Ok(output)
    }

    /// Run LM head manually.
    pub fn run_lm_head_with_inputs(&self, hidden_states: &Tensor) -> Result<Tensor, CandleError> {
        let lm_outputs = self.lm_head.forward_all(&[hidden_states])?;
        self.combine_lm_head_outputs(lm_outputs)
    }

    /// Run embeddings manually.
    pub fn run_embeddings_with_inputs(&self, input_ids: &Tensor) -> Result<Tensor, CandleError> {
        self.embeddings.forward(&[input_ids])
    }
}
