use nom::{
    bytes::complete::tag,
    combinator::{not, peek},
    error::VerboseError,
    IResult,
};

pub fn identifier(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    peek(not(tag("end")))(i)?;
    peek(not(tag("return")))(i)?;
    let (remaining, id) =
        nom::bytes::complete::is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(i)?;
    Ok((remaining, id.to_string()))
}
