use crate::{search, search_case_insensitive, Config};
use std::error::Error;
use std::fs;

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

pub unsafe extern "C" fn run_c(config: *const Config) {
    let config = &*config;
    run(config.clone()).unwrap()
}
