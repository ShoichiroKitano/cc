use std::fs;

use crate::parser;
use string_builder::Builder;

pub fn compile(source_file: &str) {
    let input = fs::read_to_string(source_file).unwrap();
    let compilation_unint = match parser::parse_compilation_unit(input.as_str()) {
        Ok((input, compilation_unint)) => {
            println!("input: {}", input);
            println!("");
            println!("");
            println!("{:?}", compilation_unint);
            compilation_unint
        },
        Err(_) => panic!("compile error"),
    };
    let mut builder = Builder::new(1000);
    compilation_unint.write_assembly(&mut builder);
    println!("{}", builder.string().unwrap());
}
