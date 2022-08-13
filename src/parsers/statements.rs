use nom::{branch::alt, multi::many0, IResult};

use super::{
    function_call::{function_call, FunctionCall},
    variable::{variable, Assignement},
    ws::ws,
};

#[derive(Debug)]
pub struct Statements {
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Assignement(Assignement),
    FunctionCall(FunctionCall),
    String(String),
    Tuple(),
    Closure(),
}

pub fn statement(i: &str) -> IResult<&str, Statement> {
    alt((variable, function_call))(i)
}

pub fn statements(i: &str) -> IResult<&str, Statements> {
    let (remaining, list) = many0(ws(statement))(i)?;
    Ok((remaining, Statements { body: list }))
}
