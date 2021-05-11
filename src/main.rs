use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match run(config) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("Program error: {}", e);
            process::exit(1);
        }
    }
}
