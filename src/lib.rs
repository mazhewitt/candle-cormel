pub mod builder;
pub mod clean_git_lfs_downloader;
pub mod config;
pub mod conversion;
pub mod model;
pub mod model_downloader;
pub mod qwen;
pub mod state;

pub use builder::CoreMLModelBuilder;
pub use config::Config;
pub use model::CoreMLModel;
pub use qwen::{QwenConfig, QwenModel};
pub use state::CoreMLState;

// Main unified downloader API (recommended)
pub use model_downloader::{
    download_model, download_model_to, ensure_model_downloaded, get_cached_model_path,
};

// Advanced downloader API (for specific use cases)
pub use clean_git_lfs_downloader::{
    download_hf_model_clean, verify_download_completeness, CleanDownloadConfig,
};

use std::path::PathBuf;

/// Helper function to get a file locally first, then download from HuggingFace Hub if needed.
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
