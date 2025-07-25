//! Tensor conversion utilities for CoreML integration

use candle_core::{Device, Error as CandleError, Tensor};

#[cfg(target_os = "macos")]
use block2::StackBlock;
#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Retained};
#[cfg(target_os = "macos")]
use objc2::runtime::{AnyObject, ProtocolObject};
#[cfg(target_os = "macos")]
use objc2::AnyThread;
#[cfg(target_os = "macos")]
use objc2_core_ml::{
    MLDictionaryFeatureProvider, MLFeatureProvider, MLFeatureValue, MLMultiArray,
    MLMultiArrayDataType,
};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSString};
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicBool, Ordering};

/// Tensor to MLMultiArray conversion
#[cfg(target_os = "macos")]
pub fn tensor_to_mlmultiarray(tensor: &Tensor) -> Result<Retained<MLMultiArray>, CandleError> {
    use candle_core::DType;

    let contiguous_tensor = if tensor.is_contiguous() {
        tensor.clone()
    } else {
        tensor.contiguous()?
    };

    let element_count = tensor.elem_count();
    let dims = tensor.dims();
    let mut shape = Vec::with_capacity(dims.len());
    for &dim in dims {
        shape.push(NSNumber::new_usize(dim));
    }
    let shape_nsarray = NSArray::from_retained_slice(&shape);

    // Choose MLMultiArrayDataType based on tensor dtype
    let (ml_data_type, element_size) = match tensor.dtype() {
        DType::F32 => (MLMultiArrayDataType::Float32, std::mem::size_of::<f32>()),
        DType::I64 => (MLMultiArrayDataType::Int32, std::mem::size_of::<i32>()), // Convert I64 to Int32
        _ => {
            return Err(CandleError::Msg(format!(
                "Unsupported tensor dtype {:?} for CoreML conversion. Only F32 and I64 tensors are supported.",
                tensor.dtype()
            )))
        }
    };

    let multi_array_result = unsafe {
        MLMultiArray::initWithShape_dataType_error(
            MLMultiArray::alloc(),
            &shape_nsarray,
            ml_data_type,
        )
    };

    match multi_array_result {
        Ok(ml_array) => {
            let copied = AtomicBool::new(false);

            let flattened_tensor = contiguous_tensor.flatten_all()?;

            // Handle different data types
            match tensor.dtype() {
                DType::F32 => {
                    let data_vec = flattened_tensor.to_vec1::<f32>()?;
                    unsafe {
                        ml_array.getMutableBytesWithHandler(&StackBlock::new(
                            |ptr: std::ptr::NonNull<std::ffi::c_void>, len, _| {
                                let dst = ptr.as_ptr() as *mut f32;
                                let src = data_vec.as_ptr();
                                let copy_elements =
                                    element_count.min(len as usize / element_size);

                                if copy_elements > 0
                                    && len as usize >= copy_elements * element_size
                                {
                                    std::ptr::copy_nonoverlapping(src, dst, copy_elements);
                                    copied.store(true, Ordering::Relaxed);
                                }
                            },
                        ));
                    }
                }
                DType::I64 => {
                    // Convert I64 to I32 for CoreML
                    let data_vec = flattened_tensor.to_vec1::<i64>()?;
                    let i32_data: Vec<i32> = data_vec.into_iter().map(|x| x as i32).collect();

                    unsafe {
                        ml_array.getMutableBytesWithHandler(&StackBlock::new(
                            |ptr: std::ptr::NonNull<std::ffi::c_void>, len, _| {
                                let dst = ptr.as_ptr() as *mut i32;
                                let src = i32_data.as_ptr();
                                let copy_elements =
                                    element_count.min(len as usize / element_size);

                                if copy_elements > 0
                                    && len as usize >= copy_elements * element_size
                                {
                                    std::ptr::copy_nonoverlapping(src, dst, copy_elements);
                                    copied.store(true, Ordering::Relaxed);
                                }
                            },
                        ));
                    }
                }
                _ => unreachable!(), // Already handled above
            }

            if copied.load(Ordering::Relaxed) {
                Ok(ml_array)
            } else {
                Err(CandleError::Msg(
                    "Failed to copy data to MLMultiArray".to_string(),
                ))
            }
        }
        Err(err) => Err(CandleError::Msg(format!(
            "Failed to create MLMultiArray: {:?}",
            err
        ))),
    }
}

/// Create feature provider with multiple named inputs
#[cfg(target_os = "macos")]
pub fn create_multi_feature_provider(
    input_names: &[String],
    input_arrays: &[Retained<MLMultiArray>],
) -> Result<Retained<MLDictionaryFeatureProvider>, CandleError> {
    autoreleasepool(|_| {
        let mut keys = Vec::with_capacity(input_names.len());
        let mut values: Vec<Retained<MLFeatureValue>> = Vec::with_capacity(input_arrays.len());

        for (name, array) in input_names.iter().zip(input_arrays.iter()) {
            let key = NSString::from_str(name);
            let value = unsafe { MLFeatureValue::featureValueWithMultiArray(array) };
            keys.push(key);
            values.push(value);
        }

        let key_refs: Vec<&NSString> = keys.iter().map(|k| &**k).collect();
        let value_refs: Vec<&AnyObject> =
            values.iter().map(|v| v.as_ref() as &AnyObject).collect();
        let dict: Retained<NSDictionary<NSString, AnyObject>> =
            NSDictionary::from_slices::<NSString>(&key_refs, &value_refs);

        unsafe {
            MLDictionaryFeatureProvider::initWithDictionary_error(
                MLDictionaryFeatureProvider::alloc(),
                dict.as_ref(),
            )
        }
        .map_err(|e| CandleError::Msg(format!("CoreML initWithDictionary_error: {:?}", e)))
    })
}

/// Extract output tensor from CoreML prediction result
#[cfg(target_os = "macos")]
pub fn extract_output(
    prediction: &ProtocolObject<dyn MLFeatureProvider>,
    output_name: &str,
    input_device: &Device,
) -> Result<Tensor, CandleError> {
    use std::cell::RefCell;

    autoreleasepool(|_| unsafe {
        let name = NSString::from_str(output_name);
        let value = prediction
            .featureValueForName(&name)
            .ok_or_else(|| CandleError::Msg(format!("Output '{}' not found", output_name)))?;

        let marray = value.multiArrayValue().ok_or_else(|| {
            CandleError::Msg(format!("Output '{}' is not MLMultiArray", output_name))
        })?;

        let count = marray.count() as usize;
        let mut buf = vec![0.0f32; count];

        let buf_cell = RefCell::new(&mut buf);

        marray.getBytesWithHandler(&StackBlock::new(
            |ptr: std::ptr::NonNull<std::ffi::c_void>, len: isize| {
                let src = ptr.as_ptr() as *const f32;
                let copy_elements = count.min(len as usize / std::mem::size_of::<f32>());
                if copy_elements > 0 && len as usize >= copy_elements * std::mem::size_of::<f32>() {
                    if let Ok(mut buf_ref) = buf_cell.try_borrow_mut() {
                        std::ptr::copy_nonoverlapping(src, buf_ref.as_mut_ptr(), copy_elements);
                    }
                }
            },
        ));

        // Get shape from MLMultiArray
        let shape_nsarray = marray.shape();
        let shape_count = shape_nsarray.count();
        let mut shape = Vec::with_capacity(shape_count);

        for i in 0..shape_count {
            let dim_number = shape_nsarray.objectAtIndex(i);
            let dim_value = dim_number.integerValue() as usize;
            shape.push(dim_value);
        }

        // Create tensor with the same device as input
        Tensor::from_vec(buf, shape, input_device)
            .map_err(|e| CandleError::Msg(format!("Failed to create output tensor: {}", e)))
    })
}