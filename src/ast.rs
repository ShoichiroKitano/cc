#[derive(Debug)]
pub struct CompilationUnit {
    pub functions: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub return_type: Identifier,
    pub argments: Vec<Argment>,
    pub body: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    EmptyStatement,
    ExpressionStatement { expression: Expression },
    ReturnStatement { expression: Expression },
}

//pub trait Statement: std::fmt::Debug {}
//
//#[cfg(test)]
//impl PartialEq<dyn Statement> for dyn Statement {
//    fn eq(&self, other: &Self) -> bool {
//        format!("{:?}", self) == format!("{:?}", other)
//    }
//}

//#[derive(Debug, PartialEq)]
//pub struct EmptyStatement {}
// impl Statement for EmptyStatement {}

//#[derive(Debug)]
//pub struct ReturnStatement {}
// impl Statement for ReturnStatement {}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    BinaryOperation {
        operand1: Box<Expression>,
        operator: Operator,
        operand2: Box<Expression>,
    },
}
// pub trait Expression: std::fmt::Debug {}

//#[derive(Debug)]
//pub struct BinaryOperation {
//    pub operand1: Box<dyn Expression>,
//    pub operator: Operator,
//    pub operand2: Box<dyn Expression>,
//}
// impl Expression for BinaryOperation {}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
}

impl Operator {
    pub fn new(operator: &str) -> Self {
        match operator {
            "+" => Operator::Add,
            "-" => Operator::Sub,
            _ => unimplemented!("そんなもんしらん"),
        }
    }
}

#[derive(Debug)]
pub struct Argment {
    pub value_type: Identifier,
    pub name: Identifier,
}

#[derive(Debug, PartialEq)]
pub struct Identifier(pub String);
