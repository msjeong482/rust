
use std::process;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Occurs error parsing args: {}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.filename)
        .expect("can not read a file.");

    println!("contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("the count of args is too small.. it must be greater than 3.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
