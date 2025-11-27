/// main.rs
///
/// MetaViewer - A modern metadata inspection tool
///
/// Filetypes for first version:
/// - TXT: word count, encoding
/// - PDF: author, title, creation date
/// - DOCX/XLSX: MS Office filetypes
/// 
/// Future expansion: implement modular parsers for each file type.
/// ------------------------------------------------
///
//! Author: Daniel Bretschneider, daniel@bretschneider.cc
//! Version: 1.0
//! Date: 27/11/2025

// Provides access to environment variables and command-line arguments
use std::env; 

// add module utils
mod utils;


fn main() 
{
    // get cmd args from utils
    let args = utils::get_command_line_args();

    // print args - run with 'cargo run -- apple banana peach'
    println!("CMD Args: {:?}", args);
}

