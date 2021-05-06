use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);
    let contents = fs::read_to_string(config.filename).expect("There was a problem opening file");
    let lines = match_lines(&contents, &config.query);

    println!("{:?}", lines)
}

struct Config {
    query: String,
    filename: String
}

fn parse_args(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
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
