use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    combinator::opt,
    error::VerboseError,
    IResult,
};

use super::{statements::Statement, ws::ws};

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i32),
    Float(f32),
}

fn int(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, result) = opt(tag("-"))(i)?;

    let sign = result.unwrap_or("");

    let (remaining, result) = is_a("0123456789")(remaining)?;
    let result = format!("{sign}{result}");
    let number = result.parse::<i32>().unwrap();

    Ok((remaining, Statement::Number(Number::Int(number))))
}

fn float(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, whole_part) = is_a("0123456789")(i)?;
    let (remaining, _) = ws(tag("."))(remaining)?;
    let (remaining, decimal_part) = is_a("0123456789")(remaining)?;

    let str_rep = format!("{}.{}", whole_part, decimal_part);

    let number = str_rep.parse::<f32>().unwrap();

    Ok((remaining, Statement::Number(Number::Float(number))))
}

pub fn number(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    alt((float, int))(i)
}
