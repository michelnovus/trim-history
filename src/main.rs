// [MIT License] Copyright (c) 2024 Michel Novus

mod config;
mod functions;

use config::AppConfig;
use std::process;

fn main() {
    let config = AppConfig::new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}
