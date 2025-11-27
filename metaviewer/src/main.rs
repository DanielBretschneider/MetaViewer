/// main.rs
///
/// MetaViewer - A modern metadata inspection tool
/// ------------------------------------------------
/// This is a clean starting point for your Rust project.
/// It sets up a basic CLI structure, error handling, and room for future expansion.
/// ------------------------------------------------
///
/// Filetypes for first version:
/// - TXT: word count, encoding
/// - PDF: author, title, creation date
/// - DOCX/XLSX: MS Office filetypes
/// 
/// Future expansion: implement modular parsers for each file type.
/// ------------------------------------------------

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

