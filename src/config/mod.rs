//! Configuration module for candle-coreml
//!
//! This module provides all configuration-related functionality including:
//! - Basic CoreML configuration structures
//! - Advanced model configuration with shape discovery
//! - Automatic configuration generation from CoreML packages

pub mod basic;
pub mod generator;
pub mod model;

// Re-export main types for convenience
pub use basic::Config;
pub use generator::ConfigGenerator;
pub use model::{ComponentConfig, ModelConfig, ModelInfo, NamingConfig, ShapeConfig, TensorConfig};
