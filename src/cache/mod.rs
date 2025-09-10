//! Cache module for candle-coreml
//!
//! This module provides cache management functionality including:
//! - Model file caching
//! - Configuration caching
//! - Cache cleanup utilities

pub mod manager;

// Re-export main types for convenience
pub use manager::CacheManager;
