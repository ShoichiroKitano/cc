use crate::ast::*;
use crate::extruct::*;

use nom::{character::complete::space1, IResult};

pub fn parse_compilation_unit(input: &str) -> IResult<&str, CompilationUnit> {
    let (input, function) = parse_function(input)?;
    Ok((
        input,
        CompilationUnit {
            functions: function,
        },
    ))
}

pub fn parse_function(input: &str) -> IResult<&str, Function> {
    let (input, ret_type) = extruct_identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, function_name) = extruct_identifier(input)?;
    let (input, argments) = parse_argments(input)?;
    Ok((
        input,
        Function {
            name: Identifier {
                value: function_name.to_string(),
            },
            return_type: Identifier {
                value: ret_type.to_string(),
            },
            argments: argments,
            body: Vec::new(),
        },
    ))
}

pub fn parse_argments(input: &str) -> IResult<&str, Vec<Argment>> {
    let (input, argment_list) = extruct_argments(input)?;
    let argments = argment_list
        .iter()
        .map(|a| Argment {
            value_type: Identifier {
                value: a.0.to_string(),
            },
            name: Identifier {
                value: a.1.to_string(),
            },
        })
        .collect();
    Ok((input, argments))
}
