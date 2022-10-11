#[derive(Debug)]
pub struct CompilationUnit {
    pub functions: Function,
}

#[derive(Debug)]
pub struct Function {
    pub name: Identifier,
    pub return_type: Identifier,
    pub argments: Vec<Argment>,
    pub body: Vec<Box<dyn Statement>>,
}

pub trait Statement: std::fmt::Debug {}

#[cfg(test)]
impl PartialEq<dyn Statement> for dyn Statement {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
}

#[derive(Debug, PartialEq)]
pub struct EmptyStatement {}
impl Statement for EmptyStatement {}

#[derive(Debug)]
pub struct ReturnStatement {}
impl Statement for ReturnStatement {}

pub trait Expression: std::fmt::Debug {}

#[derive(Debug)]
pub struct BinaryOperation {
    pub operand1: Box<dyn Expression>,
    pub operator: Operator,
    pub operand2: Box<dyn Expression>,
}
impl Expression for BinaryOperation {}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
}

#[derive(Debug)]
pub struct Variable {
    pub name: Identifier,
}
impl Expression for Variable {}

#[derive(Debug)]
pub struct Argment {
    pub value_type: Identifier,
    pub name: Identifier,
}

#[derive(Debug)]
pub struct Identifier {
    pub value: String,
}
