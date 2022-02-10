use std::fs::File;
use std::io::prelude::*;
use std::string::String;

mod parser;
use parser::parser;

fn main() {
    parser(open_file("../assets/example_simple.txt"));
}

// open contents of a file and returns it as a string
fn open_file(path: &str) -> String {
    let mut file = File::open(path).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
    contents
}