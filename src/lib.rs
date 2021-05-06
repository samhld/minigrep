use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines = match_lines(&contents, &config.query);
    println!("{:?}", lines);

    Ok(())
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        
        Ok(Config{query, filename})
    }
}

pub fn match_lines(s: &String, q: &String) -> Vec<String> {
    let mut lines = Vec::new();
    for line in s.lines() {
        if line.contains(q) {
            lines.push(line.to_string())
        }
    }
    lines
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = String::from("public");
        let contents = String::from("\
This is text containing
the word 'public'.
Confirmed.");

        assert_eq!(vec!["the word 'public'."], match_lines(&contents, &query))
    }
}