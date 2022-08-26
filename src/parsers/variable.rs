#[derive(Debug, PartialEq, Clone)]
pub struct Assignement {
    pub name: String,
    pub declaration: bool,
    pub id: Option<String>,
    pub value: Box<Statement>,
}

use nom::combinator::opt;
use nom::error::VerboseError;
use nom::{self, bytes::complete::tag, IResult};

use crate::parsers::identifier::identifier;
use crate::parsers::ws::ws;
use uuid::Uuid;

use super::statements::Statement;
use super::value::value;

pub fn variable(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let mut is_declaration = false;
    let mut id = None;

    let (remaining, result) = opt(ws(tag("let")))(i)?;

    if result.is_some() {
        is_declaration = true;
        id = Some(Uuid::new_v4().to_string());
    }

    let (remaining, name) = identifier(remaining)?;
    let (remaining, _) = ws(equals)(remaining)?;
    let (remaining, value) = value(remaining)?;

    let name = name.to_string();

    Ok((
        remaining,
        Statement::Assignement(Assignement {
            name,
            id,
            declaration: is_declaration,
            value: Box::new(value),
        }),
    ))
}

fn equals(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    nom::character::complete::char('=')(i)
}
