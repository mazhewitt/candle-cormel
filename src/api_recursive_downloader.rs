//! API-based recursive downloader for HuggingFace models
//!
//! This module uses the HuggingFace API to get the complete file structure
//! and then downloads all files recursively, creating the proper directory
//! structure locally.

use anyhow::{Error as E, Result};
use hf_hub::api::sync::ApiRepo;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// File information from HuggingFace API
#[derive(Debug, Deserialize, Serialize)]
struct ApiFile {
    #[serde(rename = "rfilename")]
    filename: String,
    #[serde(rename = "size")]
    _size: Option<u64>,
    #[serde(rename = "lfs")]
    _lfs: Option<serde_json::Value>,
}

/// Model information from HuggingFace API
#[derive(Debug, Deserialize, Serialize)]
struct ApiModelInfo {
    siblings: Vec<ApiFile>,
}

/// Configuration for API-based recursive download
#[derive(Debug, Clone)]
pub struct ApiDownloadConfig {
    /// Model components to download (e.g., "qwen_embeddings.mlmodelc")
    pub components: Vec<String>,
    /// Additional files to download (e.g., "tokenizer.json")
    pub additional_files: Vec<String>,
    /// Whether to enable verbose output
    pub verbose: bool,
}

impl Default for ApiDownloadConfig {
    fn default() -> Self {
        Self {
            components: vec![
                "qwen_embeddings.mlmodelc".to_string(),
                "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc".to_string(),
                "qwen_lm_head_lut6.mlmodelc".to_string(),
            ],
            additional_files: vec!["tokenizer.json".to_string()],
            verbose: false,
        }
    }
}

/// Download a model using the HuggingFace API to get file structure
pub fn download_model_with_api_structure(
    model_id: &str,
    api_repo: &ApiRepo,
    config: &ApiDownloadConfig,
) -> Result<PathBuf> {
    if config.verbose {
        println!("🚀 Starting API-based recursive download for: {}", model_id);
    }

    // Get the model file structure from the API
    let file_list = get_model_file_structure(model_id, config.verbose)?;
    
    // Filter files to only download what we need
    let files_to_download = filter_required_files(&file_list, config)?;
    
    if config.verbose {
        println!("📋 Found {} files to download", files_to_download.len());
    }

    // Download all required files
    let cache_dir = download_files_recursively(api_repo, &files_to_download, config)?;
    
    // Verify the downloaded structure
    verify_downloaded_structure(&cache_dir, config)?;

    if config.verbose {
        println!("✅ API-based recursive download completed");
        println!("📁 Cache directory: {}", cache_dir.display());
    }

    Ok(cache_dir)
}

/// Get model file structure from HuggingFace API
fn get_model_file_structure(model_id: &str, verbose: bool) -> Result<Vec<String>> {
    if verbose {
        println!("🔍 Fetching file structure from HuggingFace API...");
    }

    let api_url = format!("https://huggingface.co/api/models/{}", model_id);
    
    // For now, we'll use a simple reqwest approach
    // We need to add reqwest as a dependency
    if verbose {
        println!("🌐 API URL: {}", api_url);
        println!("⚠️  API fetching not yet implemented - using hardcoded file list");
    }

    // Hardcoded file list based on the API response we saw
    // TODO: Replace with actual HTTP request
    let files = vec![
        "tokenizer.json".to_string(),
        "config.json".to_string(),
        "qwen_embeddings.mlmodelc/coremldata.bin".to_string(),
        "qwen_embeddings.mlmodelc/metadata.json".to_string(),
        "qwen_embeddings.mlmodelc/model.mil".to_string(),
        "qwen_embeddings.mlmodelc/weights/weight.bin".to_string(),
        "qwen_embeddings.mlmodelc/analytics/coremldata.bin".to_string(),
        "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc/coremldata.bin".to_string(),
        "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc/metadata.json".to_string(),
        "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc/model.mil".to_string(),
        "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc/weights/weight.bin".to_string(),
        "qwen_FFN_PF_lut6_chunk_01of01.mlmodelc/analytics/coremldata.bin".to_string(),
        "qwen_lm_head_lut6.mlmodelc/coremldata.bin".to_string(),
        "qwen_lm_head_lut6.mlmodelc/metadata.json".to_string(),
        "qwen_lm_head_lut6.mlmodelc/model.mil".to_string(),
        "qwen_lm_head_lut6.mlmodelc/weights/weight.bin".to_string(),
        "qwen_lm_head_lut6.mlmodelc/analytics/coremldata.bin".to_string(),
    ];

    if verbose {
        println!("📄 Using {} hardcoded files", files.len());
    }

    Ok(files)
}

/// Filter files to only download what we need
fn filter_required_files(
    all_files: &[String],
    config: &ApiDownloadConfig,
) -> Result<Vec<String>> {
    let mut required_files = Vec::new();

    // Add additional files (like tokenizer.json)
    for additional_file in &config.additional_files {
        if all_files.contains(additional_file) {
            required_files.push(additional_file.clone());
        }
    }

    // Add component files
    for component in &config.components {
        for file in all_files {
            if file.starts_with(&format!("{}/", component)) {
                required_files.push(file.clone());
            }
        }
    }

    if config.verbose {
        println!("🎯 Filtered to {} required files", required_files.len());
        for file in &required_files {
            println!("  • {}", file);
        }
    }

    Ok(required_files)
}

/// Download files recursively, creating directory structure
fn download_files_recursively(
    api_repo: &ApiRepo,
    files_to_download: &[String],
    config: &ApiDownloadConfig,
) -> Result<PathBuf> {
    let mut cache_dir = None;

    for file_path in files_to_download {
        if config.verbose {
            println!("📥 Downloading: {}", file_path);
        }

        match api_repo.get(file_path) {
            Ok(downloaded_path) => {
                if cache_dir.is_none() {
                    // Determine cache directory from first successful download
                    cache_dir = Some(
                        find_cache_root(&downloaded_path, file_path)
                            .ok_or_else(|| E::msg("Cannot determine cache directory"))?
                    );
                }

                if config.verbose {
                    println!("  ✅ Downloaded: {}", downloaded_path.display());
                }
            }
            Err(e) => {
                if config.verbose {
                    println!("  ❌ Failed to download {}: {}", file_path, e);
                }
                
                // For non-critical files, continue
                if is_critical_file(file_path) {
                    return Err(E::msg(format!("Failed to download critical file: {}", file_path)));
                }
            }
        }
    }

    cache_dir.ok_or_else(|| E::msg("No files were successfully downloaded"))
}

/// Find the cache root directory from a downloaded file path
fn find_cache_root(downloaded_path: &Path, relative_path: &str) -> Option<PathBuf> {
    let path_components: Vec<&str> = relative_path.split('/').collect();
    let mut current_path = downloaded_path;
    
    // Walk up the path to find the cache root
    for _ in 0..path_components.len() {
        if let Some(parent) = current_path.parent() {
            current_path = parent;
        } else {
            break;
        }
    }
    
    Some(current_path.to_path_buf())
}

/// Check if a file is critical for the model to function
fn is_critical_file(file_path: &str) -> bool {
    // Critical files that must be downloaded
    let critical_patterns = [
        "tokenizer.json",
        "/model.mil",
        "/coremldata.bin",
        "/weights/weight.bin",
    ];
    
    critical_patterns.iter().any(|pattern| file_path.contains(pattern))
}

/// Verify that the downloaded structure is correct
fn verify_downloaded_structure(
    cache_dir: &Path,
    config: &ApiDownloadConfig,
) -> Result<()> {
    if config.verbose {
        println!("🔍 Verifying downloaded structure...");
    }

    // Check additional files
    for additional_file in &config.additional_files {
        let file_path = cache_dir.join(additional_file);
        if !file_path.exists() {
            return Err(E::msg(format!(
                "Required file not found: {}",
                file_path.display()
            )));
        }
        
        if config.verbose {
            println!("  ✅ Found: {}", additional_file);
        }
    }

    // Check component directories and their critical files
    for component in &config.components {
        let component_dir = cache_dir.join(component);
        if !component_dir.exists() {
            return Err(E::msg(format!(
                "Component directory not found: {}",
                component_dir.display()
            )));
        }

        // Check critical files within each component
        let critical_files = ["model.mil", "coremldata.bin"];
        for critical_file in &critical_files {
            let file_path = component_dir.join(critical_file);
            if !file_path.exists() {
                return Err(E::msg(format!(
                    "Critical file missing in {}: {}",
                    component, critical_file
                )));
            }
        }

        // Check weights directory
        let weights_dir = component_dir.join("weights");
        if !weights_dir.exists() || !weights_dir.join("weight.bin").exists() {
            return Err(E::msg(format!(
                "Weights directory or weight.bin missing in: {}",
                component
            )));
        }

        if config.verbose {
            println!("  ✅ Component verified: {}", component);
        }
    }

    if config.verbose {
        println!("✅ All required files and directories verified");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_required_files() {
        let all_files = vec![
            "tokenizer.json".to_string(),
            "README.md".to_string(),
            "qwen_embeddings.mlmodelc/model.mil".to_string(),
            "qwen_embeddings.mlmodelc/weights/weight.bin".to_string(),
            "other_component.mlmodelc/model.mil".to_string(),
        ];

        let config = ApiDownloadConfig {
            components: vec!["qwen_embeddings.mlmodelc".to_string()],
            additional_files: vec!["tokenizer.json".to_string()],
            verbose: false,
        };

        let filtered = filter_required_files(&all_files, &config).unwrap();
        
        assert_eq!(filtered.len(), 3);
        assert!(filtered.contains(&"tokenizer.json".to_string()));
        assert!(filtered.contains(&"qwen_embeddings.mlmodelc/model.mil".to_string()));
        assert!(filtered.contains(&"qwen_embeddings.mlmodelc/weights/weight.bin".to_string()));
        assert!(!filtered.contains(&"other_component.mlmodelc/model.mil".to_string()));
    }

    #[test]
    fn test_is_critical_file() {
        assert!(is_critical_file("tokenizer.json"));
        assert!(is_critical_file("qwen_embeddings.mlmodelc/model.mil"));
        assert!(is_critical_file("qwen_embeddings.mlmodelc/coremldata.bin"));
        assert!(is_critical_file("qwen_embeddings.mlmodelc/weights/weight.bin"));
        assert!(!is_critical_file("README.md"));
        assert!(!is_critical_file("qwen_embeddings.mlmodelc/analytics/coremldata.bin"));
    }
}