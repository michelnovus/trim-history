// [MIT License] Copyright (c) 2024 Michel Novus

mod config;
mod functions;

use config::AppConfig;
use functions::{is_histfile, rm_dups};
use std::fs::{read, write};
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
    let file_content_raw = read(&config.history_file).unwrap_or_else(|_| {
        eprintln!("Cannot open {}", &config.history_file.to_str().unwrap());
        process::exit(1);
    });
    let file_content = String::from_utf8_lossy(&file_content_raw);
    let file_content: Vec<&str> = file_content.lines().collect();

    let trimed_file = match rm_dups(&file_content) {
        Some(data) => data,
        None => {
            eprintln!("Nothing to do.");
            process::exit(0);
        }
    };

    let mut trimed_file_raw = String::new();
    let _ = trimed_file
        .into_iter()
        .map(|string| {
            trimed_file_raw.push_str(format!("{string}\n").as_str());
        })
        .collect::<Vec<_>>();

    let trimed_file_raw = trimed_file_raw.into_bytes();
    write(&config.history_file, trimed_file_raw).unwrap_or_else(|err| {
        eprintln!("Error on saving: {}", err);
        process::exit(1);
    });
}
