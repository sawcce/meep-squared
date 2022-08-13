use nom::IResult;

pub fn identifier(i: &str) -> IResult<&str, String> {
    let (remaining, id) =
        nom::bytes::complete::is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(i)?;
    Ok((remaining, id.to_string()))
}
