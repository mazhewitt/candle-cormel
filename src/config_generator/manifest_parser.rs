//! CoreML manifest parsing utilities
//!
//! Handles different CoreML package formats and function-based components

use crate::model_config::{ComponentConfig, TensorConfig};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use tracing::debug;

pub struct ManifestParser;

impl Default for ManifestParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ManifestParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a CoreML package into component configurations
    pub fn parse_package(
        &self,
        package_path: &Path,
        manifest: &Value,
        component_name: &str,
    ) -> Result<Vec<(String, ComponentConfig)>> {
        let mut components = Vec::new();

        // First try to extract function-based components
        if let Some(function_components) =
            self.extract_function_components(manifest, component_name)?
        {
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
        manifest: &Value,
        base_component_name: &str,
    ) -> Result<Option<Vec<(String, ComponentConfig)>>> {
        let functions = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()));

        let Some(funcs) = functions else {
            return Ok(None);
        };

        let mut function_components = Vec::new();

        for function in funcs {
            if let Some(function_name) = function.get("name").and_then(|n| n.as_str()) {
                let component_config = self.create_function_component(function)?;

                // Create component name: either "componentname_functionname" or just "functionname"
                let component_key = if funcs.len() > 1 {
                    format!("{base_component_name}_{function_name}")
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
    fn create_base_component(
        &self,
        package_path: &Path,
        manifest: &Value,
    ) -> Result<ComponentConfig> {
        // Try to extract inputs/outputs from manifest
        let inputs = self.extract_inputs_from_manifest(manifest)?;
        let outputs = self.extract_outputs_from_manifest(manifest)?;

        // If no schema found in manifest, create default based on component name
        let (final_inputs, final_outputs) = if inputs.is_empty() && outputs.is_empty() {
            self.create_default_schema_from_filename(package_path)?
        } else {
            (inputs, outputs)
        };

        Ok(ComponentConfig {
            file_path: Some(package_path.to_string_lossy().to_string()),
            inputs: final_inputs,
            outputs: final_outputs,
            functions: Vec::new(),
            input_order: None,
        })
    }

    /// Create a function-specific component configuration
    fn create_function_component(&self, function: &Value) -> Result<ComponentConfig> {
        let function_name = function
            .get("name")
            .and_then(|n| n.as_str())
            .unwrap_or("unknown");

        let empty_vec: Vec<Value> = Vec::new();
        let in_arr = function
            .get("inputSchema")
            .and_then(|s| s.as_array())
            .unwrap_or(&empty_vec);
        let out_arr = function
            .get("outputSchema")
            .and_then(|s| s.as_array())
            .unwrap_or(&empty_vec);

        let inputs = self.parse_tensor_schema(in_arr)?;
        let outputs = self.parse_tensor_schema(out_arr)?;

        Ok(ComponentConfig {
            file_path: None,
            inputs,
            outputs,
            functions: vec![function_name.to_string()],
            input_order: None,
        })
    }

    /// Extract inputs using schema extractor pattern
    fn extract_inputs_from_manifest(
        &self,
        manifest: &Value,
    ) -> Result<HashMap<String, TensorConfig>> {
        // This is a simplified version - in practice you'd use SchemaExtractor
        let mut inputs = HashMap::new();

        if let Some(input_schema) = manifest
            .get(0)
            .and_then(|m| m.get("inputSchema").and_then(|s| s.as_array()))
        {
            inputs = self.parse_tensor_schema(input_schema)?;
        }

        Ok(inputs)
    }

    /// Extract outputs using schema extractor pattern  
    fn extract_outputs_from_manifest(
        &self,
        manifest: &Value,
    ) -> Result<HashMap<String, TensorConfig>> {
        // This is a simplified version - in practice you'd use SchemaExtractor
        let mut outputs = HashMap::new();

        if let Some(output_schema) = manifest
            .get(0)
            .and_then(|m| m.get("outputSchema").and_then(|s| s.as_array()))
        {
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

    /// Create default tensor configurations based on component filename
    fn create_default_schema_from_filename(
        &self,
        package_path: &Path,
    ) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        let filename = package_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
        
        debug!("📖 Creating default schema for component: {}", filename);
        
        let mut inputs = HashMap::new();
        let mut outputs = HashMap::new();
        
        // Create reasonable defaults based on common ANEMLL component patterns
    if filename.contains("embeddings") {
            // Embeddings component: token IDs -> hidden states
            inputs.insert("input_ids".to_string(), TensorConfig {
                name: "input_ids".to_string(),
        shape: vec![1, 128], // Prefer 128-token batches for CoreML enumerated shapes
                data_type: "INT32".to_string(),
            });
            outputs.insert("hidden_states".to_string(), TensorConfig {
                name: "hidden_states".to_string(), 
        shape: vec![1, 128, 1024], // Match embeddings input seq length
                data_type: "FLOAT16".to_string(),
            });
    } else if filename.contains("ffn") || filename.contains("FFN") {
            // FFN component: hidden states + position/mask inputs -> transformed hidden states
            inputs.insert("hidden_states".to_string(), TensorConfig {
                name: "hidden_states".to_string(),
        shape: vec![1, 128, 1024],
                data_type: "FLOAT16".to_string(),
            });
            inputs.insert("position_ids".to_string(), TensorConfig {
                name: "position_ids".to_string(),
        shape: vec![128],
                data_type: "INT32".to_string(),
            });
            inputs.insert("causal_mask".to_string(), TensorConfig {
                name: "causal_mask".to_string(),
        shape: vec![1, 1, 128, 512],
                data_type: "FLOAT16".to_string(),
            });
            // Some CoreML models expect a vector current_pos of length == batch/seq length
            inputs.insert("current_pos".to_string(), TensorConfig {
                name: "current_pos".to_string(),
                shape: vec![128],
                data_type: "INT32".to_string(),
            });
            outputs.insert("output_hidden_states".to_string(), TensorConfig {
                name: "output_hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            });
        } else if filename.contains("lm_head") || filename.contains("head") {
            // LM head: hidden states -> logits
            inputs.insert("hidden_states".to_string(), TensorConfig {
                name: "hidden_states".to_string(),
                shape: vec![1, 1, 1024],
                data_type: "FLOAT16".to_string(),
            });
            outputs.insert("logits".to_string(), TensorConfig {
                name: "logits".to_string(),
                shape: vec![1, 1, 151936], // Common Qwen vocab size
                data_type: "FLOAT16".to_string(),
            });
        } else {
            // Generic fallback - minimal input/output
            inputs.insert("input".to_string(), TensorConfig {
                name: "input".to_string(),
                shape: vec![1, 64],
                data_type: "FLOAT16".to_string(),
            });
            outputs.insert("output".to_string(), TensorConfig {
                name: "output".to_string(),
                shape: vec![1, 64],
                data_type: "FLOAT16".to_string(),
            });
        }
        
        debug!("📖 Created default schema with {} inputs, {} outputs", inputs.len(), outputs.len());
        Ok((inputs, outputs))
    }

    /// Determine execution mode from parsed components
    pub fn infer_execution_mode(&self, components: &[(String, ComponentConfig)]) -> String {
        // Count components with multiple functions
        let multi_function_components = components
            .iter()
            .filter(|(_, config)| config.functions.len() > 1)
            .count();

        // Count total number of function-based components
        let function_based_components = components
            .iter()
            .filter(|(_, config)| !config.functions.is_empty())
            .count();

        if multi_function_components > 0 {
            debug!(
                "🔧 Found {} multi-function components - using unified mode",
                multi_function_components
            );
            "unified".to_string()
        } else if function_based_components > 1 {
            debug!(
                "🔧 Found {} separate function components - using split mode",
                function_based_components
            );
            "split".to_string()
        } else {
            debug!("🔧 Standard component structure - using unified mode");
            "unified".to_string()
        }
    }
}
