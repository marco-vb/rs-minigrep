use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    let res = search(&config.expression, &contents);

    print_res(&res);
    Ok(())
}

fn print_res(vec: &Vec<&str>) {
    for el in vec {
        println!("{el}");
    }
}

pub struct Config<'a> {
    pub expression: &'a String,
    pub file: &'a String,
}

impl<'a> Config<'a> {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments.");
        }

        Ok(Config {
            expression: &args[1],
            file: &args[2],
        })
    }
}

fn search<'a>(expression: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.contains(expression) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
