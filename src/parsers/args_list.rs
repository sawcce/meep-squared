use nom::{branch::alt, bytes::complete::tag, combinator::opt, multi::separated_list0, IResult};

use super::{identifier::identifier, statements::Statement, value::value, ws::ws};

pub fn list(i: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(","), ws(identifier))(i)
}

pub fn args_call_list(i: &str) -> IResult<&str, Vec<Statement>> {
    separated_list0(tag(","), ws(value))(i)
}

pub fn underscore<T>(i: &str) -> IResult<&str, Vec<T>> {
    let (remaining, _) = nom::bytes::complete::tag("_")(i)?;
    Ok((remaining, Vec::new()))
}

pub fn args_list(i: &str) -> IResult<&str, Vec<String>> {
    let (remaining, args) = opt(alt((underscore, list)))(i)?;
    let args = args.unwrap_or(Vec::new());

    Ok((remaining, args))
}

pub fn call_list(i: &str) -> IResult<&str, Vec<Statement>> {
    let (remaining, args) = opt(alt((underscore, args_call_list)))(i)?;
    let args = args.unwrap_or(Vec::new());

    Ok((remaining, args))
}
