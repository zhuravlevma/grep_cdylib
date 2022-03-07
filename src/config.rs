use std::env;

#[repr(C)]
#[derive(Clone)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn _new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let query = match args.next() {
            None => return Err("query not received"),
            Some(value) => value,
        };
        let filename = match args.next() {
            None => return Err("filename not received"),
            Some(value) => value,
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
