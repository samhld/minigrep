use std::env;
use std::process;

use minigrep::{Config, run, match_lines};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {} in file {}", config.query, config.filename); 

    match run(config) {
        Ok(()) => println!("Process complete"),
        Err(e) => {
            println!("Program error: {}", e);
            process::exit(1);
        }
    }
}
