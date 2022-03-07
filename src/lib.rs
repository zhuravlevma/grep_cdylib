use crate::config::Config;
use crate::functions::GetStrResult;
use crate::search::{search, search_case_insensitive};

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
        assert_eq!(vec!["safe, productive, fast"], search(query, contents))
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

mod config;
mod functions;
mod run;
mod search;
