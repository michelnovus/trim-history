// [MIT License] Copyright (c) 2024 Michel Novus

#![allow(dead_code)]

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() {}

/// Creates the program configuration file.
fn generate_config_file(path: PathBuf) -> io::Result<()> {
    Ok(())
}

/// Contains the initial configuration of the program.
struct AppConfig {}

impl AppConfig {
    /// Generates a new `AppConfig` struct with defaults attributes.
    fn new() -> Self {
        AppConfig {}
    }

    /// Tries to generates a new `AppConfig` struct from file configuration.
    ///
    /// The function return `Err` if file is not valid.
    fn from_file(path: PathBuf) -> io::Result<Self> {
        Ok(AppConfig {})
    }
}
