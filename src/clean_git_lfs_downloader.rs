//! Clean Git2 + HF Hub LFS Downloader
//!
//! This module provides a robust approach to downloading HuggingFace models:
//! 1. Use git2 to clone the repository (gets structure + LFS pointers)
//! 2. Detect LFS pointer files in the cloned repo
//! 3. Use hf-hub to download actual LFS file content
//! 4. Replace pointer files with real content
//!
//! This eliminates the need for external git tools while properly handling LFS files.

use anyhow::{Error as E, Result};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Configuration for the clean git+LFS downloader
#[derive(Debug, Clone)]
pub struct CleanDownloadConfig {
    /// HuggingFace model ID (e.g., "microsoft/DialoGPT-medium")
    pub model_id: String,
    /// Target directory for the complete download
    pub target_dir: PathBuf,
    /// Whether to enable verbose logging
    pub verbose: bool,
    /// Whether to keep the .git directory after download
    pub keep_git_dir: bool,
}

impl CleanDownloadConfig {
    /// Create config for downloading a HF model to cache
    pub fn for_hf_model(model_id: &str, cache_base: &Path) -> Self {
        let model_cache_name = model_id.replace('/', "--");
        let target_dir = cache_base.join(format!("clean-{}", model_cache_name));

        Self {
            model_id: model_id.to_string(),
            target_dir,
            verbose: false,
            keep_git_dir: false,
        }
    }

    /// Enable verbose logging
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Keep .git directory after download
    pub fn with_keep_git(mut self, keep_git: bool) -> Self {
        self.keep_git_dir = keep_git;
        self
    }
}

/// LFS pointer file information
#[derive(Debug, Clone)]
pub struct LfsPointer {
    pub file_path: PathBuf,
    pub version: String,
    pub oid: String,
    pub size: u64,
}

/// Download a HuggingFace model using the clean git2 + LFS approach
pub fn download_hf_model_clean(config: &CleanDownloadConfig) -> Result<PathBuf> {
    if config.verbose {
        println!("🚀 Starting clean git2+LFS download");
        println!("📦 Model: {}", config.model_id);
        println!("📁 Target: {}", config.target_dir.display());
    }

    // Step 1: Clone the git repository using git2
    let repo_path = clone_hf_repo_git2(config)?;

    // Step 2: Scan for LFS pointer files
    let lfs_pointers = scan_for_lfs_pointers(&repo_path, config.verbose)?;

    if config.verbose {
        println!("🔍 Found {} LFS pointer files", lfs_pointers.len());
    }

    // Step 3: Download actual LFS content using hf-hub
    download_lfs_content(&lfs_pointers, &config.model_id, config.verbose)?;

    // Step 4: Cleanup .git directory if requested
    if !config.keep_git_dir {
        let git_dir = repo_path.join(".git");
        if git_dir.exists() {
            if config.verbose {
                println!("🗑️  Removing .git directory");
            }
            fs::remove_dir_all(&git_dir)
                .map_err(|e| E::msg(format!("Failed to remove .git directory: {}", e)))?;
        }
    }

    if config.verbose {
        println!("✅ Clean download completed successfully");
    }

    Ok(repo_path)
}

/// Clone HuggingFace repository using git2
fn clone_hf_repo_git2(config: &CleanDownloadConfig) -> Result<PathBuf> {
    let repo_url = format!("https://huggingface.co/{}", config.model_id);

    if config.verbose {
        println!("📥 Cloning repository: {}", repo_url);
    }

    // Remove existing directory if it exists
    if config.target_dir.exists() {
        if config.verbose {
            println!("🗑️  Removing existing directory");
        }
        fs::remove_dir_all(&config.target_dir)
            .map_err(|e| E::msg(format!("Failed to remove existing directory: {}", e)))?;
    }

    // Create parent directory
    if let Some(parent) = config.target_dir.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| E::msg(format!("Failed to create parent directory: {}", e)))?;
    }

    // Clone with git2
    let mut builder = git2::build::RepoBuilder::new();

    // Use shallow clone for efficiency
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.depth(1);
    builder.fetch_options(fetch_options);

    let _repo = builder
        .clone(&repo_url, &config.target_dir)
        .map_err(|e| E::msg(format!("Git clone failed: {}", e)))?;

    if config.verbose {
        println!("✅ Repository cloned successfully");
    }

    Ok(config.target_dir.clone())
}

/// Scan the cloned repository for LFS pointer files
fn scan_for_lfs_pointers(repo_path: &Path, verbose: bool) -> Result<Vec<LfsPointer>> {
    if verbose {
        println!("🔍 Scanning for LFS pointer files...");
    }

    let mut lfs_pointers = Vec::new();
    scan_directory_for_lfs(repo_path, repo_path, &mut lfs_pointers, verbose)?;

    if verbose {
        println!("  Found {} LFS pointer files", lfs_pointers.len());
        for pointer in &lfs_pointers {
            println!(
                "    • {} (size: {} bytes)",
                pointer
                    .file_path
                    .strip_prefix(repo_path)
                    .unwrap_or(&pointer.file_path)
                    .display(),
                pointer.size
            );
        }
    }

    Ok(lfs_pointers)
}

/// Recursively scan a directory for LFS pointer files
fn scan_directory_for_lfs(
    dir: &Path,
    repo_root: &Path,
    lfs_pointers: &mut Vec<LfsPointer>,
    _verbose: bool,
) -> Result<()> {
    let entries = fs::read_dir(dir)
        .map_err(|e| E::msg(format!("Failed to read directory {}: {}", dir.display(), e)))?;

    for entry in entries {
        let entry = entry.map_err(|e| E::msg(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip .git directory
            if path.file_name() == Some(std::ffi::OsStr::new(".git")) {
                continue;
            }

            // Recursively scan subdirectories
            scan_directory_for_lfs(&path, repo_root, lfs_pointers, _verbose)?;
        } else if path.is_file() {
            // Check if this file is an LFS pointer
            if let Ok(pointer) = check_lfs_pointer_file(&path, repo_root) {
                lfs_pointers.push(pointer);
            }
        }
    }

    Ok(())
}

/// Check if a file is an LFS pointer file
fn check_lfs_pointer_file(file_path: &Path, _repo_root: &Path) -> Result<LfsPointer> {
    // Read first 1024 bytes (LFS pointers must be < 1024 bytes)
    let mut file = fs::File::open(file_path).map_err(|e| {
        E::msg(format!(
            "Failed to open file {}: {}",
            file_path.display(),
            e
        ))
    })?;

    let mut buffer = [0; 1024];
    let bytes_read = file.read(&mut buffer).map_err(|e| {
        E::msg(format!(
            "Failed to read file {}: {}",
            file_path.display(),
            e
        ))
    })?;

    let content = std::str::from_utf8(&buffer[..bytes_read])
        .map_err(|_| E::msg("File is not valid UTF-8"))?;

    // Parse LFS pointer format
    parse_lfs_pointer(content, file_path.to_path_buf())
}

/// Parse LFS pointer file content
fn parse_lfs_pointer(content: &str, file_path: PathBuf) -> Result<LfsPointer> {
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return Err(E::msg("Empty file"));
    }

    // First line must be version
    if !lines[0].starts_with("version https://git-lfs.github.com/spec/v") {
        return Err(E::msg("Not an LFS pointer file"));
    }

    let version = lines[0].to_string();
    let mut oid = String::new();
    let mut size = 0u64;

    // Parse remaining lines
    for line in &lines[1..] {
        if let Some(stripped) = line.strip_prefix("oid sha256:") {
            oid = stripped.to_string();
        } else if let Some(stripped) = line.strip_prefix("size ") {
            size = stripped
                .parse()
                .map_err(|e| E::msg(format!("Invalid size in LFS pointer: {}", e)))?;
        }
    }

    if oid.is_empty() || size == 0 {
        return Err(E::msg("Invalid LFS pointer: missing oid or size"));
    }

    Ok(LfsPointer {
        file_path,
        version,
        oid,
        size,
    })
}

/// Download actual LFS content using hf-hub and replace pointer files
fn download_lfs_content(lfs_pointers: &[LfsPointer], model_id: &str, verbose: bool) -> Result<()> {
    if lfs_pointers.is_empty() {
        if verbose {
            println!("📄 No LFS files to download");
        }
        return Ok(());
    }

    if verbose {
        println!(
            "📥 Downloading {} LFS files using hf-hub...",
            lfs_pointers.len()
        );
    }

    // Setup HuggingFace API
    let api = hf_hub::api::sync::Api::new()
        .map_err(|e| E::msg(format!("Failed to create HF API: {}", e)))?;
    let repo = api.model(model_id.to_string());

    for (i, pointer) in lfs_pointers.iter().enumerate() {
        if verbose {
            println!(
                "  📥 [{}/{}] Downloading: {}",
                i + 1,
                lfs_pointers.len(),
                pointer
                    .file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            );
        }

        // Get relative path from repo root for hf-hub API
        // We need to find the repo root by looking for .git directory
        let mut repo_root = pointer.file_path.parent();
        while let Some(parent) = repo_root {
            if parent.join(".git").exists() {
                repo_root = Some(parent);
                break;
            }
            repo_root = parent.parent();
        }

        let repo_root = repo_root.ok_or_else(|| E::msg("Cannot find repo root"))?;
        let relative_path = pointer
            .file_path
            .strip_prefix(repo_root)
            .map_err(|e| E::msg(format!("Failed to get relative path: {}", e)))?;

        let relative_path_str = relative_path.to_string_lossy();

        // Download the actual file content using hf-hub
        match repo.get(&relative_path_str) {
            Ok(downloaded_path) => {
                // Copy the downloaded content over the pointer file
                fs::copy(&downloaded_path, &pointer.file_path)
                    .map_err(|e| E::msg(format!("Failed to replace pointer file: {}", e)))?;

                if verbose {
                    println!(
                        "    ✅ Downloaded and replaced: {} ({} bytes)",
                        relative_path_str, pointer.size
                    );
                }
            }
            Err(e) => {
                if verbose {
                    println!("    ⚠️  Failed to download {}: {}", relative_path_str, e);
                }
                return Err(E::msg(format!(
                    "Failed to download LFS file {}: {}",
                    relative_path_str, e
                )));
            }
        }
    }

    if verbose {
        println!("✅ All LFS files downloaded successfully");
    }

    Ok(())
}

/// Verify that the downloaded model is complete
pub fn verify_download_completeness(
    model_path: &Path,
    expected_files: &[&str],
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("🔍 Verifying download completeness...");
    }

    for expected_file in expected_files {
        let file_path = model_path.join(expected_file);
        if !file_path.exists() {
            return Err(E::msg(format!(
                "Expected file not found: {}",
                expected_file
            )));
        }

        // Check that it's not still an LFS pointer
        if is_lfs_pointer_file(&file_path)? {
            return Err(E::msg(format!(
                "File {} is still an LFS pointer",
                expected_file
            )));
        }

        if verbose {
            let size = fs::metadata(&file_path)
                .map_err(|e| E::msg(format!("Failed to get file metadata: {}", e)))?
                .len();
            println!("  ✅ {} ({} bytes)", expected_file, size);
        }
    }

    if verbose {
        println!("✅ Download verification completed");
    }

    Ok(())
}

/// Check if a file is still an LFS pointer file
fn is_lfs_pointer_file(file_path: &Path) -> Result<bool> {
    match check_lfs_pointer_file(file_path, file_path.parent().unwrap_or(file_path)) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_lfs_pointer_parsing() {
        let content = "version https://git-lfs.github.com/spec/v1\noid sha256:abc123\nsize 12345\n";
        let result = parse_lfs_pointer(content, PathBuf::from("test.bin"));

        assert!(result.is_ok());
        let pointer = result.unwrap();
        assert_eq!(pointer.oid, "abc123");
        assert_eq!(pointer.size, 12345);
    }

    #[test]
    fn test_invalid_lfs_pointer() {
        let content = "This is not an LFS pointer file";
        let result = parse_lfs_pointer(content, PathBuf::from("test.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_config_creation() {
        let temp_dir = env::temp_dir();
        let config = CleanDownloadConfig::for_hf_model("test/model", &temp_dir);

        assert_eq!(config.model_id, "test/model");
        assert!(config
            .target_dir
            .to_string_lossy()
            .contains("clean-test--model"));
        assert!(!config.verbose);
        assert!(!config.keep_git_dir);
    }
}
