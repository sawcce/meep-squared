use nom::{branch::alt, combinator::all_consuming, error::VerboseError, multi::many0, IResult};

use super::{
    closure::Closure,
    conditional::{conditional_statement, Conditional},
    function_call::{function_call, FunctionCall},
    function_declaration::{function_declaration, FunctionDeclaration},
    number::Number,
    return_statement::return_statement,
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
    Number(Number),
    Boolean(bool),
    Return(Box<Statement>),
    Conditional(Conditional),
}

pub fn statement(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    alt((
        conditional_statement,
        return_statement,
        function_declaration,
        function_call,
        variable,
    ))(i)
}

pub fn statements(i: &str) -> IResult<&str, Statements, VerboseError<&str>> {
    let (remaining, list) = /*all_consuming(*/many0(ws(statement))/*)*/(i)?;

    Ok((remaining, Statements { body: list }))
}
