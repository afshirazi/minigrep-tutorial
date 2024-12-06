
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let poem = fs::read_to_string(config.file_path)?;

    println!("Has content: {poem}");

    Ok(())
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Usage: minigrep <query> <file>");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
