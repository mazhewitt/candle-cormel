//! Qwen model integration for candle-coreml
//!
//! This module provides a complete implementation of the Qwen multi-component architecture
//! with proper tokenization, state management, and inference pipeline.

pub mod config;
pub mod embeddings;
pub mod inference;
pub mod model;
pub mod naming;
pub mod tensors;
pub mod utilities;

// Re-export main components from their respective modules
pub use config::QwenConfig;
pub use model::QwenModel;
pub use naming::ModelNamingConfig;

// Re-export deprecated constants from config module for backward compatibility
#[allow(deprecated)]
pub use config::{QWEN_BATCH_SIZE, QWEN_CONTEXT_LENGTH, QWEN_HIDDEN_SIZE, QWEN_VOCAB_SIZE};
