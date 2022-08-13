use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::none_of,
    sequence::delimited,
    IResult,
};

use super::statements::Statement::{self};

pub fn string(input: &str) -> IResult<&str, Statement> {
    let esc = escaped(none_of("\\\""), '\\', tag("\""));
    let esc_or_empty = alt((esc, tag("")));
    let (remaining, res) = delimited(tag("\""), esc_or_empty, tag("\""))(input)?;

    Ok((remaining, Statement::String(String::from(res))))
}
