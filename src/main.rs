use std::env;
use std::error::Error;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}\nin {}.", config.query, config.file_path);

    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
