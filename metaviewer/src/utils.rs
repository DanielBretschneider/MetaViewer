//! MetaViewer - A modern metadata inspection tool
//!
//! This module holds a few methods which will be used in main.rs like the handling 
//! of command line args or string operations.
//!
//! Author: Daniel Bretschneider, daniel@bretschneider.cc
//! Version: 1.0
//! Date: 27/11/2025

//! import env module from Rust's stand library
use std::env;

/// Returns all command line arguments as a vector of strings,
/// excluding the program name. (growable list of strings)
pub fn get_command_line_args() -> Vec<String>
{
    return env::args().skip(1).collect();
}