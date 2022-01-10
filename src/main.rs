mod lib;
use std::env;
use std::process;
use lib::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem with parse arguments, {}", err);
        process::exit(1);
    });
    if let Err(e) = lib::run(config) {
        eprintln!("Error in app is {}", e);
        process::exit(1);
    };
}