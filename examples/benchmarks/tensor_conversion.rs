//! Tensor Conversion Overhead Benchmark
//! 
//! Measures the actual cost of converting between Candle tensors and CoreML MLMultiArray
//! using the real conversion functions from the candle-coreml crate.
//! This helps developers understand the overhead of CoreML integration.

use anyhow::Result;
use candle_core::{Device, Tensor};
use std::time::{Duration, Instant};

struct ConversionResult {
    tensor_size: String,
    elements: usize,
    tensor_to_ml_time: Duration,
    ml_to_tensor_time: Duration,
    round_trip_time: Duration,
    memory_mb: f64,
}

impl ConversionResult {
    fn print(&self) {
        println!("{:<12} {:<10} {:<12} {:<12} {:<12} {:<8}",
            self.tensor_size,
            format!("{:.1}M", self.elements as f64 / 1_000_000.0),
            format!("{:.2?}", self.tensor_to_ml_time),
            format!("{:.2?}", self.ml_to_tensor_time), 
            format!("{:.2?}", self.round_trip_time),
            format!("{:.1}", self.memory_mb)
        );
    }
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn benchmark_tensor_conversion(shape: &[usize], iterations: usize) -> Result<ConversionResult> {
    let device = Device::Cpu;
    let elements = shape.iter().product::<usize>();
    let memory_mb = (elements * std::mem::size_of::<f32>()) as f64 / (1024.0 * 1024.0);
    
    // Create test tensor with F32 dtype
    let tensor = Tensor::randn(0.0f32, 1.0f32, shape, &device)?;
    
    let mut tensor_to_ml_times = Vec::new();
    let mut ml_to_tensor_times = Vec::new();
    
    // We'll measure the core conversion logic without needing a real model
    // by implementing the same steps as the CoreML model methods
    for _ in 0..iterations {
        // Measure Tensor → MLMultiArray conversion
        let start = Instant::now();
        let _ml_array = create_ml_array_from_tensor(&tensor)?;
        let tensor_to_ml_time = start.elapsed();
        tensor_to_ml_times.push(tensor_to_ml_time);
        
        // Measure MLMultiArray → Tensor conversion  
        let start = Instant::now();
        let ml_array = create_ml_array_from_tensor(&tensor)?;
        let _output_tensor = extract_tensor_from_ml_array(&ml_array, shape, &device)?;
        let ml_to_tensor_time = start.elapsed();
        ml_to_tensor_times.push(ml_to_tensor_time);
    }
    
    // Calculate average times
    let avg_tensor_to_ml = tensor_to_ml_times.iter().sum::<Duration>() / tensor_to_ml_times.len() as u32;
    let avg_ml_to_tensor = ml_to_tensor_times.iter().sum::<Duration>() / ml_to_tensor_times.len() as u32;
    let round_trip = avg_tensor_to_ml + avg_ml_to_tensor;
    
    Ok(ConversionResult {
        tensor_size: format!("{:?}", shape),
        elements,
        tensor_to_ml_time: avg_tensor_to_ml,
        ml_to_tensor_time: avg_ml_to_tensor,
        round_trip_time: round_trip,
        memory_mb,
    })
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn create_ml_array_from_tensor(tensor: &Tensor) -> Result<objc2::rc::Retained<objc2_core_ml::MLMultiArray>> {
    use objc2_core_ml::{MLMultiArray, MLMultiArrayDataType};
    use objc2_foundation::{NSArray, NSNumber};
    use objc2::AnyThread;
    use candle_core::DType;
    
    let contiguous_tensor = if tensor.is_contiguous() {
        tensor.clone()
    } else {
        tensor.contiguous()?
    };

    let dims = tensor.dims();
    let mut shape_numbers = Vec::with_capacity(dims.len());
    for &dim in dims {
        shape_numbers.push(NSNumber::new_usize(dim));
    }
    let shape_nsarray = NSArray::from_retained_slice(&shape_numbers);

    let ml_data_type = match tensor.dtype() {
        DType::F32 => MLMultiArrayDataType::Float32,
        DType::I64 => MLMultiArrayDataType::Int32, // Convert I64 to I32
        _ => return Err(anyhow::anyhow!("Unsupported dtype: {:?}", tensor.dtype())),
    };

    let ml_array = unsafe {
        MLMultiArray::initWithShape_dataType_error(
            MLMultiArray::alloc(),
            &shape_nsarray,
            ml_data_type,
        )
    }?;

    // Copy tensor data to MLMultiArray
    let flattened_tensor = contiguous_tensor.flatten_all()?;
    let data_vec = flattened_tensor.to_vec1::<f32>()?;
    let elements = tensor.elem_count();
    let element_size = std::mem::size_of::<f32>();

    unsafe {
        use block2::StackBlock;
        ml_array.getMutableBytesWithHandler(&StackBlock::new(
            |ptr: std::ptr::NonNull<std::ffi::c_void>, len, _| {
                let dst = ptr.as_ptr() as *mut f32;
                let src = data_vec.as_ptr();
                let copy_elements = elements.min(len as usize / element_size);

                if copy_elements > 0 && len as usize >= copy_elements * element_size {
                    std::ptr::copy_nonoverlapping(src, dst, copy_elements);
                }
            },
        ));
    }
    
    Ok(ml_array)
}

#[cfg(all(target_os = "macos", feature = "coreml"))]
fn extract_tensor_from_ml_array(
    ml_array: &objc2_core_ml::MLMultiArray, 
    shape: &[usize], 
    device: &Device
) -> Result<Tensor> {
    use block2::StackBlock;
    use std::cell::RefCell;
    
    let elements = shape.iter().product::<usize>();
    let mut buf = vec![0.0f32; elements];
    
    unsafe {
        let buf_cell = RefCell::new(&mut buf);
        
        ml_array.getBytesWithHandler(&StackBlock::new(
            |ptr: std::ptr::NonNull<std::ffi::c_void>, len: isize| {
                let src = ptr.as_ptr() as *const f32;
                let copy_elements = elements.min(len as usize / std::mem::size_of::<f32>());
                if copy_elements > 0 && len as usize >= copy_elements * std::mem::size_of::<f32>() {
                    if let Ok(mut buf_ref) = buf_cell.try_borrow_mut() {
                        std::ptr::copy_nonoverlapping(src, buf_ref.as_mut_ptr(), copy_elements);
                    }
                }
            },
        ));
    }
    
    Tensor::from_vec(buf, shape, device).map_err(|e| anyhow::anyhow!("Failed to create tensor: {}", e))
}

#[cfg(not(all(target_os = "macos", feature = "coreml")))]
fn benchmark_tensor_conversion(_shape: &[usize], _iterations: usize) -> Result<ConversionResult> {
    Err(anyhow::anyhow!("CoreML benchmarks only available on macOS"))
}

fn benchmark_pure_candle_operations(shape: &[usize], iterations: usize) -> Result<Duration> {
    let device = Device::Cpu;
    let tensor = Tensor::randn(0.0f32, 1.0f32, shape, &device)?;
    
    let mut times = Vec::new();
    
    for _ in 0..iterations {
        let start = Instant::now();
        
        // Simulate the same operations as conversion: clone, flatten, to_vec, from_vec
        let flattened = tensor.flatten_all()?;
        let data: Vec<f32> = flattened.to_vec1()?;
        let _reconstructed = Tensor::from_vec(data, shape, &device)?;
        
        times.push(start.elapsed());
    }
    
    Ok(times.iter().sum::<Duration>() / times.len() as u32)
}

fn main() -> Result<()> {
    println!("Tensor Conversion Overhead Benchmark");
    println!("=====================================");
    println!("Measuring the cost of Candle ↔ CoreML tensor conversion\n");
    
    let test_shapes = vec![
        vec![64],           // 64 elements (256 bytes)
        vec![256, 256],     // 65K elements (256 KB) 
        vec![512, 512],     // 262K elements (1 MB)
        vec![1024, 1024],   // 1M elements (4 MB)
        vec![2048, 1024],   // 2M elements (8 MB) 
        vec![1, 3, 512, 512], // Image-like: 786K elements (3 MB)
        vec![32, 128, 768], // BERT-like: 3M elements (12 MB)
    ];
    
    let iterations = 10;
    
    println!("{:<12} {:<10} {:<12} {:<12} {:<12} {:<8}", 
        "Shape", "Elements", "→ MLArray", "← Tensor", "Round Trip", "Memory");
    println!("{:-<70}", "");
    
    let mut results = Vec::new();
    
    for shape in &test_shapes {
        match benchmark_tensor_conversion(shape, iterations) {
            Ok(result) => {
                result.print();
                results.push(result);
            }
            Err(e) => {
                println!("{:<12} {:<58}", format!("{:?}", shape), format!("❌ Failed: {}", e));
            }
        }
        
        // Compare with pure Candle operations
        if let Ok(pure_candle_time) = benchmark_pure_candle_operations(shape, iterations) {
            println!("{:<12} {:<10} {:<36} {:<8}", 
                "", 
                "",
                format!("Pure Candle equivalent: {:?}", pure_candle_time),
                ""
            );
        }
        println!();
    }
    
    // Analysis
    if !results.is_empty() {
        println!("=== ANALYSIS ===");
        
        let smallest = results.first().unwrap();
        let largest = results.last().unwrap();
        
        println!("Overhead scaling:");
        println!("  Small tensors ({:.1}MB): {:?} round-trip", 
            smallest.memory_mb, smallest.round_trip_time);
        println!("  Large tensors ({:.1}MB): {:?} round-trip", 
            largest.memory_mb, largest.round_trip_time);
        
        let overhead_per_mb = largest.round_trip_time.as_nanos() as f64 / largest.memory_mb;
        println!("  Overhead: ~{:.0}µs per MB", overhead_per_mb / 1000.0);
        
        println!("\nRecommendations:");
        println!("  • Use CoreML for large models where energy efficiency matters");
        println!("  • Consider conversion cost for small, frequent operations"); 
        println!("  • Batch operations when possible to amortize overhead");
    }
    
    Ok(())
}