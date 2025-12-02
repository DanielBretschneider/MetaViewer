/// MetaViewer - A modern metadata inspection tool
///
/// This module holds a few methods which will be used in main.rs like the handling 
/// of command line args or string operations.
///
/// Author: Daniel Bretschneider, daniel@bretschneider.cc
/// Version: 1.1
/// Date: 27/11/2025

/// import env module from Rust's stand library
use std::env;
use std::path::Path;
use colored::*;

/// Global Vars
pub const CMD_HELP: &str = "help";

/// Returns all command line arguments as a vector of strings,
/// excluding the program name. (growable list of strings)
pub fn get_command_line_args() -> Vec<String>
{
    return env::args().skip(1).collect();
}

/// Checks if args contain exactly one valid filename string.
pub fn check_command_line_args(args : Vec<String>) -> bool
{
    // Must be exactly one argument (for Version 1.0 of MetaViewer)
    if args.len() != 1
    {
        println!("[-] Command line arguments must be exactly 1.");
        println!("[-] Type 'help' for information on usage.");
        println!("[-] Example usage: metaviewer file.pdf");
        println!("[-] Exiting.");
        return false;
    }

    // get single command line argument and store in variable
    let candidate = &args[0];

    // Disallow empty string
    if candidate.is_empty()
    {
        println!("[-] Command line arguments cannot be empty.");
        println!("[-] Type 'help' for information on usage.");
        println!("[-] Example usage: metaviewer file.pdf");
        println!("[-] Exiting.");
        return false;
    }

    // Disallow common invalid characters
    // create a list of common invalid characters 
    // TODO: extend in future versions
    let invalid_chars = ['/', '\\', ':', ';', '*', '?', '"', '<', '>', '|'];

    // check if candidate contains those invalid_chars
    if candidate.chars().any(|c| invalid_chars.contains(&c))
    {
        println!("[-] Command line argument contains invalid character. Please check file name.");
        println!("[-] Example usage: metaviewer file.pdf");
        println!("[-] Exiting.");
        return false;
    }

    // check if help msg desired
    if candidate == CMD_HELP
    {
        //print help message
        print_help_message();
        return true;
    }

    // check if the file exists
    let path = Path::new(candidate);
    
    // convert Option<&OsStr> to &str
    if let Some(os_str) = path.file_name()
    {
        if let Some(fname) = os_str.to_str()
        {
            // check using file_exists function
            if file_exists(fname)
            {
                println!("[+] Command line arguments check succesfull");
                println!("[+] File '{}' exists", fname);
                println!("[+] Proceed with metadata extraction");

                // checks all positive
                return true;
            } 
            else
            {
                println!("[+] Command line arguments check failed");
                println!("[+] File '{}' does no exist", fname);
                println!("[+] Exiting.");

                // last check failed
                return false;
            }
        }
    }

    // default return
    return false;
}

/// Checks if a file exists using an if/else structure.
pub fn file_exists(filename : &str) -> bool
{
    // store path in variable
    let path = Path::new(filename);

    // check if path exists
    path.exists()
}

/// prints usage message to console if user types 'help'
pub fn print_help_message()
{
    println!("[*] Show help message");
    println!("NAME:");
    println!("  {}", "MetaViewer - Metadata Inspection Tool\n".bold());
    println!("USAGE:");
    println!("  metaviewer <file>\n");
    println!("VERSION:");
    println!("  1.1\n");
    println!("AUTHORS:");
    println!("  Daniel Bretschneider (@jeromeium), daniel@bretschneider.cc\n");
    println!("OPTIONS:");
    println!("  file.extension   {}", "input name of file".green());
    println!("  help             {}", "Show this help message".green());
}