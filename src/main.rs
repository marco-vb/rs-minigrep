use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = parse_args(&args);

    println!("Searching for {} in {}.", config.expression, config.file);

    let contents = fs::read_to_string(config.file).expect("Could not read from specified file");

    println!("File contents:\n{contents}");
}

struct Config<'a> {
    expression: &'a String,
    file: &'a String,
}

fn parse_args(args: &Vec<String>) -> Config {
    if args.len() != 3 {
        panic!("usage: minigrep <expression> <file_path>");
    }

    Config { expression: &args[1], file: &args[2] }
}
