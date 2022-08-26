use nom::{error::VerboseError, IResult};

pub fn identifier(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    let (remaining, id) =
        nom::bytes::complete::is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(i)?;
    Ok((remaining, id.to_string()))
}
