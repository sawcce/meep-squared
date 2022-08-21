use nom::{branch::alt, multi::many0, IResult};

use super::{
    closure::Closure,
    function_call::{function_call, FunctionCall},
    function_declaration::{function_declaration, FunctionDeclaration},
    variable::{variable, Assignement},
    ws::ws,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Statements {
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Assignement(Assignement),
    FunctionCall(FunctionCall),
    FunctionDeclaration(FunctionDeclaration),
    String(String),
    Closure(Closure),
    Variable(String),
}

pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((function_declaration, variable, function_call))(i)
}

pub fn statements(i: &str) -> IResult<&str, Statements> {
    let (remaining, list) = many0(ws(statement))(i)?;
    Ok((remaining, Statements { body: list }))
}
