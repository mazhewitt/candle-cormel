pub mod model;

pub use model::{CoreMLModel, CoreMLModelBuilder, Config};

use std::path::PathBuf;

/// Helper function to get a file locally first, then download from HuggingFace Hub if needed.
/// Follows the same pattern as quantized-t5 example.
pub fn get_local_or_remote_file(filename: &str, api: &hf_hub::api::sync::ApiRepo) -> anyhow::Result<PathBuf> {
    let local_filename = PathBuf::from(filename);
    if local_filename.exists() {
        Ok(local_filename)
    } else {
        Ok(api.get(filename)?)
    }
}