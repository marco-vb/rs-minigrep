use minigrep::{run, Config};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config: Config = Config::build(&args).unwrap_or_else(|_| {
        println!("Usage: minigrep <expression> <file_path>");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: {e}");
        process::exit(1);
    }
}
