//! Core CoreML model implementation

use crate::config::Config;
use crate::state::CoreMLState;

#[cfg(target_os = "macos")]
use crate::conversion::{
    create_multi_feature_provider, extract_all_outputs, extract_output, tensor_to_mlmultiarray,
};
use candle_core::{Device, Error as CandleError, Tensor};
use std::path::Path;

#[cfg(target_os = "macos")]
use tracing::{debug, info};

#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Retained};
#[cfg(target_os = "macos")]
use objc2::runtime::ProtocolObject;
#[cfg(target_os = "macos")]
use objc2_core_ml::{
    MLDictionaryFeatureProvider, MLFeatureProvider, MLModel, MLModelConfiguration,
};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSString, NSURL};

/// CoreML model wrapper that provides Candle tensor integration
pub struct CoreMLModel {
    #[cfg(target_os = "macos")]
    pub(crate) inner: Retained<MLModel>,
    #[cfg(not(target_os = "macos"))]
    _phantom: std::marker::PhantomData<()>,
    pub(crate) config: Config,
    pub(crate) function_name: Option<String>,
}

impl std::fmt::Debug for CoreMLModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreMLModel")
            .field("config", &self.config)
            .field("function_name", &self.function_name)
            .finish_non_exhaustive()
    }
}

impl CoreMLModel {
    /// Load a CoreML model from a .mlmodelc directory with default configuration
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CandleError> {
        let config = Config::default();
        Self::load_from_file(path, &config)
    }

    /// Load a CoreML model with a specific function name
    pub fn load_with_function<P: AsRef<Path>>(
        path: P,
        config: &Config,
        function_name: &str,
    ) -> Result<Self, CandleError> {
        Self::load_from_file_with_function(path, config, Some(function_name))
    }

    /// Load a CoreML model from a .mlmodelc directory following standard Candle patterns
    ///
    /// Note: Unlike other Candle models, CoreML models are pre-compiled and don't use VarBuilder.
    /// This method provides a Candle-compatible interface while loading from CoreML files.
    pub fn load_from_file<P: AsRef<Path>>(path: P, config: &Config) -> Result<Self, CandleError> {
        Self::load_from_file_with_function(path, config, None)
    }

    /// Load a CoreML model with optional function name specification
    pub fn load_from_file_with_function<P: AsRef<Path>>(
        path: P,
        config: &Config,
        function_name: Option<&str>,
    ) -> Result<Self, CandleError> {
        #[cfg(target_os = "macos")]
        {
            let path = path.as_ref();
            if !path.exists() {
                return Err(CandleError::Msg(format!(
                    "Model file not found: {}",
                    path.display()
                )));
            }

            autoreleasepool(|_| {
                let url =
                    unsafe { NSURL::fileURLWithPath(&NSString::from_str(&path.to_string_lossy())) };

                // Show loading progress for large models
                info!("Loading and compiling CoreML model at {}", path.display());
                let load_start = std::time::Instant::now();

                // Create configuration with function name if provided
                let model_result = if let Some(func_name) = function_name {
                    let config = unsafe { MLModelConfiguration::new() };
                    let ns_func_name = NSString::from_str(func_name);
                    unsafe { config.setFunctionName(Some(&ns_func_name)) };
                    unsafe { MLModel::modelWithContentsOfURL_configuration_error(&url, &config) }
                } else {
                    unsafe { MLModel::modelWithContentsOfURL_error(&url) }
                };

                let load_time = load_start.elapsed();

                // Try to load the model with function name support
                match model_result {
                    Ok(model) => {
                        info!(
                            "Model loaded and compiled in {:.1}s",
                            load_time.as_secs_f32()
                        );
                        Ok(CoreMLModel {
                            inner: model,
                            config: config.clone(),
                            function_name: function_name.map(|s| s.to_string()),
                        })
                    }
                    Err(err) => {
                        // If direct loading fails, try compiling first
                        let err_msg = format!("{err:?}");
                        if err_msg.contains("Compile the model") {
                            debug!("Model requires compilation, compiling now");
                            #[allow(deprecated)]
                            match unsafe { MLModel::compileModelAtURL_error(&url) } {
                                Ok(compiled_url) => {
                                    debug!("Compilation completed, loading compiled model");
                                    // Try loading the compiled model
                                    match unsafe {
                                        MLModel::modelWithContentsOfURL_error(&compiled_url)
                                    } {
                                        Ok(model) => {
                                            info!(
                                                "Compiled model loaded in {:.1}s total",
                                                load_time.as_secs_f32()
                                            );
                                            Ok(CoreMLModel {
                                                inner: model,
                                                config: config.clone(),
                                                function_name: function_name.map(|s| s.to_string()),
                                            })
                                        }
                                        Err(compile_err) => Err(CandleError::Msg(format!(
                                            "Failed to load compiled CoreML model: {compile_err:?}"
                                        ))),
                                    }
                                }
                                Err(compile_err) => Err(CandleError::Msg(format!(
                                    "Failed to compile CoreML model: {compile_err:?}. Original error: {err:?}"
                                ))),
                            }
                        } else {
                            // Check for common CoreML version compatibility issues
                            let err_msg = format!("{err:?}");
                            if err_msg.contains("compiler major version")
                                && err_msg.contains("more recent than this framework")
                            {
                                Err(CandleError::Msg(format!(
                                    "CoreML version compatibility issue: {err_msg}\n\
                                    This model was compiled with a newer CoreML compiler than this system supports.\n\
                                    Solutions:\n\
                                    • Update to a newer macOS version\n\
                                    • Use models compiled for your CoreML framework version\n\
                                    • Set RUST_LOG=debug for more details"
                                )))
                            } else {
                                Err(CandleError::Msg(format!(
                                    "Failed to load CoreML model: {err:?}"
                                )))
                            }
                        }
                    }
                }
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = (path, config, function_name);
            Err(CandleError::Msg(
                "CoreML is only available on macOS".to_string(),
            ))
        }
    }

    /// Run forward pass through the model with multiple inputs
    ///
    /// Accepts tensors from CPU or Metal devices, rejects CUDA tensors.
    /// Returns output tensor on the same device as the input tensors.
    ///
    /// # Arguments
    /// * `inputs` - Slice of tensors corresponding to the input_names in config order
    ///
    /// Convenience method for single-input models (backward compatibility)
    pub fn forward_single(&self, input: &Tensor) -> Result<Tensor, CandleError> {
        self.forward(&[input])
    }

    pub fn forward(&self, inputs: &[&Tensor]) -> Result<Tensor, CandleError> {
        // Validate we have the expected number of inputs
        if inputs.len() != self.config.input_names.len() {
            return Err(CandleError::Msg(format!(
                "Expected {} inputs, got {}. Input names: {:?}",
                self.config.input_names.len(),
                inputs.len(),
                self.config.input_names
            )));
        }

        // Validate all input devices are compatible - accept CPU/Metal, reject CUDA
        for (i, input) in inputs.iter().enumerate() {
            match input.device() {
                Device::Cpu | Device::Metal(_) => {
                    // Valid devices for CoreML
                }
                Device::Cuda(_) => {
                    return Err(CandleError::Msg(format!(
                            "CoreML models do not support CUDA tensors. Input {} '{}' is on CUDA device. Please move tensor to CPU or Metal device first.",
                            i, self.config.input_names[i]
                        )));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            self.forward_impl(inputs)
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = inputs;
            Err(CandleError::Msg(
                "CoreML is only available on macOS".to_string(),
            ))
        }
    }

    /// Forward pass returning all outputs as a HashMap
    ///
    /// This is useful for models that have multiple outputs, such as the Qwen LM head
    /// which produces 16 different logits chunks that need to be concatenated.
    pub fn forward_all(
        &self,
        inputs: &[&Tensor],
    ) -> Result<std::collections::HashMap<String, Tensor>, CandleError> {
        // Validate we have the expected number of inputs
        if inputs.len() != self.config.input_names.len() {
            return Err(CandleError::Msg(format!(
                "Expected {} inputs, got {}. Input names: {:?}",
                self.config.input_names.len(),
                inputs.len(),
                self.config.input_names
            )));
        }

        // Validate all input devices are compatible - accept CPU/Metal, reject CUDA
        for (i, input) in inputs.iter().enumerate() {
            match input.device() {
                Device::Cpu | Device::Metal(_) => {
                    // Valid devices for CoreML
                }
                Device::Cuda(_) => {
                    return Err(CandleError::Msg(format!(
                            "CoreML models do not support CUDA tensors. Input {} '{}' is on CUDA device. Please move tensor to CPU or Metal device first.",
                            i, self.config.input_names[i]
                        )));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            self.forward_all_impl(inputs)
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = inputs;
            Err(CandleError::Msg(
                "CoreML is only available on macOS".to_string(),
            ))
        }
    }

    /// Get the model configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get access to the inner MLModel for advanced usage (testing only)
    #[cfg(target_os = "macos")]
    pub fn inner_model(&self) -> &Retained<MLModel> {
        &self.inner
    }

    /// Create a CoreMLModel from an existing MLModel (for testing)
    #[cfg(target_os = "macos")]
    pub fn from_mlmodel(inner: Retained<MLModel>, config: Config) -> Self {
        CoreMLModel {
            inner,
            config,
            function_name: None,
        }
    }

    /// Create a fresh state object for this model.
    ///
    /// This enables efficient autoregressive generation by maintaining
    /// persistent KV-cache across multiple prediction calls.
    ///
    /// # Returns
    ///
    /// A new `CoreMLState` instance that can be used with `predict_with_state()`.
    /// For stateless models, this returns an empty state object that can still
    /// be used with stateful prediction methods (resulting in stateless behavior).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use candle_core::{Device, Tensor};
    /// use candle_coreml::{CoreMLModel, Config};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let model = CoreMLModel::load("model.mlmodelc")?;
    ///
    /// // Create state for efficient token generation
    /// let mut state = model.make_state()?;
    ///
    /// // Use state with predict_with_state() for streaming inference
    /// # Ok(())
    /// # }
    /// ```
    pub fn make_state(&self) -> Result<CoreMLState, CandleError> {
        #[cfg(target_os = "macos")]
        {
            CoreMLState::new(&self.inner)
        }

        #[cfg(not(target_os = "macos"))]
        {
            CoreMLState::new(&())
        }
    }

    /// Run forward pass through the model with persistent state.
    ///
    /// This method enables efficient autoregressive generation by maintaining
    /// KV-cache state across multiple prediction calls. Unlike the stateless
    /// `forward()` method, this preserves computation state between calls.
    ///
    /// # Arguments
    ///
    /// * `inputs` - Slice of tensors corresponding to input_names in config order
    /// * `state` - Mutable reference to the model state (will be updated)
    ///
    /// # Returns
    ///
    /// Output tensor on the same device as the input tensors.
    ///
    /// # Device Compatibility
    ///
    /// Accepts tensors from CPU or Metal devices, rejects CUDA tensors.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use candle_core::{Device, Tensor};
    /// use candle_coreml::{CoreMLModel, Config};
    ///
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let model = CoreMLModel::load("model.mlmodelc")?;
    /// let device = Device::Cpu;
    ///
    /// let mut state = model.make_state()?;
    ///
    /// // Generate tokens with persistent KV-cache
    /// for i in 0..10 {
    ///     let input = Tensor::ones((1, 1), candle_core::DType::I64, &device)?;
    ///     let output = model.predict_with_state(&[&input], &mut state)?;
    ///     println!("Token {}: {:?}", i, output);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn predict_with_state(
        &self,
        inputs: &[&Tensor],
        state: &mut CoreMLState,
    ) -> Result<Tensor, CandleError> {
        // Validate we have the expected number of inputs
        if inputs.len() != self.config.input_names.len() {
            return Err(CandleError::Msg(format!(
                "Expected {} inputs, got {}. Input names: {:?}",
                self.config.input_names.len(),
                inputs.len(),
                self.config.input_names
            )));
        }

        // Validate all input devices are compatible - accept CPU/Metal, reject CUDA
        for (i, input) in inputs.iter().enumerate() {
            match input.device() {
                Device::Cpu | Device::Metal(_) => {
                    // Valid devices for CoreML
                }
                Device::Cuda(_) => {
                    return Err(CandleError::Msg(format!(
                            "CoreML models do not support CUDA tensors. Input {} '{}' is on CUDA device. Please move tensor to CPU or Metal device first.",
                            i, self.config.input_names[i]
                        )));
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            // Debug print input shapes and names
            tracing::debug!("predict_with_state function={:?}", self.function_name);
            for (i, t) in inputs.iter().enumerate() {
                tracing::debug!(
                    "predict_with_state input {} '{}' shape={:?}",
                    i,
                    self.config.input_names[i],
                    t.dims()
                );
            }
            self.predict_with_state_impl(inputs, state)
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = (inputs, state);
            Err(CandleError::Msg(
                "CoreML is only available on macOS".to_string(),
            ))
        }
    }

    #[cfg(target_os = "macos")]
    fn forward_impl(&self, inputs: &[&Tensor]) -> Result<Tensor, CandleError> {
        autoreleasepool(|_| {
            // Convert all Candle tensors to MLMultiArrays
            let mut ml_arrays = Vec::with_capacity(inputs.len());
            for input in inputs {
                let ml_array = tensor_to_mlmultiarray(input)?;
                ml_arrays.push(ml_array);
            }

            // Create feature provider with all named inputs
            let provider = create_multi_feature_provider(&self.config.input_names, &ml_arrays)?;

            // Run prediction
            let prediction = self.run_prediction(&provider)?;

            // Extract output with configured output name (use first input device for output)
            let output_tensor =
                extract_output(&prediction, &self.config.output_name, inputs[0].device())?;

            Ok(output_tensor)
        })
    }

    #[cfg(target_os = "macos")]
    fn forward_all_impl(
        &self,
        inputs: &[&Tensor],
    ) -> Result<std::collections::HashMap<String, Tensor>, CandleError> {
        autoreleasepool(|_| {
            // Convert all Candle tensors to MLMultiArrays
            let mut ml_arrays = Vec::with_capacity(inputs.len());
            for input in inputs {
                let ml_array = tensor_to_mlmultiarray(input)?;
                ml_arrays.push(ml_array);
            }

            // Create feature provider with all named inputs
            let provider = create_multi_feature_provider(&self.config.input_names, &ml_arrays)?;

            // Run prediction
            let prediction = self.run_prediction(&provider)?;

            // Extract all outputs
            extract_all_outputs(&prediction, inputs[0].device())
        })
    }

    #[cfg(target_os = "macos")]
    fn run_prediction(
        &self,
        provider: &MLDictionaryFeatureProvider,
    ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, CandleError> {
        autoreleasepool(|_| unsafe {
            let protocol_provider = ProtocolObject::from_ref(provider);

            // Function name is now handled during model loading via MLModelConfiguration
            self.inner
                .predictionFromFeatures_error(protocol_provider)
                .map_err(|e| CandleError::Msg(format!("CoreML prediction error: {e:?}")))
        })
    }

    #[cfg(target_os = "macos")]
    fn predict_with_state_impl(
        &self,
        inputs: &[&Tensor],
        state: &mut CoreMLState,
    ) -> Result<Tensor, CandleError> {
        autoreleasepool(|_| {
            // Convert all Candle tensors to MLMultiArrays (reuse existing logic)
            let mut ml_arrays = Vec::with_capacity(inputs.len());
            for input in inputs {
                let ml_array = tensor_to_mlmultiarray(input)?;
                ml_arrays.push(ml_array);
            }

            // Create feature provider with all named inputs (reuse existing logic)
            let provider = create_multi_feature_provider(&self.config.input_names, &ml_arrays)?;

            // Run stateful prediction
            let prediction = self.run_prediction_with_state(&provider, state)?;

            // Extract output with configured output name (use first input device for output)
            let output_tensor =
                extract_output(&prediction, &self.config.output_name, inputs[0].device())?;

            Ok(output_tensor)
        })
    }

    #[cfg(target_os = "macos")]
    fn run_prediction_with_state(
        &self,
        provider: &MLDictionaryFeatureProvider,
        state: &mut CoreMLState,
    ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, CandleError> {
        autoreleasepool(|_| unsafe {
            let protocol_provider = ProtocolObject::from_ref(provider);

            self.inner
                .predictionFromFeatures_usingState_error(protocol_provider, state.inner())
                .map_err(|e| CandleError::Msg(format!("CoreML stateful prediction error: {e:?}")))
        })
    }
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "macos")]
    use super::*;

    #[test]
    #[cfg(target_os = "macos")]
    fn test_model_creation() {
        // This test requires an actual .mlmodelc file
        // Skip if file doesn't exist
        let model_path = "models/test.mlmodelc";
        if !std::path::Path::new(model_path).exists() {
            return;
        }

        let config = Config::default();
        let device = Device::Cpu;

        let model = CoreMLModel::load_from_file(model_path, &config).expect("Failed to load model");

        // Test config access
        assert_eq!(model.config().input_names[0], "input_ids");

        // Test with dummy input tensor on CPU device
        let input = Tensor::ones((1, 10), candle_core::DType::F32, &device)
            .expect("Failed to create input tensor");

        // This will fail without a real model but tests the interface
        let _result = model.forward_single(&input);
    }
}
