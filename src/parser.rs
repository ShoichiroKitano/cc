use crate::ast::*;
use crate::extruct::*;

use nom::{
    branch::{alt},
    character::complete::{multispace0, multispace1},
    bytes::complete::tag,
    combinator::{opt},
    error::VerboseError,
    multi::{many_till},
    sequence::{tuple},
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

fn parse_function(input: &str) -> IResult<&str, Function, VerboseError<&str>> {
    let (input, ret_type) = extruct_identifier(input)?;
    let (input, _) = multispace1(input)?; // type␣name
    let (input, function_name) = extruct_identifier(input)?;
    let (input, _) = multispace0(input)?; // name␣(argmennts)
    let (input, argments) = parse_argments(input)?;
    let (input, _) = tuple((multispace0, tag("{"), multispace0))(input)?; // () { statements or (){statements
    let (input, (statements, _)) = many_till(parse_statement, tag("}"))(input)?; // statements }
    let (input, _) = multispace0(input)?;
    // Ok((input, statements)) => (input, statements),
    // Err(nom::Err::Error(e)) => {
    //     println!("{}", nom::error::convert_error(input, e.clone()));
    //     return Err(nom::Err::Error(e));
    // }
    // Err(e) => return Err(e),

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
            body: statements,
        },
    ))
}

fn parse_statement(input: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    if let Ok((input, Some(_))) = opt(tag::<&str, &str, nom::error::Error<&str>>(";"))(input) {
        return Ok((input, Statement::EmptyStatement));
    }
    if let Ok((input, Some(_))) = opt(
        tuple((tag::<&str, &str, nom::error::Error<&str>>("return"), multispace1))
        )(input) {
        let (input, expression) = parse_expression(input)?;
        let (input, _) = tuple((multispace0, tag(";"), multispace0))(input)?;
        return Ok((input, Statement::ReturnStatement {expression: expression}));
    }
    if let Ok((input, Some(expression))) = opt(parse_expression)(input) {
        let (input, _) = tuple((multispace0, tag(";"), multispace0))(input)?;
        return Ok((input, Statement::ExpressionStatement {expression: expression}));
    }
    panic!("parse_statement error!");
}

fn parse_expression(input: &str) -> IResult<&str, Expression, VerboseError<&str>> {
    let (input, identifier) = extruct_identifier(input)?;
    let (input, _) = multispace0(input)?;
    match opt(alt((tag("+"), tag("-"))))(input)? {
        (input, Some(operator)) => {
            let (input, _) = multispace0(input)?;
            let (input, operand) = parse_expression(input)?;
            return Ok((input, Expression::BinaryOperation{
                operand1: Box::new(Expression::Identifier(Identifier{value: identifier.to_string()})),
                operator: Operator::new(operator),
                operand2: Box::new(operand),
            }));
        },
        _ => {},
    };
    Ok((input, Expression::Identifier(Identifier {value: identifier.to_string()})))
}

// parse "(argment list)" to argmensts
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
    fn parse_no_expression_before_semicolon() {
        assert_eq!(
            super::parse_statement(";rest code"),
            Ok((
                "rest code",
                Statement::EmptyStatement,
            ))
        );
    }

    #[test]
    fn parse_return_with_variable() {
        assert_eq!(
            super::parse_statement("return a;rest code"),
            Ok((
                "rest code",
                Statement::ReturnStatement{
                    expression: Expression::Identifier(Identifier{value: "a".to_string() }),
                }
            ))
        );
    }

    #[test]
    fn parse_return_with_addition() {
        assert_eq!(
            super::parse_statement("return a + b;rest code"),
            Ok((
                "rest code",
                Statement::ReturnStatement{
                    expression: Expression::BinaryOperation {
                        operand1: Box::new(Expression::Identifier(Identifier{value: "a".to_string() })),
                        operator: Operator::Add,
                        operand2: Box::new(Expression::Identifier(Identifier{value: "b".to_string() })),
                    }
                }
            ))
        );
    }

    #[test]
    fn parse_return_with_subtraction() {
        assert_eq!(
            super::parse_statement("return a - b;rest code"),
            Ok((
                "rest code",
                Statement::ReturnStatement{
                    expression: Expression::BinaryOperation {
                        operand1: Box::new(Expression::Identifier(Identifier{value: "a".to_string() })),
                        operator: Operator::Sub,
                        operand2: Box::new(Expression::Identifier(Identifier{value: "b".to_string() })),
                    }
                }
            ))
        );
    }

    #[test]
    fn parse_expression_statement() {
        assert_eq!(
            super::parse_statement("a;rest code"),
            Ok((
                "rest code",
                Statement::ExpressionStatement{
                    expression: Expression::Identifier(Identifier{value: "a".to_string() })
                }
            ))
        );
    }
}
