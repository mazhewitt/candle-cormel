//! Modular configuration generator for CoreML models
//!
//! This module provides automatic configuration generation from CoreML .mlpackage files
//! with a clean, modular architecture that's truly model-agnostic.

use crate::cache_manager::CacheManager;
use crate::model_config::{ComponentConfig, ModelConfig, NamingConfig};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

pub mod caching;
pub mod coreml_metadata;
pub mod file_discovery;
pub mod manifest_parser;
pub mod schema_extractor;
pub mod shape_inference;

use caching::ConfigCaching;
use file_discovery::FileDiscovery;
use manifest_parser::ManifestParser;
use schema_extractor::SchemaExtractor;
use shape_inference::ShapeInference;

// Re-export ComponentRole for external use
pub use schema_extractor::ComponentRole;
// Re-export CoreML metadata extractor for testing
pub use coreml_metadata::CoreMLMetadataExtractor;

/// Modular configuration generator for auto-detecting model parameters
pub struct ConfigGenerator {
    file_discovery: FileDiscovery,
    manifest_parser: ManifestParser,
    schema_extractor: SchemaExtractor,
    shape_inference: ShapeInference,
    caching: ConfigCaching,
    metadata_extractor: CoreMLMetadataExtractor,
}

impl ConfigGenerator {
    /// Create a new config generator with all modules initialized
    pub fn new() -> Result<Self> {
        let cache_manager = CacheManager::new()?;
        let caching = ConfigCaching::new(cache_manager);

        Ok(Self {
            file_discovery: FileDiscovery::new(),
            manifest_parser: ManifestParser::new(),
            schema_extractor: SchemaExtractor::new(),
            shape_inference: ShapeInference::new(),
            caching,
            metadata_extractor: coreml_metadata::CoreMLMetadataExtractor::new(),
        })
    }

    /// Generate a config from a downloaded model directory using enhanced metadata-driven detection
    ///
    /// This function inspects .mlpackage files in a directory and generates a complete 
    /// ModelConfig with proper shapes and component configurations, using metadata-driven
    /// component role detection to support both unified and split FFN architectures.
    pub fn generate_config_from_directory_enhanced(
        &self,
        model_dir: &Path,
        model_id: &str,
        model_type: &str,
    ) -> Result<ModelConfig> {
        info!("🔍 Generating config (enhanced) for model: {}", model_id);
        debug!("   Model directory: {}", model_dir.display());
        debug!("   Model type: {}", model_type);

        // Validate and discover CoreML packages
        self.file_discovery.validate_model_directory(model_dir)?;
        let packages = self.file_discovery.find_coreml_packages(model_dir)?;

        info!("📦 Found {} CoreML model files", packages.len());
        for package in &packages {
            debug!(
                "   • {}",
                package.file_name().unwrap_or_default().to_string_lossy()
            );
        }

        // Analyze and parse each package using metadata-driven detection
        let mut components = HashMap::new();
        for package_path in &packages {
            self.process_package_with_metadata_detection(package_path, &mut components)?;
        }

        // Check for required components
        self.validate_required_components(&components)?;

        // Generate final configuration with enhanced shape inference
        let config = self.build_model_config_enhanced(
            model_id,
            model_type,
            model_dir,
            components,
            &packages,
        )?;

        info!(
            "✅ Generated enhanced config for {} with {} components",
            model_id,
            config.components.len()
        );

        // Cache the generated config
        self.caching.cache_config(model_id, &config)?;

        Ok(config)
    }

    /// Generate a config from a downloaded model directory (legacy method)
    ///
    /// This function inspects .mlpackage files in a directory and generates
    /// a complete ModelConfig with proper shapes and component configurations.
    pub fn generate_config_from_directory(
        &self,
        model_dir: &Path,
        model_id: &str,
        model_type: &str,
    ) -> Result<ModelConfig> {
        info!("🔍 Generating config for model: {}", model_id);
        debug!("   Model directory: {}", model_dir.display());
        debug!("   Model type: {}", model_type);

        // Validate and discover CoreML packages
        self.file_discovery.validate_model_directory(model_dir)?;
        let packages = self.file_discovery.find_coreml_packages(model_dir)?;

        info!("📦 Found {} CoreML model files", packages.len());
        for package in &packages {
            debug!(
                "   • {}",
                package.file_name().unwrap_or_default().to_string_lossy()
            );
        }

        // Analyze and parse each package
        let mut components = HashMap::new();
        for package_path in &packages {
            self.process_package(package_path, &mut components)?;
        }

        // Generate final configuration
        let config = self.build_model_config(
            model_id,
            model_type,
            model_dir,
            components,
            &packages,
        )?;

        info!(
            "✅ Generated config for {} with {} components",
            model_id,
            config.components.len()
        );

        // Cache the generated config
        self.caching.cache_config(model_id, &config)?;

        Ok(config)
    }

    /// Load a cached configuration if available
    pub fn load_cached_config(&self, model_id: &str) -> Result<Option<ModelConfig>> {
        self.caching.load_cached_config(model_id)
    }

    /// Check if a cached configuration exists
    pub fn has_cached_config(&self, model_id: &str) -> bool {
        self.caching.has_cached_config(model_id)
    }

    /// Clear cached configuration for a model
    pub fn clear_cached_config(&self, model_id: &str) -> Result<()> {
        self.caching.clear_cached_config(model_id)
    }

    // Private implementation methods

    fn process_package(
        &self,
        package_path: &Path,
        components: &mut HashMap<String, ComponentConfig>,
    ) -> Result<()> {
        let manifest = self.file_discovery.read_manifest(package_path)?;
        let base_component_name = self.file_discovery.infer_component_name(package_path);

        // Parse package into component configurations
        let parsed_components = self.manifest_parser.parse_package(
            package_path,
            &manifest,
            &base_component_name,
        )?;

        // Add all components to the collection
        for (name, config) in parsed_components {
            debug!(
                "📋 Component '{}': inputs={:?} outputs={:?}",
                name,
                config.inputs.keys().collect::<Vec<_>>(),
                config.outputs.keys().collect::<Vec<_>>()
            );
            components.insert(name, config);
        }

        Ok(())
    }

    fn process_package_with_metadata_detection(
        &self,
        package_path: &Path,
        components: &mut HashMap<String, ComponentConfig>,
    ) -> Result<()> {
        // Get manifest source to determine parsing strategy
        let manifest_source = self.file_discovery.find_manifest_source(package_path)?;
        let manifest = self.file_discovery.read_manifest(package_path)?;

        debug!("🔍 Processing package with source: {:?}", manifest_source);

        // Parse package using enhanced method that handles all source types
        let parsed_components = self.manifest_parser.parse_package_enhanced(
            package_path,
            &manifest_source,
            &manifest,
            &self.schema_extractor,
        )?;

        // Add all components to the collection
        for (name, config) in parsed_components {
            debug!(
                "📋 Enhanced component '{}': inputs={:?} outputs={:?}",
                name,
                config.inputs.keys().collect::<Vec<_>>(),
                config.outputs.keys().collect::<Vec<_>>()
            );
            components.insert(name, config);
        }

        Ok(())
    }

    fn validate_required_components(&self, components: &HashMap<String, ComponentConfig>) -> Result<()> {
        let required_components = ["embeddings", "lm_head"];
        let missing_components: Vec<_> = required_components
            .iter()
            .filter(|&comp| !components.contains_key(*comp))
            .collect();

        if !missing_components.is_empty() {
            let found_components: Vec<_> = components.keys().collect();
            debug!("🔍 Found components: {:?}", found_components);
            debug!("❌ Missing required components: {:?}", missing_components);
            
            return Err(anyhow::Error::msg(format!(
                "ModelConfig missing required components: {:?}. Found: {:?}",
                missing_components, found_components
            )));
        }

        // Check for FFN components (either unified or split)
        let has_unified_ffn = components.contains_key("ffn_prefill") && 
                              !components.contains_key("ffn_infer");
        let has_split_ffn = components.contains_key("ffn_prefill") && 
                           components.contains_key("ffn_infer");

        if !has_unified_ffn && !has_split_ffn {
            return Err(anyhow::Error::msg(
                "ModelConfig missing FFN components. Expected either 'ffn_prefill' (unified) or both 'ffn_prefill' and 'ffn_infer' (split)"
            ));
        }

        info!("✅ All required components found: embeddings, FFN, lm_head");
        Ok(())
    }

    fn build_model_config_enhanced(
        &self,
        model_id: &str,
        model_type: &str,
        model_dir: &Path,
        components: HashMap<String, ComponentConfig>,
        packages: &[PathBuf],
    ) -> Result<ModelConfig> {
        // Compute shape configuration using enhanced inference (fails fast on empty tensor metadata)
        let shape_config = self.shape_inference.infer_shapes_with_schema_extractor(&components, &self.schema_extractor)?;

        // Generate naming patterns (generic approach)
        let naming_config = self.generate_naming_config(packages);

        // Determine execution mode using enhanced detection
        let component_list: Vec<(String, ComponentConfig)> = components.into_iter().collect();
        let ffn_execution = self.manifest_parser.infer_execution_mode(&component_list);
        info!("🔧 Detected execution mode: {}", ffn_execution);

        let final_components: HashMap<String, ComponentConfig> = component_list.into_iter().collect();

        Ok(ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some(model_id.to_string()),
                path: Some(model_dir.to_string_lossy().to_string()),
                model_type: model_type.to_string(),
                discovered_at: Some(chrono::Utc::now().to_rfc3339()),
            },
            shapes: shape_config,
            components: final_components,
            naming: naming_config,
            ffn_execution: Some(ffn_execution),
        })
    }

    fn build_model_config(
        &self,
        model_id: &str,
        model_type: &str,
        model_dir: &Path,
        components: HashMap<String, ComponentConfig>,
        packages: &[PathBuf],
    ) -> Result<ModelConfig> {
        // Compute shape configuration from discovered components (fails fast on empty tensor metadata)
        let shape_config = self.shape_inference.infer_shapes(&components)?;

        // Generate naming patterns (generic approach - mostly empty for truly generic models)
        let naming_config = self.generate_naming_config(packages);

        // Determine execution mode
        let component_list: Vec<(String, ComponentConfig)> = components.into_iter().collect();
        let ffn_execution = self.manifest_parser.infer_execution_mode(&component_list);
        info!("🔧 Detected execution mode: {}", ffn_execution);

        let final_components: HashMap<String, ComponentConfig> = component_list.into_iter().collect();

        Ok(ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some(model_id.to_string()),
                path: Some(model_dir.to_string_lossy().to_string()),
                model_type: model_type.to_string(),
                discovered_at: Some(chrono::Utc::now().to_rfc3339()),
            },
            shapes: shape_config,
            components: final_components,
            naming: naming_config,
            ffn_execution: Some(ffn_execution),
        })
    }

    fn generate_naming_config(&self, _packages: &[PathBuf]) -> NamingConfig {
        // For truly generic models, we don't assume specific naming patterns
        // Just return empty patterns since we're being model-agnostic
        NamingConfig {
            embeddings_pattern: None,
            ffn_infer_pattern: None,
            ffn_prefill_pattern: None,
            lm_head_pattern: None,
        }
    }
}

// Re-export the old interface for backward compatibility
impl ConfigGenerator {
    /// Find all .mlpackage files in a directory (legacy interface)
    pub fn find_mlpackage_files(&self, model_dir: &Path) -> Result<Vec<PathBuf>> {
        self.file_discovery.find_coreml_packages(model_dir)
    }

    /// Infer component name from package filename (legacy interface)
    pub fn infer_component_name_from_file(&self, package_path: &Path) -> String {
        self.file_discovery.infer_component_name(package_path)
    }

    /// Analyze a single .mlpackage file (legacy interface)
    pub fn analyze_mlpackage(&self, package_path: &Path) -> Result<ComponentConfig> {
        let manifest = self.file_discovery.read_manifest(package_path)?;
        let inputs = self.schema_extractor.extract_inputs(&manifest)?;
        let outputs = self.schema_extractor.extract_outputs(&manifest)?;

        Ok(ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs,
            outputs,
            functions: Vec::new(),
            input_order: None,
        })
    }

    /// Extract function-based components (legacy interface)
    pub fn extract_function_based_components(
        &self,
        package_path: &Path,
        _base_config: &ComponentConfig,
    ) -> Result<Option<HashMap<String, ComponentConfig>>> {
        let manifest = self.file_discovery.read_manifest(package_path)?;
        let base_component_name = self.file_discovery.infer_component_name(package_path);

        let parsed_components = self.manifest_parser.parse_package(
            package_path,
            &manifest,
            &base_component_name,
        )?;

        if parsed_components.len() > 1 {
            // Has multiple function-based components
            let function_components: HashMap<String, ComponentConfig> = 
                parsed_components.into_iter().collect();
            Ok(Some(function_components))
        } else {
            // Single component, no functions
            Ok(None)
        }
    }

    /// Parse tensor configurations from schema (legacy interface)
    pub fn parse_tensor_configs_from_schema(&self, schema: &[serde_json::Value]) -> Result<HashMap<String, crate::model_config::TensorConfig>> {
        self.schema_extractor.parse_tensor_configs(schema)
    }

    /// Compute shape info (legacy interface) 
    /// Returns an error if components have insufficient tensor metadata
    pub fn compute_shape_info_generic(&self, components: &HashMap<String, ComponentConfig>) -> Result<crate::model_config::ShapeConfig> {
        self.shape_inference.infer_shapes(components)
    }

    /// Generate naming config (legacy interface)  
    pub fn generate_naming_config_generic(&self, packages: &[PathBuf]) -> NamingConfig {
        self.generate_naming_config(packages)
    }

    /// Determine execution mode (legacy interface)
    pub fn determine_execution_mode_generic(&self, components: &HashMap<String, ComponentConfig>) -> String {
        let component_list: Vec<(String, ComponentConfig)> = components.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        self.manifest_parser.infer_execution_mode(&component_list)
    }

    /// Cache generated config (legacy interface)
    pub fn cache_generated_config(&self, model_id: &str, config: &ModelConfig) -> Result<()> {
        self.caching.cache_config(model_id, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Create a mock .mlpackage directory structure for testing
    fn create_mock_mlpackage(temp_dir: &Path, name: &str) -> Result<PathBuf> {
        let package_path = temp_dir.join(format!("{}.mlpackage", name));
        std::fs::create_dir_all(&package_path)?;

        // Create a minimal manifest.json
        let manifest = serde_json::json!({
            "fileFormatVersion": "1.0.0",
            "itemInfoEntries": [
                {
                    "path": "model.mlmodel",
                    "digestType": "SHA256"
                }
            ]
        });

        let manifest_path = package_path.join("Manifest.json");
        std::fs::write(manifest_path, serde_json::to_string_pretty(&manifest)?)?;

        // Create a minimal model file (empty for testing)
        let model_path = package_path.join("model.mlmodel");
        std::fs::write(model_path, b"mock model data")?;

        Ok(package_path)
    }

    #[test]
    fn test_modular_config_generator_creation() -> Result<()> {
        let generator = ConfigGenerator::new()?;
        
        // Should have all modules initialized
        assert!(!generator.caching.has_cached_config("nonexistent")); // Should return false but not crash
        
        Ok(())
    }

    #[test]
    fn test_modular_file_discovery() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let generator = ConfigGenerator::new()?;

        // Create some mock packages
        create_mock_mlpackage(temp_dir.path(), "embeddings")?;
        create_mock_mlpackage(temp_dir.path(), "transformer")?;
        create_mock_mlpackage(temp_dir.path(), "head")?;

        let packages = generator.file_discovery.find_coreml_packages(temp_dir.path())?;
        
        assert_eq!(packages.len(), 3);
        assert!(packages.iter().any(|p| p.file_name().unwrap().to_string_lossy().contains("embeddings")));
        assert!(packages.iter().any(|p| p.file_name().unwrap().to_string_lossy().contains("transformer")));
        assert!(packages.iter().any(|p| p.file_name().unwrap().to_string_lossy().contains("head")));

        Ok(())
    }

    // Note: Legacy generation on mock/placeholder packages intentionally fails under strict validation.
    // Tests that assumed permissive behavior have been removed to keep assertions strict.

    #[test]
    fn test_enhanced_parsing_falls_back_to_filename_when_metadata_empty() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let generator = ConfigGenerator::new()?;

        // Create a mock package named "embeddings" with Manifest.json and an empty model file.
        // This simulates environments where metadata extraction yields empty inputs/outputs.
        let pkg_path = create_mock_mlpackage(temp_dir.path(), "embeddings")?;

        let mut components = std::collections::HashMap::new();
        generator
            .process_package_with_metadata_detection(&pkg_path, &mut components)
            .expect("enhanced parsing should not fail");

        // Expect that we classified the component as "embeddings" via filename fallback
        assert!(components.contains_key("embeddings"), "expected 'embeddings' component, got: {:?}", components.keys().collect::<Vec<_>>());
        Ok(())
    }
}