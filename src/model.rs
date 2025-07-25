//! Core CoreML model implementation

use crate::config::Config;
use crate::conversion::{create_multi_feature_provider, extract_output, tensor_to_mlmultiarray};
use crate::state::CoreMLState;
use candle_core::{Device, Error as CandleError, Tensor};
use std::path::Path;

#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Retained};
#[cfg(target_os = "macos")]
use objc2::runtime::ProtocolObject;
#[cfg(target_os = "macos")]
use objc2_core_ml::{MLDictionaryFeatureProvider, MLFeatureProvider, MLModel};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSString, NSURL};

/// CoreML model wrapper that provides Candle tensor integration
pub struct CoreMLModel {
    #[cfg(target_os = "macos")]
    inner: Retained<MLModel>,
    #[cfg(not(target_os = "macos"))]
    _phantom: std::marker::PhantomData<()>,
    config: Config,
}

impl std::fmt::Debug for CoreMLModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreMLModel")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl CoreMLModel {
    /// Load a CoreML model from a .mlmodelc directory with default configuration
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, CandleError> {
        let config = Config::default();
        Self::load_from_file(path, &config)
    }

    /// Load a CoreML model from a .mlmodelc directory following standard Candle patterns
    ///
    /// Note: Unlike other Candle models, CoreML models are pre-compiled and don't use VarBuilder.
    /// This method provides a Candle-compatible interface while loading from CoreML files.
    pub fn load_from_file<P: AsRef<Path>>(path: P, config: &Config) -> Result<Self, CandleError> {
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

                match unsafe { MLModel::modelWithContentsOfURL_error(&url) } {
                    Ok(model) => Ok(CoreMLModel {
                        inner: model,
                        config: config.clone(),
                    }),
                    Err(err) => Err(CandleError::Msg(format!(
                        "Failed to load CoreML model: {:?}",
                        err
                    ))),
                }
            })
        }

        #[cfg(not(target_os = "macos"))]
        {
            let _ = (path, config);
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

    /// Get the model configuration
    pub fn config(&self) -> &Config {
        &self.config
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
            let output_tensor = extract_output(&prediction, &self.config.output_name, inputs[0].device())?;

            Ok(output_tensor)
        })
    }

    #[cfg(target_os = "macos")]
    fn run_prediction(
        &self,
        provider: &MLDictionaryFeatureProvider,
    ) -> Result<Retained<ProtocolObject<dyn MLFeatureProvider>>, CandleError> {
        autoreleasepool(|_| unsafe {
            let protocol_provider = ProtocolObject::from_ref(provider);

            self.inner
                .predictionFromFeatures_error(protocol_provider)
                .map_err(|e| CandleError::Msg(format!("CoreML prediction error: {:?}", e)))
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
            let output_tensor = extract_output(&prediction, &self.config.output_name, inputs[0].device())?;

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
                .map_err(|e| CandleError::Msg(format!("CoreML stateful prediction error: {:?}", e)))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::{Device, Tensor};

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