/// MetaViewer - A modern metadata inspection tool
///
/// This module holds a few methods which will be used to extract meta data from files.
///
/// Author: Daniel Bretschneider, daniel@bretschneider.cc
/// Version: 1.2
/// Date: 04/12/2025

use std::fs;
use colored::*;
use std::path::Path;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use std::os::unix::fs::MetadataExt; // For Unix-specific extensions (inode, uid, gid, etc.)
//use chrono::{DateTime, NaiveDateTime, Utc}; // for conversion of unix timestampt into utc

/// print global file metadata (name, size, creation date, etc.)
pub fn print_global_file_attributes(path: &Path) -> std::io::Result<()> 
{
    // Get metadata
    let metadata = fs::metadata(path)?;

    println!("\nFile: \t\t\t{}", path.display().to_string().blue());
    println!("Size: \t\t\t{} {}", metadata.len().to_string().blue(), "bytes".blue());
    println!("Is file: \t\t{}", metadata.is_file().to_string().blue());
    println!("Is directory: \t\t{}", metadata.is_dir().to_string().blue());

    // convert unix system time to human readable time format
    let created: SystemTime = metadata.created()?;
    let modified: SystemTime = metadata.modified()?;
    let accessed: SystemTime = metadata.accessed()?;

    let daytime_created: DateTime<Local> = created.into();
    let daytime_modified: DateTime<Local> = modified.into();
    let daytime_accessed: DateTime<Local> = accessed.into();

    let formatted_created: String = daytime_created.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_modified: String = daytime_modified.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_accessed: String = daytime_accessed.format("%Y-%m-%d %H:%M:%S").to_string();

    // Timestamps
    println!("Created: \t\t{}", formatted_created.blue());
    println!("Modified: \t\t{}", formatted_modified.blue());
    println!("Accessed: \t\t{}", formatted_accessed.blue());

    // Unix-specific attributes
    println!("Inode: \t\t\t{}", metadata.ino().to_string().blue());
    println!("Owner UID: \t\t{}", format!("{}", metadata.uid()).blue());
    println!("Group GID: \t\t{}", format!("{}", metadata.gid()).blue());
    println!("Permissions (mode): \t{}\n", format!("{}", metadata.uid()).blue());
    
    // end of txt file analysis
    Ok(())
}


/// print txt file specific metadata
pub fn print_txt_specific_metadata(path: &Path) -> std::io::Result<()> 
{
    // convert path to &str
    let path_str = path.display().to_string();

    // read the entire file into a string
    let file_content = fs::read_to_string(path_str)?;

    // get character count of given fike
    let num_of_chars_in_file = file_content.chars().count();

    // get word count of given txt file
    let num_of_words_in_file = file_content.split_whitespace().count();

    // get number of lines in txt file
    let num_of_lines_in_file = file_content.lines().count();

    // print results in console
    println!("Characters: \t\t{}", format!("{}", num_of_chars_in_file.to_string().blue()));
    println!("Words: \t\t\t{}", format!("{}", num_of_words_in_file.to_string().blue()));
    println!("Lines: \t\t\t{}", format!("{}", num_of_lines_in_file.to_string().blue()));

    // finish
    Ok(())
}


/// print file specific metadata for PDF
pub fn print_pdf_specific_metadata(path: &Path) -> std::io::Result<()> 
{
    // this is just a placeholder
    let _path_str = path.display().to_string();

    // err handling
    Ok(())
}
