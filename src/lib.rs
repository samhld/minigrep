use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let lines = match config.case_sensitive {
        true => match_lines(&contents, &config.query),
        false => match_lines_case_insensitive(&contents, &config.query)
    };
    println!("{:?}", lines);

    Ok(())
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file to search")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config{query, filename, case_sensitive})
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

pub fn match_lines_case_insensitive(s: &String, q: &String) -> Vec<String> {
    let mut lines = Vec::new();
    let q = q.to_lowercase();
    for line in s.lines() {
        if line.to_lowercase().contains(&q) {
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

    #[test]
    fn two_results() {
        let query = String::from("t");
        let contents = String::from("\
This is text containing
the word 'public'.
Confirmed.");
        
        assert_eq!(vec!["This is text containing","the word 'public'."], match_lines(&contents, &query))
    }

    #[test]
    fn match_case_insensitive() {
        let query = String::from("TH");
        let contents = String::from("\
This is text containing
the word 'public'.
Confirmed.");
        
        assert_eq!(vec!["This is text containing","the word 'public'."], match_lines_case_insensitive(&contents, &query))
    }
}