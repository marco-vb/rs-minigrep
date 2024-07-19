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

pub struct Config {
    pub expression: String,
    pub file: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let expression = match args.next() {
            Some(val) => val,
            None => return Err("Too few arguments."),
        };

        let file = match args.next() {
            Some(val) => val,
            None => return Err("Too few arguments."),
        };

        let case_sensitivity = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            expression,
            file,
            case_insensitive: case_sensitivity,
        })
    }
}

fn search<'a>(expression: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|el| el.contains(expression))
        .collect()
}

fn search_case_insensitive<'a>(expression: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|el| el.to_lowercase().contains(&expression.to_lowercase()))
        .collect()
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
