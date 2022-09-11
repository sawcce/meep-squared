use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

use super::{statements::Statement, ws::ws};

fn True(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, _) = tag("true")(i)?;
    Ok((remaining, Statement::Boolean(true)))
}

fn False(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, _) = tag("false")(i)?;
    Ok((remaining, Statement::Boolean(false)))
}

pub fn boolean(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    alt((True, False))(i)
}
