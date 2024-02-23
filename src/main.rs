// [MIT License] Copyright (c) 2024 Michel Novus

mod config;
mod functions;

use config::AppConfig;
use functions::*;
use std::process;

fn main() {
    let config = AppConfig::new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    if !is_histfile(&config.history_file) {
        eprintln!(
            "`{}` file does not look like a valid history file, \
            abort for safety.",
            &config.history_file.to_str().unwrap()
        );
        process::exit(1);
    }
    let file_content = readlines(&config.history_file).unwrap_or_else(|err| {
        eprintln!("Error in read history file: {}", err);
        process::exit(1);
    });
    let file_content: Vec<&str> =
        file_content.iter().map(AsRef::as_ref).collect();

    let trimed_file = match rm_dups(&file_content) {
        Some(data) => data,
        None => {
            eprintln!("Nothing to do.");
            process::exit(0);
        }
    };

    writelines(&config.history_file, &trimed_file).unwrap_or_else(|err| {
        eprintln!("Error on saving: {}", err);
        process::exit(1);
    });
}
