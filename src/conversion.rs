//! Tensor conversion utilities for CoreML integration

use candle_core::{Device, Error as CandleError, Tensor};

#[cfg(target_os = "macos")]
use half::f16;
#[cfg(target_os = "macos")]
use tracing::{debug, trace};

#[cfg(target_os = "macos")]
use objc2_core_ml::{MLMultiArray, MLMultiArrayDataType};

#[cfg(target_os = "macos")]
use block2::StackBlock;
#[cfg(target_os = "macos")]
use objc2::rc::{autoreleasepool, Retained};
#[cfg(target_os = "macos")]
use objc2::runtime::{AnyObject, ProtocolObject};
#[cfg(target_os = "macos")]
use objc2::AnyThread;
#[cfg(target_os = "macos")]
use objc2_core_ml::{MLDictionaryFeatureProvider, MLFeatureProvider, MLFeatureValue};
#[cfg(target_os = "macos")]
use objc2_foundation::{NSArray, NSDictionary, NSNumber, NSString};

/// Extract tensor data from MLMultiArray with proper type handling
/// This function handles all the different MLMultiArray data types including Float16
#[cfg(target_os = "macos")]
pub fn convert_mlmultiarray_to_tensor(
    marray: &MLMultiArray,
    input_device: &Device,
) -> Result<Tensor, CandleError> {
    unsafe {
        // Get shape
        let shape_nsarray = marray.shape();
        let mut shape = Vec::with_capacity(shape_nsarray.count());
        for i in 0..shape_nsarray.count() {
            let dim_number = shape_nsarray.objectAtIndex(i);
            let dim_value = dim_number.integerValue() as usize;
            shape.push(dim_value);
        }

        // Extract data with proper type handling
        let data_type = marray.dataType();
        let data_type_raw = data_type.0;
        let count = marray.count() as usize;
        let mut buf = Vec::with_capacity(count);

        match data_type {
            MLMultiArrayDataType::Float32 => {
                // Extract as float values
                for i in 0..count {
                    let val = marray.objectAtIndexedSubscript(i as isize).floatValue();
                    buf.push(val);
                }
            }
            MLMultiArrayDataType::Int32 => {
                // Extract as integer values and convert to float
                for i in 0..count {
                    let val = marray.objectAtIndexedSubscript(i as isize).intValue() as f32;
                    buf.push(val);
                }
            }
            MLMultiArrayDataType::Double => {
                // Extract as double values and convert to float
                for i in 0..count {
                    let val = marray.objectAtIndexedSubscript(i as isize).doubleValue() as f32;
                    buf.push(val);
                }
            }
            _ => {
                // Handle Float16 and other unknown types
                if data_type_raw == 65552 {
                    // Float16 type - extract raw bytes and convert properly
                    trace!("Detected Float16 data type (65552), using proper half-precision conversion");

                    // Get raw bytes from MLMultiArray
                    #[allow(deprecated)]
                    let data_ptr = marray.dataPointer();
                    let byte_slice =
                        std::slice::from_raw_parts(data_ptr.as_ptr().cast::<u8>(), count * 2); // 2 bytes per f16

                    // Convert f16 bytes to f32
                    for i in 0..count {
                        let byte_offset = i * 2;
                        let f16_bytes = [byte_slice[byte_offset], byte_slice[byte_offset + 1]];
                        let f16_bits = u16::from_le_bytes(f16_bytes);
                        let f16_val = f16::from_bits(f16_bits);
                        buf.push(f16_val.to_f32());
                    }
                } else {
                    // For other unknown types, try floatValue as fallback
                    trace!(
                        "Unknown MLMultiArray data type: {:?} (raw: {}), using floatValue fallback",
                        data_type, data_type_raw
                    );

                    for i in 0..count {
                        let val = marray.objectAtIndexedSubscript(i as isize).floatValue();
                        buf.push(val);
                    }
                }
            }
        }

        Tensor::from_vec(buf, shape, input_device)
    }
}
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
                                let copy_elements = element_count.min(len as usize / element_size);

                                if copy_elements > 0 && len as usize >= copy_elements * element_size
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
                                let copy_elements = element_count.min(len as usize / element_size);

                                if copy_elements > 0 && len as usize >= copy_elements * element_size
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
            "Failed to create MLMultiArray: {err:?}"
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
        let value_refs: Vec<&AnyObject> = values.iter().map(|v| v.as_ref() as &AnyObject).collect();
        let dict: Retained<NSDictionary<NSString, AnyObject>> =
            NSDictionary::from_slices::<NSString>(&key_refs, &value_refs);

        unsafe {
            MLDictionaryFeatureProvider::initWithDictionary_error(
                MLDictionaryFeatureProvider::alloc(),
                dict.as_ref(),
            )
        }
        .map_err(|e| CandleError::Msg(format!("CoreML initWithDictionary_error: {e:?}")))
    })
}

/// Extract output tensor from CoreML prediction result
/// Extract all outputs from a CoreML prediction
///
/// This is useful for models with multiple outputs, such as the Qwen LM head
/// which produces 16 different logits chunks.
#[cfg(target_os = "macos")]
pub fn extract_all_outputs(
    prediction: &ProtocolObject<dyn MLFeatureProvider>,
    input_device: &Device,
) -> Result<std::collections::HashMap<String, Tensor>, CandleError> {
    autoreleasepool(|pool| unsafe {
        let mut outputs = std::collections::HashMap::new();

        let feature_names = prediction.featureNames();
        let feature_names_iter = feature_names.iter();

        for feature_name in feature_names_iter {
            let feature_name_str = feature_name.to_str(pool);

            let value = prediction
                .featureValueForName(&feature_name)
                .ok_or_else(|| {
                    CandleError::Msg(format!("Output '{feature_name_str}' not found"))
                })?;

            let marray = value.multiArrayValue().ok_or_else(|| {
                CandleError::Msg(format!("Output '{feature_name_str}' is not MLMultiArray"))
            })?;

            // Get shape
            let shape_nsarray = marray.shape();
            let mut shape = Vec::with_capacity(shape_nsarray.count());
            for i in 0..shape_nsarray.count() {
                let dim_number = shape_nsarray.objectAtIndex(i);
                let dim_value = dim_number.integerValue() as usize;
                shape.push(dim_value);
            }

            // Use the shared conversion function with proper Float16 handling
            let tensor = convert_mlmultiarray_to_tensor(&marray, input_device).map_err(|e| {
                CandleError::Msg(format!(
                    "Failed to create output tensor '{feature_name_str}': {e}"
                ))
            })?;

            outputs.insert(feature_name_str.to_string(), tensor);
        }

        Ok(outputs)
    })
}

#[cfg(target_os = "macos")]
pub fn extract_output(
    prediction: &ProtocolObject<dyn MLFeatureProvider>,
    output_name: &str,
    input_device: &Device,
) -> Result<Tensor, CandleError> {
    use objc2_foundation::NSString;

    autoreleasepool(|_| unsafe {
        let name = NSString::from_str(output_name);
        let value = prediction
            .featureValueForName(&name)
            .ok_or_else(|| CandleError::Msg(format!("Output '{output_name}' not found")))?;

        let marray = value.multiArrayValue().ok_or_else(|| {
            CandleError::Msg(format!("Output '{output_name}' is not MLMultiArray"))
        })?;

        // Use the shared conversion function with proper Float16 handling
        convert_mlmultiarray_to_tensor(&marray, input_device)
    })
}

// Non-macOS fallback implementations
// These functions provide stub implementations that return appropriate errors
// when CoreML functionality is not available on the target platform

#[cfg(not(target_os = "macos"))]
/// Placeholder type for MLMultiArray on non-macOS platforms
pub type MLMultiArrayStub = ();

#[cfg(not(target_os = "macos"))]
/// Placeholder type for MLDictionaryFeatureProvider on non-macOS platforms  
pub type MLFeatureProviderStub = ();

#[cfg(not(target_os = "macos"))]
pub fn tensor_to_mlmultiarray(_tensor: &Tensor) -> Result<MLMultiArrayStub, CandleError> {
    Err(CandleError::Msg(
        "CoreML tensor conversion is only available on macOS".to_string(),
    ))
}

#[cfg(not(target_os = "macos"))]
pub fn create_multi_feature_provider(
    _input_names: &[String],
    _arrays: &[MLMultiArrayStub],
) -> Result<MLFeatureProviderStub, CandleError> {
    Err(CandleError::Msg(
        "CoreML feature provider is only available on macOS".to_string(),
    ))
}

#[cfg(not(target_os = "macos"))]
pub fn extract_output(
    _prediction: &MLFeatureProviderStub,
    _output_name: &str,
    _input_device: &Device,
) -> Result<Tensor, CandleError> {
    Err(CandleError::Msg(
        "CoreML extraction is only available on macOS".to_string(),
    ))
}

#[cfg(not(target_os = "macos"))]
pub fn extract_all_outputs(
    _prediction: &MLFeatureProviderStub,
    _input_device: &Device,
) -> Result<std::collections::HashMap<String, Tensor>, CandleError> {
    Err(CandleError::Msg(
        "CoreML extraction is only available on macOS".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    #[cfg(target_os = "macos")]
    mod macos_tests {
        use crate::conversion::convert_mlmultiarray_to_tensor;
        use candle_core::Device;
        use objc2::rc::Retained;
        use objc2::AnyThread;
        use objc2_core_ml::{MLMultiArray, MLMultiArrayDataType};
        use objc2_foundation::{NSArray, NSNumber};

        /// Helper function to create a test MLMultiArray with Float32 data
        fn create_test_float32_array(data: &[f32], shape: &[usize]) -> Retained<MLMultiArray> {
            let shape_numbers: Vec<Retained<NSNumber>> =
                shape.iter().map(|&dim| NSNumber::new_usize(dim)).collect();
            let shape_refs: Vec<&NSNumber> = shape_numbers.iter().map(|n| n.as_ref()).collect();
            let shape_array = NSArray::from_slice(&shape_refs);

            let array = unsafe {
                MLMultiArray::initWithShape_dataType_error(
                    MLMultiArray::alloc(),
                    &shape_array,
                    MLMultiArrayDataType::Float32,
                )
                .unwrap()
            };

            // Fill with test data
            for (i, &value) in data.iter().enumerate() {
                let number = NSNumber::new_f32(value);
                unsafe {
                    array.setObject_atIndexedSubscript(&number, i as isize);
                }
            }

            array
        }

        /// Helper function to create a test MLMultiArray with Int32 data
        fn create_test_int32_array(data: &[i32], shape: &[usize]) -> Retained<MLMultiArray> {
            let shape_numbers: Vec<Retained<NSNumber>> =
                shape.iter().map(|&dim| NSNumber::new_usize(dim)).collect();
            let shape_refs: Vec<&NSNumber> = shape_numbers.iter().map(|n| n.as_ref()).collect();
            let shape_array = NSArray::from_slice(&shape_refs);

            let array = unsafe {
                MLMultiArray::initWithShape_dataType_error(
                    MLMultiArray::alloc(),
                    &shape_array,
                    MLMultiArrayDataType::Int32,
                )
                .unwrap()
            };

            // Fill with test data
            for (i, &value) in data.iter().enumerate() {
                let number = NSNumber::new_i32(value);
                unsafe {
                    array.setObject_atIndexedSubscript(&number, i as isize);
                }
            }

            array
        }

        /// Create an MLMultiArray that contains actual f16 binary data
        /// This creates the exact memory layout that CoreML produces for MLMultiArrayDataType(65552)
        fn create_actual_float16_array(data: &[f32], shape: &[usize]) -> Retained<MLMultiArray> {
            use half::f16;

            let shape_numbers: Vec<Retained<NSNumber>> =
                shape.iter().map(|&dim| NSNumber::new_usize(dim)).collect();
            let shape_refs: Vec<&NSNumber> = shape_numbers.iter().map(|n| n.as_ref()).collect();
            let shape_array = NSArray::from_slice(&shape_refs);

            // Create MLMultiArray with Float32 type - we'll overwrite with f16 binary data
            let array = unsafe {
                MLMultiArray::initWithShape_dataType_error(
                    MLMultiArray::alloc(),
                    &shape_array,
                    MLMultiArrayDataType::Float16,
                )
                .unwrap()
            };

            // Write actual f16 binary data into the MLMultiArray memory
            unsafe {
                #[allow(deprecated)]
                let data_ptr = array.dataPointer();
                let byte_ptr = data_ptr.as_ptr() as *mut u8;

                for (i, &f32_val) in data.iter().enumerate() {
                    // Convert f32 to f16 and get raw bytes
                    let f16_val = f16::from_f32(f32_val);
                    let f16_bytes = f16_val.to_bits().to_le_bytes();

                    // Write f16 bytes directly into memory (2 bytes per f16)
                    *byte_ptr.add(i * 2) = f16_bytes[0];
                    *byte_ptr.add(i * 2 + 1) = f16_bytes[1];
                }
            }

            array
        }

        #[test]
        fn test_convert_float32_1d_array() {
            let device = Device::Cpu;
            let test_data = vec![1.0, 2.5, -3.0, 4.25];
            let shape = vec![4];

            let mlarray = create_test_float32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify shape matches expected dimensions
            assert_eq!(tensor.shape().dims(), &[4]);

            // Verify tensor data matches input MLMultiArray values
            let tensor_data = tensor.to_vec1::<f32>().unwrap();
            assert_eq!(tensor_data, vec![1.0, 2.5, -3.0, 4.25]);

            // Verify exact float precision preservation
            for (i, &expected) in test_data.iter().enumerate() {
                assert!(
                    (tensor_data[i] - expected).abs() < f32::EPSILON,
                    "Value at index {} differs: expected {}, got {}",
                    i,
                    expected,
                    tensor_data[i]
                );
            }
        }

        #[test]
        fn test_convert_float32_2d_array() {
            let device = Device::Cpu;
            let test_data = vec![1.0, 2.0, 3.0, 4.0];
            let shape = vec![2, 2];

            let mlarray = create_test_float32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify 2D shape is preserved correctly
            assert_eq!(tensor.shape().dims(), &[2, 2]);
            assert_eq!(tensor.elem_count(), 4);

            // Verify 2D tensor data structure and values
            let tensor_data = tensor.to_vec2::<f32>().unwrap();
            let expected_2d = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
            assert_eq!(tensor_data, expected_2d);

            // Verify row-major ordering is maintained
            assert_eq!(tensor_data[0][0], 1.0); // First element
            assert_eq!(tensor_data[0][1], 2.0); // Second element (same row)
            assert_eq!(tensor_data[1][0], 3.0); // Third element (next row)
            assert_eq!(tensor_data[1][1], 4.0); // Fourth element
        }

        #[test]
        fn test_convert_int32_array() {
            let device = Device::Cpu;
            let test_data = vec![1, -2, 3, 4];
            let shape = vec![4];

            let mlarray = create_test_int32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify shape is preserved
            assert_eq!(tensor.shape().dims(), &[4]);

            // Verify Int32 → Float32 conversion works correctly
            let tensor_data = tensor.to_vec1::<f32>().unwrap();
            assert_eq!(tensor_data, vec![1.0, -2.0, 3.0, 4.0]);

            // Verify each int32 value converts to exact f32 equivalent
            for (i, &int_val) in test_data.iter().enumerate() {
                let expected_f32 = int_val as f32;
                assert_eq!(
                    tensor_data[i], expected_f32,
                    "Int32 conversion failed at index {}: {} → {}, expected {}",
                    i, int_val, tensor_data[i], expected_f32
                );
            }

            // Verify negative values are handled correctly
            assert!(
                tensor_data[1] < 0.0,
                "Negative int32 value should remain negative after conversion"
            );
        }

        #[test]
        fn test_convert_3d_array() {
            let device = Device::Cpu;
            let test_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
            let shape = vec![2, 2, 2];

            let mlarray = create_test_float32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify 3D shape dimensions are correct
            assert_eq!(tensor.shape().dims(), &[2, 2, 2]);
            assert_eq!(tensor.elem_count(), 8);
            assert_eq!(tensor.rank(), 3);

            // Verify 3D tensor structure and all values
            let tensor_data = tensor.to_vec3::<f32>().unwrap();
            let expected = vec![
                vec![vec![1.0, 2.0], vec![3.0, 4.0]],
                vec![vec![5.0, 6.0], vec![7.0, 8.0]],
            ];
            assert_eq!(tensor_data, expected);

            // Verify 3D indexing works correctly
            assert_eq!(tensor_data[0][0][0], 1.0); // First element
            assert_eq!(tensor_data[0][0][1], 2.0); // Second element
            assert_eq!(tensor_data[0][1][0], 3.0); // Third element
            assert_eq!(tensor_data[1][1][1], 8.0); // Last element

            // Verify all dimensions have expected size
            assert_eq!(tensor_data.len(), 2); // First dimension
            assert_eq!(tensor_data[0].len(), 2); // Second dimension
            assert_eq!(tensor_data[0][0].len(), 2); // Third dimension
        }

        #[test]
        fn test_convert_large_values() {
            let device = Device::Cpu;
            let test_data = vec![f32::MAX, f32::MIN, 0.0];
            let shape = vec![3];

            let mlarray = create_test_float32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify shape
            assert_eq!(tensor.shape().dims(), &[3]);

            // Verify extreme float values are preserved exactly
            let tensor_data = tensor.to_vec1::<f32>().unwrap();
            assert_eq!(
                tensor_data[0],
                f32::MAX,
                "f32::MAX should be preserved exactly"
            );
            assert_eq!(
                tensor_data[1],
                f32::MIN,
                "f32::MIN should be preserved exactly"
            );
            assert_eq!(tensor_data[2], 0.0, "Zero should be preserved exactly");

            // Verify the values are actually the extreme values we expect
            assert!(
                tensor_data[0] > 3.4e38,
                "MAX value should be extremely large"
            );
            assert!(
                tensor_data[1] < -3.4e38,
                "MIN value should be extremely negative"
            );
            assert_eq!(tensor_data[2], 0.0);

            // Verify no NaN or infinite values were introduced
            for &val in &tensor_data {
                if val != 0.0 {
                    // Skip zero check since it's finite
                    assert!(!val.is_nan(), "No NaN values should be present: {val}");
                    // MAX/MIN are finite extreme values, not infinity
                    assert!(
                        val.is_finite() || val == f32::MAX || val == f32::MIN,
                        "Values should be finite or expected extremes: {val}"
                    );
                }
            }
        }

        #[test]
        fn test_convert_mixed_int_conversion() {
            let device = Device::Cpu;
            let test_data = vec![i32::MAX, i32::MIN, 0, -1, 1];
            let shape = vec![5];

            let mlarray = create_test_int32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify shape
            assert_eq!(tensor.shape().dims(), &[5]);

            // Verify extreme int32 values convert correctly to f32
            let tensor_data = tensor.to_vec1::<f32>().unwrap();
            assert_eq!(tensor_data[0], i32::MAX as f32, "i32::MAX conversion");
            assert_eq!(tensor_data[1], i32::MIN as f32, "i32::MIN conversion");
            assert_eq!(tensor_data[2], 0.0, "Zero conversion");
            assert_eq!(tensor_data[3], -1.0, "Negative one conversion");
            assert_eq!(tensor_data[4], 1.0, "Positive one conversion");

            // Verify extreme values are in expected ranges
            assert!(
                tensor_data[0] > 2.0e9,
                "i32::MAX should convert to ~2.1 billion"
            );
            assert!(
                tensor_data[1] < -2.0e9,
                "i32::MIN should convert to ~-2.1 billion"
            );

            // Verify signs are preserved correctly
            assert!(tensor_data[0] > 0.0, "i32::MAX should be positive");
            assert!(tensor_data[1] < 0.0, "i32::MIN should be negative");
            assert_eq!(tensor_data[2], 0.0, "Zero should remain zero");
            assert!(
                tensor_data[3] < 0.0,
                "Negative values should remain negative"
            );
            assert!(
                tensor_data[4] > 0.0,
                "Positive values should remain positive"
            );

            // Verify no precision loss for small integers
            assert_eq!(tensor_data[2] as i32, 0);
            assert_eq!(tensor_data[3] as i32, -1);
            assert_eq!(tensor_data[4] as i32, 1);
        }

        #[test]
        fn test_convert_empty_array() {
            let device = Device::Cpu;
            let test_data = vec![];
            let shape = vec![0];

            let mlarray = create_test_float32_array(&test_data, &shape);
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify empty array shape
            assert_eq!(tensor.shape().dims(), &[0]);
            assert_eq!(tensor.elem_count(), 0);
            assert_eq!(tensor.rank(), 1); // Still 1D, just with 0 elements

            // Verify empty tensor contains no data
            let tensor_data = tensor.to_vec1::<f32>().unwrap();
            assert_eq!(tensor_data, Vec::<f32>::new());
            assert!(
                tensor_data.is_empty(),
                "Empty tensor should have no elements"
            );
            assert_eq!(tensor_data.len(), 0, "Empty tensor length should be 0");

            // Verify tensor is valid but empty
            assert!(
                !tensor.shape().dims().is_empty(),
                "Tensor should have at least one dimension"
            );
            assert_eq!(
                tensor.shape().dims()[0],
                0,
                "First dimension should be 0 for empty tensor"
            );
        }

        #[test]
        fn test_convert_actual_float16_mlarray() {
            use half::f16;
            use tracing::{debug, info};

            let device = Device::Cpu;

            // Test values that demonstrate f16 precision behavior
            let test_values = vec![
                std::f32::consts::PI, // π - loses precision in f16
                1.0,                  // Should be exact
                -2.5,                 // Simple negative value
                0.0,                  // Zero
                65504.0,              // f16::MAX
                -1.0,                 // Negative one
            ];

            let shape = vec![6];

            // Create MLMultiArray with actual f16 binary data and Float16 data type
            let mlarray = create_actual_float16_array(&test_values, &shape);

            // Calculate expected results (what we should get after f16 conversion)
            let expected_results: Vec<f32> = test_values
                .iter()
                .map(|&val| f16::from_f32(val).to_f32())
                .collect();

            // TEST: Direct conversion using convert_mlmultiarray_to_tensor
            // This should now properly detect the Float16 data type and convert correctly
            let tensor = convert_mlmultiarray_to_tensor(&mlarray, &device).unwrap();

            // Verify tensor structure
            assert_eq!(tensor.shape().dims(), &[6]);
            let tensor_data = tensor.to_vec1::<f32>().unwrap();

            // Verify the conversion matches our expected f16 results
            for (i, (&tensor_val, &expected)) in
                tensor_data.iter().zip(expected_results.iter()).enumerate()
            {
                assert_eq!(
                    tensor_val, expected,
                    "F16 conversion mismatch at index {}: tensor {}, expected {} (original: {})",
                    i, tensor_val, expected, test_values[i]
                );
            }

            // Test specific precision behaviors
            assert_eq!(tensor_data[1], 1.0, "1.0 should be exact in f16");
            assert_eq!(tensor_data[2], -2.5, "-2.5 should be exact in f16");
            assert_eq!(tensor_data[3], 0.0, "0.0 should be exact in f16");
            assert_eq!(tensor_data[4], 65504.0, "f16::MAX should be preserved");
            assert_eq!(tensor_data[5], -1.0, "-1.0 should be exact in f16");

            // Verify π loses precision as expected
            let pi_f16_expected = f16::from_f32(test_values[0]).to_f32();
            assert_eq!(
                tensor_data[0], pi_f16_expected,
                "π should match f16 precision"
            );
            assert!(
                (tensor_data[0] - test_values[0]).abs() > 0.0001,
                "π should lose precision in f16"
            );

            info!("Actual Float16 MLMultiArray conversion test passed!");
            debug!("   Original values: {:?}", test_values);
            debug!("   F16 converted:   {:?}", tensor_data);
        }
    }

    #[cfg(not(target_os = "macos"))]
    mod non_macos_tests {
        #[test]
        fn test_conversion_functions_not_available() {
            // This test just ensures the module compiles on non-macOS platforms
            // The actual conversion functions are only available on macOS
            println!("Conversion functions are only available on macOS");
        }
    }
}
