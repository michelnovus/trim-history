// [MIT License] Copyright (c) 2024 Michel Novus

#![allow(dead_code, unused_variables)]

use anyhow::{bail, Result};
use std::env;
use std::path::PathBuf;

/// Name of the history file environment variable.
const HISTORY_FILE_ENVAR_NAME: &str = "HISTFILE";
/// Name of shell environment variable, should be SHELL.
const SHELL_ENVAR_NAME: &str = "SHELL";

/// Creates the program configuration file.
pub fn generate_config_file(path: PathBuf) -> Result<()> {
    Ok(())
}

/// Contains the initial configuration of the program.
pub struct AppConfig {
    pub history_file: PathBuf,
}

impl AppConfig {
    /// Generates a new `AppConfig` struct with defaults attributes.
    pub fn new() -> Result<Self> {
        let history_file = env::var(HISTORY_FILE_ENVAR_NAME);
        let history_file = if history_file.is_err() {
            let shell = env::var(SHELL_ENVAR_NAME)?;
            // "history file could not be resolved, because \
            // shell name is unknown"
            let shell = PathBuf::from(shell);
            let history_filename = match shell.file_name().unwrap().to_str() {
                Some("bash") => ".bash_history",
                Some("zsh") => ".zsh_history",
                Some(_shell) => {
                    bail!("not implemented history file for shell: {_shell}");
                }
                None => ".bash_history",
            };
            let user_home = env::var("HOME")?;
            let history_file = format!("{}/{}", user_home, history_filename);
            PathBuf::from(history_file)
        } else {
            PathBuf::from(history_file.unwrap())
        };

        Ok(AppConfig { history_file })
    }

    /// Tries to generates a new `AppConfig` struct from file configuration.
    ///
    /// The function return `Err` if file is not valid.
    pub fn from_file(path: PathBuf) -> Result<Self> {
        AppConfig::new() // TODO: Implementar archivo de configuración.
    }
}
