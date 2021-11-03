use std::env;
use std::process;

use minigrep::Config;
use colored::*;

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("{} {} !", "it".green(), "works".blue().bold());

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}