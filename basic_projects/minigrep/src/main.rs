
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("the count of args is too small.. it must be greater than 3.");
    }

    let (query, filename) = parse_config(&args);

    let contents = fs::read_to_string(filename)
        .expect("can not read a file.");

    println!("contents: {}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
