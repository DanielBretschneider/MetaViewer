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
/// Author: Daniel Bretschneider, daniel@bretschneider.cc
/// Version: 1.0
/// Date: 27/11/2025

// Provides access to environment variables and command-line arguments
//use std::env; 
use std::process;

// add module utils
mod utils;


fn main() 
{
    // get cmd args from utils
    let args = utils::get_command_line_args();

    // get number of given args
    // let argc = args.len();

    // check argc (arguemnt count)
    let args_check = utils::check_command_line_args(args);

    // proceed if true and exit metaviewer if checks failed
    if args_check
    {
        // checks positive
        println!("[*] Looks good!");
    }
    else
    {
        // 0 = success, nonzero = error
        process::exit(0);
    }
}

