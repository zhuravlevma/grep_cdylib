use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let search_string = &args[1];
    let filename = &args[2];

    let content = fs::read_to_string(filename)
        .expect("Oops");
    println!("{:?}", content);
}
