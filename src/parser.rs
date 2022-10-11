use crate::ast::*;
use crate::extruct::*;

use nom::{
    branch::permutation,
    character::complete::{char, multispace0, space1},
    combinator::peek,
    error::VerboseError,
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

pub fn parse_compilation_unit(input: &str) -> IResult<&str, CompilationUnit, VerboseError<&str>> {
    let (input, function) = parse_function(input)?;
    Ok((
        input,
        CompilationUnit {
            functions: function,
        },
    ))
}

pub fn parse_function(input: &str) -> IResult<&str, Function, VerboseError<&str>> {
    let (input, ret_type) = extruct_identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, function_name) = extruct_identifier(input)?;
    let (input, argments) = parse_argments(input)?;
    let (input, statements) = match delimited(
        multispace0,
        delimited(
            char('{'),
            delimited(
                multispace0,
                separated_list0(
                    permutation((multispace0, char(';'), multispace0)),
                    parse_statement,
                ),
                multispace0,
            ),
            char('}'),
        ),
        multispace0,
    )(input)
    {
        Ok((input, statements)) => (input, statements),
        Err(nom::Err::Error(e)) => {
            println!("{}", nom::error::convert_error(input, e.clone()));
            return Err(nom::Err::Error(e));
        }
        Err(e) => return Err(e),
    };

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

fn parse_statement(input: &str) -> IResult<&str, Box<dyn Statement>, VerboseError<&str>> {
    if let Ok(_) = peek(char::<&str, nom::error::Error<&str>>(';'))(input) {
        return Ok((input, Box::new(EmptyStatement {})));
    }
    Ok((input, Box::new(EmptyStatement {})))
}

pub fn parse_argments(input: &str) -> IResult<&str, Vec<Argment>, VerboseError<&str>> {
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

#[cfg(test)]
mod parse_statement {
    use crate::ast::*;

    #[test]
    fn parse_only_semicolon_statement_to_empty_statement() {
        assert_eq!(
            super::parse_statement(";rest code"),
            Ok((
                ";rest code",
                Box::new(EmptyStatement {}) as Box<dyn Statement>
            ))
        );
    }
}
