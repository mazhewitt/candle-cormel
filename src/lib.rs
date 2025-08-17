pub mod builder;
pub mod builtin_configs;
pub mod cache_manager;
pub mod clean_git_lfs_downloader;
pub mod config;
pub mod config_generator;
pub mod conversion;
#[cfg(test)]
pub mod infer_shape_test;
pub mod model;
pub mod model_config;
pub mod model_downloader;
pub mod pipeline;
pub mod qwen;
#[cfg(test)]
pub mod qwen_shapes_test;
pub mod state;
pub mod test_utils;
pub mod unified_model_loader;
pub mod utils;

pub use builder::CoreMLModelBuilder;
pub use builtin_configs::{get_builtin_config, list_builtin_models};
pub use cache_manager::CacheManager;
pub use config::Config;
pub use config_generator::ConfigGenerator;
pub use model::CoreMLModel;
pub use model_config::{ComponentConfig, ModelConfig, NamingConfig, ShapeConfig, TensorConfig};
pub use qwen::{ModelNamingConfig, QwenConfig, QwenModel};
pub use state::CoreMLState;
pub use unified_model_loader::{CachedModelInfo, UnifiedModelLoader};

// Main unified downloader API (recommended)
pub use model_downloader::{
    download_model, download_model_to, ensure_model_downloaded, get_cached_model_path,
};

// Advanced downloader API (for specific use cases)
pub use clean_git_lfs_downloader::{
    download_hf_model_clean, verify_download_completeness, CleanDownloadConfig,
};

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
