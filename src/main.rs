use std::env;
use std::fs;

struct Config {
    search_string: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let search_string = args[1].clone();
        let filename = args[2].clone();
        Config {search_string, filename}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let content = fs::read_to_string(config.filename)
        .expect("Oops");
    println!("{:?}", content);
}