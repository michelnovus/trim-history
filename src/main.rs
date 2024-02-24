// [MIT License] Copyright (c) 2024 Michel Novus

mod config;
mod files;
mod trimmer;

use config::AppConfig;
use files::is_histfile;
use std::fs;
use std::process;
use trimmer::Trimmer;

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

    let file_content = fs::read(&config.history_file).unwrap_or_else(|err| {
        eprintln!("Error in read history file: {}", err);
        process::exit(1);
    });
    let file_content = String::from_utf8_lossy(&file_content).into_owned();
    let mut trimmer = Trimmer::from(file_content.as_str());

    trimmer.allow_dedup();

    match trimmer.trim().unwrap() {
        Some(output) => {
            fs::write(&config.history_file, output.as_bytes()).unwrap_or_else(
                |err| {
                    eprintln!("Error on saving: {}", err);
                    process::exit(1);
                },
            );
            println!("Trimming success!");
        }
        None => {
            eprintln!("Nothing to do.");
            process::exit(0);
        }
    };
}
