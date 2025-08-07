//! Utility functions and low-level pipeline methods for Qwen models
//!
//! This module contains helper functions, debugging utilities, and granular
//! pipeline methods that expose individual steps for testing and debugging.

use crate::qwen::model::QwenModel;
use crate::utils::multi_component;
use candle_core::{Error as CandleError, Tensor};
use std::collections::HashMap;
use tracing::{debug, trace, warn};

impl QwenModel {
    /// Combine 16 LM head output chunks into full vocabulary using shared utility
    pub fn combine_lm_head_outputs(
        &self,
        outputs: HashMap<String, Tensor>,
    ) -> Result<Tensor, CandleError> {
        multi_component::combine_chunked_logits(outputs, 16)
    }
    /// Run FFN prefill phase with exact inputs (for testing)
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
        let state = self.unified_state.as_mut().unwrap(); // Use the same unified state
        let output = self.ffn_prefill.predict_with_state(&inputs, state)?;

        Ok(output)
    }
    /// Run FFN infer phase with exact inputs (for testing)
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

        // CRITICAL FIX: Match Python reference implementation input order
        // Python infer inputs: hidden_states, update_mask, position_ids, causal_mask, current_pos
        // where current_pos should equal position_ids for proper state continuity
        trace!(
            "DEBUG: Infer inputs - position_ids: {:?}, current_pos: {:?}",
            position_ids.to_vec1::<f32>().unwrap_or_default(),
            current_pos.to_vec1::<f32>().unwrap_or_default()
        );

        // DEBUGGING: Validate all inputs before CoreML call
        trace!("INFER INPUT VALIDATION:");
        trace!(
            "  hidden_states: shape={:?}, sample={:?}",
            hidden_states.shape(),
            hidden_states.to_vec3::<f32>().unwrap_or_default()[0][0]
                [..3.min(hidden_states.dim(2).unwrap_or(0))]
                .to_vec()
        );

        trace!(
            "  position_ids: shape={:?}, values={:?}",
            position_ids.shape(),
            position_ids.to_vec1::<f32>().unwrap_or_default()
        );

        let causal_nonzeros = if let Ok(flat) = causal_mask.flatten_all() {
            if let Ok(vec) = flat.to_vec1::<f32>() {
                vec.iter().filter(|&&x| x != 0.0).count()
            } else {
                0
            }
        } else {
            0
        };
        trace!(
            "  causal_mask: shape={:?}, nonzeros={}",
            causal_mask.shape(),
            causal_nonzeros
        );

        let inputs = [hidden_states, position_ids, causal_mask, current_pos];
        let state = self.unified_state.as_mut().unwrap(); // Use the SAME unified state as prefill

        trace!("About to call CoreML infer model...");
        let output = if self
            .config
            .model_config
            .components
            .contains_key("ffn_infer")
        {
            // Typo-fixer style: Use separate FFN infer component
            debug!("Using separate FFN infer component");
            self.ffn_infer.predict_with_state(&inputs, state)?
        } else {
            // Standard ANEMLL style: Use FFN prefill component in infer mode
            debug!("Using unified FFN prefill component in infer mode");
            self.ffn_prefill.predict_with_state(&inputs, state)?
        };

        // DEBUGGING: Check output immediately after CoreML call
        let output_sample = output.to_vec3::<f32>().unwrap_or_default()[0][0]
            [..5.min(output.dim(2).unwrap_or(0))]
            .to_vec();
        trace!("INFER OUTPUT VALIDATION:");
        trace!(
            "  output: shape={:?}, sample={:?}",
            output.shape(),
            output_sample
        );

        if output_sample.iter().all(|&x| x == 0.0) {
            warn!("ZEROS DETECTED: CoreML infer model returned all zeros!");
        } else {
            trace!("NON-ZERO OUTPUT: CoreML infer model returned valid data");
        }

        Ok(output)
    }
    /// Run LM head with exact inputs (for testing)
    pub fn run_lm_head_with_inputs(&self, hidden_states: &Tensor) -> Result<Tensor, CandleError> {
        let lm_outputs = self.lm_head.forward_all(&[hidden_states])?;
        let combined_logits = self.combine_lm_head_outputs(lm_outputs)?;
        Ok(combined_logits)
    }

    /// Get direct access to embeddings model (for testing)
    pub fn run_embeddings_with_inputs(&self, input_ids: &Tensor) -> Result<Tensor, CandleError> {
        self.embeddings.forward(&[input_ids])
    }
}
