pub mod config;
pub mod conversion;
pub mod state;
pub mod builder;
pub mod model;
pub mod multi_component_downloader;
pub mod git_lfs_downloader;
pub mod robust_downloader;
pub mod api_recursive_downloader;
pub mod git2_downloader;

pub use config::Config;
pub use model::CoreMLModel;
pub use builder::CoreMLModelBuilder;
pub use state::CoreMLState;
pub use multi_component_downloader::{download_multi_component_model, MultiComponentConfig, get_model_component_paths};
pub use robust_downloader::{download_multi_component_model_robust, get_component_paths_from_repo};
pub use api_recursive_downloader::{download_model_with_api_structure, ApiDownloadConfig};
pub use git2_downloader::{clone_hf_repository_git2, Git2DownloadConfig};

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
