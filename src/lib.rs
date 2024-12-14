use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Usage: minigrep <query> <file>");
        // }
        
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query not provided.\nUsage: minigrep <query> <file>"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path not provided.\nUsage: minigrep <query> <file>")
        };
        
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let poem = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &poem) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }

    // res
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}