use crate::parsers::args_list::args_list;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take},
    character::complete::char,
    error::VerboseError,
    IResult,
};

use super::{
    statements::{statement, statements, Statements},
    ws::ws,
};

fn one_statement(i: &str) -> IResult<&str, Statements, VerboseError<&str>> {
    let (remaining, statement) = statement(i)?;
    Ok((
        remaining,
        Statements {
            body: vec![statement],
        },
    ))
}

fn multiple_statements(i: &str) -> IResult<&str, Statements, VerboseError<&str>> {
    let (remaining, statements) = ws(statements)(i)?;
    let (remaining, _) = ws(tag("end"))(remaining)?;
    Ok((remaining, statements))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    pub arguments: Vec<String>,
    pub body: Statements,
}

pub fn closure(i: &str) -> IResult<&str, Closure, VerboseError<&str>> {
    let (remaining, arguments) = ws(args_list)(i)?;
    let (remaining, _) = ws(tag("->"))(remaining)?;
    let (remaining, body) = multiple_statements(remaining)?;
    //let (remaining, body) = /*alt((*/multiple_statements/*, one_statement))*/(remaining)?;

    Ok((remaining, Closure { arguments, body }))
}
