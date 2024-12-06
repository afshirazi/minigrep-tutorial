use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}\nin {}.", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
