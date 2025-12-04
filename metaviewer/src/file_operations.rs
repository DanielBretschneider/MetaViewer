/// MetaViewer - A modern metadata inspection tool
///
/// This module holds a few methods which will be used to extract meta data from files.
///
/// Author: Daniel Bretschneider, daniel@bretschneider.cc
/// Version: 1.1
/// Date: 04/12/2025

use std::fs;
use colored::*;
use std::path::Path;
use std::os::unix::fs::MetadataExt; // For Unix-specific extensions (inode, uid, gid, etc.)

pub fn print_global_file_attributes(path: &Path) -> std::io::Result<()> {
    // Get metadata
    let metadata = fs::metadata(path)?;

    println!("\nFile: {}", path.display().to_string().blue());
    println!("Size: {} {}", metadata.len().to_string().blue(), "bytes".blue());
    println!("Is file: {}", metadata.is_file().to_string().blue());
    println!("Is directory: {}", metadata.is_dir().to_string().blue());

    // Timestamps
    println!("Created: {}", format!("{:?}", metadata.created()?).blue());
    println!("Modified: {}", format!("{:?}", metadata.modified()?).blue());
    println!("Accessed: {}", format!("{:?}", metadata.accessed()?).blue());

    // Unix-specific attributes
    println!("Inode: {}", metadata.ino().to_string().blue());
    println!("Owner UID: {}", format!("{}", metadata.uid()).blue());
    println!("Group GID: {}", format!("{}", metadata.gid()).blue());
    println!("Permissions (mode): {}", format!("{}", metadata.uid()).blue());

    Ok(())
}