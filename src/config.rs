// [MIT License] Copyright (c) 2024 Michel Novus

use std::env;
use std::io;
use std::path::PathBuf;
use std::process;

/// Name of the history file environment variable.
const HISTORY_FILE_ENVAR_NAME: &str = "HISTFILE";
/// Name of shell environment variable, should be SHELL.
const SHELL_ENVAR_NAME: &str = "SHELL";

/// Creates the program configuration file.
pub fn generate_config_file(path: PathBuf) -> io::Result<()> {
    Ok(())
}

/// Contains the initial configuration of the program.
pub struct AppConfig {
    pub history_file: PathBuf,
}

impl AppConfig {
    /// Generates a new `AppConfig` struct with defaults attributes.
    pub fn new() -> Self {
        let history_file = PathBuf::from(
            env::var(HISTORY_FILE_ENVAR_NAME).unwrap_or_else(|_| {
                let shell = PathBuf::from(
                    env::var(SHELL_ENVAR_NAME).unwrap_or_else(|err| {
                        eprintln!("An error has ocurred: `{}`", err);
                        process::exit(1);
                    }),
                );
                let history_filename = match shell.file_name().unwrap().to_str()
                {
                    Some("bash") => ".bash_history",
                    Some("zsh") => ".zsh_history",
                    Some(_shell) => {
                        eprintln!("There is not implemented for: `{_shell}`");
                        process::exit(1);
                    }
                    None => {
                        eprintln!(
                            "It looks like the SHELL environment \
                        variable is empty."
                        );
                        process::exit(1);
                    }
                };
                let user_home = env::var("HOME").unwrap();
                let history_file =
                    format!("{}/{}", user_home, history_filename);
                history_file
            }),
        );
        AppConfig { history_file }
    }

    /// Tries to generates a new `AppConfig` struct from file configuration.
    ///
    /// The function return `Err` if file is not valid.
    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        Ok(AppConfig::new()) // TODO: Implementar archivo de configuración.
    }
}
