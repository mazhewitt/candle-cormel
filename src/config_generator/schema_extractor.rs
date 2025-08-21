//! Schema extraction utilities for CoreML manifest files
//!
//! Handles parsing of input/output tensor schemas from various CoreML manifest formats

use crate::model_config::TensorConfig;
use anyhow::{Error as E, Result};
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

pub struct SchemaExtractor;

impl Default for SchemaExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract input tensor configurations from manifest
    pub fn extract_inputs(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        let mut inputs = HashMap::new();

        // Handle both .mlpackage (Manifest.json) and .mlmodelc (metadata.json) formats
        if let Some(input_schema) = manifest
            .get(0)
            .and_then(|m| m.get("inputSchema").and_then(|s| s.as_array()))
        {
            debug!("📖 Parsing input schema with {} inputs", input_schema.len());

            // Try function-specific schemas first
            if let Some(function_inputs) = self.try_extract_function_inputs(manifest)? {
                return Ok(function_inputs);
            }

            // Parse top-level input schema
            inputs = self.parse_tensor_configs(input_schema)?;
        } else if manifest
            .get("itemInfoEntries")
            .and_then(|v| v.as_object())
            .is_some()
        {
            debug!("📖 Parsing .mlpackage manifest (limited). Using minimal fallback");
            // For generic models, we can't assume specific tensor names
        } else {
            debug!("📖 Unknown manifest format, no inputs extracted");
        }

        debug!("📖 Extracted {} input tensors", inputs.len());
        Ok(inputs)
    }

    /// Extract output tensor configurations from manifest  
    pub fn extract_outputs(&self, manifest: &Value) -> Result<HashMap<String, TensorConfig>> {
        let mut outputs = HashMap::new();

        if let Some(output_schema) = manifest
            .get(0)
            .and_then(|m| m.get("outputSchema").and_then(|s| s.as_array()))
        {
            debug!(
                "📖 Parsing output schema with {} outputs",
                output_schema.len()
            );

            outputs = self.parse_tensor_configs(output_schema)?;

            // Backfill empty shapes from function schemas if needed
            if self.has_empty_shapes(&outputs) {
                self.backfill_from_function_schemas(manifest, &mut outputs)?;
            }
        } else if let Some(functions) = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()))
        {
            debug!(
                "📖 Parsing outputs from functions schema ({} functions)",
                functions.len()
            );
            outputs = self.extract_outputs_from_functions(functions)?;
        } else {
            debug!("📖 Unknown manifest format, no outputs extracted");
        }

        debug!("📖 Extracted {} output tensors", outputs.len());
        Ok(outputs)
    }

    /// Parse tensor configurations from a schema array
    pub fn parse_tensor_configs(&self, schema: &[Value]) -> Result<HashMap<String, TensorConfig>> {
        let mut configs = HashMap::new();

        for tensor_def in schema {
            if let Some(tensor_config) = self.parse_tensor_definition(tensor_def)? {
                configs.insert(tensor_config.name.clone(), tensor_config);
            }
        }

        debug!("📖 Extracted {} tensor configs", configs.len());
        Ok(configs)
    }

    // Private helper methods

    fn try_extract_function_inputs(
        &self,
        manifest: &Value,
    ) -> Result<Option<HashMap<String, TensorConfig>>> {
        let functions = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()));

        if let Some(funcs) = functions {
            // Prefer prefill function shapes if available
            for prefer in ["prefill", "infer"] {
                for function in funcs {
                    if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                        if func_name == prefer {
                            if let Some(input_schema) =
                                function.get("inputSchema").and_then(|s| s.as_array())
                            {
                                debug!(
                                    "📖 Using {} function input schema with {} inputs",
                                    prefer,
                                    input_schema.len()
                                );
                                return Ok(Some(self.parse_tensor_configs(input_schema)?));
                            }
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    fn has_empty_shapes(&self, outputs: &HashMap<String, TensorConfig>) -> bool {
        outputs.values().any(|tensor| tensor.shape.is_empty())
    }

    fn backfill_from_function_schemas(
        &self,
        manifest: &Value,
        outputs: &mut HashMap<String, TensorConfig>,
    ) -> Result<()> {
        let functions = manifest
            .get(0)
            .and_then(|m| m.get("functions").and_then(|f| f.as_array()));

        if let Some(funcs) = functions {
            for prefer in ["prefill", "infer"] {
                for function in funcs {
                    if let Some(func_name) = function.get("name").and_then(|n| n.as_str()) {
                        if func_name == prefer {
                            if let Some(func_output_schema) =
                                function.get("outputSchema").and_then(|s| s.as_array())
                            {
                                self.merge_function_outputs(func_output_schema, outputs)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn merge_function_outputs(
        &self,
        func_output_schema: &[Value],
        outputs: &mut HashMap<String, TensorConfig>,
    ) -> Result<()> {
        let function_outputs = self.parse_tensor_configs(func_output_schema)?;

        for (name, tensor_config) in function_outputs {
            let entry = outputs.entry(name.clone()).or_insert(TensorConfig {
                name: name.clone(),
                shape: vec![],
                data_type: tensor_config.data_type.clone(),
            });

            if entry.shape.is_empty() {
                entry.shape = tensor_config.shape;
                entry.data_type = tensor_config.data_type;
            }
        }

        Ok(())
    }

    fn extract_outputs_from_functions(
        &self,
        functions: &[Value],
    ) -> Result<HashMap<String, TensorConfig>> {
        let mut outputs = HashMap::new();

        for function in functions {
            if let Some(func_output_schema) =
                function.get("outputSchema").and_then(|s| s.as_array())
            {
                let function_outputs = self.parse_tensor_configs(func_output_schema)?;
                outputs.extend(function_outputs);
            }
        }

        Ok(outputs)
    }

    fn parse_tensor_definition(&self, tensor_def: &Value) -> Result<Option<TensorConfig>> {
        let (name, shape_str, data_type) = match (
            tensor_def.get("name").and_then(|n| n.as_str()),
            tensor_def.get("shape").and_then(|s| s.as_str()),
            tensor_def.get("dataType").and_then(|d| d.as_str()),
        ) {
            (Some(n), Some(s), Some(d)) => (n, s, d),
            _ => return Ok(None),
        };

        let shape = self.parse_shape_string(shape_str)?;
        debug!("   Tensor: {} -> {:?} ({})", name, shape, data_type);

        Ok(Some(TensorConfig {
            name: name.to_string(),
            shape,
            data_type: data_type.to_uppercase(),
        }))
    }

    fn parse_shape_string(&self, shape_str: &str) -> Result<Vec<usize>> {
        let trimmed = shape_str.trim_start_matches('[').trim_end_matches(']');
        if trimmed.is_empty() {
            return Ok(vec![]);
        }

        let mut dims = Vec::new();
        for dim_str in trimmed.split(',') {
            match dim_str.trim().parse::<usize>() {
                Ok(dim) => dims.push(dim),
                Err(_) => return Err(E::msg("Failed to parse tensor dimension")),
            }
        }

        Ok(dims)
    }
}
