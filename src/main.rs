// [MIT License] Copyright (c) 2024 Michel Novus

mod config;
mod functions;

use config::AppConfig;
use functions::is_histfile;
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
}
