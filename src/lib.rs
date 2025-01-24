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
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!(
                "Paramater not enough, expect 2, got{}",
                args.len() - 1,
            ));
        }

        let query_str = args[1].clone();
        let filename = args[2].clone();
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

pub fn search_case_sensitive<'a>(query_str: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query_str) {
            result.push(line);
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query_str: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query_str = query_str.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query_str) {
            result.push(line);
        }
    }

    result
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
