//! Cache Management for CoreML Models and Runtime Data
//!
//! This module provides centralized cache management for:
//! 1. Downloaded models from HuggingFace
//! 2. CoreML runtime caches (e5rt)
//! 3. Temporary build artifacts
//!
//! The goal is to provide better control over cache locations and cleanup.

use anyhow::Result;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

/// Simple title case conversion for directory names
trait ToTitleCase {
    fn to_title_case(&self) -> String;
}

impl ToTitleCase for str {
    fn to_title_case(&self) -> String {
        self.split(|c: char| c.is_whitespace() || c == '-' || c == '_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(target_os = "macos")]
use objc2_foundation::NSBundle;

/// Central cache manager for candle-coreml
pub struct CacheManager {
    /// Base cache directory (defaults to ~/.cache/candle-coreml)
    cache_base: PathBuf,
    /// Current bundle identifier (affects CoreML cache locations)
    bundle_id: Option<String>,
}

impl CacheManager {
    /// Create a new cache manager with default settings
    pub fn new() -> Result<Self> {
        let cache_base = Self::default_cache_dir()?;
        let bundle_id = Self::get_current_bundle_identifier();

        std::fs::create_dir_all(&cache_base)?;

        let manager = Self {
            cache_base,
            bundle_id,
        };

        // Initialize the unified cache structure
        manager.initialize_cache_structure()?;

        info!("🗂️  Cache manager initialized");
        info!("   Base directory: {}", manager.cache_base.display());
        if let Some(ref id) = manager.bundle_id {
            info!("   Bundle identifier: {}", id);
        } else {
            warn!("   Bundle identifier: nil (command-line process)");
        }

        Ok(manager)
    }

    /// Get the default cache directory
    fn default_cache_dir() -> Result<PathBuf> {
        if let Some(cache_dir) = dirs::cache_dir() {
            Ok(cache_dir.join("candle-coreml"))
        } else {
            // Fallback for systems without standard cache dir
            let home = dirs::home_dir()
                .ok_or_else(|| anyhow::Error::msg("Cannot determine home directory"))?;
            Ok(home.join(".cache").join("candle-coreml"))
        }
    }

    /// Get the current bundle identifier using NSBundle (macOS only)
    #[cfg(target_os = "macos")]
    fn get_current_bundle_identifier() -> Option<String> {
        unsafe {
            let main_bundle = NSBundle::mainBundle();
            let bundle_id = main_bundle.bundleIdentifier();

            bundle_id.map(|id| {
                let bundle_str = id.to_string();
                debug!("📱 Current bundle identifier: {}", bundle_str);
                bundle_str
            })
        }
    }

    /// Get the current bundle identifier (non-macOS fallback)
    #[cfg(not(target_os = "macos"))]
    fn get_current_bundle_identifier() -> Option<String> {
        None
    }

    /// Get models cache directory
    pub fn models_dir(&self) -> PathBuf {
        self.cache_base.join("models")
    }

    /// Get configs cache directory  
    pub fn configs_dir(&self) -> PathBuf {
        self.cache_base.join("configs")
    }

    /// Get CoreML runtime cache directory
    pub fn coreml_runtime_dir(&self) -> PathBuf {
        self.cache_base.join("coreml-runtime")
    }

    /// Get temp directory for build artifacts
    pub fn temp_dir(&self) -> PathBuf {
        self.cache_base.join("temp")
    }

    /// Initialize the unified cache directory structure
    pub fn initialize_cache_structure(&self) -> Result<()> {
        let directories = [
            ("models", "Downloaded models from HuggingFace"),
            ("configs", "Auto-generated model configurations"),
            ("coreml-runtime", "CoreML runtime session data"),
            ("temp", "Temporary build and processing artifacts"),
        ];

        for (dir_name, description) in &directories {
            let dir_path = self.cache_base.join(dir_name);
            std::fs::create_dir_all(&dir_path)?;

            // Create a README file in each directory
            let readme_path = dir_path.join("README.md");
            if !readme_path.exists() {
                let readme_content = format!(
                    "# {} Cache Directory\n\n{}\n\nThis directory is managed by candle-coreml's CacheManager.\n",
                    dir_name.replace('-', " ").to_title_case(),
                    description
                );
                std::fs::write(readme_path, readme_content)?;
            }
        }

        // Create main cache directory README
        let main_readme = self.cache_base.join("README.md");
        if !main_readme.exists() {
            let main_content = format!(
                r#"# candle-coreml Cache Directory

This directory contains cached data for the candle-coreml library:

## Directory Structure

- `models/` - Downloaded models from HuggingFace
- `configs/` - Auto-generated model configurations  
- `coreml-runtime/` - CoreML runtime session data
- `temp/` - Temporary build and processing artifacts

## Management

Use the candle-coreml CacheManager API or cleanup scripts to manage this cache:

```bash
# Enhanced cleanup script
./cleanup_coreml_caches_enhanced.sh

# Rust API
use candle_coreml::CacheManager;
let manager = CacheManager::new()?;
manager.cleanup_old_caches(7)?; // Clean files older than 7 days
```

## Bundle Identifier

Current bundle identifier: {:?}

---
Generated by candle-coreml v{} at {}
"#,
                self.bundle_id,
                env!("CARGO_PKG_VERSION"),
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
            );
            std::fs::write(main_readme, main_content)?;
        }

        info!(
            "✅ Cache directory structure initialized at {}",
            self.cache_base.display()
        );
        Ok(())
    }

    /// Get the current bundle identifier
    pub fn bundle_identifier(&self) -> Option<&str> {
        self.bundle_id.as_deref()
    }

    /// Get the base cache directory path
    pub fn cache_base(&self) -> &Path {
        &self.cache_base
    }

    /// Report potential CoreML cache locations based on bundle ID
    pub fn report_coreml_cache_locations(&self) -> Vec<PathBuf> {
        let mut locations = Vec::new();

        // Standard system cache location
        if let Some(cache_dir) = dirs::cache_dir() {
            if let Some(bundle_id) = &self.bundle_id {
                // CoreML typically creates: ~/Library/Caches/{bundle_id}/com.apple.e5rt.e5bundlecache
                locations.push(
                    cache_dir
                        .join(bundle_id)
                        .join("com.apple.e5rt.e5bundlecache"),
                );
            }

            // Also check for process-name based caches (common pattern)
            let process_name = std::env::current_exe()
                .ok()
                .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
                .unwrap_or_else(|| "unknown".to_string());

            // Pattern: {process_name}-{hash}/com.apple.e5rt.e5bundlecache
            if let Ok(entries) = std::fs::read_dir(&cache_dir) {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if name.starts_with(&format!("{process_name}-")) {
                        locations.push(entry.path().join("com.apple.e5rt.e5bundlecache"));
                    }
                }
            }
        }

        locations
    }

    /// Clean up old cache entries based on policy
    pub fn cleanup_old_caches(&self, max_age_days: u64) -> Result<()> {
        info!("🧹 Starting cache cleanup (max age: {} days)", max_age_days);

        let cutoff_time = std::time::SystemTime::now()
            - std::time::Duration::from_secs(max_age_days * 24 * 60 * 60);

        // Clean up temp directory
        self.cleanup_directory(&self.temp_dir(), cutoff_time)?;

        // Report CoreML cache locations (but don't clean them - Apple manages these)
        let coreml_locations = self.report_coreml_cache_locations();
        if !coreml_locations.is_empty() {
            info!(
                "📍 Found {} potential CoreML cache locations:",
                coreml_locations.len()
            );
            for location in &coreml_locations {
                if location.exists() {
                    info!("   • {}", location.display());
                }
            }
            info!("   Note: CoreML caches are managed by Apple's system");
        }

        Ok(())
    }

    /// Find all candle-coreml related cache directories (for enhanced cleanup)
    pub fn find_all_candle_coreml_caches(&self) -> Result<Vec<(PathBuf, u64)>> {
        let mut caches = Vec::new();

        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::Error::msg("Cannot determine cache directory"))?;

        // Patterns to search for based on investigation
        let patterns = [
            "candle_coreml-*",
            "candle-coreml-*",
            "integration_tests-*",
            "performance_regression_tests-*",
            "qwen_tests-*",
            "typo_fixer_test*",
            "typo_fixer_tests-*",
            "flex_pipeline_tests-*",
            "builder_tests-*",
            "tensor_regression_tests-*",
            "utils_tests-*",
            "bundle_id_*",
        ];

        for pattern in &patterns {
            if let Ok(output) = std::process::Command::new("find")
                .args([
                    &cache_dir.to_string_lossy(),
                    "-maxdepth",
                    "1",
                    "-name",
                    pattern,
                    "-type",
                    "d",
                ])
                .output()
            {
                let entries_str = String::from_utf8_lossy(&output.stdout);
                for line in entries_str.lines() {
                    let entry = PathBuf::from(line.trim());
                    if entry.is_dir() {
                        // Check if it contains CoreML-specific files
                        let has_coreml = entry.join("com.apple.e5rt.e5bundlecache").exists()
                            || entry.join(".coreml_cache").exists()
                            || entry.to_string_lossy().contains("coreml");

                        if has_coreml {
                            let size = self.get_directory_size(&entry)?;
                            caches.push((entry, size));
                        }
                    }
                }
            }
        }

        // Also find standalone e5rt caches
        if let Ok(output) = std::process::Command::new("find")
            .args([
                &cache_dir.to_string_lossy(),
                "-maxdepth",
                "1",
                "-name",
                "*e5rt*",
                "-type",
                "d",
            ])
            .output()
        {
            let entries_str = String::from_utf8_lossy(&output.stdout);
            for line in entries_str.lines() {
                let entry = PathBuf::from(line.trim());
                if entry.is_dir() && !caches.iter().any(|(path, _)| path == &entry) {
                    let size = self.get_directory_size(&entry)?;
                    caches.push((entry, size));
                }
            }
        }

        // Sort by size (largest first)
        caches.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(caches)
    }

    /// Get the size of a directory in bytes
    fn get_directory_size(&self, path: &Path) -> Result<u64> {
        let mut total_size = 0;

        fn visit_dir(dir: &Path, total: &mut u64) -> Result<()> {
            if dir.is_dir() {
                for entry in std::fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        visit_dir(&path, total)?;
                    } else {
                        *total += entry.metadata()?.len();
                    }
                }
            }
            Ok(())
        }

        visit_dir(path, &mut total_size)?;
        Ok(total_size)
    }

    /// Remove specific cache directories with safety checks
    pub fn remove_cache_directories(
        &self,
        paths: &[PathBuf],
        dry_run: bool,
    ) -> Result<(usize, u64)> {
        let mut removed_count = 0;
        let mut freed_bytes = 0;

        for path in paths {
            if !path.exists() {
                continue;
            }

            // Safety check: ensure we're only removing cache directories
            if let Some(cache_dir) = dirs::cache_dir() {
                if !path.starts_with(&cache_dir) {
                    warn!("Skipping path outside cache directory: {}", path.display());
                    continue;
                }
            }

            // Additional safety: don't remove important system directories
            let path_str = path.to_string_lossy();
            if path_str.contains("System")
                || path_str.contains("Applications")
                || path_str.contains("/usr/")
                || path_str.contains("/bin/")
            {
                warn!("Skipping system path: {}", path.display());
                continue;
            }

            let size = self.get_directory_size(path).unwrap_or(0);

            if dry_run {
                info!("Would remove: {} ({} bytes)", path.display(), size);
            } else {
                info!("Removing: {}", path.display());
                match std::fs::remove_dir_all(path) {
                    Ok(()) => {
                        removed_count += 1;
                        freed_bytes += size;
                        debug!("✅ Removed: {}", path.display());
                    }
                    Err(e) => {
                        warn!("⚠️  Failed to remove {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok((removed_count, freed_bytes))
    }

    /// Clean up a specific directory based on age
    fn cleanup_directory(&self, dir: &Path, cutoff_time: std::time::SystemTime) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(dir)?;
        let mut cleaned_count = 0;

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata()?;

            if let Ok(modified) = metadata.modified() {
                if modified < cutoff_time {
                    let path = entry.path();
                    if path.is_dir() {
                        std::fs::remove_dir_all(&path)?;
                    } else {
                        std::fs::remove_file(&path)?;
                    }
                    cleaned_count += 1;
                    debug!("🗑️  Cleaned: {}", path.display());
                }
            }
        }

        if cleaned_count > 0 {
            info!("✅ Cleaned {} items from {}", cleaned_count, dir.display());
        }

        Ok(())
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize cache manager")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager_creation() {
        let manager = CacheManager::new().expect("Failed to create cache manager");

        // Test directory creation
        assert!(manager.models_dir().parent().unwrap().exists());

        // Test bundle identifier detection
        println!("Bundle ID: {:?}", manager.bundle_identifier());
    }

    #[test]
    fn test_coreml_cache_location_detection() {
        let manager = CacheManager::new().expect("Failed to create cache manager");
        let locations = manager.report_coreml_cache_locations();

        println!("Potential CoreML cache locations:");
        for location in &locations {
            println!("  {}", location.display());
        }
    }

    #[test]
    fn test_find_all_candle_coreml_caches() {
        let manager = CacheManager::new().expect("Failed to create cache manager");

        match manager.find_all_candle_coreml_caches() {
            Ok(caches) => {
                println!("Found {} candle-coreml cache directories:", caches.len());
                for (path, size) in &caches {
                    let size_mb = *size as f64 / (1024.0 * 1024.0);
                    println!("  {} ({:.1} MB)", path.display(), size_mb);
                }

                let total_size: u64 = caches.iter().map(|(_, size)| size).sum();
                let total_gb = total_size as f64 / (1024.0 * 1024.0 * 1024.0);
                println!("Total size: {total_gb:.2} GB");
            }
            Err(e) => {
                println!("Error finding caches: {e}");
            }
        }
    }
}
