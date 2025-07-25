//! Multi-component model downloader for Qwen and similar architectures
//!
//! This module provides utilities to download multi-component CoreML models
//! from HuggingFace Hub, handling the complex directory structures of .mlmodelc
//! files that require downloading multiple files per component.

use anyhow::{Error as E, Result};
use hf_hub::api::sync::ApiRepo;
use std::path::{Path, PathBuf};

/// Configuration for multi-component model downloading
#[derive(Debug, Clone)]
pub struct MultiComponentConfig {
    /// Model components to download (e.g., "qwen_embeddings.mlmodelc")
    pub components: Vec<String>,
    /// Additional files to download (e.g., "tokenizer.json")
    pub additional_files: Vec<String>,
    /// Whether to enable verbose output
    pub verbose: bool,
}

impl Default for MultiComponentConfig {
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

/// Download all components for a multi-component model
///
/// This function downloads:
/// 1. All files within each .mlmodelc directory using the bert_inference.rs strategy
/// 2. Additional files like tokenizer.json 
/// 3. Verifies all required files are present
///
/// Returns the cache directory where all files are stored.
pub fn download_multi_component_model(
    api_repo: &ApiRepo,
    config: &MultiComponentConfig,
) -> Result<PathBuf> {
    if config.verbose {
        println!("📥 Downloading multi-component model...");
        println!("  Components: {:?}", config.components);
        println!("  Additional files: {:?}", config.additional_files);
    }

    // Download additional files first (like tokenizer) to establish cache directory
    let mut cache_dir = None;
    for file in &config.additional_files {
        if config.verbose {
            println!("  • Downloading: {}", file);
        }
        
        let file_path = api_repo.get(file)
            .map_err(|e| E::msg(format!("Failed to download {}: {}", file, e)))?;
        
        if cache_dir.is_none() {
            cache_dir = Some(
                file_path
                    .parent()
                    .ok_or_else(|| E::msg("Cannot determine cache directory"))?
                    .to_path_buf(),
            );
        }
        
        if config.verbose {
            println!("    ✅ Downloaded to: {}", file_path.display());
        }
    }

    let cache_dir = cache_dir.ok_or_else(|| E::msg("No cache directory established"))?;

    // Download each .mlmodelc component using bert_inference.rs strategy
    for component in &config.components {
        if config.verbose {
            println!("  • Downloading component: {}", component);
        }
        
        download_mlmodelc_component_robust(api_repo, component, config.verbose)?;
        
        if config.verbose {
            println!("    ✅ Component downloaded successfully");
        }
    }

    // Verify all components exist
    verify_multi_component_model(&cache_dir, config)?;

    if config.verbose {
        println!("✅ Multi-component model download completed");
        println!("📁 Cache directory: {}", cache_dir.display());
    }

    Ok(cache_dir)
}

/// Download a .mlmodelc component using the robust bert_inference.rs strategy
///
/// This approach downloads a key file deep in the directory structure and lets
/// hf-hub handle the LFS downloading automatically. This is more reliable than
/// trying to download individual files with complex paths.
fn download_mlmodelc_component_robust(
    api_repo: &ApiRepo,
    component_name: &str,
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("    • Using robust download strategy for: {}", component_name);
    }

    // Strategy: Download a key file that's likely to exist and trigger directory download
    // Priority order: weights/weight.bin > coremldata.bin > model.mil
    let key_files = [
        format!("{}/weights/weight.bin", component_name),
        format!("{}/coremldata.bin", component_name), 
        format!("{}/model.mil", component_name),
    ];

    let mut success = false;
    for key_file in &key_files {
        if verbose {
            println!("      • Attempting to download key file: {}", key_file);
        }

        match api_repo.get(key_file) {
            Ok(downloaded_path) => {
                if verbose {
                    println!("      ✅ Successfully downloaded: {}", downloaded_path.display());
                }
                success = true;
                break;
            }
            Err(e) => {
                if verbose {
                    println!("      ⚠️  Could not download {}: {}", key_file, e); 
                }
                // Continue to next key file
            }
        }
    }

    if !success {
        return Err(E::msg(format!(
            "Could not download any key files for component: {}",
            component_name
        )));
    }

    // Try to download additional important files now that the directory structure exists
    download_additional_component_files(api_repo, component_name, verbose)?;

    Ok(())
}

/// Download additional files for a component after the key file is downloaded
fn download_additional_component_files(
    api_repo: &ApiRepo,
    component_name: &str,
    verbose: bool,
) -> Result<()> {
    // List of additional files to try downloading
    let additional_files = [
        format!("{}/model.mil", component_name),
        format!("{}/metadata.json", component_name),
        format!("{}/coremldata.bin", component_name),
    ];

    for additional_file in &additional_files {
        match api_repo.get(&additional_file) {
            Ok(downloaded_path) => {
                if verbose {
                    println!("      ✅ Downloaded additional file: {}", downloaded_path.display());
                }
            }
            Err(e) => {
                if verbose {
                    println!("      ⚠️  Could not download {} with standard API: {}", additional_file, e);
                }
                
                // Try direct URL approach for LFS files
                if let Err(direct_err) = try_direct_download(api_repo, &additional_file, verbose) {
                    if verbose {
                        println!("      ⚠️  Direct download also failed: {}", direct_err);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Try downloading a file using alternative hf-hub methods
fn try_direct_download(
    api_repo: &ApiRepo,
    file_path: &str,
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("      🔄 Trying alternative download method for: {}", file_path);
    }

    // Try using the download method instead of get
    // download() might handle LFS files differently than get()
    match api_repo.download(file_path) {
        Ok(downloaded_path) => {
            if verbose {
                println!("      ✅ Downloaded with download() method: {}", downloaded_path.display());
            }
            Ok(())
        }
        Err(e) => {
            if verbose {
                println!("      ⚠️  download() method also failed: {}", e);
            }
            Err(E::msg(format!("Alternative download failed for {}: {}", file_path, e)))
        }
    }
}

/// Legacy download function - kept for compatibility but not used
fn download_mlmodelc_component(
    api_repo: &ApiRepo,
    component_name: &str,
    _cache_dir: &Path,
    verbose: bool,
) -> Result<()> {
    // List of files typically found in .mlmodelc directories
    // We'll try to download them but won't fail if some are missing
    let mlmodelc_files = [
        "coremldata.bin",  // Most likely to be available
        "model.mil",
        "metadata.json",
    ];

    let mut downloaded_count = 0;
    
    // Try to download each file within the component directory
    for file in &mlmodelc_files {
        let file_path = format!("{}/{}", component_name, file);
        
        if verbose {
            println!("    • Downloading: {}", file_path);
        }
        
        match api_repo.get(&file_path) {
            Ok(downloaded_path) => {
                if verbose {
                    println!("      ✅ Downloaded: {}", downloaded_path.display());
                }
                downloaded_count += 1;
            }
            Err(e) => {
                if verbose {
                    println!("      ⚠️  Could not download {}: {}", file_path, e);
                }
                // Continue - some files might be optional or have different names
            }
        }
    }

    // Try to download weights directory files
    if download_weights_directory(api_repo, component_name, verbose).is_ok() {
        downloaded_count += 1;
    }

    // Try to download analytics directory (optional)
    if let Err(e) = download_analytics_directory(api_repo, component_name, verbose) {
        if verbose {
            println!("      ℹ️  Analytics directory not available: {}", e);
        }
    }

    if downloaded_count == 0 {
        return Err(E::msg(format!(
            "Could not download any files for component: {}",
            component_name
        )));
    }

    Ok(())
}

/// Download files from the weights/ subdirectory
fn download_weights_directory(
    api_repo: &ApiRepo,
    component_name: &str,
    verbose: bool,
) -> Result<()> {
    // Common weight file patterns
    let weight_files = [
        "weights/weight.bin",
        "weights/weight_0.bin", 
        "weights/weight_1.bin",
        "weights/weights.bin",
    ];

    let mut downloaded_any = false;
    
    for weight_file in &weight_files {
        let weight_path = format!("{}/{}", component_name, weight_file);
        
        match api_repo.get(&weight_path) {
            Ok(downloaded_path) => {
                if verbose {
                    println!("      ✅ Downloaded weight: {}", downloaded_path.display());
                }
                downloaded_any = true;
            }
            Err(_) => {
                // Try next weight file pattern
                continue;
            }
        }
    }

    if !downloaded_any && verbose {
        println!("      ⚠️  No weight files found in weights/ directory");
    }

    Ok(())
}

/// Download files from the analytics/ subdirectory (optional)
fn download_analytics_directory(
    api_repo: &ApiRepo,
    component_name: &str,
    verbose: bool,
) -> Result<()> {
    // Analytics files are typically optional
    let analytics_files = [
        "analytics/analytics.json",
        "analytics/performance.json",
    ];

    for analytics_file in &analytics_files {
        let analytics_path = format!("{}/{}", component_name, analytics_file);
        
        match api_repo.get(&analytics_path) {
            Ok(downloaded_path) => {
                if verbose {
                    println!("      ✅ Downloaded analytics: {}", downloaded_path.display());
                }
            }
            Err(_) => {
                // Analytics files are optional
                continue;
            }
        }
    }

    Ok(())
}

/// Verify that all required components were downloaded successfully
fn verify_multi_component_model(
    cache_dir: &Path,
    config: &MultiComponentConfig,
) -> Result<()> {
    // Check additional files
    for file in &config.additional_files {
        let file_path = cache_dir.join(file);
        if !file_path.exists() {
            return Err(E::msg(format!(
                "Required file not found: {}",
                file_path.display()
            )));
        }
    }

    // Check each component directory
    for component in &config.components {
        let component_dir = cache_dir.join(component);
        if !component_dir.exists() {
            return Err(E::msg(format!(
                "Component directory not found: {}",
                component_dir.display()
            )));
        }

        // Check for at least one essential file within the component
        // CoreML models need at least coremldata.bin or weights to function
        let essential_files = ["coremldata.bin", "model.mil", "metadata.json"];
        let mut found_essential = false;
        
        for essential_file in &essential_files {
            let file_path = component_dir.join(essential_file);
            if file_path.exists() {
                found_essential = true;
                break;
            }
        }
        
        // Also check for weights directory
        let weights_dir = component_dir.join("weights");
        if weights_dir.exists() && weights_dir.is_dir() {
            found_essential = true;
        }
        
        if !found_essential {
            return Err(E::msg(format!(
                "No essential files found in component {}: checked for {:?}, weights/",
                component, essential_files
            )));
        }
    }

    Ok(())
}

/// Get paths to all downloaded model components
///
/// Returns a tuple of (cache_dir, component_paths, additional_file_paths)
pub fn get_model_component_paths(
    cache_dir: &Path,
    config: &MultiComponentConfig,
) -> Result<(PathBuf, Vec<PathBuf>, Vec<PathBuf>)> {
    let cache_dir = cache_dir.to_path_buf();
    
    let component_paths: Vec<PathBuf> = config
        .components
        .iter()
        .map(|component| cache_dir.join(component))
        .collect();

    let additional_paths: Vec<PathBuf> = config
        .additional_files
        .iter()
        .map(|file| cache_dir.join(file))
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

    Ok((cache_dir, component_paths, additional_paths))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_component_config_default() {
        let config = MultiComponentConfig::default();
        assert_eq!(config.components.len(), 3);
        assert!(config.components.contains(&"qwen_embeddings.mlmodelc".to_string()));
        assert!(config.components.contains(&"qwen_FFN_PF_lut6_chunk_01of01.mlmodelc".to_string()));
        assert!(config.components.contains(&"qwen_lm_head_lut6.mlmodelc".to_string()));
        assert_eq!(config.additional_files.len(), 1);
        assert!(config.additional_files.contains(&"tokenizer.json".to_string()));
    }

    #[test]
    fn test_multi_component_config_custom() {
        let config = MultiComponentConfig {
            components: vec!["custom_model.mlmodelc".to_string()],
            additional_files: vec!["config.json".to_string()],
            verbose: true,
        };
        
        assert_eq!(config.components.len(), 1);
        assert_eq!(config.additional_files.len(), 1);
        assert!(config.verbose);
    }
}