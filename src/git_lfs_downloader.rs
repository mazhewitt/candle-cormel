//! Git LFS-based downloader for HuggingFace models with complex directory structures
//!
//! Some HuggingFace models (especially .mlmodelc files) have complex nested structures
//! that don't download properly through the standard hf_hub API. This module provides
//! a fallback approach using git clone with LFS support.

use anyhow::{Error as E, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Download a HuggingFace model using git clone with LFS support
///
/// This is useful for models that have complex directory structures like .mlmodelc
/// files that don't download properly through the hf_hub API.
pub fn git_clone_hf_model(
    model_id: &str,
    target_dir: &Path,
    verbose: bool,
) -> Result<PathBuf> {
    let repo_url = format!("https://huggingface.co/{}", model_id);
    
    if verbose {
        println!("📥 Git cloning model from: {}", repo_url);
        println!("📁 Target directory: {}", target_dir.display());
    }

    // Create target directory if it doesn't exist
    if !target_dir.exists() {
        std::fs::create_dir_all(target_dir)
            .map_err(|e| E::msg(format!("Failed to create target directory: {}", e)))?;
    }

    // Run git clone with LFS support
    let mut cmd = Command::new("git");
    cmd.args([
        "clone",
        "--depth", "1",  // Shallow clone for faster download
        &repo_url,
        &target_dir.to_string_lossy(),
    ]);

    if verbose {
        println!("🔄 Running: {:?}", cmd);
    }

    let output = cmd.output()
        .map_err(|e| E::msg(format!("Failed to execute git clone: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(E::msg(format!(
            "Git clone failed: {}\n\nStderr: {}",
            output.status, stderr
        )));
    }

    if verbose {
        println!("✅ Git clone completed");
    }

    // Initialize LFS and pull LFS files
    let lfs_result = pull_lfs_files(target_dir, verbose);
    if let Err(e) = lfs_result {
        if verbose {
            println!("⚠️  LFS pull failed (continuing anyway): {}", e);
        }
    }

    Ok(target_dir.to_path_buf())
}

/// Pull LFS files for a git repository
fn pull_lfs_files(repo_dir: &Path, verbose: bool) -> Result<()> {
    if verbose {
        println!("🔄 Pulling LFS files...");
    }

    let mut cmd = Command::new("git");
    cmd.args(["lfs", "pull"])
        .current_dir(repo_dir);

    if verbose {
        println!("🔄 Running: {:?}", cmd);
    }

    let output = cmd.output()
        .map_err(|e| E::msg(format!("Failed to execute git lfs pull: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(E::msg(format!(
            "Git LFS pull failed: {}\n\nStderr: {}",
            output.status, stderr
        )));
    }

    if verbose {
        println!("✅ LFS files pulled successfully");
    }

    Ok(())
}

/// Verify that all required .mlmodelc components exist in the cloned repository
pub fn verify_mlmodelc_components(
    repo_dir: &Path,
    components: &[String],
    verbose: bool,
) -> Result<Vec<PathBuf>> {
    let mut component_paths = Vec::new();

    for component in components {
        let component_path = repo_dir.join(component);
        
        if !component_path.exists() {
            return Err(E::msg(format!(
                "Component directory not found: {}",
                component_path.display()
            )));
        }

        // Check for essential CoreML files
        let model_mil = component_path.join("model.mil");
        let coremldata_bin = component_path.join("coremldata.bin");
        let weights_dir = component_path.join("weights");

        let mut has_essential_files = false;
        
        if model_mil.exists() {
            has_essential_files = true;
            if verbose {
                println!("  ✅ Found model.mil: {}", model_mil.display());
            }
        }
        
        if coremldata_bin.exists() {
            has_essential_files = true;
            if verbose {
                println!("  ✅ Found coremldata.bin: {}", coremldata_bin.display());
            }
        }
        
        if weights_dir.exists() && weights_dir.is_dir() {
            has_essential_files = true;
            if verbose {
                println!("  ✅ Found weights directory: {}", weights_dir.display());
            }
        }

        if !has_essential_files {
            return Err(E::msg(format!(
                "Component {} is missing essential files (model.mil, coremldata.bin, or weights/)",
                component
            )));
        }

        component_paths.push(component_path);
    }

    Ok(component_paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_git_clone_url_generation() {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let expected_url = "https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let actual_url = format!("https://huggingface.co/{}", model_id);
        assert_eq!(actual_url, expected_url);
    }

    #[test]
    fn test_verify_components_empty() {
        let temp_dir = env::temp_dir().join("test_verify_empty");
        let components = vec![];
        let result = verify_mlmodelc_components(&temp_dir, &components, false);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}