//! Test utilities for candle-coreml
//!
//! This module provides common testing utilities including automatic cache cleanup,
//! test setup/teardown, and testing helpers.

use candle_coreml::cache_manager::CacheManager;
use std::sync::Once;
use tracing::{info, warn};

static INIT: Once = Once::new();

/// Initialize tracing for tests (called once)
pub fn init_test_logging() {
    INIT.call_once(|| {
        // Only initialize if RUST_LOG is set to avoid spam in normal test runs
        if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_test_writer()
                .init();
        }
    });
}

/// Test cleanup guard that automatically cleans up caches when dropped
pub struct TestCleanupGuard {
    test_name: String,
    cache_manager: CacheManager,
    cleanup_on_drop: bool,
}

impl TestCleanupGuard {
    /// Create a new cleanup guard for a test
    pub fn new(test_name: &str) -> Self {
        init_test_logging();

        let cache_manager = CacheManager::new().expect("Failed to create cache manager for test");

        // Check if cleanup is enabled (default: yes, but can be disabled)
        let cleanup_on_drop =
            std::env::var("CANDLE_COREML_SKIP_TEST_CLEANUP") != Ok("1".to_string());

        if cleanup_on_drop {
            info!("🧪 Test '{}' started - cleanup enabled", test_name);
        } else {
            info!(
                "🧪 Test '{}' started - cleanup disabled by env var",
                test_name
            );
        }

        Self {
            test_name: test_name.to_string(),
            cache_manager,
            cleanup_on_drop,
        }
    }

    /// Get access to the cache manager
    pub fn cache_manager(&self) -> &CacheManager {
        &self.cache_manager
    }

    /// Manually trigger cleanup (useful for explicit cleanup in tests)
    pub fn cleanup_now(&self) -> Result<(usize, u64), anyhow::Error> {
        self.cleanup_test_caches()
    }

    /// Find and clean up caches related to this test process
    fn cleanup_test_caches(&self) -> Result<(usize, u64), anyhow::Error> {
        // Find all candle-coreml caches
        let all_caches = self.cache_manager.find_all_candle_coreml_caches()?;

        // Filter to caches that might be from our current test process
        let current_process = std::env::current_exe()
            .ok()
            .and_then(|p| p.file_stem().map(|s| s.to_string_lossy().to_string()))
            .unwrap_or_else(|| "unknown".to_string());

        let mut test_caches = Vec::new();

        for (path, _size) in all_caches {
            let path_str = path.to_string_lossy();

            // Include caches that match our current process or contain our test name
            if path_str.contains(&current_process) ||
               path_str.contains(&self.test_name) ||
               // Also clean up some patterns that are likely from our testing
               path_str.contains("candle_coreml-") ||
               path_str.contains("bundle_id_")
            {
                test_caches.push(path);
            }
        }

        if test_caches.is_empty() {
            info!("🧹 No test caches found for cleanup");
            return Ok((0, 0));
        }

        info!(
            "🧹 Cleaning up {} test cache directories",
            test_caches.len()
        );
        self.cache_manager
            .remove_cache_directories(&test_caches, false)
    }
}

impl Drop for TestCleanupGuard {
    fn drop(&mut self) {
        if !self.cleanup_on_drop {
            info!("🧪 Test '{}' completed - cleanup skipped", self.test_name);
            return;
        }

        match self.cleanup_test_caches() {
            Ok((removed_count, freed_bytes)) => {
                if removed_count > 0 {
                    let freed_mb = freed_bytes as f64 / (1024.0 * 1024.0);
                    info!(
                        "✅ Test '{}' cleanup: removed {} caches, freed {:.1} MB",
                        self.test_name, removed_count, freed_mb
                    );
                } else {
                    info!(
                        "🧪 Test '{}' completed - no caches to clean",
                        self.test_name
                    );
                }
            }
            Err(e) => {
                warn!("⚠️  Test '{}' cleanup failed: {}", self.test_name, e);
            }
        }
    }
}

/// Macro to create a test with automatic cleanup
#[macro_export]
macro_rules! test_with_cleanup {
    ($test_name:ident, $body:block) => {
        #[test]
        fn $test_name() {
            let _guard = $crate::test_utils::TestCleanupGuard::new(stringify!($test_name));
            $body
        }
    };
}

/// Helper function to run CoreML tests with models
pub fn with_test_model<F, R>(model_id: &str, test_fn: F) -> R
where
    F: FnOnce(&std::path::Path) -> R,
{
    use crate::model_downloader::ensure_model_downloaded;

    let model_path = ensure_model_downloaded(model_id, false)
        .unwrap_or_else(|_| panic!("Failed to download test model: {model_id}"));

    test_fn(&model_path)
}

/// Check if we're running in CI environment
pub fn is_ci() -> bool {
    std::env::var("CI").is_ok()
        || std::env::var("GITHUB_ACTIONS").is_ok()
        || std::env::var("GITLAB_CI").is_ok()
}

/// Skip test if models are not available (useful for quick unit tests)
pub fn require_models() {
    if std::env::var("CANDLE_COREML_SKIP_MODEL_TESTS") == Ok("1".to_string()) {
        panic!("Model tests skipped by environment variable");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleanup_guard_creation() {
        let guard = TestCleanupGuard::new("test_cleanup_guard_creation");
        assert_eq!(guard.test_name, "test_cleanup_guard_creation");

        // Should be able to access cache manager
        let _manager = guard.cache_manager();
    }

    #[test]
    fn test_manual_cleanup() {
        let guard = TestCleanupGuard::new("test_manual_cleanup");

        // Manual cleanup should work (may find 0 caches, that's fine)
        let result = guard.cleanup_now();
        assert!(result.is_ok());

        let (removed_count, freed_bytes) = result.unwrap();
        println!("Manual cleanup: {removed_count} caches, {freed_bytes} bytes");
    }

    // Test the macro
    test_with_cleanup!(test_macro_usage, {
        // Test body - cleanup will happen automatically
        assert_eq!(2 + 2, 4);
        println!("This test will auto-cleanup on completion");
    });
}
