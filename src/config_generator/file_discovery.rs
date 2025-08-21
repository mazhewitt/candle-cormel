//! File discovery utilities for CoreML packages
//!
//! Handles finding and analyzing .mlpackage and .mlmodelc files

use anyhow::{Error as E, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tracing::debug;

pub struct FileDiscovery;

impl Default for FileDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl FileDiscovery {
    pub fn new() -> Self {
        Self
    }

    /// Find all .mlpackage and .mlmodelc files in a directory
    pub fn find_coreml_packages(&self, model_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut packages = Vec::new();

        for entry in std::fs::read_dir(model_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    // Support both .mlpackage and .mlmodelc formats
                    if ext == "mlpackage" || ext == "mlmodelc" {
                        packages.push(path);
                    }
                }
            }
        }

        // Sort for consistent ordering
        packages.sort();
        Ok(packages)
    }

    /// Read and parse manifest from a CoreML package
    pub fn read_manifest(&self, package_path: &Path) -> Result<Value> {
        debug!("🔎 Reading manifest from: {}", package_path.display());

        // Look for the manifest file inside the package
        // .mlpackage uses Manifest.json, .mlmodelc uses metadata.json
        let manifest_path = self.find_manifest_file(package_path)?;

        // Read and parse the manifest
        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Value = serde_json::from_str(&manifest_content)?;

        Ok(manifest)
    }

    /// Generate component name from package filename
    pub fn infer_component_name(&self, package_path: &Path) -> String {
        let filename = package_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        debug!("🔍 Using filename as component name: {}", filename);

        // Clean up filename for use as component key
        filename.replace(['-', '.'], "_").to_lowercase()
    }

    /// Validate that a directory contains CoreML packages
    pub fn validate_model_directory(&self, model_dir: &Path) -> Result<()> {
        if !model_dir.exists() {
            return Err(E::msg(format!(
                "Model directory does not exist: {}",
                model_dir.display()
            )));
        }

        if !model_dir.is_dir() {
            return Err(E::msg(format!(
                "Path is not a directory: {}",
                model_dir.display()
            )));
        }

        let packages = self.find_coreml_packages(model_dir)?;
        if packages.is_empty() {
            return Err(E::msg(format!(
                "No .mlpackage or .mlmodelc files found in directory: {}",
                model_dir.display()
            )));
        }

        Ok(())
    }

    /// Get summary information about discovered packages
    pub fn analyze_packages(&self, packages: &[PathBuf]) -> PackageAnalysis {
        let mut analysis = PackageAnalysis::default();

        for package in packages {
            let filename = package
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            if filename.contains("embedding") {
                analysis.embeddings_packages.push(package.clone());
            } else if filename.contains("ffn") || filename.contains("transformer") {
                analysis.transformer_packages.push(package.clone());
            } else if filename.contains("head") || filename.contains("lm") {
                analysis.head_packages.push(package.clone());
            } else {
                analysis.other_packages.push(package.clone());
            }
        }

        analysis.total_packages = packages.len();
        analysis
    }

    // Private helper methods

    fn find_manifest_file(&self, package_path: &Path) -> Result<PathBuf> {
        let manifest_path = if package_path.join("Manifest.json").exists() {
            package_path.join("Manifest.json")
        } else if package_path.join("metadata.json").exists() {
            package_path.join("metadata.json")
        } else {
            return Err(E::msg(format!(
                "Neither Manifest.json nor metadata.json found in package: {}",
                package_path.display()
            )));
        };

        Ok(manifest_path)
    }
}

#[derive(Debug, Default)]
pub struct PackageAnalysis {
    pub total_packages: usize,
    pub embeddings_packages: Vec<PathBuf>,
    pub transformer_packages: Vec<PathBuf>,
    pub head_packages: Vec<PathBuf>,
    pub other_packages: Vec<PathBuf>,
}

impl PackageAnalysis {
    /// Check if this looks like a standard transformer architecture
    pub fn is_transformer_like(&self) -> bool {
        !self.embeddings_packages.is_empty()
            && !self.transformer_packages.is_empty()
            && !self.head_packages.is_empty()
    }

    /// Get a summary string of the package distribution
    pub fn summary(&self) -> String {
        format!(
            "Found {} packages: {} embeddings, {} transformer, {} heads, {} other",
            self.total_packages,
            self.embeddings_packages.len(),
            self.transformer_packages.len(),
            self.head_packages.len(),
            self.other_packages.len()
        )
    }
}
