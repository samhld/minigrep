use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);

    let contents = fs::read_to_string(filename).expect("There was a problem opening file");

    let lines = match_lines(&contents, query);

    println!("{:?}", lines)
}



fn parse_args(args: &[String]) -> (&String, &String) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
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
