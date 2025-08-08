//! Generic CoreML pipeline abstraction (model-agnostic)
//!
//! This module provides a simple, model-agnostic pipeline that loads
//! CoreML components from a ModelConfig and supports prefill/infer/head
//! execution with a shared CoreMLState.

use crate::{CoreMLModel, CoreMLState, ModelConfig};
use anyhow::Result;

/// A minimal execution contract for multi-component CoreML models.
pub struct CoreMLPipeline {
    pub embeddings: Option<CoreMLModel>,
    pub ffn_prefill: Option<CoreMLModel>,
    pub ffn_infer: Option<CoreMLModel>,
    pub lm_head: Option<CoreMLModel>,
    pub state: Option<CoreMLState>,
    pub config: ModelConfig,
}

impl CoreMLPipeline {
    pub fn new(config: ModelConfig) -> Self {
        Self {
            embeddings: None,
            ffn_prefill: None,
            ffn_infer: None,
            lm_head: None,
            state: None,
            config,
        }
    }

    /// Provide a placeholder for future loading by paths if needed; currently, Qwen handles loading.
    pub fn with_loaded_components(
        mut self,
        embeddings: Option<CoreMLModel>,
        ffn_prefill: Option<CoreMLModel>,
        ffn_infer: Option<CoreMLModel>,
        lm_head: Option<CoreMLModel>,
    ) -> Self {
        self.embeddings = embeddings;
        self.ffn_prefill = ffn_prefill;
        self.ffn_infer = ffn_infer;
        self.lm_head = lm_head;
        self
    }

    pub fn init_state(&mut self, model_for_state: &CoreMLModel) -> Result<()> {
        if self.state.is_none() {
            self.state = Some(model_for_state.make_state()?);
        }
        Ok(())
    }
}
