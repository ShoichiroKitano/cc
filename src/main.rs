use std::fs;

use cc::parser;

fn main() {
    let input = fs::read_to_string("test/add.c").unwrap();
    match parser::parse_compilation_unit(input.as_str()) {
        Ok(n) => println!("{:?}", n),
        Err(_) => println!("Error"),
    }
}
