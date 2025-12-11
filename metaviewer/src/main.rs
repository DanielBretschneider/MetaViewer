
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
/// Version: 1.2
/// Date: 27/11/2025

// Provides access to environment variables and command-line arguments
//use std::env; 
use std::process;
use std::path::Path;

// add modules
mod utils;
mod file_operations;

fn main() -> Result<(), std::io::Error>
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
        // print basic file info
        // get path from filename
        let path = Path::new(filename);

        // get extension
        let file_extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        // analizing txt file (ver 1.2)
        let _ = file_operations::print_global_file_attributes(path);
        
        // next operations depending on file extension
        match file_extension
        {
            "txt" => file_operations::print_txt_specific_metadata(path)?,
            "pdf" => file_operations::print_pdf_specific_metadata(path)?,
            "docx" => println!("[*] Word document"),
            "xlsx" => println!("[*] Excel file"),
            _ => println!("[-] Unknown file extension."),
        }
    }
    else
    {
        // 0 = success, nonzero = error
        process::exit(0);
    }

    // err handling
    Ok(())
}

