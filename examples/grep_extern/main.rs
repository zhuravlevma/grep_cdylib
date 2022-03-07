use crate::interface::Config;
use crate::lib::Lib;
use crate::search::Search;
use interface::GetStrResult;
use interface::{lib_path, Functions, FunctionsFn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        query: "the".to_string(),
        filename: "poem.txt".to_string(),
        case_sensitive: true,
    };
    let factory = GrepFactory::new()?;
    let result = factory.search("Ha", "Hahaha\n HjHo\n HoHaha\n ")?;
    let result2 = factory.search_case_insensitive("Test", "test\ntet\nTEST")?;
    factory.run(config);
    println!("{:?}", result);
    println!("{:?}", result2);
    Ok(())
}

pub struct GrepFactory {
    lib: Lib,
}

impl GrepFactory {
    pub fn new() -> Result<Self, anyhow::Error> {
        let lib = unsafe {
            let lib = libloading::Library::new(lib_path())?;
            Lib::new(lib)?
        };

        Ok(Self { lib })
    }

    pub fn search(&self, query: &str, content: &str) -> Result<Vec<String>, anyhow::Error> {
        Ok(Search::search(self.lib.clone(), query, content))
    }

    pub fn search_case_insensitive(
        &self,
        query: &str,
        content: &str,
    ) -> Result<Vec<String>, anyhow::Error> {
        Ok(Search::search_case_insensitive(
            self.lib.clone(),
            query,
            content,
        ))
    }

    pub fn run(&self, config: Config) {
        Search::run(self.lib.clone(), config)
    }
}

mod interface;
mod lib;
mod search;
