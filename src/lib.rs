pub mod builder;
pub mod cache;
pub mod config;
pub mod conversion;
pub mod download;
pub mod model;
pub mod pipeline;
pub mod qwen;
pub mod state;
pub mod unified_model_loader;
pub mod utils;

// Legacy module re-exports for backward compatibility
pub mod cache_manager {
    pub use crate::cache::*;
}
pub mod clean_git_lfs_downloader {
    pub use crate::download::git_lfs::*;
}
pub mod config_generator {
    pub use crate::config::generator::*;
}
pub mod model_config {
    pub use crate::config::model::*;
}
pub mod model_downloader {
    pub use crate::download::unified::*;
}

pub use builder::CoreMLModelBuilder;
pub use cache::CacheManager;
pub use config::{
    ComponentConfig, Config, ConfigGenerator, ModelConfig, NamingConfig, ShapeConfig, TensorConfig,
};
pub use model::CoreMLModel;
pub use qwen::{ModelNamingConfig, QwenConfig, QwenModel};
pub use state::CoreMLState;
pub use unified_model_loader::{CachedModelInfo, UnifiedModelLoader};

// Main unified downloader API (recommended)
pub use download::{
    download_model, download_model_to, ensure_model_downloaded, get_cached_model_path,
};

// Advanced downloader API (for specific use cases)
pub use download::{download_hf_model_clean, verify_download_completeness, CleanDownloadConfig};

// Shared utilities for transformer models
pub use utils::{mask, multi_component, sampling};

use std::path::PathBuf;

/// Helper function to get a file locally first, then download from `HuggingFace` Hub if needed.
/// Follows the same pattern as quantized-t5 example.
pub fn get_local_or_remote_file(
    filename: &str,
    api: &hf_hub::api::sync::ApiRepo,
) -> anyhow::Result<PathBuf> {
    let local_filename = PathBuf::from(filename);
    if local_filename.exists() {
        Ok(local_filename)
    } else {
        Ok(api.get(filename)?)
    }
}
