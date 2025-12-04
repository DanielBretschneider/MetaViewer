
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
/// Version: 1.1
/// Date: 27/11/2025

// Provides access to environment variables and command-line arguments
//use std::env; 
use std::process;
use std::path::Path;

// add modules
mod utils;
mod file_operations;

fn main() 
{
    // start mesage
    println!("[*] MetaViewer 1.1");

    // get cmd args from utils
    let args = utils::get_command_line_args();

    // get number of given args
    // let argc = args.len();

    // check argc (arguemnt count)
    let args_check = utils::check_command_line_args(args.clone());

    // extract file name
    let filename = &args[0];

    // proceed if true and exit metaviewer if checks failed
    if args_check
    {
        if filename.ends_with(".txt")
        {
            // get path from filename
            let path = Path::new(filename);

            // analizing txt file (ver 1.2)
            let _ = file_operations::print_global_file_attributes(path);
        }
    }
    else
    {
        // 0 = success, nonzero = error
        process::exit(0);
    }
}

