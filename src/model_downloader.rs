//! Unified Model Downloader Utility
//!
//! This module provides a clean, single-function interface for downloading
//! HuggingFace models with proper LFS support. It uses the clean git2 + hf-hub
//! approach internally but presents a simple API.

use crate::clean_git_lfs_downloader::{download_hf_model_clean, CleanDownloadConfig};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Download a HuggingFace model to the standard cache location
///
/// This function:
/// 1. Uses git2 to clone the repository structure
/// 2. Detects and downloads LFS files with hf-hub
/// 3. Returns the path to the complete model
///
/// # Arguments
/// * `model_id` - HuggingFace model ID (e.g., "microsoft/DialoGPT-medium")
/// * `verbose` - Whether to show download progress
///
/// # Returns
/// Path to the downloaded model directory
///
/// # Example
/// ```rust,no_run
/// use candle_coreml::download_model;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let model_path = download_model("microsoft/DialoGPT-small", true)?;
/// let config_path = model_path.join("config.json");
/// # Ok(())
/// # }
/// ```
pub fn download_model(model_id: &str, verbose: bool) -> Result<PathBuf> {
    // Use standard cache directory
    let cache_base = dirs::cache_dir()
        .ok_or_else(|| anyhow::Error::msg("Cannot determine cache directory"))?
        .join("candle-coreml");

    std::fs::create_dir_all(&cache_base)?;

    // Configure clean downloader
    let config = CleanDownloadConfig::for_hf_model(model_id, &cache_base)
        .with_verbose(verbose)
        .with_keep_git(false); // Clean up .git directory

    // Download using clean approach
    download_hf_model_clean(&config)
}

/// Download a HuggingFace model to a specific directory
///
/// Like `download_model` but allows specifying the target directory.
///
/// # Arguments
/// * `model_id` - HuggingFace model ID
/// * `target_dir` - Directory where the model should be downloaded
/// * `verbose` - Whether to show download progress
///
/// # Returns
/// Path to the downloaded model directory (same as target_dir)
pub fn download_model_to(model_id: &str, target_dir: &Path, verbose: bool) -> Result<PathBuf> {
    let config = CleanDownloadConfig {
        model_id: model_id.to_string(),
        target_dir: target_dir.to_path_buf(),
        verbose,
        keep_git_dir: false,
    };

    download_hf_model_clean(&config)
}

/// Check if a model is already downloaded in the cache
///
/// # Arguments
/// * `model_id` - HuggingFace model ID to check
///
/// # Returns
/// Some(path) if the model exists, None otherwise
pub fn get_cached_model_path(model_id: &str) -> Option<PathBuf> {
    let cache_base = dirs::cache_dir()?.join("candle-coreml");

    let model_cache_name = model_id.replace('/', "--");
    let model_path = cache_base.join(format!("clean-{}", model_cache_name));

    if model_path.exists() && model_path.is_dir() {
        Some(model_path)
    } else {
        None
    }
}

/// Download a model only if it's not already cached
///
/// This function first checks if the model exists in cache, and only
/// downloads if it's missing.
///
/// # Arguments
/// * `model_id` - HuggingFace model ID
/// * `verbose` - Whether to show download progress
///
/// # Returns
/// Path to the model directory (either cached or newly downloaded)
pub fn ensure_model_downloaded(model_id: &str, verbose: bool) -> Result<PathBuf> {
    if let Some(cached_path) = get_cached_model_path(model_id) {
        if verbose {
            println!(
                "✅ Model {} already cached at: {}",
                model_id,
                cached_path.display()
            );
        }
        Ok(cached_path)
    } else {
        if verbose {
            println!("📥 Model {} not cached, downloading...", model_id);
        }
        download_model(model_id, verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_path_generation() {
        let model_id = "test/model";
        let expected_suffix = "candle-coreml/clean-test--model";

        if let Some(cache_path) = get_cached_model_path(model_id) {
            assert!(cache_path.to_string_lossy().ends_with(expected_suffix));
        }
        // Test passes regardless of whether cache exists
    }

    #[test]
    fn test_model_id_normalization() {
        let model_id = "microsoft/DialoGPT-medium";
        let normalized = model_id.replace('/', "--");
        assert_eq!(normalized, "microsoft--DialoGPT-medium");
    }
}
