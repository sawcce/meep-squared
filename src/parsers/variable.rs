#[derive(Debug)]
pub struct Assignement {
    name: String,
    value: String,
}

use nom::{self, IResult};

use crate::parsers::identifier::identifier;
use crate::parsers::string::string;
use crate::parsers::ws::ws;

pub fn variable(i: &str) -> IResult<&str, Assignement> {
    let (remaining, name) = identifier(i)?;
    println!("Name: {name}");
    let (remaining, _) = ws(equals)(remaining)?;
    println!("whitespace");
    let (remaining, value) = string(remaining)?;
    println!("String {value}");

    let name = name.to_string();
    let value = value.to_string();

    Ok((remaining, Assignement { name, value }))
}

fn equals(i: &str) -> IResult<&str, char> {
    nom::character::complete::char('=')(i)
}
