use crate::parsers::args_list::args_list;
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    combinator::opt,
    IResult,
};

use super::{
    statements::{statement, statements, Statement, Statements},
    ws::ws,
};

pub fn underscore(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::tag("_")(i)
}

fn one_statement(i: &'static str) -> IResult<&str, Statements> {
    let (remaining, statement) = statement(i)?;
    Ok((
        remaining,
        Statements {
            body: vec![statement],
        },
    ))
}

fn multiple_statements(i: &'static str) -> IResult<&str, Statements> {
    let (remaining, statements) = ws(statements)(i)?;
    ws(tag("end"))(i)?;
    Ok((remaining, statements))
}

pub fn closure(i: &'static str) -> IResult<&str, Statements> {
    let (remaining, arguments) = args_list(i)?;
    ws(tag_no_case("->"))(remaining)?;
    let (remaining, body) = alt((one_statement, multiple_statements))(remaining)?;

    Ok((remaining, body))
}
