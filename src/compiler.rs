use std::fs;

use crate::parser;

pub fn compile(source_file: &str) {
    let input = fs::read_to_string(source_file).unwrap();
    match parser::parse_compilation_unit(input.as_str()) {
        Ok(n) => println!("{:?}", n),
        Err(_) => println!("Error"),
    }
}
