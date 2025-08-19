//! Unified model loader that combines downloading, config generation, and model loading
//!
//! This module provides a simplified API that replaces hardcoded paths with
//! automatic HuggingFace downloading and config generation.

use crate::model_config::ModelConfig;
use crate::model_downloader::{download_model, ensure_model_downloaded};
use crate::{CacheManager, ConfigGenerator, QwenConfig, QwenModel};
use anyhow::Result;
use serde_json::Value;
use std::path::Path;
use tracing::{debug, info};

/// Unified model loader that handles downloading, config generation, and model loading
pub struct UnifiedModelLoader {
    cache_manager: CacheManager,
    pub config_generator: ConfigGenerator,
}

impl UnifiedModelLoader {
    /// Create a new unified model loader
    pub fn new() -> Result<Self> {
        let cache_manager = CacheManager::new()?;
        let config_generator = ConfigGenerator::new()?;

        Ok(Self {
            cache_manager,
            config_generator,
        })
    }

    /// Load a model by HuggingFace model ID with automatic downloading and config generation
    ///
    /// This replaces the pattern of hardcoded paths in config files.
    ///
    /// # Example
    /// ```rust,no_run
    /// use candle_coreml::UnifiedModelLoader;
    ///
    /// let loader = UnifiedModelLoader::new()?;
    /// let model = loader.load_model("mazhewitt/qwen-typo-fixer-coreml").await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn load_model(&self, model_id: &str) -> Result<QwenModel> {
        info!("🚀 Loading model: {}", model_id);

        // Step 1: Check if we have a cached config
        if let Some(cached_config) = self.config_generator.load_cached_config(model_id)? {
            info!("📖 Found cached config for {}", model_id);

            // Verify the model files still exist
            if self.verify_model_files_exist(&cached_config) {
                // Validate config and internal wiring; regenerate if invalid or inconsistent
                let valid_basic = cached_config.validate();
                let valid_wiring = cached_config.validate_internal_wiring();
                if valid_basic.is_ok() && valid_wiring.is_ok() {
                    // Extra: if FFN package exposes both prefill & infer functions but config lacks ffn_infer, regenerate
                    if self.config_requires_ffn_split_upgrade(&cached_config) {
                        info!(
                            "♻️  Cached config lacks 'ffn_infer' but FFN manifest has both functions; regenerating config"
                        );
                        if let Some(model_path_str) = &cached_config.model_info.path {
                            let model_path = std::path::PathBuf::from(model_path_str);
                            if model_path.exists() {
                                let config = self.config_generator.generate_config_from_directory(
                                    &model_path,
                                    model_id,
                                    "qwen",
                                )?;
                                return self.load_model_from_config(&config);
                            }
                        }
                    }

                    info!("✅ Cached config validated, using it");
                    return self.load_model_from_config(&cached_config);
                } else {
                    // Log why we are regenerating
                    if let Err(e) = valid_basic {
                        info!("♻️  Cached config failed validation, regenerating: {e}");
                    }
                    if let Err(e) = valid_wiring {
                        info!("♻️  Cached config failed internal wiring, regenerating: {e}");
                    }

                    // Regenerate from existing model directory if available
                    if let Some(model_path_str) = &cached_config.model_info.path {
                        let model_path = std::path::PathBuf::from(model_path_str);
                        if model_path.exists() {
                            info!("🔍 Regenerating config from existing model at {}", model_path.display());
                            let config = self.config_generator.generate_config_from_directory(
                                &model_path,
                                model_id,
                                "qwen",
                            )?;
                            return self.load_model_from_config(&config);
                        } else {
                            info!("⚠️  Cached model path missing, will re-download");
                        }
                    } else {
                        info!("⚠️  Cached config missing model path, will re-download");
                    }
                }
            } else {
                info!("⚠️  Model files missing, will re-download");
            }
        }

        // Step 2: Download the model from HuggingFace
        info!("⬇️  Downloading model from HuggingFace: {}", model_id);
        let model_path = download_model(model_id, false)?;

        // Step 3: Generate config from downloaded files
        info!("🔍 Generating config from downloaded model");
        let config = self.config_generator.generate_config_from_directory(
            &model_path,
            model_id,
            "qwen", // Auto-detect this in the future
        )?;

        // Step 4: Load the model using the generated config
        self.load_model_from_config(&config)
    }

    /// Determine if a cached config should be upgraded to include a separate ffn_infer
    /// component by inspecting the FFN package manifest for both 'prefill' and 'infer' functions.
    fn config_requires_ffn_split_upgrade(&self, config: &ModelConfig) -> bool {
        // If ffn_infer already exists, nothing to do
        if config.components.contains_key("ffn_infer") {
            return false;
        }

        // Look for any FFN component file path to inspect its manifest
        let ffn_component = config
            .components
            .iter()
            .find(|(name, _)| name.to_lowercase().contains("ffn"))
            .and_then(|(_, comp)| comp.file_path.as_ref());

        let Some(ffn_path_str) = ffn_component else { return false };
        let ffn_path = std::path::Path::new(ffn_path_str);

        // Determine manifest path (.mlpackage -> Manifest.json, .mlmodelc -> metadata.json)
        let manifest_path = if ffn_path.join("Manifest.json").exists() {
            ffn_path.join("Manifest.json")
        } else if ffn_path.join("metadata.json").exists() {
            ffn_path.join("metadata.json")
        } else {
            return false;
        };

        // Read and parse manifest
        let Ok(content) = std::fs::read_to_string(&manifest_path) else { return false };
        let Ok(json): Result<Value, _> = serde_json::from_str(&content) else { return false };

        // Extract functions array
        let funcs = json
            .get(0)
            .and_then(|m| m.get("functions"))
            .and_then(|f| f.as_array());

        if let Some(functions) = funcs {
            let mut has_prefill = false;
            let mut has_infer = false;
            for f in functions {
                if let Some(name) = f.get("name").and_then(|n| n.as_str()) {
                    if name == "prefill" {
                        has_prefill = true;
                    } else if name == "infer" {
                        has_infer = true;
                    }
                }
            }
            // If both are present but config lacks ffn_infer, we should regenerate
            return has_prefill && has_infer;
        }

        false
    }

    /// Load a model from a pre-existing config (useful for advanced use cases)
    pub fn load_model_from_config(&self, config: &ModelConfig) -> Result<QwenModel> {
        info!("🔧 Loading model from config");

        // Convert ModelConfig to QwenConfig
        let qwen_config = QwenConfig::from_model_config(config.clone());

        // Extract the model directory from the config
        let model_dir = config
            .model_info
            .path
            .as_ref()
            .ok_or_else(|| anyhow::Error::msg("Model config missing path"))?;

        // Load the QwenModel
        let mut model = QwenModel::load_from_directory(model_dir, Some(qwen_config))?;
        model.initialize_states()?;

        info!("✅ Model loaded successfully");
        Ok(model)
    }

    /// Ensure model is downloaded and return the path (useful for external tools)
    pub fn ensure_model_available(&self, model_id: &str) -> Result<std::path::PathBuf> {
        ensure_model_downloaded(model_id, false)
    }

    /// Generate or update config for a model without loading it
    pub fn generate_config(&self, model_id: &str) -> Result<ModelConfig> {
        let model_path = self.ensure_model_available(model_id)?;

        self.config_generator
            .generate_config_from_directory(&model_path, model_id, "qwen")
    }

    /// List all cached models and their status
    pub fn list_cached_models(&self) -> Result<Vec<CachedModelInfo>> {
        let models_dir = self.cache_manager.models_dir();
        let configs_dir = self.cache_manager.configs_dir();

        let mut cached_models = Vec::new();

        // Scan models directory
        if models_dir.exists() {
            for entry in std::fs::read_dir(&models_dir)? {
                let entry = entry?;
                if entry.file_type()?.is_dir() {
                    let model_name = entry.file_name().to_string_lossy().to_string();
                    let model_id = model_name.replace("--", "/"); // Convert back from filename

                    let config_path = configs_dir.join(format!("{model_name}.json"));
                    let has_config = config_path.exists();

                    // Check if .mlpackage files exist
                    let model_files = self.count_mlpackage_files(&entry.path())?;

                    cached_models.push(CachedModelInfo {
                        model_id,
                        model_path: entry.path(),
                        has_config,
                        config_path: if has_config { Some(config_path) } else { None },
                        mlpackage_count: model_files,
                        size_bytes: self.get_directory_size(&entry.path())?,
                    });
                }
            }
        }

        // Sort by model ID for consistent output
        cached_models.sort_by(|a, b| a.model_id.cmp(&b.model_id));
        Ok(cached_models)
    }

    /// Verify that all model files referenced in config still exist
    fn verify_model_files_exist(&self, config: &ModelConfig) -> bool {
        for (component_name, component) in &config.components {
            if let Some(file_path) = &component.file_path {
                let path = Path::new(file_path);
                if !path.exists() {
                    debug!("Component '{}' file missing: {}", component_name, file_path);
                    return false;
                }
            }
        }
        true
    }

    /// Count .mlpackage files in a directory
    fn count_mlpackage_files(&self, dir: &Path) -> Result<usize> {
        let mut count = 0;

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "mlpackage" {
                        count += 1;
                    }
                }
            }
        }

        Ok(count)
    }

    /// Get directory size in bytes
    fn get_directory_size(&self, dir: &Path) -> Result<u64> {
        let mut total_size = 0;
        Self::visit_dir_size(dir, &mut total_size)?;
        Ok(total_size)
    }

    fn visit_dir_size(dir: &Path, total: &mut u64) -> Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                Self::visit_dir_size(&path, total)?;
            } else {
                *total += entry.metadata()?.len();
            }
        }
        Ok(())
    }
}

/// Information about a cached model
#[derive(Debug, Clone)]
pub struct CachedModelInfo {
    pub model_id: String,
    pub model_path: std::path::PathBuf,
    pub has_config: bool,
    pub config_path: Option<std::path::PathBuf>,
    pub mlpackage_count: usize,
    pub size_bytes: u64,
}

impl CachedModelInfo {
    /// Get human-readable size
    pub fn size_human(&self) -> String {
        let size = self.size_bytes as f64;

        if size >= 1_000_000_000.0 {
            format!("{:.1} GB", size / 1_000_000_000.0)
        } else if size >= 1_000_000.0 {
            format!("{:.1} MB", size / 1_000_000.0)
        } else if size >= 1_000.0 {
            format!("{:.1} KB", size / 1_000.0)
        } else {
            format!("{} B", size as u64)
        }
    }

    /// Check if the model appears to be complete
    pub fn is_complete(&self) -> bool {
        self.has_config && self.mlpackage_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_loader_creation() {
        let loader = UnifiedModelLoader::new().expect("Failed to create unified loader");

        // Should have valid cache manager and config generator
        let models = loader
            .list_cached_models()
            .expect("Failed to list cached models");
        println!("Found {} cached models", models.len());

        for model in &models {
            println!(
                "  • {} ({}, {} packages, {})",
                model.model_id,
                model.size_human(),
                model.mlpackage_count,
                if model.is_complete() {
                    "complete"
                } else {
                    "incomplete"
                }
            );
        }
    }

    #[test]
    fn test_cached_model_info() {
        let info = CachedModelInfo {
            model_id: "test/model".to_string(),
            model_path: std::path::PathBuf::from("/tmp/test"),
            has_config: true,
            config_path: Some(std::path::PathBuf::from("/tmp/test.json")),
            mlpackage_count: 4,
            size_bytes: 1_500_000_000,
        };

        assert_eq!(info.size_human(), "1.5 GB");
        assert!(info.is_complete());
    }
}
