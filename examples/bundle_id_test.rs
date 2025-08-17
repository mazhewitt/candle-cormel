//! Test bundle identifier detection and potential override methods
//!
//! This example investigates how CoreML cache directories are named
//! and whether we can influence the bundle identifier.

use candle_coreml::CacheManager;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Bundle Identifier Investigation");
    println!("==================================");

    // Test 1: Current bundle identifier
    let manager = CacheManager::new()?;
    println!("Current bundle ID: {:?}", manager.bundle_identifier());

    // Test 2: Current process name
    let current_exe = std::env::current_exe()?;
    let process_name = current_exe
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("Process name: {process_name}");

    // Test 3: Environment variables that might affect bundle ID
    let env_vars = [
        "CFBundleIdentifier",
        "BUNDLE_ID",
        "APP_BUNDLE_ID",
        "MACOS_BUNDLE_ID",
        "DYLD_LIBRARY_PATH",
    ];

    println!("\nEnvironment variables:");
    for var in &env_vars {
        if let Ok(value) = std::env::var(var) {
            println!("  {var}: {value}");
        } else {
            println!("  {var}: (not set)");
        }
    }

    // Test 4: Check current CoreML cache locations
    println!("\nPotential CoreML cache locations:");
    let locations = manager.report_coreml_cache_locations();
    for (i, location) in locations.iter().enumerate() {
        if location.exists() {
            println!("  {}. {} ✅", i + 1, location.display());

            // Check size of cache
            if let Ok(metadata) = std::fs::metadata(location) {
                if metadata.is_dir() {
                    if let Ok(entries) = std::fs::read_dir(location) {
                        let count = entries.count();
                        println!("     ({count} items)");
                    }
                }
            }
        } else {
            println!("  {}. {} ❌", i + 1, location.display());
        }
    }

    // Test 5: Check all current cache directories
    if let Some(cache_dir) = dirs::cache_dir() {
        println!("\nActual cache directories containing 'e5rt' or test names:");
        if let Ok(entries) = std::fs::read_dir(&cache_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.contains("e5rt")
                    || name.contains("test")
                    || name.contains("candle")
                    || name.contains("typo")
                    || name.contains("performance")
                    || name.contains("qwen")
                {
                    println!("  📁 {name}");

                    // Check if it contains CoreML caches
                    let path = entry.path();
                    if path.join("com.apple.e5rt.e5bundlecache").exists() {
                        println!("     └── com.apple.e5rt.e5bundlecache ✅");
                    }
                }
            }
        }
    }

    // Test 6: Attempt to set bundle ID via environment and see if it affects anything
    println!("\n🧪 Testing environment variable override...");

    // Set environment variable and spawn a child process to test
    let output = Command::new("cargo")
        .args(["run", "--example", "bundle_id_child_test"])
        .env("CFBundleIdentifier", "com.candle-coreml.unified-cache-test")
        .env("BUNDLE_ID", "com.candle-coreml.unified-cache-test")
        .output();

    match output {
        Ok(output) => {
            println!("Child process output:");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("Child process stderr:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("Failed to spawn child process: {e}");
            println!("(This is expected - we need to create the child example first)");
        }
    }

    Ok(())
}
