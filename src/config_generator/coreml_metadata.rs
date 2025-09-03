//! CoreML metadata extraction from binary model.mlmodel files
//!
//! This module uses Python coremltools to extract tensor metadata for accurate
//! component role detection. We avoid heuristics and prefer exact values from
//! the model specification (MLProgram when available).

use crate::model_config::TensorConfig;
use anyhow::{Error as E, Result};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use tracing::debug;

pub struct CoreMLMetadataExtractor;

impl CoreMLMetadataExtractor {
    pub fn new() -> Self {
        Self
    }

    // Returns: (model_inputs, model_outputs, functions)
    // functions: name -> (inputs, outputs)
    pub fn extract_full_metadata(
        &self,
        model_path: &Path,
    ) -> Result<
        (
            HashMap<String, TensorConfig>,
            HashMap<String, TensorConfig>,
            HashMap<String, (HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)>,
        ),
    > {
        self.extract_full_with_coremltools(model_path)
    }

    pub fn extract_tensor_signatures(
        &self,
        model_path: &Path,
    ) -> Result<(HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)> {
        let (inputs, outputs, _functions) = self.extract_full_with_coremltools(model_path)?;
        Ok((inputs, outputs))
    }

    fn extract_full_with_coremltools(
        &self,
        model_path: &Path,
    ) -> Result<
        (
            HashMap<String, TensorConfig>,
            HashMap<String, TensorConfig>,
            HashMap<String, (HashMap<String, TensorConfig>, HashMap<String, TensorConfig>)>,
        ),
    > {
        debug!(
            "Extracting CoreML metadata with coremltools from: {}",
            model_path.display()
        );

        let python_script = r#"
import coremltools as ct
import json
import sys

# Minimal shape + dtype extractor covering tensorType (MLProgram) and multiArrayType

def extract_shape_and_dtype(t):
    try:
        if hasattr(t, 'tensorType') and t.HasField('tensorType'):
            tt = t.tensorType
            dtype = getattr(tt, 'dataType', None)
            dtype = str(dtype).split('.')[-1] if dtype is not None else 'UNKNOWN'
            dims = []
            if tt.HasField('shape'):
                for d in tt.shape.dimensions:
                    if hasattr(d, 'size') and d.HasField('size'):
                        dims.append(int(d.size))
                    elif hasattr(d, 'enumeratedSizes') and d.HasField('enumeratedSizes'):
                        sizes = list(d.enumeratedSizes.sizes)
                        dims.append(int(max(sizes))) if sizes else dims.append(0)
                    else:
                        dims.append(0)
            return dims, dtype
        if t.HasField('multiArrayType'):
            mat = t.multiArrayType
            shape = list(mat.shape)
            dtype = str(mat.dataType).split('.')[-1]
            return shape, dtype
    except Exception:
        pass
    return [], 'UNKNOWN'

try:
    model_path = sys.argv[1]
    spec = ct.models.utils.load_spec(model_path)

    inputs = {}
    for input_desc in spec.description.input:
        shape, dtype = extract_shape_and_dtype(input_desc.type)
        inputs[input_desc.name] = {'name': input_desc.name, 'shape': shape, 'data_type': dtype}

    outputs = {}
    for output_desc in spec.description.output:
        shape, dtype = extract_shape_and_dtype(output_desc.type)
        outputs[output_desc.name] = {'name': output_desc.name, 'shape': shape, 'data_type': dtype}

    functions = {}
    if hasattr(spec, 'mlProgram') and spec.mlProgram:
        # spec.mlProgram.functions is a map from name to Function proto
        for fname in spec.mlProgram.functions:
            f = spec.mlProgram.functions[fname]
            finputs = {}
            foutputs = {}
            for v in (getattr(f, 'inputs', []) or []):
                shape, dtype = extract_shape_and_dtype(v.type)
                name = getattr(v, 'name', '') or f"in_{len(finputs)}"
                finputs[name] = {'name': name, 'shape': shape, 'data_type': dtype}
            for v in (getattr(f, 'outputs', []) or []):
                shape, dtype = extract_shape_and_dtype(v.type)
                name = getattr(v, 'name', '') or f"out_{len(foutputs)}"
                foutputs[name] = {'name': name, 'shape': shape, 'data_type': dtype}
            functions[fname] = {'inputs': finputs, 'outputs': foutputs}

    print(json.dumps({'inputs': inputs, 'outputs': outputs, 'functions': functions}))

except Exception as e:
    import traceback
    traceback.print_exc()
    print(json.dumps({'error': str(e)}), file=sys.stderr)
    sys.exit(1)
"#;

        let output = Command::new("python3")
            .arg("-c")
            .arg(&python_script)
            .arg(model_path.to_string_lossy().to_string())
            .output()
            .map_err(|e| E::msg(format!("Failed to run Python script: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            debug!("Python coremltools script failed: {}", stderr);
            return Err(E::msg(format!("Python script failed: {}", stderr)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(&stdout)
            .map_err(|e| E::msg(format!("Failed to parse Python output: {}", e)))?;

        if result.get("error").is_some() {
            return Err(E::msg("Python script reported an error"));
        }

        let inputs = self.parse_tensor_configs(result.get("inputs"))?;
        let outputs = self.parse_tensor_configs(result.get("outputs"))?;

        let mut functions_map: HashMap<
            String,
            (
                HashMap<String, TensorConfig>,
                HashMap<String, TensorConfig>,
            ),
        > = HashMap::new();
        if let Some(funcs) = result.get("functions").and_then(|v| v.as_object()) {
            for (fname, fval) in funcs {
                let finputs = self.parse_tensor_configs(fval.get("inputs"))?;
                let foutputs = self.parse_tensor_configs(fval.get("outputs"))?;
                if !(finputs.is_empty() && foutputs.is_empty()) {
                    functions_map.insert(fname.clone(), (finputs, foutputs));
                }
            }
        }

        debug!(
            "Extracted: model IO ({} in, {} out), functions: {}",
            inputs.len(),
            outputs.len(),
            functions_map.len()
        );

        Ok((inputs, outputs, functions_map))
    }

    fn parse_tensor_configs(
        &self,
        tensor_json: Option<&serde_json::Value>,
    ) -> Result<HashMap<String, TensorConfig>> {
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

    fn parse_single_tensor_json(
        &self,
        name: &str,
        tensor_data: &serde_json::Value,
    ) -> Result<Option<TensorConfig>> {
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

        let data_type = tensor_data
            .get("data_type")
            .and_then(|d| d.as_str())
            .unwrap_or("UNKNOWN")
            .to_uppercase();

        Ok(Some(TensorConfig {
            name: name.to_string(),
            shape,
            data_type,
        }))
    }

    pub fn is_coremltools_available(&self) -> bool {
        Command::new("python3")
            .arg("-c")
            .arg("import coremltools")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}