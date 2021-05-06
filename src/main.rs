use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename); 

    run(config)

}

fn run(config: Config) {
    let contents = fs::read_to_string(config.filename).expect("There was a problem opening file");
    let lines = match_lines(&contents, &config.query);
    println!("{:?}", lines)
}

struct Config {
    query: String,
    filename: String
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config{query, filename})
    }
}

fn match_lines(s: &String, q: &String) -> Vec<String> {
    let mut lines = Vec::new();
    for line in s.lines() {
        if line.contains(q) {
            lines.push(line.to_string())
        }
    }
    lines
}
