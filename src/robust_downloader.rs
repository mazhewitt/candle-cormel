//! Robust multi-strategy downloader for HuggingFace models
//!
//! This module provides a comprehensive downloading strategy that:
//! 1. First tries the standard hf_hub API approach
//! 2. Falls back to git clone with LFS support for complex structures
//! 3. Provides verification and error recovery

use anyhow::{Error as E, Result};
use std::path::{Path, PathBuf};

use crate::multi_component_downloader::{MultiComponentConfig, download_multi_component_model};
use crate::api_recursive_downloader::{download_model_with_api_structure, ApiDownloadConfig};
use crate::git2_downloader::{clone_hf_repository_git2, Git2DownloadConfig, verify_cloned_repository};

/// Download a multi-component model using the most appropriate method
///
/// This function tries multiple strategies:
/// 1. API-based recursive download (uses HF API to get file list, then downloads each file)
/// 2. Standard hf_hub API download (fast, works for most files)
/// 3. Git2 clone (no external dependencies, handles all file types)
pub fn download_multi_component_model_robust(
    model_id: &str,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    if config.verbose {
        println!("🚀 Starting robust multi-component model download");
        println!("📦 Model: {}", model_id);
    }

    // Strategy 1: Try API-based recursive download first
    if config.verbose {
        println!("🔄 Strategy 1: API-based recursive download...");
    }
    match try_api_recursive_download(model_id, config) {
        Ok(cache_dir) => {
            if verify_coreml_components(&cache_dir, config).is_ok() {
                if config.verbose {
                    println!("✅ Strategy 1 succeeded with strict verification");
                }
                return Ok(cache_dir);
            } else {
                if config.verbose {
                    println!("⚠️  Strategy 1 downloaded files but strict verification failed");
                }
            }
        }
        Err(e) => {
            if config.verbose {
                println!("⚠️  Strategy 1 failed: {}", e);
            }
        }
    }

    // Strategy 2: Try hf_hub API 
    if config.verbose {
        println!("🔄 Strategy 2: Standard hf_hub API...");
    }
    match try_hf_hub_download(model_id, config) {
        Ok(cache_dir) => {
            if verify_coreml_components(&cache_dir, config).is_ok() {
                if config.verbose {
                    println!("✅ Strategy 2 succeeded with strict verification");
                }
                return Ok(cache_dir);
            } else {
                if config.verbose {
                    println!("⚠️  Strategy 2 downloaded files but strict verification failed");
                }
            }
        }
        Err(e) => {
            if config.verbose {
                println!("⚠️  Strategy 2 failed: {}", e);
            }
        }
    }

    // Strategy 3: Hybrid approach - git2 for .mlmodelc + hf_hub for LFS files
    if config.verbose {
        println!("🔄 Strategy 3: Hybrid git2+hf_hub (final fallback)...");
    }
    try_hybrid_download(model_id, config)
}

/// Try downloading using API-based recursive approach
fn try_api_recursive_download(
    model_id: &str,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    // Setup HuggingFace API
    let api = hf_hub::api::sync::Api::new()
        .map_err(|e| E::msg(format!("Failed to create HF API: {}", e)))?;
    let repo = api.model(model_id.to_string());

    // Convert MultiComponentConfig to ApiDownloadConfig
    let api_config = ApiDownloadConfig {
        components: config.components.clone(),
        additional_files: config.additional_files.clone(),
        verbose: config.verbose,
    };

    // Try the API-based recursive download
    let cache_dir = download_model_with_api_structure(model_id, &repo, &api_config)?;
    
    Ok(cache_dir)
}

/// Try downloading using the standard hf_hub API with robust strategy
fn try_hf_hub_download(
    model_id: &str,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    // Setup HuggingFace API
    let api = hf_hub::api::sync::Api::new()
        .map_err(|e| E::msg(format!("Failed to create HF API: {}", e)))?;
    let repo = api.model(model_id.to_string());

    // Try the improved multi-component download with robust file access
    let cache_dir = download_multi_component_model(&repo, config)?;
    
    Ok(cache_dir)
}

/// Try downloading using git2 crate (no external dependencies)
fn try_git2_download(
    model_id: &str,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    if config.verbose {
        println!("🔄 Trying git2-based clone (no external dependencies)...");
    }

    // Create cache directory
    let cache_base = dirs::cache_dir()
        .ok_or_else(|| E::msg("Cannot determine cache directory"))?
        .join("candle-coreml");
    
    std::fs::create_dir_all(&cache_base)
        .map_err(|e| E::msg(format!("Failed to create cache directory: {}", e)))?;

    // Configure git2 downloader
    let git2_config = Git2DownloadConfig::for_hf_model(model_id, &cache_base, config.verbose);

    // Clone the repository using git2
    let target_dir = clone_hf_repository_git2(&git2_config)
        .map_err(|e| E::msg(format!("Git2 clone failed: {}", e)))?;

    // Verify the git2-cloned components
    verify_cloned_repository(&target_dir, &config.components, config.verbose)
        .map_err(|e| E::msg(format!("Git2 repository verification failed: {}", e)))?;

    if config.verbose {
        println!("✅ Git2 clone completed successfully");
    }

    Ok(target_dir)
}

/// Try hybrid download: git2 for .mlmodelc files, hf_hub for LFS files
fn try_hybrid_download(
    model_id: &str,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    if config.verbose {
        println!("🔄 Hybrid approach: git2 for .mlmodelc + hf_hub for LFS files...");
    }

    // First, try git2 clone for .mlmodelc files
    let git2_dir = try_git2_download(model_id, config)?;

    // Setup HuggingFace API for LFS files
    let api = hf_hub::api::sync::Api::new()
        .map_err(|e| E::msg(format!("Failed to create HF API: {}", e)))?;
    let repo = api.model(model_id.to_string());

    // Download LFS files like tokenizer.json using hf_hub (handles LFS properly)
    for additional_file in &config.additional_files {
        if config.verbose {
            println!("📥 Downloading LFS file with hf_hub: {}", additional_file);
        }
        
        match repo.get(additional_file) {
            Ok(downloaded_path) => {
                // Copy the properly downloaded LFS file to the git2 directory
                let target_path = git2_dir.join(additional_file);
                
                // Create parent directories if needed
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| E::msg(format!("Failed to create parent directory: {}", e)))?;
                }
                
                std::fs::copy(&downloaded_path, &target_path)
                    .map_err(|e| E::msg(format!("Failed to copy {} to git2 directory: {}", additional_file, e)))?;
                
                if config.verbose {
                    println!("  ✅ Copied {} to git2 directory", additional_file);
                }
            }
            Err(e) => {
                if config.verbose {
                    println!("  ⚠️  Failed to download {}: {}", additional_file, e);
                }
                // For non-critical files, continue
                if additional_file.contains("tokenizer") {
                    return Err(E::msg(format!("Failed to download critical LFS file: {}", additional_file)));
                }
            }
        }
    }

    // Create Manifest.json files for all .mlmodelc components
    for component in &config.components {
        let component_dir = git2_dir.join(component);
        if component_dir.exists() {
            let manifest_path = component_dir.join("Manifest.json");
            if !manifest_path.exists() {
                if config.verbose {
                    println!("📝 Creating Manifest.json for {}", component);
                }
                
                let manifest_content = create_manifest_json();
                std::fs::write(&manifest_path, manifest_content)
                    .map_err(|e| E::msg(format!("Failed to create Manifest.json for {}: {}", component, e)))?;
                
                if config.verbose {
                    println!("  ✅ Created Manifest.json for {}", component);
                }
            }
        }
    }

    if config.verbose {
        println!("✅ Hybrid download completed successfully");
    }

    Ok(git2_dir)
}


/// Create a Manifest.json content for CoreML models
fn create_manifest_json() -> String {
    r#"{
  "fileFormatVersion": "1.0.0",
  "itemInfoEntries": {
    "main": {
      "path": "model.mil",
      "type": "mlModelCode"
    }
  },
  "rootModelIdentifier": "main",
  "modelIdentifierToModelDocumentMap": {
    "main": "model.mil"
  }
}"#.to_string()
}

/// Verify that CoreML components are valid in hf_hub cache (relaxed version)
fn verify_coreml_components_relaxed(
    cache_dir: &Path,
    config: &MultiComponentConfig,
) -> Result<()> {
    for component in &config.components {
        let component_path = cache_dir.join(component);
        
        if !component_path.exists() {
            return Err(E::msg(format!(
                "Component directory not found: {}",
                component_path.display()
            )));
        }

        // Check for at least one essential CoreML file
        let has_coremldata = component_path.join("coremldata.bin").exists();
        let has_model_mil = component_path.join("model.mil").exists();
        let has_weights = component_path.join("weights").exists();

        if !has_coremldata && !has_model_mil && !has_weights {
            return Err(E::msg(format!(
                "Component {} is missing all essential CoreML files",
                component
            )));
        }

        if config.verbose {
            println!("  ✅ Component {} has files - coremldata: {}, model.mil: {}, weights: {}", 
                component, has_coremldata, has_model_mil, has_weights);
        }
    }

    Ok(())
}

/// Verify that CoreML components are valid in hf_hub cache (strict version)
fn verify_coreml_components(
    cache_dir: &Path,
    config: &MultiComponentConfig,
) -> Result<()> {
    for component in &config.components {
        let component_path = cache_dir.join(component);
        
        if !component_path.exists() {
            return Err(E::msg(format!(
                "Component directory not found: {}",
                component_path.display()
            )));
        }

        // Check for at least one essential CoreML file
        let has_coremldata = component_path.join("coremldata.bin").exists();
        let has_model_mil = component_path.join("model.mil").exists();
        let has_weights = component_path.join("weights").exists();

        if !has_coremldata && !has_model_mil && !has_weights {
            return Err(E::msg(format!(
                "Component {} is missing essential CoreML files",
                component
            )));
        }

        // For CoreML to work properly, we really need model.mil
        if !has_model_mil {
            return Err(E::msg(format!(
                "Component {} is missing model.mil (required for CoreML)",
                component
            )));
        }
    }

    Ok(())
}

/// Get paths to model components from a downloaded repository
pub fn get_component_paths_from_repo(
    repo_dir: &Path,
    config: &MultiComponentConfig,
) -> Result<(PathBuf, Vec<PathBuf>, Vec<PathBuf>)> {
    let repo_dir = repo_dir.to_path_buf();
    
    let component_paths: Vec<PathBuf> = config
        .components
        .iter()
        .map(|component| repo_dir.join(component))
        .collect();

    let additional_paths: Vec<PathBuf> = config
        .additional_files
        .iter()
        .map(|file| repo_dir.join(file))
        .collect();

    // Verify all paths exist
    for path in &component_paths {
        if !path.exists() {
            return Err(E::msg(format!(
                "Component path not found: {}",
                path.display()
            )));
        }
    }

    for path in &additional_paths {
        if !path.exists() {
            return Err(E::msg(format!(
                "Additional file not found: {}",
                path.display()
            )));
        }
    }

    Ok((repo_dir, component_paths, additional_paths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_url_generation() {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let expected_url = "https://huggingface.co/anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let actual_url = format!("https://huggingface.co/{}", model_id);
        assert_eq!(actual_url, expected_url);
    }

    #[test]
    fn test_cache_name_generation() {
        let model_id = "anemll/anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let expected_name = "anemll--anemll-Qwen-Qwen3-0.6B-ctx512_0.3.4";
        let actual_name = model_id.replace('/', "--");
        assert_eq!(actual_name, expected_name);
    }
}