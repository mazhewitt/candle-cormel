//! CoreML state management for autoregressive inference

use candle_core::Error as CandleError;

#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Retained};
#[cfg(target_os = "macos")]
use objc2_core_ml::{MLModel, MLState};

/// Opaque wrapper around Core ML's `MLState` for stateful inference.
///
/// This provides persistent state management for autoregressive models,
/// enabling efficient KV-cache reuse across token generation steps.
///
/// # Thread Safety
///
/// Each `CoreMLState` instance must be used by only one thread at a time.
/// Concurrent predictions using the same state object result in undefined behavior.
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
/// // Create state for efficient autoregressive generation
/// let mut state = model.make_state()?;
///
/// // Generate tokens sequentially with persistent KV-cache
/// for i in 0..10 {
///     let input = Tensor::ones((1, 1), candle_core::DType::I64, &device)?;
///     let output = model.predict_with_state(&[&input], &mut state)?;
///     // Process output...
/// }
/// # Ok(())
/// # }
/// ```
#[cfg(target_os = "macos")]
pub struct CoreMLState {
    inner: Retained<MLState>,
}

#[cfg(not(target_os = "macos"))]
pub struct CoreMLState {
    _phantom: std::marker::PhantomData<()>,
}

impl std::fmt::Debug for CoreMLState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CoreMLState").finish_non_exhaustive()
    }
}

#[cfg(target_os = "macos")]
impl CoreMLState {
    /// Create a new state object for the given CoreML model.
    ///
    /// # Arguments
    ///
    /// * `model` - Reference to the MLModel to create state for
    ///
    /// # Returns
    ///
    /// A new `CoreMLState` instance, or an error if state creation fails.
    /// For stateless models, this returns an empty state object that can
    /// still be used with stateful prediction methods.
    pub(crate) fn new(model: &Retained<MLModel>) -> Result<Self, CandleError> {
        autoreleasepool(|_| {
            let state = unsafe { model.newState() };
            Ok(CoreMLState { inner: state })
        })
    }

    /// Get a reference to the underlying MLState for CoreML operations.
    pub(crate) fn inner(&self) -> &MLState {
        &self.inner
    }
}

#[cfg(not(target_os = "macos"))]
impl CoreMLState {
    pub(crate) fn new(_model: &()) -> Result<Self, CandleError> {
        Err(CandleError::Msg(
            "CoreML state is only available on macOS".to_string(),
        ))
    }
}
