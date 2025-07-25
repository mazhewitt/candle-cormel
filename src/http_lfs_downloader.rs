//! Simple HTTP-based LFS file downloader for HuggingFace repositories
//!
//! This module provides utilities to download LFS files directly from HuggingFace
//! using HTTP requests when git-lfs is not available.

use anyhow::{Error as E, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Download an LFS file from HuggingFace using direct URLs
///
/// This function reads LFS pointer files and downloads the actual content
/// using HuggingFace's direct file access URLs.
pub fn download_lfs_file_direct(
    model_id: &str,
    file_path: &str,
    local_path: &Path,
    verbose: bool,
) -> Result<PathBuf> {
    if verbose {
        println!("🔄 Downloading LFS file directly: {}", file_path);
    }

    // Check if the local file is an LFS pointer
    if local_path.exists() {
        let lfs_info = read_lfs_pointer(local_path)?;
        if let Some((oid, size)) = lfs_info {
            if verbose {
                println!("  📄 Found LFS pointer: oid={}, size={}", oid, size);
            }
            
            // Download the actual file content
            let actual_content_path = download_lfs_content(model_id, file_path, &oid, size, local_path, verbose)?;
            return Ok(actual_content_path);
        }
    }

    // If not an LFS file, return the original path
    Ok(local_path.to_path_buf())
}

/// Read LFS pointer information from a file
fn read_lfs_pointer(file_path: &Path) -> Result<Option<(String, u64)>> {
    let file = File::open(file_path)
        .map_err(|e| E::msg(format!("Failed to open LFS pointer file: {}", e)))?;
    
    let reader = BufReader::new(file);
    let mut oid = None;
    let mut size = None;
    
    for line in reader.lines() {
        let line = line.map_err(|e| E::msg(format!("Failed to read line: {}", e)))?;
        
        if line.starts_with("version https://git-lfs.github.com/spec/v1") {
            // This is an LFS pointer file
            continue;
        } else if line.starts_with("oid sha256:") {
            oid = Some(line.strip_prefix("oid sha256:").unwrap_or("").to_string());
        } else if line.starts_with("size ") {
            if let Ok(parsed_size) = line.strip_prefix("size ").unwrap_or("").parse::<u64>() {
                size = Some(parsed_size);
            }
        }
    }
    
    match (oid, size) {
        (Some(oid), Some(size)) => Ok(Some((oid, size))),
        _ => Ok(None), // Not an LFS pointer file
    }
}

/// Download LFS content using HuggingFace direct URLs
fn download_lfs_content(
    model_id: &str,
    file_path: &str,
    _oid: &str,
    expected_size: u64,
    local_path: &Path,
    verbose: bool,
) -> Result<PathBuf> {
    // Construct HuggingFace direct URL
    let download_url = format!(
        "https://huggingface.co/{}/resolve/main/{}",
        model_id, file_path
    );
    
    if verbose {
        println!("  🌐 Downloading from: {}", download_url);
    }

    // Create a temporary file for the download
    let temp_path = local_path.with_extension("tmp");
    
    // Use reqwest to download the file (we'll need to add this dependency)
    // For now, let's create a placeholder implementation
    if verbose {
        println!("  ⚠️  HTTP download not yet implemented");
        println!("  💡 Would download {} bytes from {}", expected_size, download_url);
    }
    
    // Return the original path for now
    Err(E::msg("HTTP LFS download not yet implemented"))
}

/// Check if a file is an LFS pointer
pub fn is_lfs_pointer(file_path: &Path) -> bool {
    if let Ok(file) = File::open(file_path) {
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        if reader.read_line(&mut first_line).is_ok() {
            return first_line.starts_with("version https://git-lfs.github.com/spec/v1");
        }
    }
    false
}

/// Fix LFS files in a directory by downloading their actual content
pub fn fix_lfs_files_in_directory(
    model_id: &str,
    directory: &Path,
    verbose: bool,
) -> Result<()> {
    if verbose {
        println!("🔄 Fixing LFS files in directory: {}", directory.display());
    }
    
    // Find all LFS pointer files
    let lfs_files = find_lfs_files(directory)?;
    
    if lfs_files.is_empty() {
        if verbose {
            println!("  ℹ️  No LFS files found");
        }
        return Ok(());
    }
    
    if verbose {
        println!("  📄 Found {} LFS files to fix", lfs_files.len());
    }
    
    for lfs_file in lfs_files {
        // Calculate relative path from directory
        let relative_path = lfs_file
            .strip_prefix(directory)
            .map_err(|e| E::msg(format!("Failed to get relative path: {}", e)))?;
        
        let relative_path_str = relative_path
            .to_str()
            .ok_or_else(|| E::msg("Invalid UTF-8 in file path"))?;
        
        if verbose {
            println!("  🔧 Attempting to fix LFS file: {}", relative_path_str);
        }
        
        // Try to download the actual content
        match download_lfs_file_direct(model_id, relative_path_str, &lfs_file, verbose) {
            Ok(_) => {
                if verbose {
                    println!("    ✅ Fixed: {}", relative_path_str);
                }
            }
            Err(e) => {
                if verbose {
                    println!("    ⚠️  Could not fix {}: {}", relative_path_str, e);
                }
            }
        }
    }
    
    Ok(())
}

/// Find all LFS pointer files in a directory
fn find_lfs_files(directory: &Path) -> Result<Vec<PathBuf>> {
    let mut lfs_files = Vec::new();
    
    fn visit_dir(dir: &Path, lfs_files: &mut Vec<PathBuf>) -> Result<()> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| E::msg(format!("Failed to read directory: {}", e)))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| E::msg(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();
            
            if path.is_dir() {
                visit_dir(&path, lfs_files)?;
            } else if path.is_file() && is_lfs_pointer(&path) {
                lfs_files.push(path);
            }
        }
        
        Ok(())
    }
    
    visit_dir(directory, &mut lfs_files)?;
    Ok(lfs_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_is_lfs_pointer() {
        let temp_dir = std::env::temp_dir();
        let lfs_file = temp_dir.join("test_lfs.txt");
        let normal_file = temp_dir.join("test_normal.txt");
        
        // Create LFS pointer file
        let mut lfs_f = File::create(&lfs_file).unwrap();
        writeln!(lfs_f, "version https://git-lfs.github.com/spec/v1").unwrap();
        writeln!(lfs_f, "oid sha256:abc123").unwrap();
        writeln!(lfs_f, "size 1234").unwrap();
        
        // Create normal file
        let mut normal_f = File::create(&normal_file).unwrap();
        writeln!(normal_f, "This is normal content").unwrap();
        
        assert!(is_lfs_pointer(&lfs_file));
        assert!(!is_lfs_pointer(&normal_file));
        
        // Clean up
        fs::remove_file(lfs_file).ok();
        fs::remove_file(normal_file).ok();
    }

    #[test]
    fn test_read_lfs_pointer() {
        let temp_dir = std::env::temp_dir();
        let lfs_file = temp_dir.join("test_lfs_parse.txt");
        
        // Create LFS pointer file
        let mut f = File::create(&lfs_file).unwrap();
        writeln!(f, "version https://git-lfs.github.com/spec/v1").unwrap();
        writeln!(f, "oid sha256:abcdef123456789").unwrap();
        writeln!(f, "size 9876543").unwrap();
        
        let result = read_lfs_pointer(&lfs_file).unwrap();
        assert_eq!(result, Some(("abcdef123456789".to_string(), 9876543)));
        
        // Clean up
        fs::remove_file(lfs_file).ok();
    }
}