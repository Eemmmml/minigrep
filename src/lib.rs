use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query_str: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, String> {
        args.next();

        let query_str = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(String::from("Expected query_str got None"));
            }
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(String::from("Expected filename got None"));
            }
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query_str,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let res = if config.case_sensitive {
        search_case_sensitive(&config.query_str, &content)
    } else {
        search_case_insensitive(&config.query_str, &content)
    };

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

/// # Example:
/// ```
/// let x = assert_eq!(1, 1);
/// ```
pub fn search_case_sensitive<'a>(query_str: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query_str))
        .collect()
}

pub fn search_case_insensitive<'a>(query_str: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&(query_str.to_lowercase())))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query_str = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query_str, content)
        );
    }

    #[test]
    fn case_insensitive() {
        let query_str = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query_str, content)
        );
    }
}
