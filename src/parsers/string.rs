use nom::{
    branch::alt,
    bytes::complete::{escaped, tag},
    character::complete::none_of,
    sequence::delimited,
    IResult,
};

pub fn string(input: &str) -> IResult<&str, &str> {
    let esc = escaped(none_of("\\\""), '\\', tag("\""));
    let esc_or_empty = alt((esc, tag("")));
    let res = delimited(tag("\""), esc_or_empty, tag("\""))(input)?;

    Ok(res)
}
