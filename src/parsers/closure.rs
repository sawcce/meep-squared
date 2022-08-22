use crate::parsers::args_list::args_list;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::char,
    IResult,
};

use super::{
    statements::{statement, statements, Statements},
    ws::ws,
};

fn one_statement(i: &str) -> IResult<&str, Statements> {
    let (remaining, statement) = statement(i)?;
    Ok((
        remaining,
        Statements {
            body: vec![statement],
        },
    ))
}

fn multiple_statements(i: &str) -> IResult<&str, Statements> {
    let (remaining, statements) = ws(statements)(i)?;
    ws(tag("end"))(i)?;
    Ok((remaining, statements))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub arguments: Vec<String>,
    pub body: Statements,
}

pub fn closure(i: &str) -> IResult<&str, Closure> {
    let (remaining, arguments) = ws(args_list)(i)?;
    let (remaining, _) = ws(tag("->"))(remaining)?;
    let (remaining, body) = /*alt((*/multiple_statements/*, one_statement))*/(remaining)?;

    Ok((remaining, Closure { arguments, body }))
}
