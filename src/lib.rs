use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            None => return Err("query not received"),
            Some(value) => value
        };
        let filename = match args.next() {
            None => return Err("filename not received"),
            Some(value) => value
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &content)
    } else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{:?}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, productive, fast
Pick three.
        ";
        assert_eq!(
            vec!["safe, productive, fast"],
            search(query, contents)
        )
    }
    #[test]
    fn case_insensitive() {
        let query = "RuST";
        let contents = "\
Rust:
safe, productive, fast
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}