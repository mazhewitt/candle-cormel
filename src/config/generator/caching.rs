//! Configuration caching utilities
//!
//! Handles saving and loading generated model configurations

use crate::cache::manager::CacheManager;
use crate::config::model::ModelConfig;
use anyhow::Result;
use tracing::{debug, info};

pub struct ConfigCaching {
    cache_manager: CacheManager,
}

impl ConfigCaching {
    pub fn new(cache_manager: CacheManager) -> Self {
        Self { cache_manager }
    }

    /// Cache a generated configuration
    pub fn cache_config(&self, model_id: &str, config: &ModelConfig) -> Result<()> {
        let configs_dir = self.cache_manager.configs_dir();
        std::fs::create_dir_all(&configs_dir)?;

        let config_filename = self.normalize_model_id_for_filename(model_id);
        let config_path = configs_dir.join(config_filename);

        let config_json = serde_json::to_string_pretty(config)?;
        std::fs::write(&config_path, config_json)?;

        info!("💾 Cached generated config at: {}", config_path.display());
        Ok(())
    }

    /// Load a cached configuration if available
    pub fn load_cached_config(&self, model_id: &str) -> Result<Option<ModelConfig>> {
        let configs_dir = self.cache_manager.configs_dir();
        let config_filename = self.normalize_model_id_for_filename(model_id);
        let config_path = configs_dir.join(config_filename);

        if !config_path.exists() {
            debug!("📖 No cached config found for: {}", model_id);
            return Ok(None);
        }

        let config_json = std::fs::read_to_string(&config_path)?;
        let config: ModelConfig = serde_json::from_str(&config_json)?;

        debug!("📖 Loaded cached config for: {}", model_id);
        Ok(Some(config))
    }

    /// Check if a cached configuration exists
    pub fn has_cached_config(&self, model_id: &str) -> bool {
        let configs_dir = self.cache_manager.configs_dir();
        let config_filename = self.normalize_model_id_for_filename(model_id);
        let config_path = configs_dir.join(config_filename);

        config_path.exists()
    }

    /// Clear cached configuration for a model
    pub fn clear_cached_config(&self, model_id: &str) -> Result<()> {
        let configs_dir = self.cache_manager.configs_dir();
        let config_filename = self.normalize_model_id_for_filename(model_id);
        let config_path = configs_dir.join(config_filename);

        if config_path.exists() {
            std::fs::remove_file(&config_path)?;
            info!("🗑️ Cleared cached config for: {}", model_id);
        }

        Ok(())
    }

    /// List all cached model configurations
    pub fn list_cached_configs(&self) -> Result<Vec<String>> {
        let configs_dir = self.cache_manager.configs_dir();
        let mut model_ids = Vec::new();

        if !configs_dir.exists() {
            return Ok(model_ids);
        }

        for entry in std::fs::read_dir(&configs_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                    // Convert filename back to model ID
                    let model_id = filename.replace("--", "/");
                    model_ids.push(model_id);
                }
            }
        }

        model_ids.sort();
        Ok(model_ids)
    }

    /// Get cache statistics
    pub fn get_cache_stats(&self) -> Result<CacheStats> {
        let configs_dir = self.cache_manager.configs_dir();

        if !configs_dir.exists() {
            return Ok(CacheStats::default());
        }

        let mut stats = CacheStats::default();
        let mut total_size = 0u64;

        for entry in std::fs::read_dir(&configs_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                stats.cached_configs += 1;

                if let Ok(metadata) = std::fs::metadata(&path) {
                    total_size += metadata.len();
                }
            }
        }

        stats.total_size_bytes = total_size;
        Ok(stats)
    }

    /// Clear all cached configurations
    pub fn clear_all_cached_configs(&self) -> Result<usize> {
        let configs_dir = self.cache_manager.configs_dir();
        let mut cleared_count = 0;

        if !configs_dir.exists() {
            return Ok(cleared_count);
        }

        for entry in std::fs::read_dir(&configs_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                std::fs::remove_file(&path)?;
                cleared_count += 1;
            }
        }

        info!("🗑️ Cleared {} cached configurations", cleared_count);
        Ok(cleared_count)
    }

    // Private helper methods

    fn normalize_model_id_for_filename(&self, model_id: &str) -> String {
        format!("{}.json", model_id.replace('/', "--"))
    }
}

#[derive(Debug, Default)]
pub struct CacheStats {
    pub cached_configs: usize,
    pub total_size_bytes: u64,
}

impl CacheStats {
    pub fn total_size_mb(&self) -> f64 {
        self.total_size_bytes as f64 / (1024.0 * 1024.0)
    }

    pub fn average_size_kb(&self) -> f64 {
        if self.cached_configs == 0 {
            0.0
        } else {
            (self.total_size_bytes as f64 / self.cached_configs as f64) / 1024.0
        }
    }
}
