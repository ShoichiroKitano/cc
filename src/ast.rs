use string_builder::Builder;

#[derive(Debug)]
pub struct CompilationUnit {
    pub functions: Vec<FunctionDeclaration>,
}

impl CompilationUnit {

    pub fn write_assembly(&self, visitor: &mut Builder) {
        for function in self.functions.iter() {
            visitor.append("\t.section\t__TEXT,__text,regular,pure_instructions\n");
            visitor.append("\t.build_version macos, 12, 0>sdk_version 12, 3");
            function.write_assembly(visitor);
        }
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: Identifier,
    pub return_type: Identifier,
    pub argments: Vec<Argment>,
    pub body: Vec<Statement>,
}

impl FunctionDeclaration {

    pub fn write_assembly(&self, _visitor: &Builder) {
        //visitor.append(String::from("\t.global\t") + self.mangled_name().as_str());
        //visitor.append("\t.p2align\t4, 0x90");
        //visitor.append(self.mangled_name() + ":".as_str());
        //visitor.append("\t.pushq\t%rbp");
    }

    pub fn mangled_name(&self) -> String {
        let Identifier(name) = &self.name;
        String::from("_") + name.as_str()
    }
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
