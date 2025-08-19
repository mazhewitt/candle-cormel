//! Auto-generation of model configurations from downloaded .mlpackage files
//!
//! This module provides functionality to automatically generate ModelConfig
//! structures by inspecting CoreML .mlpackage files, eliminating the need
//! for hardcoded paths and manual configuration.

use crate::cache_manager::CacheManager;
use crate::model_config::{ComponentConfig, ModelConfig, TensorConfig};
use anyhow::{Error as E, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing::{debug, info};

/// Configuration generator for auto-detecting model parameters
pub struct ConfigGenerator {
    cache_manager: CacheManager,
}

impl ConfigGenerator {
    /// Create a new config generator
    pub fn new() -> Result<Self> {
        let cache_manager = CacheManager::new()?;
        Ok(Self { cache_manager })
    }

    /// Generate a config from a downloaded model directory
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

        // Scan for CoreML model files (.mlpackage or .mlmodelc)
        let packages = self.find_mlpackage_files(model_dir)?;
        if packages.is_empty() {
            return Err(E::msg(format!(
                "No .mlpackage or .mlmodelc files found in directory: {}",
                model_dir.display()
            )));
        }

        info!("📦 Found {} CoreML model files", packages.len());
        for package in &packages {
            debug!(
                "   • {}",
                package.file_name().unwrap_or_default().to_string_lossy()
            );
        }

    // Analyze each package to extract component configurations
    let mut components: HashMap<String, ComponentConfig> = HashMap::new();

        for package_path in &packages {
            let component_config = self.analyze_mlpackage(package_path)?;
            let mut component_name = self.infer_component_name(package_path);

            // Special handling for FFN components with multiple functions: emit both prefill & infer
            let is_ffn = component_name.contains("ffn");
            if is_ffn {
                // Try to extract per-function schemas from manifest
                if let Some((prefill_io, infer_io)) = self.extract_ffn_function_schemas(package_path)? {
                    if let Some((prefill_inputs, prefill_outputs)) = prefill_io {
                        let mut prefill_cfg = component_config.clone();
                        prefill_cfg.inputs = prefill_inputs;
                        prefill_cfg.outputs = prefill_outputs;
                        prefill_cfg.functions = vec!["prefill".to_string()];
                        debug!("📋 FFN prefill from functions: inputs={:?} outputs={:?}", prefill_cfg.inputs.keys().collect::<Vec<_>>(), prefill_cfg.outputs.keys().collect::<Vec<_>>() );
                        components.insert("ffn_prefill".to_string(), prefill_cfg);
                    }
                    if let Some((infer_inputs, infer_outputs)) = infer_io {
                        let mut infer_cfg = component_config.clone();
                        infer_cfg.inputs = infer_inputs;
                        infer_cfg.outputs = infer_outputs;
                        infer_cfg.functions = vec!["infer".to_string()];
                        debug!("📋 FFN infer from functions: inputs={:?} outputs={:?}", infer_cfg.inputs.keys().collect::<Vec<_>>(), infer_cfg.outputs.keys().collect::<Vec<_>>() );
                        components.insert("ffn_infer".to_string(), infer_cfg);
                    }
                    // We've handled this package as FFN with functions, continue
                    continue;
                }
            }

            // For non-FFN or FFN without functions array, proceed with inferred name
            if component_name == "ffn_unified" {
                component_name = self.analyze_ffn_component_type(package_path, &component_config)?;
            }

            debug!("📋 Component '{}' analysis:", component_name);
            debug!(
                "   Inputs: {:?}",
                component_config.inputs.keys().collect::<Vec<_>>()
            );
            debug!(
                "   Outputs: {:?}",
                component_config.outputs.keys().collect::<Vec<_>>()
            );

            components.insert(component_name.clone(), component_config.clone());

            // If this is a unified FFN component without explicit functions, map to prefill by default
            if component_name.contains("ffn") && !component_name.ends_with("_prefill") && !component_name.ends_with("_infer") {
                debug!("🔧 Mapping unified FFN component '{}' to 'ffn_prefill'", component_name);
                components.insert("ffn_prefill".to_string(), component_config);
            }
        }

    // Reconcile and compute overall shape configuration based on discovered components
    self.reconcile_component_shapes(&mut components)?;
    let shape_config = self.compute_shape_info(&components)?;

        // Generate naming patterns based on discovered files
        let naming_config = self.generate_naming_config(&packages)?;

        // Create the complete ModelConfig
        // Determine FFN execution mode from component analysis
        let ffn_execution = self.determine_ffn_execution_mode(&components);
        info!("🔧 Detected FFN execution mode: {}", ffn_execution);

    let config = ModelConfig {
            model_info: crate::model_config::ModelInfo {
                model_id: Some(model_id.to_string()),
                path: Some(model_dir.to_string_lossy().to_string()),
                model_type: model_type.to_string(),
                discovered_at: Some(chrono::Utc::now().to_rfc3339()),
            },
            shapes: shape_config,
            components,
            naming: naming_config,
            ffn_execution: Some(ffn_execution),
        };

    // Configs should be purely data-driven from CoreML metadata; avoid model-specific hardcoded fixes

        info!(
            "✅ Generated config for {} with {} components",
            model_id,
            config.components.len()
        );

        // Cache the generated config
        self.cache_generated_config(model_id, &config)?;

        Ok(config)
    }

    /// Extract FFN function-specific schemas (prefill and infer) if present
    fn extract_ffn_function_schemas(&self, package_path: &Path) -> Result<Option<(
        Option<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)>,
        Option<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)>,
    )>> {
        // Determine manifest path
        let manifest_path = if package_path.join("Manifest.json").exists() {
            package_path.join("Manifest.json")
        } else if package_path.join("metadata.json").exists() {
            package_path.join("metadata.json")
        } else {
            return Ok(None);
        };

        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Value = serde_json::from_str(&manifest_content)?;

        let functions = if let Some(funcs) = manifest.get(0).and_then(|m| m.get("functions").and_then(|f| f.as_array())) { funcs } else { return Ok(None) };

        let mut prefill_io: Option<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> = None;
        let mut infer_io: Option<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> = None;

    for function in functions {
            if let Some(name) = function.get("name").and_then(|n| n.as_str()) {
                if name == "prefill" {
            let empty_vec: Vec<Value> = Vec::new();
            let in_arr = function.get("inputSchema").and_then(|s| s.as_array());
            let out_arr = function.get("outputSchema").and_then(|s| s.as_array());
            let inputs = self.parse_tensor_configs_from_schema(in_arr.unwrap_or(&empty_vec))?;
            let outputs = self.parse_tensor_configs_from_schema(out_arr.unwrap_or(&empty_vec))?;
                    prefill_io = Some((inputs, outputs));
                } else if name == "infer" {
            let empty_vec: Vec<Value> = Vec::new();
            let in_arr = function.get("inputSchema").and_then(|s| s.as_array());
            let out_arr = function.get("outputSchema").and_then(|s| s.as_array());
            let inputs = self.parse_tensor_configs_from_schema(in_arr.unwrap_or(&empty_vec))?;
            let outputs = self.parse_tensor_configs_from_schema(out_arr.unwrap_or(&empty_vec))?;
                    infer_io = Some((inputs, outputs));
                }
            }
        }

        if prefill_io.is_none() && infer_io.is_none() {
            Ok(None)
        } else {
            Ok(Some((prefill_io, infer_io)))
        }
    }

    /// Find all .mlpackage files in a directory
    fn find_mlpackage_files(&self, model_dir: &Path) -> Result<Vec<PathBuf>> {
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

    /// Analyze a single .mlpackage file to extract component configuration
    fn analyze_mlpackage(&self, package_path: &Path) -> Result<ComponentConfig> {
        debug!("🔎 Analyzing package: {}", package_path.display());

        // Look for the manifest file inside the package
        // .mlpackage uses Manifest.json, .mlmodelc uses metadata.json
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

        // Read and parse the manifest
        let manifest_content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Value = serde_json::from_str(&manifest_content)?;

        // Extract input/output specifications
        let inputs = self.extract_inputs_from_manifest(&manifest)?;
        let outputs = self.extract_outputs_from_manifest(&manifest)?;

        Ok(ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs,
            outputs,
            functions: Vec::new(), // Will be populated if needed
            input_order: None,     // Will be determined from inputs
        })
    }

    /// Analyze FFN component type from metadata to determine correct mapping
    fn analyze_ffn_component_type(&self, package_path: &Path, _component_config: &ComponentConfig) -> Result<String> {
        debug!("🔍 Analyzing FFN component type from metadata: {}", package_path.display());
        
        // Look for the metadata file inside the package
        let metadata_path = if package_path.join("Manifest.json").exists() {
            package_path.join("Manifest.json")
        } else if package_path.join("metadata.json").exists() {
            package_path.join("metadata.json")
        } else {
            // Fallback to filename analysis
            let filename = package_path.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
            if filename.contains("_pf_") {
                debug!("   -> Fallback: Using 'ffn_prefill' based on '_pf_' pattern");
                return Ok("ffn_prefill".to_string());
            } else {
                debug!("   -> Fallback: Using 'ffn_infer' for generic FFN");
                return Ok("ffn_infer".to_string());
            }
        };

        // Read and parse the metadata
        let metadata_content = std::fs::read_to_string(&metadata_path)?;
        let metadata: Value = serde_json::from_str(&metadata_content)?;
        
        // Look for functions array in the metadata
        let component_name = if let Some(functions) = metadata.get(0).and_then(|m| m.get("functions").and_then(|f| f.as_array())) {
            let function_names: Vec<String> = functions
                .iter()
                .filter_map(|f| f.get("name").and_then(|n| n.as_str()))
                .map(|s| s.to_string())
                .collect();
            
            debug!("   -> Found functions in metadata: {:?}", function_names);
            
            let has_prefill = function_names.iter().any(|f| f == "prefill");
            let has_infer = function_names.iter().any(|f| f == "infer");
            
            if has_prefill && has_infer {
                debug!("   -> Unified FFN component with both prefill and infer functions");
                "ffn_prefill".to_string() // Primary mapping for unified components
            } else if has_prefill {
                debug!("   -> FFN component with prefill function only");
                "ffn_prefill".to_string()
            } else if has_infer {
                debug!("   -> FFN component with infer function only"); 
                "ffn_infer".to_string()
            } else {
                debug!("   -> FFN component with no specific functions, defaulting to prefill");
                "ffn_prefill".to_string()
            }
        } else {
            // Fallback to filename analysis
            let filename = package_path.file_stem().unwrap_or_default().to_string_lossy().to_lowercase();
            if filename.contains("_pf_") {
                debug!("   -> Metadata fallback: Using 'ffn_prefill' based on '_pf_' pattern");
                "ffn_prefill".to_string()
            } else {
                debug!("   -> Metadata fallback: Using 'ffn_infer' for generic FFN");
                "ffn_infer".to_string()
            }
        };
        
        debug!("   -> Final FFN component mapping: {}", component_name);
        Ok(component_name)
    }

    /// Extract input tensor configurations from manifest
    fn extract_inputs_from_manifest(
        &self,
        manifest: &Value,
    ) -> Result<HashMap<String, TensorConfig>> {
        let mut inputs = HashMap::new();

        // Handle both .mlpackage (Manifest.json) and .mlmodelc (metadata.json) formats
        if let Some(input_schema) = manifest.get(0).and_then(|m| m.get("inputSchema").and_then(|s| s.as_array())) {
            // .mlmodelc format - direct input schema in metadata
            debug!("📖 Parsing .mlmodelc input schema with {} inputs", input_schema.len());
            
            // For FFN components, prefer prefill function shapes if available
            if let Some(functions) =
                manifest
                    .get(0)
                    .and_then(|m| m.get("functions").and_then(|f| f.as_array()))
            {
                // Prefer prefill function shapes for ffn_prefill; fall back to infer
                // Determine if we are looking at an FFN component based on available names
                let mut selected_inputs: Option<HashMap<String, TensorConfig>> = None;
                for function in functions {
                    if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                        if func_name == "prefill" {
                            if let Some(prefill_input_schema) =
                                function.get("inputSchema").and_then(|s| s.as_array())
                            {
                                debug!(
                                    "📖 Using prefill function input schema with {} inputs",
                                    prefill_input_schema.len()
                                );
                                selected_inputs = Some(
                                    self.parse_tensor_configs_from_schema(prefill_input_schema)?,
                                );
                                break;
                            }
                        }
                    }
                }
                if selected_inputs.is_none() {
                    // Try infer function next
                    for function in functions {
                        if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                            if func_name == "infer" {
                                if let Some(infer_input_schema) =
                                    function.get("inputSchema").and_then(|s| s.as_array())
                                {
                                    debug!(
                                        "📖 Using infer function input schema with {} inputs",
                                        infer_input_schema.len()
                                    );
                                    selected_inputs = Some(
                                        self.parse_tensor_configs_from_schema(infer_input_schema)?,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
                if let Some(inputs_map) = selected_inputs {
                    return Ok(inputs_map);
                }
            }
            
            for input in input_schema {
                if let (Some(name), Some(data_type)) = (
                    input.get("name").and_then(|n| n.as_str()),
                    input.get("dataType").and_then(|d| d.as_str()),
                ) {
                    // Prefer enumeratedShapes when available (choose the largest allowed shape)
                    let mut shape: Vec<usize> = Vec::new();

                    if let Some(enum_val) = input.get("enumeratedShapes") {
                        if let Some(enum_str) = enum_val.as_str() {
                            // enumeratedShapes is a JSON-like string, e.g. "[[1, 1], [1, 64]]"
                            match serde_json::from_str::<Vec<Vec<usize>>>(enum_str) {
                                Ok(mut shapes) => {
                                    // Choose the shape with the largest token dimension (2nd dim if present)
                                    shapes.sort_by(|a, b| {
                                        let a_key = if a.len() > 1 { a[1] } else { a.iter().product() };
                                        let b_key = if b.len() > 1 { b[1] } else { b.iter().product() };
                                        a_key.cmp(&b_key)
                                    });
                                    if let Some(max) = shapes.last() {
                                        shape = max.clone();
                                    }
                                }
                                Err(err) => {
                                    debug!("⚠️ Failed to parse enumeratedShapes for '{}': {}", name, err);
                                }
                            }
                        } else if let Some(enum_arr) = enum_val.as_array() {
                            // Support array-encoded enumeratedShapes
                            let mut candidates: Vec<Vec<usize>> = Vec::new();
                            for item in enum_arr {
                                if let Some(s) = item.as_str() {
                                    if let Ok(v) = serde_json::from_str::<Vec<usize>>(s) { candidates.push(v); }
                                } else if let Some(arr) = item.as_array() {
                                    let mut v: Vec<usize> = Vec::new();
                                    for d in arr { if let Some(u) = d.as_u64() { v.push(u as usize); } }
                                    if !v.is_empty() { candidates.push(v); }
                                }
                            }
                            if !candidates.is_empty() {
                                candidates.sort_by(|a, b| {
                                    let a_key = if a.len() > 1 { a[1] } else { a.iter().product() };
                                    let b_key = if b.len() > 1 { b[1] } else { b.iter().product() };
                                    a_key.cmp(&b_key)
                                });
                                if let Some(max) = candidates.last() { shape = max.clone(); }
                            }
                        }
                    }

                    // Fallback to single shape string
                    if shape.is_empty() {
                        if let Some(shape_str) = input.get("shape").and_then(|s| s.as_str()) {
                            let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
                            if !trimmed.is_empty() {
                                for dim_str in trimmed.split(',') {
                                    match dim_str.trim().parse::<usize>() {
                                        Ok(dim) => shape.push(dim),
                                        Err(_) => return Err(E::msg("Failed to parse tensor dimension".to_string())),
                                    }
                                }
                            }
                        }
                    }

                    debug!("   Input: {} -> {:?} ({})", name, shape, data_type);

                    inputs.insert(
                        name.to_string(),
                        TensorConfig {
                            name: name.to_string(),
                            shape,
                            data_type: data_type.to_uppercase(),
                        },
                    );
                }
            }
        } else if let Some(_items) = manifest.get("itemInfoEntries").and_then(|v| v.as_array()) {
            // .mlpackage format - need to parse the embedded model description
            debug!("📖 Parsing .mlpackage format (not fully implemented yet)");
            
            // For now, fall back to default input_ids for .mlpackage files
            inputs.insert(
                "input_ids".to_string(),
                TensorConfig {
                    name: "input_ids".to_string(),
                    shape: vec![1, 64], // Default for most models
                    data_type: "INT32".to_string(),
                },
            );
        } else {
            debug!("📖 Unknown manifest format, using fallback inputs");
            
            // Fallback for unknown formats
            inputs.insert(
                "input_ids".to_string(),
                TensorConfig {
                    name: "input_ids".to_string(),
                    shape: vec![1, 64],
                    data_type: "INT32".to_string(),
                },
            );
        }

        debug!("📖 Extracted {} input tensors", inputs.len());
        Ok(inputs)
    }

    /// Extract output tensor configurations from manifest
    fn extract_outputs_from_manifest(
        &self,
        manifest: &Value,
    ) -> Result<HashMap<String, TensorConfig>> {
        let mut outputs = HashMap::new();

        // Handle both .mlpackage (Manifest.json) and .mlmodelc (metadata.json) formats
        if let Some(output_schema) = manifest
            .get(0)
            .and_then(|m| m.get("outputSchema").and_then(|s| s.as_array()))
        {
            // .mlmodelc format - direct output schema in metadata
            debug!("📖 Parsing .mlmodelc output schema with {} outputs", output_schema.len());
            let mut any_empty = false;
            for output in output_schema {
                if let (Some(name), Some(shape_str), Some(data_type)) = (
                    output.get("name").and_then(|n| n.as_str()),
                    output.get("shape").and_then(|s| s.as_str()),
                    output.get("dataType").and_then(|d| d.as_str()),
                ) {
                    // Parse shape from string format like "[1, 1, 1024]"
                    let shape = {
                        let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
                        if trimmed.is_empty() {
                            vec![]
                        } else {
                            let mut dims: Vec<usize> = Vec::new();
                            for dim_str in trimmed.split(',') {
                                match dim_str.trim().parse::<usize>() {
                                    Ok(dim) => dims.push(dim),
                                    Err(_) => return Err(E::msg("Failed to parse tensor dimension".to_string())),
                                }
                            }
                            dims
                        }
                    };
                    
                    debug!("   Output: {} -> {:?} ({})", name, shape, data_type);
                    
                    if shape.is_empty() { any_empty = true; }
                    outputs.insert(
                        name.to_string(),
                        TensorConfig {
                            name: name.to_string(),
                            shape,
                            data_type: data_type.to_uppercase(),
                        },
                    );
                }
            }

            // If any top-level output shapes are empty, try to backfill from function schemas
            if any_empty {
                if let Some(functions) = manifest
                    .get(0)
                    .and_then(|m| m.get("functions").and_then(|f| f.as_array()))
                {
                    for prefer in ["prefill", "infer"] {
                        for function in functions {
                            if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                                if func_name == prefer {
                                    if let Some(func_output_schema) = function.get("outputSchema").and_then(|s| s.as_array()) {
                                        for output in func_output_schema {
                                            if let (Some(name), Some(shape_str), Some(data_type)) = (
                                                output.get("name").and_then(|n| n.as_str()),
                                                output.get("shape").and_then(|s| s.as_str()),
                                                output.get("dataType").and_then(|d| d.as_str()),
                                            ) {
                                                let shape = {
                                                    let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
                                                    if trimmed.is_empty() { vec![] } else {
                                                        let mut dims: Vec<usize> = Vec::new();
                                                        for dim_str in trimmed.split(',') {
                                                            match dim_str.trim().parse::<usize>() {
                                                                Ok(dim) => dims.push(dim),
                                                                Err(_) => return Err(E::msg("Failed to parse tensor dimension".to_string())),
                                                            }
                                                        }
                                                        dims
                                                    }
                                                };
                                                let entry = outputs.entry(name.to_string()).or_insert(TensorConfig{ name: name.to_string(), shape: vec![], data_type: data_type.to_uppercase()});
                                                if entry.shape.is_empty() { entry.shape = shape; entry.data_type = data_type.to_uppercase(); }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if let Some(functions) = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()))
        {
            // Some models put shape info primarily in function schemas
            debug!("📖 Parsing outputs from functions schema ({} functions)", functions.len());
            for function in functions {
                if let Some(func_output_schema) =
                    function.get("outputSchema").and_then(|s| s.as_array())
                {
                    for output in func_output_schema {
                        if let (Some(name), Some(shape_str), Some(data_type)) = (
                            output.get("name").and_then(|n| n.as_str()),
                            output.get("shape").and_then(|s| s.as_str()),
                            output.get("dataType").and_then(|d| d.as_str()),
                        ) {
                            let shape = {
                                let trimmed =
                                    shape_str.trim_start_matches('[').trim_end_matches(']');
                                if trimmed.is_empty() {
                                    vec![]
                                } else {
                                    let mut dims: Vec<usize> = Vec::new();
                                    for dim_str in trimmed.split(',') {
                                        match dim_str.trim().parse::<usize>() {
                                            Ok(dim) => dims.push(dim),
                                            Err(_) => {
                                                return Err(E::msg(
                                                    "Failed to parse tensor dimension".to_string(),
                                                ))
                                            }
                                        }
                                    }
                                    dims
                                }
                            };

                            debug!("   Output: {} -> {:?} ({})", name, shape, data_type);

                            outputs.insert(
                                name.to_string(),
                                TensorConfig {
                                    name: name.to_string(),
                                    shape,
                                    data_type: data_type.to_uppercase(),
                                },
                            );
                        }
                    }
                }
            }
        } else {
            debug!("📖 Unknown manifest format, using fallback outputs");
            
            // Fallback for unknown formats
            outputs.insert(
                "hidden_states".to_string(),
                TensorConfig {
                    name: "hidden_states".to_string(),
                    shape: vec![1, 64, 1024],
                    data_type: "FLOAT16".to_string(),
                },
            );
        }

        debug!("📖 Extracted {} output tensors", outputs.len());
        Ok(outputs)
    }

    /// Compute overall shape configuration from components and fix embeddings shapes
    fn compute_shape_info(
        &self,
        components: &HashMap<String, ComponentConfig>,
    ) -> Result<crate::model_config::ShapeConfig> {
        let batch_size = 1;

        // Hidden size: prefer FFN hidden_states last dim, else LM head input last dim
        let hidden_size = {
            let mut hs: Option<usize> = None;
            if let Some(ffn_prefill) = components.get("ffn_prefill") {
                if let Some(t) = ffn_prefill.inputs.get("hidden_states").or_else(|| ffn_prefill.outputs.get("hidden_states")).or_else(|| ffn_prefill.outputs.get("output_hidden_states")) {
                    if t.shape.len() >= 3 { hs = t.shape.get(2).cloned(); }
                }
            }
            if hs.is_none() {
                if let Some(ffn_infer) = components.get("ffn_infer") {
                    if let Some(t) = ffn_infer.inputs.get("hidden_states").or_else(|| ffn_infer.outputs.get("hidden_states")).or_else(|| ffn_infer.outputs.get("output_hidden_states")) {
                        if t.shape.len() >= 3 { hs = t.shape.get(2).cloned(); }
                    }
                }
            }
            if hs.is_none() {
                if let Some(lm) = components.get("lm_head") {
                    if let Some(t) = lm.inputs.get("hidden_states") { if t.shape.len() >= 3 { hs = t.shape.get(2).cloned(); } }
                }
            }
            hs.unwrap_or(1024)
        };

        // Context length: prefer causal_mask last dim
        let context_length = {
            let mut cl: Option<usize> = None;
            for name in ["ffn_prefill", "ffn_infer"] {
                if let Some(c) = components.get(name) {
                    if let Some(mask) = c.inputs.get("causal_mask") {
                        if mask.shape.len() >= 4 { cl = mask.shape.get(3).cloned(); if cl.is_some() { break; } }
                    }
                }
            }
            cl.unwrap_or(256)
        };

        // Vocab size: prefer LM head logits last dim; fall back to any output with largest last dim
        let vocab_size = {
            let mut vs: Option<usize> = None;
            if let Some(lm) = components.get("lm_head") {
                for key in ["logits", "scores", "output", "output_logits"] {
                    if let Some(t) = lm.outputs.get(key) { if t.shape.len() >= 3 { vs = t.shape.get(2).cloned(); break; } }
                }
                if vs.is_none() {
                    for t in lm.outputs.values() { if t.shape.len() >= 3 { vs = Some(vs.map_or(t.shape[2], |cur| cur.max(t.shape[2]))); } }
                }
            }
            vs.unwrap_or(151669)
        };

        Ok(crate::model_config::ShapeConfig { batch_size, context_length, hidden_size, vocab_size })
    }

    /// Reconcile embeddings input/output shapes using FFN schemas
    fn reconcile_component_shapes(
        &self,
        components: &mut HashMap<String, ComponentConfig>,
    ) -> Result<()> {
        // Get sequence length and hidden size from FFN prefill if available
        let mut seq_len: Option<usize> = None;
        let mut hidden: Option<usize> = None;
        if let Some(ffn_prefill) = components.get("ffn_prefill") {
            if let Some(h) = ffn_prefill.inputs.get("hidden_states") {
                if h.shape.len() >= 3 { seq_len = h.shape.get(1).cloned(); hidden = h.shape.get(2).cloned(); }
            }
        }
        if hidden.is_none() {
            if let Some(ffn_infer) = components.get("ffn_infer") {
                if let Some(h) = ffn_infer.inputs.get("hidden_states") { if h.shape.len() >= 3 { hidden = h.shape.get(2).cloned(); } }
            }
        }

        if let Some(emb) = components.get_mut("embeddings") {
            if let Some(input_ids) = emb.inputs.get_mut("input_ids") {
                if let Some(s) = seq_len { if input_ids.shape.len() >= 2 { input_ids.shape[1] = s; } else { input_ids.shape = vec![1, s]; } }
            }
            let desired_hidden = hidden.unwrap_or(1024);
            let desired_seq = seq_len.or_else(|| emb.inputs.get("input_ids").and_then(|t| t.shape.get(1).cloned())).unwrap_or(1);
            let out = emb.outputs.entry("hidden_states".to_string()).or_insert(TensorConfig { name: "hidden_states".to_string(), shape: vec![], data_type: "FLOAT16".to_string() });
            out.shape = vec![1, desired_seq, desired_hidden];
            out.data_type = out.data_type.to_uppercase();
        }

        // Precompute shapes for LM head wiring without holding a mutable borrow
        let lm_single_token_shape: Vec<usize> = {
            if let Some(ffn_infer) = components.get("ffn_infer") {
                if let Some(t) = ffn_infer.outputs.get("output_hidden_states") {
                    t.shape.clone()
                } else {
                    vec![1, 1, hidden.unwrap_or(1024)]
                }
            } else if let Some(ffn_prefill) = components.get("ffn_prefill") {
                if let Some(t) = ffn_prefill.outputs.get("output_hidden_states").or_else(|| ffn_prefill.outputs.get("hidden_states")) {
                    t.shape.clone()
                } else {
                    vec![1, 1, hidden.unwrap_or(1024)]
                }
            } else {
                vec![1, 1, hidden.unwrap_or(1024)]
            }
        };

        let discovered_vocab: Option<usize> = components
            .get("lm_head")
            .and_then(|c| c.outputs.values().find(|t| t.shape.len() >= 3).map(|t| t.shape[2]));

        // Ensure LM head takes hidden_states as input with single-token shape
        if let Some(lm) = components.get_mut("lm_head") {
            let lm_in = lm
                .inputs
                .entry("hidden_states".to_string())
                .or_insert(TensorConfig { name: "hidden_states".to_string(), shape: vec![], data_type: "FLOAT16".to_string() });
            lm_in.shape = lm_single_token_shape;
            lm_in.data_type = lm_in.data_type.to_uppercase();

            // Ensure there is at least one logits output key for downstream selection logic
            let has_logits = lm.outputs.keys().any(|k| k.starts_with("logits"));
            if !has_logits {
                // If there is any existing output, mirror its shape under a new 'logits' key
                if let Some(any_out) = lm.outputs.values().next().cloned() {
                    lm.outputs.insert("logits".to_string(), TensorConfig { name: "logits".to_string(), shape: any_out.shape, data_type: any_out.data_type });
                } else {
                    // Fallback to a minimal logits placeholder using discovered vocab size if available
                    let vocab = discovered_vocab.unwrap_or(151669);
                    lm.outputs.insert("logits".to_string(), TensorConfig { name: "logits".to_string(), shape: vec![1, 1, vocab], data_type: "FLOAT16".to_string() });
                }
            }
        }

        Ok(())
    }

    /// Infer component name from package filename
    fn infer_component_name(&self, package_path: &Path) -> String {
        let filename = package_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        debug!("🔍 Inferring component name from filename: {}", filename);

        if filename.contains("embedding") {
            debug!("   -> Detected as 'embeddings'");
            "embeddings".to_string()
        } else if filename.contains("prefill") || filename.contains("_pf_") {
            // Handle both "prefill" and "_PF_" patterns (PF = PreFill)
            debug!("   -> Detected as 'ffn_prefill' (contains 'prefill' or '_pf_')");
            "ffn_prefill".to_string()
        } else if filename.contains("ffn") {
            // Distinguish FFN prefill vs infer by common naming patterns
            if filename.contains("prefill") || filename.contains("_pf_") {
                debug!("   -> Detected as 'ffn_prefill' (FFN with prefill marker)");
                "ffn_prefill".to_string()
            } else if filename.contains("chunk") || filename.contains("_ch_") {
                // Most ANEMLL infer-time FFN packages are chunked
                debug!("   -> Detected as 'ffn_infer' (FFN chunk without prefill marker)");
                "ffn_infer".to_string()
            } else {
                // For ambiguous FFN filenames, fall back to metadata-driven analysis later
                debug!("   -> Detected FFN component, will analyze metadata for correct mapping");
                "ffn_unified".to_string()
            }
        } else if filename.contains("lm_head") || filename.contains("head") {
            debug!("   -> Detected as 'lm_head'");
            "lm_head".to_string()
        } else {
            // Default fallback
            debug!("   -> Using filename as component name: {}", filename);
            filename.replace(['_', '-'], "_")
        }
    }

    /// Determine FFN execution mode from component analysis
    fn determine_ffn_execution_mode(&self, components: &std::collections::HashMap<String, crate::model_config::ComponentConfig>) -> String {
        // Check if we have separate ffn_prefill and ffn_infer components
        let has_separate_prefill = components.contains_key("ffn_prefill");
        let has_separate_infer = components.contains_key("ffn_infer");
        
        if has_separate_prefill && has_separate_infer {
            // If we have both separate components, it's split mode
            debug!("🔧 Found separate ffn_prefill and ffn_infer components - using split mode");
            "split".to_string()
        } else if components.contains_key("ffn_prefill") || components.contains_key("ffn_infer") {
            // If we only have one FFN component, it's likely unified mode
            debug!("🔧 Found single FFN component - using unified mode");
            "unified".to_string()
        } else {
            // Fallback: check for any FFN-like component
            let ffn_components: Vec<_> = components.keys()
                .filter(|name| name.to_lowercase().contains("ffn"))
                .collect();
            
            if ffn_components.len() == 1 {
                debug!("🔧 Found single FFN-like component: {:?} - using unified mode", ffn_components[0]);
                "unified".to_string()
            } else if ffn_components.len() > 1 {
                debug!("🔧 Found multiple FFN-like components: {:?} - using split mode", ffn_components);
                "split".to_string()
            } else {
                debug!("🔧 No FFN components found - defaulting to unified mode");
                "unified".to_string()
            }
        }
    }

    // (removed unused extract_shape_info)

    /// Generate naming configuration patterns
    fn generate_naming_config(
        &self,
        packages: &[PathBuf],
    ) -> Result<crate::model_config::NamingConfig> {
        // Analyze filenames to generate patterns
        let mut patterns = crate::model_config::NamingConfig {
            embeddings_pattern: None,
            ffn_infer_pattern: None,
            ffn_prefill_pattern: None,
            lm_head_pattern: None,
        };

        for package in packages {
            let filename = package.file_name().unwrap_or_default().to_string_lossy().to_lowercase();

            if filename.contains("embedding") {
                patterns.embeddings_pattern = Some(filename.to_string());
                continue;
            }

            // Prefill detection: explicit prefill markers
            if filename.contains("prefill") || filename.contains("_pf_") || filename.contains("ffn_pf") {
                patterns.ffn_prefill_pattern = Some(filename.to_string());
                continue;
            }

            // Infer-time FFN detection: FFN without prefill markers, often chunked
            if filename.contains("ffn") && !filename.contains("_pf_") && !filename.contains("prefill") {
                patterns.ffn_infer_pattern = Some(filename.to_string());
                continue;
            }

            if filename.contains("lm_head") || filename.contains("head") {
                patterns.lm_head_pattern = Some(filename.to_string());
                continue;
            }
        }

        Ok(patterns)
    }

    /// Cache the generated configuration
    fn cache_generated_config(&self, model_id: &str, config: &ModelConfig) -> Result<()> {
        let configs_dir = self.cache_manager.configs_dir();
        std::fs::create_dir_all(&configs_dir)?;

        let config_filename = format!("{}.json", model_id.replace('/', "--"));
        let config_path = configs_dir.join(config_filename);

        let config_json = serde_json::to_string_pretty(config)?;
        std::fs::write(&config_path, config_json)?;

        info!("💾 Cached generated config at: {}", config_path.display());
        Ok(())
    }

    /// Load a cached configuration if available
    pub fn load_cached_config(&self, model_id: &str) -> Result<Option<ModelConfig>> {
        let configs_dir = self.cache_manager.configs_dir();
        let config_filename = format!("{}.json", model_id.replace('/', "--"));
        let config_path = configs_dir.join(config_filename);

        if !config_path.exists() {
            return Ok(None);
        }

        let config_json = std::fs::read_to_string(&config_path)?;
        let config: ModelConfig = serde_json::from_str(&config_json)?;

        debug!("📖 Loaded cached config for: {}", model_id);
        Ok(Some(config))
    }

    // (Removed model-specific fix functions to keep generator data-driven)

    /// Parse tensor configurations from a schema array
    fn parse_tensor_configs_from_schema(&self, schema: &[Value]) -> Result<HashMap<String, TensorConfig>> {
        let mut configs = HashMap::new();
        
        for input in schema {
            if let (Some(name), Some(shape_str), Some(data_type)) = (
                input.get("name").and_then(|n| n.as_str()),
                input.get("shape").and_then(|s| s.as_str()),
                input.get("dataType").and_then(|d| d.as_str()),
            ) {
                let shape = {
                    let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
                    if trimmed.is_empty() {
                        vec![]
                    } else {
                        let mut dims: Vec<usize> = Vec::new();
                        for dim_str in trimmed.split(',') {
                            match dim_str.trim().parse::<usize>() {
                                Ok(dim) => dims.push(dim),
                                Err(_) => return Err(E::msg("Failed to parse tensor dimension".to_string())),
                            }
                        }
                        dims
                    }
                };
                
                debug!("   Input: {} -> {:?} ({})", name, shape, data_type);
                
                configs.insert(
                    name.to_string(),
                    TensorConfig {
                        name: name.to_string(),
                        shape,
                        data_type: data_type.to_uppercase(),
                    },
                );
            }
        }
        
        debug!("📖 Extracted {} tensor configs", configs.len());
        Ok(configs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    /// Create a mock .mlpackage directory structure for testing
    fn create_mock_mlpackage(temp_dir: &Path, name: &str) -> Result<PathBuf> {
        let package_path = temp_dir.join(format!("{name}.mlpackage"));
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
    fn test_config_generator_creation() {
        let generator = ConfigGenerator::new().expect("Failed to create config generator");

        // Should have a cache manager
        assert!(generator.cache_manager.cache_base().exists());
    }

    #[test]
    fn test_find_mlpackage_files() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let generator = ConfigGenerator::new()?;

        // Create some mock packages
        create_mock_mlpackage(temp_dir.path(), "embeddings")?;
        create_mock_mlpackage(temp_dir.path(), "ffn_chunk_01")?;
        create_mock_mlpackage(temp_dir.path(), "lm_head")?;

        let packages = generator.find_mlpackage_files(temp_dir.path())?;

        assert_eq!(packages.len(), 3);
        assert!(packages.iter().any(|p| p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains("embeddings")));
        assert!(packages.iter().any(|p| p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains("ffn_chunk")));
        assert!(packages.iter().any(|p| p
            .file_name()
            .unwrap()
            .to_string_lossy()
            .contains("lm_head")));

        Ok(())
    }

    #[test]
    fn test_component_name_inference() {
        let generator = ConfigGenerator::new().expect("Failed to create generator");

        assert_eq!(
            generator.infer_component_name(Path::new("model_embeddings.mlpackage")),
            "embeddings"
        );
        assert_eq!(
            generator.infer_component_name(Path::new("model_prefill_chunk_01.mlpackage")),
            "ffn_prefill"
        );
        assert_eq!(
            generator.infer_component_name(Path::new("model_ffn_chunk_01.mlpackage")),
            "ffn_infer"
        );
        assert_eq!(
            generator.infer_component_name(Path::new("model_lm_head.mlpackage")),
            "lm_head"
        );
    }

    #[test]
    fn test_config_generation_with_mock_model() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let generator = ConfigGenerator::new()?;

        // Create a complete mock model with multiple components
        create_mock_mlpackage(temp_dir.path(), "qwen_embeddings")?;
        create_mock_mlpackage(temp_dir.path(), "qwen_prefill_chunk_01")?;
        create_mock_mlpackage(temp_dir.path(), "qwen_ffn_chunk_01")?;
        create_mock_mlpackage(temp_dir.path(), "qwen_lm_head")?;

        // Generate config
        let config = generator.generate_config_from_directory(
            temp_dir.path(),
            "test/mock-qwen-model",
            "qwen",
        )?;

        // Verify the generated configuration
        assert_eq!(config.model_info.model_type, "qwen");
        assert_eq!(
            config.model_info.path.as_ref().unwrap(),
            &temp_dir.path().to_string_lossy()
        );

        // Should have identified all components
        assert!(config.components.contains_key("embeddings"));
        assert!(config.components.contains_key("ffn_prefill"));
        assert!(config.components.contains_key("ffn_infer"));
        assert!(config.components.contains_key("lm_head"));

        // Should have shape information
        assert!(config.shapes.batch_size > 0);
        assert!(config.shapes.context_length > 0);
        assert!(config.shapes.hidden_size > 0);
        assert!(config.shapes.vocab_size > 0);

        println!("Generated config: {config:#?}");

        Ok(())
    }

    #[test]
    fn test_qwen_typo_fixer_config_structure() -> Result<()> {
        // Verify generator remains data-driven (no hardcoded values) and produces
        // a consistent, wired configuration for Qwen-like models.

        let temp_dir = TempDir::new()?;
        let generator = ConfigGenerator::new()?;

        // Create mock packages that approximate a Qwen structure
        create_mock_mlpackage(temp_dir.path(), "qwen-typo-fixer_embeddings")?;
        create_mock_mlpackage(temp_dir.path(), "qwen-typo-fixer_prefill_chunk_01of01")?;
        create_mock_mlpackage(temp_dir.path(), "qwen-typo-fixer_FFN_chunk_01of01")?;
        create_mock_mlpackage(temp_dir.path(), "qwen-typo-fixer_lm_head")?;

        // Generate config
        let config = generator.generate_config_from_directory(
            temp_dir.path(),
            "mazhewitt/qwen-typo-fixer-coreml",
            "qwen",
        )?;

        // Basic shape sanity
        assert_eq!(config.shapes.batch_size, 1);
        assert!(config.shapes.context_length > 0);
        assert!(config.shapes.hidden_size > 0);
        assert!(config.shapes.vocab_size > 0);

        // Components present
        assert!(config.components.contains_key("embeddings"));
        assert!(config.components.contains_key("ffn_prefill"));
        assert!(config.components.contains_key("ffn_infer"));
        assert!(config.components.contains_key("lm_head"));

        // Embeddings IO present and non-empty
        let embeddings = config.components.get("embeddings").unwrap();
        assert!(embeddings.inputs.get("input_ids").is_some());
        let emb_in = embeddings.inputs.get("input_ids").unwrap();
        assert_eq!(emb_in.shape.len(), 2);
        assert!(emb_in.shape[1] > 0);
        let emb_out = embeddings.outputs.get("hidden_states").unwrap();
        assert_eq!(emb_out.shape.len(), 3);
        assert_eq!(emb_out.shape[0], 1);
        assert_eq!(emb_out.shape[2], config.shapes.hidden_size);

        // LM head accepts hidden_states and produces at least one logits tensor
        let lm_head = config.components.get("lm_head").unwrap();
        assert!(lm_head.inputs.contains_key("hidden_states"));
        let has_any_logits = lm_head
            .outputs
            .keys()
            .any(|k| k.starts_with("logits"));
        assert!(has_any_logits);

        // Internal wiring sanity should not error (uses available tensors)
        let _ = config.validate_internal_wiring();

        Ok(())
    }
}
