use nom::{
    bytes::complete::tag,
    combinator::opt,
    error::{VerboseError, VerboseErrorKind},
    IResult,
};

use super::{
    closure::Closure,
    statements::{statement, statements, Statement, Statements},
    value::value,
    ws::ws,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    pub main_condition: Box<(Statement, Statements)>,
    pub alternates: Vec<(Statement, Statements)>,
    pub fallback: Option<Statements>,
}

fn arrow(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    ws(tag("->"))(i)
}

//fn else_if(i: &str) -> IResult<&str, (Statement, Statements), VerboseError<&str>> {}

fn else_parser(i: &str) -> IResult<&str, Statements, VerboseError<&str>> {
    let (remaining, _) = ws(tag("else"))(i)?;
    statements(remaining)
}

pub fn conditional_statement(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, _) = ws(tag("if"))(i)?;
    let (remaining, condition) = ws(value)(remaining)?;
    let (remaining, _) = arrow(remaining)?;
    let (remaining, body) = opt(ws(statements))(remaining)?;
    let body = body.unwrap_or(Statements { body: Vec::new() });

    let (remaining, fallback) = opt(else_parser)(remaining)?;
    let (remaining, _) = ws(tag("end"))(remaining)?;

    Ok((
        remaining,
        Statement::Conditional(Conditional {
            main_condition: Box::new((condition, body)),
            alternates: Vec::new(),
            fallback,
        }),
    ))
}
