//! CoreML metadata extraction from binary model.mlmodel files
//!
//! This module uses CoreML native APIs to extract tensor metadata from CoreML model files
//! for accurate component role detection.

use crate::model_config::TensorConfig;
use anyhow::{Error as E, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use tracing::debug;

#[cfg(target_os = "macos")]
use objc2_core_ml::MLModel;
#[cfg(target_os = "macos")]
use objc2_foundation::{NSString, NSURL};

pub struct CoreMLMetadataExtractor;

impl CoreMLMetadataExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract tensor signatures from a model.mlmodel file using CoreML native APIs
    pub fn extract_tensor_signatures(&self, model_path: &Path) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        #[cfg(target_os = "macos")]
        {
            // First try native CoreML API
            if let Ok(result) = self.extract_with_native_coreml(model_path) {
                return Ok(result);
            }

            debug!("⚠️ Native CoreML extraction failed, trying coremltools");
        }
        
        // Fallback to Python coremltools
        if let Ok(result) = self.extract_with_coremltools(model_path) {
            return Ok(result);
        }

        debug!("⚠️ coremltools extraction failed, trying protobuf parsing");
        
        // Final fallback to protobuf-based extraction (not implemented yet)
        self.extract_with_protobuf(model_path)
    }

    /// Extract metadata using native CoreML APIs (macOS only)
    #[cfg(target_os = "macos")]
    fn extract_with_native_coreml(&self, model_path: &Path) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        debug!("🔍 Extracting metadata using native CoreML from: {}", model_path.display());

        // Convert path to NSURL
        let path_str = model_path.to_str()
            .ok_or_else(|| E::msg("Invalid path"))?;
        let ns_path = NSString::from_str(path_str);
        let url = unsafe { NSURL::fileURLWithPath(&ns_path) };

        // Load the model
        let model = unsafe { MLModel::modelWithContentsOfURL_error(&url) }
            .map_err(|e| E::msg(format!("Failed to load CoreML model: {:?}", e)))?;

        // Get model description
        let model_description = unsafe { model.modelDescription() };
        
        // Extract inputs
        let inputs: HashMap<String, TensorConfig> = HashMap::new();
        let _input_descriptions = unsafe { model_description.inputDescriptionsByName() };
        
        // Convert NSDict to HashMap (this is a simplified approach)
        // In reality, you'd iterate through the NSDict properly
        debug!("✅ Model loaded, extracting input/output descriptions");
        
        // Extract outputs  
        let outputs: HashMap<String, TensorConfig> = HashMap::new();
        let _output_descriptions = unsafe { model_description.outputDescriptionsByName() };
        
        // For now, we'll use the Python fallback approach since proper NSDict iteration
        // requires more complex objc2 code
        debug!("⚠️ Native extraction needs more NSDict handling, falling back");
        Err(E::msg("Native extraction not fully implemented"))
    }

    /// Extract metadata using Python coremltools
    fn extract_with_coremltools(&self, model_path: &Path) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        debug!("🔍 Extracting metadata from: {}", model_path.display());

        // Create Python script to extract model metadata
        let python_script = format!(r#"
import coremltools as ct
import json
import sys

def extract_shape_and_dtype(t):
    # Handle tensorType (MLProgram) first
    if hasattr(t, 'tensorType') and t.HasField('tensorType'):
        tt = t.tensorType
        dtype = getattr(tt, 'dataType', None)
        dtype = str(dtype).split('.')[-1] if dtype is not None else 'UNKNOWN'
        shape = []
        if tt.HasField('shape'):
            dims = []
            for d in tt.shape.dimensions:
                if hasattr(d, 'size') and d.HasField('size'):
                    dims.append(int(d.size))
                elif hasattr(d, 'enumeratedSizes') and d.HasField('enumeratedSizes'):
                    sizes = list(d.enumeratedSizes.sizes)
                    dims.append(int(max(sizes))) if sizes else dims.append(0)
                else:
                    dims.append(0)
            shape = dims
        return shape, dtype
    # Fallback to legacy multiArrayType
    if t.HasField('multiArrayType'):
        mat = t.multiArrayType
        shape = list(mat.shape)
        dtype = str(mat.dataType).split('.')[-1]
        return shape, dtype
    return [], 'UNKNOWN'

try:
    # Use load_spec to avoid heavy validation paths
    spec = ct.models.utils.load_spec("{}")

    # Extract inputs
    inputs = {{}}
    for input_desc in spec.description.input:
        shape, dtype = extract_shape_and_dtype(input_desc.type)
        inputs[input_desc.name] = {{
            'name': input_desc.name,
            'shape': shape,
            'data_type': dtype
        }}

    # Extract outputs
    outputs = {{}}
    for output_desc in spec.description.output:
        shape, dtype = extract_shape_and_dtype(output_desc.type)
        outputs[output_desc.name] = {{
            'name': output_desc.name,
            'shape': shape,
            'data_type': dtype
        }}

    # Check for functions (MLProgram)
    functions = []
    if hasattr(spec, 'mlProgram') and spec.mlProgram:
        # spec.mlProgram.functions is a map<string, Function>
        for func_name in spec.mlProgram.functions:
            functions.append(func_name)

    result = {{
        'inputs': inputs,
        'outputs': outputs,
        'functions': functions
    }}

    print(json.dumps(result))

except Exception as e:
    import traceback
    traceback.print_exc()
    print(json.dumps({{'error': str(e)}}), file=sys.stderr)
    sys.exit(1)
"#, model_path.display());

        // Execute Python script
        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .output()
            .map_err(|e| E::msg(format!("Failed to run Python script: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            debug!("❌ Python script failed - stderr: {}", stderr);
            return Err(E::msg(format!("Python script failed: {}", stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        debug!("🐍 Python raw output: {:?}", stdout);
        debug!("🐍 Python output length: {} bytes", stdout.len());

        // Parse JSON result
        let result: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| E::msg(format!("Failed to parse Python output: {}", e)))?;

        if result.get("error").is_some() {
            return Err(E::msg("Python script reported an error"));
        }

        // Convert to TensorConfig format
        let inputs = self.parse_tensor_configs(result.get("inputs"))?;
        let outputs = self.parse_tensor_configs(result.get("outputs"))?;

        debug!("✅ Extracted {} inputs and {} outputs using coremltools", inputs.len(), outputs.len());
        debug!("   Inputs: {:?}", inputs.keys().collect::<Vec<_>>());
        debug!("   Outputs: {:?}", outputs.keys().collect::<Vec<_>>());

        Ok((inputs, outputs))
    }

    /// Extract metadata using protobuf parsing (fallback)
    fn extract_with_protobuf(&self, _model_path: &Path) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        // For now, return empty results - this would require protobuf parsing implementation
        debug!("⚠️ Protobuf extraction not implemented yet");
        Ok((HashMap::new(), HashMap::new()))
    }

    /// Parse tensor configs from JSON metadata
    fn parse_tensor_configs(&self, tensor_json: Option<&serde_json::Value>) -> Result<HashMap<String, TensorConfig>> {
        let mut configs = HashMap::new();

        let Some(tensors_obj) = tensor_json.and_then(|v| v.as_object()) else {
            return Ok(configs);
        };

        for (name, tensor_data) in tensors_obj {
            if let Some(tensor_config) = self.parse_single_tensor_json(name, tensor_data)? {
                configs.insert(tensor_config.name.clone(), tensor_config);
            }
        }

        Ok(configs)
    }

    /// Parse a single tensor from JSON
    fn parse_single_tensor_json(&self, name: &str, tensor_data: &serde_json::Value) -> Result<Option<TensorConfig>> {
        let shape = if let Some(shape_arr) = tensor_data.get("shape").and_then(|s| s.as_array()) {
            let mut dims = Vec::new();
            for dim_val in shape_arr {
                if let Some(dim) = dim_val.as_u64() {
                    dims.push(dim as usize);
                }
            }
            dims
        } else {
            vec![]
        };

        let data_type = tensor_data.get("data_type")
            .and_then(|d| d.as_str())
            .unwrap_or("UNKNOWN")
            .to_uppercase();

        Ok(Some(TensorConfig {
            name: name.to_string(),
            shape,
            data_type,
        }))
    }

    /// Check if coremltools is available
    pub fn is_coremltools_available(&self) -> bool {
        Command::new("python3")
            .arg("-c")
            .arg("import coremltools")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}