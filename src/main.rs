// [MIT License] Copyright (c) 2024 Michel Novus

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

/// Name of the history file environment variable.
const HISTORY_FILE_ENVAR_NAME: &str = "HISTFILE";
/// Name of shell environment variable, should be SHELL.
const SHELL_ENVAR_NAME: &str = "SHELL";

fn main() {
    let config = AppConfig::new();
}

/// Removes all duplicate elements in string list.
///
/// Function accept a vector of string slice and return a new vector
/// of string slice where every string is unique and mantains order of
/// first ocurrence. If both vectors are equals the function returns None.
fn rm_dups<'a>(list: &Vec<&'a str>) -> Option<Vec<&'a str>> {
    let mut output_list: Vec<&str> = Vec::new();
    for in_elem in list {
        if !output_list.contains(&in_elem) {
            output_list.push(*in_elem);
        }
    }
    if list.len() == output_list.len() {
        None
    } else {
        Some(output_list)
    }
}

/// Creates the program configuration file.
fn generate_config_file(path: PathBuf) -> io::Result<()> {
    Ok(())
}

/// Contains the initial configuration of the program.
struct AppConfig {
    history_file: PathBuf,
}

impl AppConfig {
    /// Generates a new `AppConfig` struct with defaults attributes.
    fn new() -> Self {
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
    fn from_file(path: PathBuf) -> io::Result<Self> {
        Ok(AppConfig::new()) // TODO: Implementar archivo de configuración.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rm_dups_test() {
        let list_dups = vec![
            "ls",
            "cd ~",
            "ls -la",
            "cargo run",
            "firefox",
            "exit",
            "logout",
            "ls",
            "lsblk",
            "lsblk -f",
            "rm -rf -no-preserve-root /",
            "rm file.txt",
            "lsblk",
            "ls",
            "cp f1 ./pepe",
            "cp f2 ../otro",
            "rm -rf -no-preserve-root /",
        ];
        let list_expected = vec![
            "ls",
            "cd ~",
            "ls -la",
            "cargo run",
            "firefox",
            "exit",
            "logout",
            "lsblk",
            "lsblk -f",
            "rm -rf -no-preserve-root /",
            "rm file.txt",
            "cp f1 ./pepe",
            "cp f2 ../otro",
        ];

        assert_eq!(list_expected, rm_dups(&list_dups).unwrap())
    }

    #[test]
    fn rm_dups_without_dups_test() {
        let list = vec!["cmd1", "cmd2", "cmd3", "cmd4", "cmd5"];
        assert_eq!(None, rm_dups(&list));
    }
}
