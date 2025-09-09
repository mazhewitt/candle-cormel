//! CoreML manifest parsing utilities
//!
//! Handles different CoreML package formats and function-based components

use crate::model_config::{ComponentConfig, TensorConfig};
use super::schema_extractor::{ComponentRole, SchemaExtractor};
use super::coreml_metadata::CoreMLMetadataExtractor;
use super::file_discovery::ManifestSource;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, trace};

pub struct ManifestParser;

impl ManifestParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a CoreML package using enhanced manifest source detection
    pub fn parse_package_enhanced(
        &self,
        package_path: &Path,
        manifest_source: &ManifestSource,
        manifest: &Value,
        schema_extractor: &SchemaExtractor,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        match manifest_source {
            ManifestSource::MetadataJson(_) | ManifestSource::ManifestJson(_) => {
                // Use existing manifest-based parsing
                self.parse_package_with_metadata_detection(package_path, manifest, schema_extractor)
            }
            ManifestSource::ModelFile(model_path) => {
                // Parse directly from model.mlmodel file
                self.parse_package_from_model_file(package_path, model_path, schema_extractor)
            }
            ManifestSource::FilenameOnly => {
                // Use pure filename-based detection
                self.parse_package_filename_only(package_path, schema_extractor)
            }
        }
    }

    /// Parse package from direct model.mlmodel file (typo-fixer style)
    fn parse_package_from_model_file(
        &self,
        package_path: &Path,
        model_path: &Path,
        schema_extractor: &SchemaExtractor,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        debug!("📦 Parsing from model.mlmodel file: {}", model_path.display());

        // Use CoreML metadata extractor to get per-function tensor signatures when available
        let metadata_extractor = CoreMLMetadataExtractor::new();
        match metadata_extractor.extract_full_metadata(model_path) {
            Ok((model_inputs, model_outputs, functions)) => {
                // If per-function metadata exists, build components accordingly
                if !functions.is_empty() {
                    let mut components: Vec<(String, ComponentConfig)> = Vec::new();

                    for (fname, (inputs, outputs)) in functions {
                        // Determine role from tensors first
                        let mut role = schema_extractor.detect_component_role(&inputs, &outputs);
                        // Fallback: infer from common function names
                        if matches!(role, ComponentRole::Unknown) {
                            let lname = fname.to_lowercase();
                            if lname.contains("prefill") {
                                role = ComponentRole::FfnPrefill;
                            } else if lname.contains("infer") || lname.contains("decode") {
                                role = ComponentRole::FfnInfer;
                            }
                        }

                        let component_name = self.role_to_component_name(&role);
                        let mut functions_vec = Vec::new();
                        functions_vec.push(fname);
                        let config = ComponentConfig {
                            file_path: Some(package_path.to_string_lossy().to_string()),
                            inputs,
                            outputs,
                            functions: functions_vec,
                            input_order: None,
                        };
                        debug!(
                            "🏷️ Function component: {} (role: {:?})",
                            component_name, role
                        );
                        components.push((component_name, config));
                    }

                    // If we ended up with duplicate names (e.g., both map to ffn_prefill), keep both by disambiguating
                    // with function names appended.
                    let mut seen: HashMap<String, usize> = HashMap::new();
                    for (name, _) in &mut components {
                        let count = seen.entry(name.clone()).or_insert(0);
                        if *count > 0 {
                            // append index
                            name.push_str(&format!("_{}", count));
                        }
                        *count += 1;
                    }

                    return Ok(components);
                }

                // No per-function info; fall back to model-level IO
                debug!(
                    "ℹ️ No per-function IO; using model-level IO ({} in, {} out)",
                    model_inputs.len(),
                    model_outputs.len()
                );

                // Detect component role from tensor signatures
                let role = schema_extractor.detect_component_role(&model_inputs, &model_outputs);

                // If detection fails or tensors empty, fallback to filename
                if matches!(role, ComponentRole::Unknown)
                    || (model_inputs.is_empty() && model_outputs.is_empty())
                {
                    debug!(
                        "⚠️ Model-level detection unknown/empty, using filename fallback"
                    );
                    return self.parse_package_filename_only(package_path, schema_extractor);
                }

                let component_config = ComponentConfig {
                    file_path: Some(package_path.to_string_lossy().to_string()),
                    inputs: model_inputs,
                    outputs: model_outputs,
                    functions: Vec::new(),
                    input_order: None,
                };

                let component_name = self.role_to_component_name(&role);
                debug!(
                    "🏷️ Model file detection: {} -> {}",
                    package_path.display(),
                    component_name
                );

                Ok(vec![(component_name, component_config)])
            }
            Err(e) => {
                debug!("⚠️ Failed to extract from model.mlmodel: {}", e);
                // Fall back to filename-only detection
                self.parse_package_filename_only(package_path, schema_extractor)
            }
        }
    }

    /// Parse package using only filename patterns (ultimate fallback)
    pub fn parse_package_filename_only(
        &self,
        package_path: &Path,
        schema_extractor: &SchemaExtractor,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        debug!("📦 Using filename-only parsing: {}", package_path.display());

        let filename = package_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // Use schema extractor's filename-based detection
        let role = schema_extractor.detect_component_role_from_filename(filename);

        // Try extracting tensor signatures from inner model file when available.
        // If not available (pure filename-only scenario), proceed with empty tensors.
        let inner_model = package_path.join("Data/com.apple.CoreML/model.mlmodel");
        let (inputs, outputs) = if inner_model.exists() {
            let extractor = CoreMLMetadataExtractor::new();
            match extractor.extract_tensor_signatures(&inner_model) {
                Ok((ins, outs)) => {
                    debug!(
                        "📖 Filename-only: populated tensors from inner model (inputs={}, outputs={})",
                        ins.len(),
                        outs.len()
                    );
                    (ins, outs)
                }
                Err(e) => {
                    debug!("⚠️ Filename-only: failed to extract metadata from inner model: {}. Proceeding with empty tensors.", e);
                    (HashMap::new(), HashMap::new())
                }
            }
        } else {
            debug!("ℹ️ Filename-only: no inner model file found, proceeding with empty tensors");
            (HashMap::new(), HashMap::new())
        };

        // Create component config using any extracted tensors
        let component_config = ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs,
            outputs,
            functions: Vec::new(),
            input_order: None,
        };
        
        let component_name = self.role_to_component_name(&role);
        debug!("🏷️ Filename-only detection: {} -> {}", filename, component_name);
        
        Ok(vec![(component_name, component_config)])
    }

    /// Parse a CoreML package into component configurations using metadata-driven detection
    pub fn parse_package_with_metadata_detection(
        &self,
        package_path: &Path,
        manifest: &Value,
        schema_extractor: &SchemaExtractor,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        let mut components = Vec::new();

        // First try to extract tensor signatures directly from the model.mlmodel file
        if let Some((inputs, outputs)) = self.extract_tensor_signatures_from_model(package_path)? {
            debug!("✅ Extracted tensor signatures from model.mlmodel file");
            let mut role = schema_extractor.detect_component_role(&inputs, &outputs);

            // If metadata-driven detection didn't yield a clear role (or tensors are empty),
            // fall back to filename-based detection to avoid returning an 'unknown' component.
            if matches!(role, ComponentRole::Unknown) || (inputs.is_empty() && outputs.is_empty()) {
                debug!("⚠️ Role unknown or empty tensors from metadata, falling back to filename-based detection");
                let filename = package_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");
                role = schema_extractor.detect_component_role_from_filename(filename);
            }

            let component_config = ComponentConfig {
                file_path: Some(package_path.to_string_lossy().to_string()),
                inputs,
                outputs,
                functions: Vec::new(),
                input_order: None,
            };

            let component_name = self.role_to_component_name(&role);
            components.push((component_name, component_config));
        } else {
            debug!("⚠️ Failed to extract from model.mlmodel, falling back to manifest parsing");
            
            // Fall back to manifest-based extraction
            if let Some(function_components) = self.extract_function_components_with_roles(package_path, manifest, schema_extractor)? {
                components.extend(function_components);
            } else {
                // Fall back to single component with role detection
                let component_config = self.create_base_component(package_path, manifest)?;
                let inputs = &component_config.inputs;
                let outputs = &component_config.outputs;
                
                // Try tensor-based detection first
                let mut role = schema_extractor.detect_component_role(inputs, outputs);
                
                // If tensor detection fails, try filename-based detection
                if role == ComponentRole::Unknown {
                    let filename = package_path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");
                    role = schema_extractor.detect_component_role_from_filename(filename);
                }
                
                let component_name = self.role_to_component_name(&role);
                components.push((component_name, component_config));
            }
        }

        Ok(components)
    }

    /// Parse a CoreML package into component configurations (legacy method)
    pub fn parse_package(
        &self,
        package_path: &Path,
        manifest: &Value,
        component_name: &str,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        let mut components = Vec::new();

        // First try to extract function-based components
        if let Some(function_components) = self.extract_function_components(package_path, manifest, component_name)? {
            components.extend(function_components);
        } else {
            // Fall back to single component
            let component_config = self.create_base_component(package_path, manifest)?;
            components.push((component_name.to_string(), component_config));
        }

        Ok(components)
    }

    /// Extract function-based components if the package has multiple functions
    fn extract_function_components(
        &self,
        package_path: &Path,
        manifest: &Value,
        base_component_name: &str,
    ) -> Result<Option<Vec<(String, ComponentConfig)>>> {
        let functions = manifest.get(0).and_then(|m| m.get("functions").and_then(|f| f.as_array()));
        
        let Some(funcs) = functions else {
            return Ok(None);
        };

        let mut function_components = Vec::new();

    for function in funcs {
            if let Some(function_name) = function.get("name").and_then(|n| n.as_str()) {
        let component_config = self.create_function_component(package_path, function)?;
                
                // Create component name: either "componentname_functionname" or just "functionname"
                let component_key = if funcs.len() > 1 {
                    format!("{}_{}", base_component_name, function_name)
                } else {
                    function_name.to_string()
                };

                debug!(
                    "📋 Function-based component '{}': inputs={:?} outputs={:?}",
                    component_key,
                    component_config.inputs.keys().collect::<Vec<_>>(),
                    component_config.outputs.keys().collect::<Vec<_>>()
                );
                
                function_components.push((component_key, component_config));
            }
        }

        if function_components.is_empty() {
            Ok(None)
        } else {
            Ok(Some(function_components))
        }
    }

    /// Create a base component configuration from manifest
    fn create_base_component(&self, package_path: &Path, manifest: &Value) -> Result<ComponentConfig> {
        let inputs = self.extract_inputs_from_manifest(manifest)?;
        let outputs = self.extract_outputs_from_manifest(manifest)?;

        Ok(ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs,
            outputs,
            functions: Vec::new(),
            input_order: None,
        })
    }

    /// Create a function-specific component configuration
    fn create_function_component(&self, package_path: &Path, function: &Value) -> Result<ComponentConfig> {
        let function_name = function.get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");

        let empty_vec: Vec<Value> = Vec::new();
        let in_arr = function.get("inputSchema").and_then(|s| s.as_array()).unwrap_or(&empty_vec);
        let out_arr = function.get("outputSchema").and_then(|s| s.as_array()).unwrap_or(&empty_vec);
        
        let inputs = self.parse_tensor_schema(in_arr)?;
        let outputs = self.parse_tensor_schema(out_arr)?;

        Ok(ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs,
            outputs,
            functions: vec![function_name.to_string()],
            input_order: None,
        })
    }

    /// Extract inputs using schema extractor pattern
    fn extract_inputs_from_manifest(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        // This is a simplified version - in practice you'd use SchemaExtractor
        let mut inputs = HashMap::new();

        if let Some(input_schema) = manifest.get(0).and_then(|m| m.get("inputSchema").and_then(|s| s.as_array())) {
            inputs = self.parse_tensor_schema(input_schema)?;
        }

        Ok(inputs)
    }

    /// Extract outputs using schema extractor pattern  
    fn extract_outputs_from_manifest(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        // This is a simplified version - in practice you'd use SchemaExtractor
        let mut outputs = HashMap::new();

        if let Some(output_schema) = manifest.get(0).and_then(|m| m.get("outputSchema").and_then(|s| s.as_array())) {
            outputs = self.parse_tensor_schema(output_schema)?;
        }

        Ok(outputs)
    }

    /// Parse tensor schema array into tensor configs
    fn parse_tensor_schema(&self, schema: &[Value]) -> Result<HashMap<String, TensorConfig>> {
        let mut configs = HashMap::new();
        
        for tensor_def in schema {
            if let Some(tensor_config) = self.parse_single_tensor(tensor_def)? {
                configs.insert(tensor_config.name.clone(), tensor_config);
            }
        }
        
        Ok(configs)
    }

    /// Parse a single tensor definition
    fn parse_single_tensor(&self, tensor_def: &Value) -> Result<Option<TensorConfig>> {
        let (name, data_type) = match (
            tensor_def.get("name").and_then(|n| n.as_str()),
            tensor_def.get("dataType").and_then(|d| d.as_str()),
        ) {
            (Some(n), Some(d)) => (n, d),
            _ => return Ok(None),
        };

        let shape = if let Some(shape_str) = tensor_def.get("shape").and_then(|s| s.as_str()) {
            self.parse_shape_string(shape_str)?
        } else {
            vec![]
        };

        Ok(Some(TensorConfig {
            name: name.to_string(),
            shape,
            data_type: data_type.to_uppercase(),
        }))
    }

    /// Parse shape string like "[1, 64, 1024]" into Vec<usize>
    fn parse_shape_string(&self, shape_str: &str) -> Result<Vec<usize>> {
        let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
        if trimmed.is_empty() {
            return Ok(vec![]);
        }

        let mut dims = Vec::new();
        for dim_str in trimmed.split(',') {
            match dim_str.trim().parse::<usize>() {
                Ok(dim) => dims.push(dim),
                Err(_) => return Err(anyhow::Error::msg("Failed to parse tensor dimension")),
            }
        }
        
        Ok(dims)
    }

    /// Extract function-based components with metadata-driven role detection
    fn extract_function_components_with_roles(
        &self,
        package_path: &Path,
        manifest: &Value,
        schema_extractor: &SchemaExtractor,
    ) -> Result<Option<Vec<(String, ComponentConfig)>>> {
        let functions = manifest.get(0).and_then(|m| m.get("functions").and_then(|f| f.as_array()));
        
        let Some(funcs) = functions else {
            return Ok(None);
        };

        let mut function_components = Vec::new();

    for function in funcs {
            if let Some(function_name) = function.get("name").and_then(|n| n.as_str()) {
        let component_config = self.create_function_component(package_path, function)?;

                // Detect component role from tensor signatures
                let inputs = &component_config.inputs;
                let outputs = &component_config.outputs;
                let mut role = schema_extractor.detect_component_role(inputs, outputs);

                // Fallback: if role is Unknown, use function name to infer FFN component
                if matches!(role, ComponentRole::Unknown) {
                    if function_name == "prefill" {
                        role = ComponentRole::FfnPrefill;
                    } else if function_name == "infer" {
                        role = ComponentRole::FfnInfer;
                    }
                }

                // Use metadata-driven component name instead of function name
                let component_name = match role {
                    ComponentRole::FfnPrefill => format!("ffn_{}", function_name),  // e.g., "ffn_prefill"
                    ComponentRole::FfnInfer => format!("ffn_{}", function_name),    // e.g., "ffn_infer"
                    ComponentRole::FfnUnified => "ffn_prefill".to_string(),        // Unified functions go to prefill
                    _ => self.role_to_component_name(&role),
                };

                debug!(
                    "📋 Metadata-driven component '{}' (role: {:?}): inputs={:?} outputs={:?}",
                    component_name,
                    role,
                    component_config.inputs.keys().collect::<Vec<_>>(),
                    component_config.outputs.keys().collect::<Vec<_>>()
                );
                
                function_components.push((component_name, component_config));
            }
        }

        if function_components.is_empty() {
            Ok(None)
        } else {
            Ok(Some(function_components))
        }
    }

    /// Convert component role to standard component name
    pub fn role_to_component_name(&self, role: &ComponentRole) -> String {
        match role {
            ComponentRole::Embeddings => "embeddings".to_string(),
            ComponentRole::FfnPrefill => "ffn_prefill".to_string(),
            ComponentRole::FfnInfer => "ffn_infer".to_string(),
            ComponentRole::FfnUnified => "ffn_prefill".to_string(),  // Unified functions go to prefill
            ComponentRole::LmHead => "lm_head".to_string(),
            ComponentRole::Unknown => "unknown".to_string(),
        }
    }

    /// Determine execution mode from parsed components based on split vs unified FFN architecture
    /// Follows ANEMLL naming convention for FFN_PF (unified) vs separate prefill/infer (split)
    pub fn infer_execution_mode(&self, components: &[(String, ComponentConfig)]) -> String {
        // Check for split FFN architecture (separate ffn_prefill and ffn_infer components)
        let has_ffn_prefill = components.iter().any(|(name, _)| name == "ffn_prefill");
        let has_ffn_infer = components.iter().any(|(name, _)| name == "ffn_infer");
        
        if has_ffn_prefill && has_ffn_infer {
            trace!("🔧 Found separate ffn_prefill and ffn_infer components - using split mode");
            return "split".to_string();
        }
        
        // Check for ANEMLL unified FFN pattern (FFN_PF in filename)
        // This indicates a single model that handles both prefill and infer modes
        let has_ffn_pf_pattern = components.iter().any(|(_, config)| {
            if let Some(file_path) = &config.file_path {
                let filename = std::path::Path::new(file_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_lowercase();
                filename.contains("ffn_pf")
            } else {
                false
            }
        });
        
        if has_ffn_pf_pattern {
            debug!("🔧 Found ANEMLL FFN_PF pattern in filename - using unified mode");
            return "unified".to_string();
        }
        
        // Check for unified FFN (single component with multiple functions)
        let multi_function_components = components
            .iter()
            .filter(|(_, config)| config.functions.len() > 1)
            .count();

        if multi_function_components > 0 {
            debug!("🔧 Found {} multi-function components - using unified mode", multi_function_components);
            "unified".to_string()
        } else {
            // Check if we have any FFN-like component
            let has_ffn_like = components.iter().any(|(name, _)| name.starts_with("ffn"));
            
            if has_ffn_like {
                debug!("🔧 Found FFN component(s) but no clear split pattern - defaulting to unified mode");
                "unified".to_string()
            } else {
                debug!("🔧 Standard component structure - defaulting to unified mode");
                "unified".to_string()
            }
        }
    }

    /// Extract tensor signatures directly from model.mlmodel file using CoreML metadata
    fn extract_tensor_signatures_from_model(
        &self,
        package_path: &Path,
    ) -> Result<Option<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)>> {
        // Look for the model.mlmodel file within the package
        let model_file_path = package_path.join("Data/com.apple.CoreML/model.mlmodel");
        
        if !model_file_path.exists() {
            // Try alternative path structure
            let alt_model_path = package_path.join("model.mlmodel");
            if !alt_model_path.exists() {
                debug!("❌ No model.mlmodel found in {}", package_path.display());
                return Ok(None);
            }
        }

        debug!("🔍 Attempting to extract metadata from: {}", model_file_path.display());

        // Use CoreML metadata extractor
        let extractor = CoreMLMetadataExtractor::new();
        match extractor.extract_tensor_signatures(&model_file_path) {
            Ok((inputs, outputs)) => {
                debug!("✅ Successfully extracted {} inputs and {} outputs", inputs.len(), outputs.len());
                Ok(Some((inputs, outputs)))
            }
            Err(e) => {
                debug!("⚠️ Failed to extract metadata: {}", e);
                Ok(None)
            }
        }
    }
}