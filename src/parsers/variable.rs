#[derive(Debug)]
pub struct Assignement {
    name: String,
    value: Box<Statement>,
}

use nom::{self, IResult};

use crate::parsers::identifier::identifier;
use crate::parsers::string::string;
use crate::parsers::ws::ws;

use super::statements::Statement;
use super::value::value;

pub fn variable(i: &str) -> IResult<&str, Statement> {
    let (remaining, name) = identifier(i)?;
    let (remaining, _) = ws(equals)(remaining)?;
    let (remaining, value) = value(remaining)?;

    let name = name.to_string();

    Ok((
        remaining,
        Statement::Assignement(Assignement {
            name,
            value: Box::new(value),
        }),
    ))
}

fn equals(i: &str) -> IResult<&str, char> {
    nom::character::complete::char('=')(i)
}
