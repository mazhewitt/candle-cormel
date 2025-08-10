//! Utility functions and low-level pipeline methods for Qwen models
//!
//! This module contains helper functions, debugging utilities, and granular
//! pipeline methods that expose individual steps for testing and debugging.

use crate::qwen::model::QwenModel;
use crate::utils::multi_component;
use candle_core::{Error as CandleError, Tensor};
use std::collections::HashMap;
use tracing::{debug, trace};

impl QwenModel {
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
                            return hidden_states
                                .narrow(1, actual_seq - 1, 1)
                                .map_err(|e| CandleError::Msg(format!(
                                    "Failed to narrow hidden_states for infer: {e}"
                                )));
                        }
                    }
                }
            }
        }
        Ok(hidden_states.clone())
    }

    /// Adapt position_ids for infer (slice to last element if config expects length=1).
    fn adapt_position_ids_for_infer(&self, position_ids: &Tensor) -> Result<Tensor, CandleError> {
        if let Some(infer_component) = self.config.model_config.components.get("ffn_infer") {
            if let Some(pos_cfg) = infer_component.inputs.get("position_ids") {
                if pos_cfg.shape.len() == 1 && pos_cfg.shape[0] == 1 {
                    if let Ok(actual_len) = position_ids.dim(0) {
                        if actual_len > 1 {
                            debug!(
                                "🔧 adapt_position_ids_for_infer: slicing length {} -> 1 (last index)",
                                actual_len
                            );
                            return position_ids
                                .narrow(0, actual_len - 1, 1)
                                .map_err(|e| CandleError::Msg(format!(
                                    "Failed to narrow position_ids for infer: {e}"
                                )));
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
        let inputs = [hidden_states, position_ids, causal_mask, current_pos];
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
    println!("[DEBUG] infer inputs pre-adapt: hidden_states={:?} position_ids={:?}", hidden_states.dims(), position_ids.dims());
    println!("[DEBUG] infer inputs adapted: hidden_states={:?} position_ids={:?}", adjusted_hidden_states.dims(), adjusted_position_ids.dims());

        let expects_update_mask = self
            .config
            .model_config
            .components
            .get("ffn_infer")
            .map(|c| c.inputs.contains_key("update_mask"))
            .unwrap_or(false);

        // Adapt causal mask if model expects singleton seq dimension
        let adapted_causal_mask = if let Some(infer_comp) = self
            .config
            .model_config
            .components
            .get("ffn_infer")
        {
            if let Some(cm_cfg) = infer_comp.inputs.get("causal_mask") {
                if cm_cfg.shape.len() == 4 && cm_cfg.shape[2] == 1 {
                    if let Ok(actual) = causal_mask.dim(2) {
                        if actual > 1 {
                            debug!("🔧 adapt_causal_mask_for_infer: slicing causal_mask dim2 {} -> 1", actual);
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

        let state = self.unified_state.as_mut().unwrap();
        // Helper closure to debug-print the shapes about to be sent to CoreML
        let debug_log_inputs = |label: &str, ordered_names: &[String], tensors: &[&Tensor]| {
            debug!("🧪 {label}: preparing {} inputs", tensors.len());
            for (idx, (name, t)) in ordered_names.iter().zip(tensors.iter()).enumerate() {
                debug!("    [{}] {} shape={:?}", idx, name, t.dims());
            }
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
                    vals.get(0).cloned().unwrap_or(0.0) as usize
                } else {
                    0
                };
                let pos_idx = pos_idx.min(context_length.saturating_sub(1));
                data[pos_idx] = 1.0;
                let update_mask = Tensor::from_vec(
                    data,
                    (1, 1, context_length, 1),
                    &self.config.device,
                )?;
                let ordered_names = self
                    .config
                    .model_config
                    .components
                    .get("ffn_infer")
                    .map(|c| c.input_order.clone().unwrap_or_else(|| vec![
                        "hidden_states".to_string(),
                        "position_ids".to_string(),
                        "update_mask".to_string(),
                        "causal_mask".to_string(),
                        "current_pos".to_string(),
                    ]))
                    .unwrap();
                // Map tensors by name for reordering
                let mut by_name: std::collections::HashMap<&str, &Tensor> = std::collections::HashMap::new();
                by_name.insert("hidden_states", &adjusted_hidden_states);
                by_name.insert("position_ids", &adjusted_position_ids);
                by_name.insert("update_mask", &update_mask);
                by_name.insert("causal_mask", &adapted_causal_mask);
                by_name.insert("current_pos", current_pos);
                let ordered: Vec<&Tensor> = ordered_names
                    .iter()
                    .filter_map(|n| by_name.get(n.as_str()).copied())
                    .collect();
                debug_log_inputs("FFN_INFER(update_mask)", &ordered_names, &ordered);
                debug!("Infer: using separate ffn_infer with update_mask (reordered)");
                match self.ffn_infer.predict_with_state(&ordered, state) {
                    Ok(o) => o,
                    Err(e) => {
                        let msg = e.to_string();
                        if msg.contains("MultiArray shape (64) does not match the shape (1)") {
                            debug!("⚠️ Auto-retry: narrowing position_ids & causal_mask for infer (update_mask path)");
                            // Create narrowed tensors
                            let narrowed_pos = adjusted_position_ids.narrow(0, adjusted_position_ids.dim(0).unwrap_or(1)-1, 1)?;
                            let narrowed_mask = adapted_causal_mask.narrow(2, adapted_causal_mask.dim(2).unwrap_or(1)-1, 1)?;
                            let mut by_name_retry: std::collections::HashMap<&str, Tensor> = std::collections::HashMap::new();
                            by_name_retry.insert("hidden_states", adjusted_hidden_states.clone());
                            by_name_retry.insert("position_ids", narrowed_pos.clone());
                            by_name_retry.insert("update_mask", update_mask.clone());
                            by_name_retry.insert("causal_mask", narrowed_mask.clone());
                            by_name_retry.insert("current_pos", current_pos.clone());
                            let retry_inputs: Vec<Tensor> = ordered_names.iter().filter_map(|n| by_name_retry.get(n.as_str()).cloned()).collect();
                            let retry_refs: Vec<&Tensor> = retry_inputs.iter().collect();
                            debug_log_inputs("FFN_INFER(update_mask)-retry", &ordered_names, &retry_refs);
                            self.ffn_infer.predict_with_state(&retry_refs, state)?
                        } else { return Err(e); }
                    }
                }
            } else {
                let ordered_names = self
                    .config
                    .model_config
                    .components
                    .get("ffn_infer")
                    .map(|c| c.input_order.clone().unwrap_or_else(|| vec![
                        "hidden_states".to_string(),
                        "position_ids".to_string(),
                        "causal_mask".to_string(),
                        "current_pos".to_string(),
                    ]))
                    .unwrap();
                let mut by_name: std::collections::HashMap<&str, &Tensor> = std::collections::HashMap::new();
                by_name.insert("hidden_states", &adjusted_hidden_states);
                by_name.insert("position_ids", &adjusted_position_ids);
                by_name.insert("causal_mask", &adapted_causal_mask);
                by_name.insert("current_pos", current_pos);
                let ordered: Vec<&Tensor> = ordered_names
                    .iter()
                    .filter_map(|n| by_name.get(n.as_str()).copied())
                    .collect();
                debug_log_inputs("FFN_INFER", &ordered_names, &ordered);
                debug!("Infer: using separate ffn_infer (no update_mask, reordered)");
                match self.ffn_infer.predict_with_state(&ordered, state) {
                    Ok(o) => o,
                    Err(e) => {
                        let msg = e.to_string();
                        if msg.contains("MultiArray shape (64) does not match the shape (1)") {
                            debug!("⚠️ Auto-retry: narrowing position_ids & causal_mask for infer (no update_mask path)");
                            let narrowed_pos = adjusted_position_ids.narrow(0, adjusted_position_ids.dim(0).unwrap_or(1)-1, 1)?;
                            let narrowed_mask = adapted_causal_mask.narrow(2, adapted_causal_mask.dim(2).unwrap_or(1)-1, 1)?;
                            let mut by_name_retry: std::collections::HashMap<&str, Tensor> = std::collections::HashMap::new();
                            by_name_retry.insert("hidden_states", adjusted_hidden_states.clone());
                            by_name_retry.insert("position_ids", narrowed_pos.clone());
                            by_name_retry.insert("causal_mask", narrowed_mask.clone());
                            by_name_retry.insert("current_pos", current_pos.clone());
                            let retry_inputs: Vec<Tensor> = ordered_names.iter().filter_map(|n| by_name_retry.get(n.as_str()).cloned()).collect();
                            let retry_refs: Vec<&Tensor> = retry_inputs.iter().collect();
                            debug_log_inputs("FFN_INFER-retry", &ordered_names, &retry_refs);
                            self.ffn_infer.predict_with_state(&retry_refs, state)?
                        } else { return Err(e); }
                    }
                }
            }
        } else {
            // Unified component fallback
            let inputs = [
                &adjusted_hidden_states,
                &adjusted_position_ids,
                &adapted_causal_mask,
                current_pos,
            ];
            debug!("Infer: using prefill component for infer phase");
            debug!("Infer: using prefill component for infer phase");
            self.ffn_prefill.predict_with_state(&inputs, state)?
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
