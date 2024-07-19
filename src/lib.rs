use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    let res = if config.case_insensitive {
        search_case_insensitive(&config.expression, &contents)
    } else {
        search(&config.expression, &contents)
    };

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
    pub case_insensitive: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Not enough arguments.");
        }

        let case_sensitivity = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            expression: &args[1],
            file: &args[2],
            case_insensitive: case_sensitivity,
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

fn search_case_insensitive<'a>(expression: &str, content: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let lower_case_expression = expression.to_lowercase();

    for line in content.lines() {
        let lower_case_line = line.to_lowercase();

        if lower_case_line.contains(&lower_case_expression) {
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
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
