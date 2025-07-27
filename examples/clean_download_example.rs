//! Clean Git2 + HF Hub LFS Download Example
//!
//! This example demonstrates the clean approach to downloading HuggingFace models:
//! 1. Use git2 to clone the repository structure
//! 2. Detect LFS pointer files
//! 3. Use hf-hub to download actual LFS content
//! 4. Replace pointers with real files
//!
//! Usage: cargo run --example clean_download_example

use anyhow::Result;
use candle_coreml::clean_git_lfs_downloader::{
    download_hf_model_clean, verify_download_completeness, CleanDownloadConfig,
};
use std::path::PathBuf;

fn main() -> Result<()> {
    println!("🚀 Clean Git2 + HF Hub LFS Download Example");
    println!("==========================================");

    // Setup cache directory
    let cache_base = dirs::cache_dir()
        .ok_or_else(|| anyhow::Error::msg("Cannot determine cache directory"))?
        .join("candle-coreml-clean-examples");

    std::fs::create_dir_all(&cache_base)?;

    // Test with a small model that has LFS files
    let model_id = "microsoft/DialoGPT-small"; // Has LFS files like pytorch_model.bin

    println!("📦 Testing with model: {}", model_id);

    // Configure the clean downloader
    let config = CleanDownloadConfig::for_hf_model(model_id, &cache_base)
        .with_verbose(true)
        .with_keep_git(false); // Remove .git after download

    println!("\n🔄 Starting clean download process...");

    // Download using the clean approach
    let model_path = download_hf_model_clean(&config)?;

    println!("\n🔍 Verifying download completeness...");

    // Verify key files are present and not LFS pointers
    let expected_files = [
        "config.json",
        "vocab.json",
        "merges.txt",
        "pytorch_model.bin", // This should be an LFS file
    ];

    verify_download_completeness(&model_path, &expected_files, true)?;

    println!("\n✅ Clean download example completed successfully!");
    println!("📁 Model downloaded to: {}", model_path.display());

    // Show directory structure
    println!("\n📂 Directory structure:");
    show_directory_structure(&model_path, 0)?;

    Ok(())
}

/// Show directory structure recursively
fn show_directory_structure(dir: &PathBuf, depth: usize) -> Result<()> {
    if depth > 3 {
        // Limit depth to avoid too much output
        return Ok(());
    }

    let entries = std::fs::read_dir(dir)?;
    let mut entries: Vec<_> = entries.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap_or_default().to_string_lossy();

        // Skip .git directory
        if file_name == ".git" {
            continue;
        }

        let indent = "  ".repeat(depth);

        if path.is_dir() {
            println!("{}📁 {}/", indent, file_name);
            show_directory_structure(&path, depth + 1)?;
        } else {
            let metadata = std::fs::metadata(&path)?;
            let size = metadata.len();

            // Show file size in human-readable format
            let size_str = if size < 1024 {
                format!("{} B", size)
            } else if size < 1024 * 1024 {
                format!("{:.1} KB", size as f64 / 1024.0)
            } else if size < 1024 * 1024 * 1024 {
                format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
            } else {
                format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
            };

            println!("{}📄 {} ({})", indent, file_name, size_str);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_download_config() {
        let cache_base = std::env::temp_dir();
        let config =
            CleanDownloadConfig::for_hf_model("test/model", &cache_base).with_verbose(true);

        assert_eq!(config.model_id, "test/model");
        assert!(config.verbose);
        assert!(!config.keep_git_dir);
    }
}
