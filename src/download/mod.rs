//! Download module for candle-coreml
//!
//! This module provides all download-related functionality including:
//! - Unified model downloading from HuggingFace Hub
//! - Clean Git LFS support for large model files

pub mod git_lfs;
pub mod unified;

// Re-export main types for convenience
pub use git_lfs::{download_hf_model_clean, verify_download_completeness, CleanDownloadConfig};
pub use unified::{
    download_model, download_model_to, ensure_model_downloaded, get_cached_model_path,
};
