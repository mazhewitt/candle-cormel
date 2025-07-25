//! Git2-based downloader for HuggingFace repositories
//!
//! This module provides git repository cloning using the git2 crate instead of
//! external git commands, making it more portable and eliminating the need for
//! git to be installed on the system.

use anyhow::{Error as E, Result};
use git2::{Repository, FetchOptions, RemoteCallbacks, Cred};
use std::path::{Path, PathBuf};
use std::fs;

/// Configuration for git2-based downloads
#[derive(Debug, Clone)]
pub struct Git2DownloadConfig {
    /// Repository URL
    pub repo_url: String,
    /// Target directory for clone
    pub target_dir: PathBuf,
    /// Whether to enable verbose output
    pub verbose: bool,
    /// Shallow clone depth (1 for fastest download)
    pub depth: Option<i32>,
}

impl Git2DownloadConfig {
    /// Create a new config for a HuggingFace model
    pub fn for_hf_model(model_id: &str, cache_base: &Path, verbose: bool) -> Self {
        let repo_url = format!("https://huggingface.co/{}", model_id);
        let model_cache_name = model_id.replace('/', "--");
        let target_dir = cache_base.join(format!("git2-{}", model_cache_name));

        Self {
            repo_url,
            target_dir,
            verbose,
            depth: Some(1), // Shallow clone for faster downloads
        }
    }
}

/// Clone a HuggingFace repository using git2
pub fn clone_hf_repository_git2(config: &Git2DownloadConfig) -> Result<PathBuf> {
    if config.verbose {
        println!("📥 Cloning repository with git2: {}", config.repo_url);
        println!("📁 Target directory: {}", config.target_dir.display());
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

    // Set up clone options
    let mut builder = git2::build::RepoBuilder::new();
    
    // Configure for shallow clone if specified
    if let Some(depth) = config.depth {
        let mut fetch_options = FetchOptions::new();
        fetch_options.depth(depth);
        builder.fetch_options(fetch_options);
    }

    // Note: Progress callbacks are not available in git2 RepoBuilder
    // For simplicity, we'll skip progress reporting for now

    // Perform the clone
    if config.verbose {
        println!("🔄 Starting git clone...");
    }

    let _repo = builder.clone(&config.repo_url, &config.target_dir)
        .map_err(|e| E::msg(format!("Git clone failed: {}", e)))?;

    if config.verbose {
        println!("✅ Git clone completed successfully");
        
        // Count files
        if let Ok(count) = count_files_in_directory(&config.target_dir) {
            println!("  📄 Files downloaded: {}", count);
        }
    }

    Ok(config.target_dir.clone())
}

/// Clone with authentication support (for private repos)
pub fn clone_hf_repository_with_auth(
    config: &Git2DownloadConfig,
    username: Option<&str>,
    token: Option<&str>,
) -> Result<PathBuf> {
    if config.verbose {
        println!("📥 Cloning repository with authentication: {}", config.repo_url);
    }

    // Remove existing directory if it exists
    if config.target_dir.exists() {
        fs::remove_dir_all(&config.target_dir)
            .map_err(|e| E::msg(format!("Failed to remove existing directory: {}", e)))?;
    }

    // Create parent directory
    if let Some(parent) = config.target_dir.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| E::msg(format!("Failed to create parent directory: {}", e)))?;
    }

    // Set up clone options with authentication
    let mut builder = git2::build::RepoBuilder::new();
    
    // Configure authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        if let (Some(user), Some(pass)) = (username, token) {
            Cred::userpass_plaintext(user, pass)
        } else if let Some(user) = username_from_url {
            // Try to use the username from URL with token as password
            if let Some(token) = token {
                Cred::userpass_plaintext(user, token)
            } else {
                Cred::default()
            }
        } else {
            Cred::default()
        }
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    if let Some(depth) = config.depth {
        fetch_options.depth(depth);
    }
    
    builder.fetch_options(fetch_options);

    // Perform the clone
    let repo = builder.clone(&config.repo_url, &config.target_dir)
        .map_err(|e| E::msg(format!("Authenticated git clone failed: {}", e)))?;

    if config.verbose {
        println!("✅ Authenticated git clone completed");
    }

    Ok(config.target_dir.clone())
}

/// Verify that a cloned repository contains the expected files
pub fn verify_cloned_repository(
    repo_dir: &Path,
    expected_components: &[String],
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("🔍 Verifying cloned repository: {}", repo_dir.display());
    }

    if !repo_dir.exists() {
        return Err(E::msg(format!(
            "Repository directory does not exist: {}",
            repo_dir.display()
        )));
    }

    // Check for .git directory
    let git_dir = repo_dir.join(".git");
    if !git_dir.exists() {
        return Err(E::msg("Directory is not a valid git repository"));
    }

    // Check for expected components
    for component in expected_components {
        let component_path = repo_dir.join(component);
        if !component_path.exists() {
            return Err(E::msg(format!(
                "Expected component not found: {}",
                component_path.display()
            )));
        }

        if verbose {
            println!("  ✅ Found component: {}", component);
        }

        // For .mlmodelc components, check for critical files
        if component.ends_with(".mlmodelc") {
            let critical_files = ["model.mil", "coremldata.bin"];
            for critical_file in &critical_files {
                let file_path = component_path.join(critical_file);
                if file_path.exists() {
                    if verbose {
                        println!("    ✅ Found critical file: {}", critical_file);
                    }
                } else {
                    if verbose {
                        println!("    ⚠️  Missing critical file: {}", critical_file);
                    }
                }
            }

            // Check weights directory
            let weights_dir = component_path.join("weights");
            if weights_dir.exists() {
                if verbose {
                    println!("    ✅ Found weights directory");
                }
            }
        }
    }

    if verbose {
        println!("✅ Repository verification completed");
    }

    Ok(())
}

/// Count files in a directory recursively
fn count_files_in_directory(dir: &Path) -> Result<usize> {
    let mut count = 0;
    
    fn visit_dir(dir: &Path, count: &mut usize) -> Result<()> {
        let entries = fs::read_dir(dir)
            .map_err(|e| E::msg(format!("Failed to read directory: {}", e)))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| E::msg(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip .git directory for counting
                if path.file_name() != Some(std::ffi::OsStr::new(".git")) {
                    visit_dir(&path, count)?;
                }
            } else {
                *count += 1;
            }
        }
        
        Ok(())
    }
    
    visit_dir(dir, &mut count)?;
    Ok(count)
}

/// Get repository information from a cloned directory
pub fn get_repository_info(repo_dir: &Path) -> Result<RepositoryInfo> {
    let repo = Repository::open(repo_dir)
        .map_err(|e| E::msg(format!("Failed to open repository: {}", e)))?;

    let head = repo.head()
        .map_err(|e| E::msg(format!("Failed to get HEAD: {}", e)))?;

    let branch_name = head.shorthand().unwrap_or("unknown").to_string();
    
    let commit = head.peel_to_commit()
        .map_err(|e| E::msg(format!("Failed to get commit: {}", e)))?;
    
    let commit_id = commit.id().to_string();
    let commit_message = commit.message().unwrap_or("").to_string();

    // Get remote URL
    let remote_url = repo.find_remote("origin")
        .and_then(|remote| remote.url().map(String::from).ok_or(git2::Error::from_str("No URL")))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(RepositoryInfo {
        branch: branch_name,
        commit_id,
        commit_message,
        remote_url,
    })
}

/// Repository information structure
#[derive(Debug, Clone)]
pub struct RepositoryInfo {
    pub branch: String,
    pub commit_id: String,
    pub commit_message: String,
    pub remote_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_git2_config_creation() {
        let temp_dir = env::temp_dir();
        let model_id = "test/model";
        let config = Git2DownloadConfig::for_hf_model(model_id, &temp_dir, true);
        
        assert_eq!(config.repo_url, "https://huggingface.co/test/model");
        assert!(config.target_dir.to_string_lossy().contains("git2-test--model"));
        assert_eq!(config.depth, Some(1));
        assert!(config.verbose);
    }

    #[test]
    fn test_count_files_empty_dir() {
        let temp_dir = env::temp_dir().join("test_empty");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let count = count_files_in_directory(&temp_dir).unwrap();
        assert_eq!(count, 0);
        
        fs::remove_dir_all(temp_dir).ok();
    }
}