
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("the count of args is too small.. it must be greater than 3.");
    }

    let config = parse_config(&args);

    let contents = fs::read_to_string(config.filename)
        .expect("can not read a file.");

    println!("contents: {}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
