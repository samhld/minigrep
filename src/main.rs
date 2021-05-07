use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.filename); 

    match run(config) {
        Ok(()) => println!("Process complete"),
        Err(e) => {
            eprintln!("Program error: {}", e);
            process::exit(1);
        }
    }
}
