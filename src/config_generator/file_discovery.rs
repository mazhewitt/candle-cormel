//! File discovery utilities for CoreML packages
//!
//! Handles finding and analyzing .mlpackage and .mlmodelc files

use anyhow::{Error as E, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};
use tracing::debug;

/// Different sources for manifest/model information
#[derive(Debug, Clone)]
pub enum ManifestSource {
    /// Standard .mlmodelc format with metadata.json
    MetadataJson(PathBuf),
    /// Standard .mlpackage format with Manifest.json  
    ManifestJson(PathBuf),
    /// Direct CoreML model file (typo-fixer style .mlpackage)
    ModelFile(PathBuf),
    /// Filename-only detection fallback
    FilenameOnly,
}

pub struct FileDiscovery;

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
        let manifest_source = self.find_manifest_source(package_path)?;

        match manifest_source {
            ManifestSource::MetadataJson(path) | ManifestSource::ManifestJson(path) => {
                debug!("📖 Reading JSON manifest: {}", path.display());
                let manifest_content = std::fs::read_to_string(&path)?;
                let manifest: Value = serde_json::from_str(&manifest_content)?;
                Ok(manifest)
            }
            ManifestSource::ModelFile(path) => {
                debug!("📖 Reading CoreML model file: {}", path.display());
                // For model.mlmodel files, we return an empty JSON array since 
                // the actual parsing will be handled by the CoreMLMetadataExtractor
                Ok(Value::Array(vec![]))
            }
            ManifestSource::FilenameOnly => {
                debug!("📖 Using filename-only detection");
                // Return empty JSON for filename-only detection
                Ok(Value::Array(vec![]))
            }
        }
    }

    /// Get the manifest source type for a package
    pub fn find_manifest_source(&self, package_path: &Path) -> Result<ManifestSource> {
        self.find_manifest_file(package_path)
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

    /// Detect if a package appears to be a typo-fixer style .mlpackage
    pub fn is_typo_fixer_style(&self, package_path: &Path) -> bool {
        package_path.extension().map_or(false, |ext| ext == "mlpackage") &&
        !package_path.join("Manifest.json").exists() &&
        package_path.join("Data/com.apple.CoreML/model.mlmodel").exists()
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
            let filename = package.file_name()
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

    fn find_manifest_file(&self, package_path: &Path) -> Result<ManifestSource> {
        // Priority order: metadata.json (mlmodelc) > Manifest.json (mlpackage) > model.mlmodel (direct) > filename only
        
        if package_path.join("metadata.json").exists() {
            debug!("🔍 Found metadata.json (.mlmodelc format)");
            Ok(ManifestSource::MetadataJson(package_path.join("metadata.json")))
        } else if package_path.join("Manifest.json").exists() {
            debug!("🔍 Found Manifest.json (.mlpackage format)");
            Ok(ManifestSource::ManifestJson(package_path.join("Manifest.json")))
        } else if package_path.join("Data/com.apple.CoreML/model.mlmodel").exists() {
            debug!("🔍 Found direct model.mlmodel (typo-fixer style .mlpackage)");
            Ok(ManifestSource::ModelFile(package_path.join("Data/com.apple.CoreML/model.mlmodel")))
        } else {
            debug!("🔍 No manifest files found, using filename-only detection");
            Ok(ManifestSource::FilenameOnly)
        }
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
        !self.embeddings_packages.is_empty() && 
        !self.transformer_packages.is_empty() && 
        !self.head_packages.is_empty()
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