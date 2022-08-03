use nom::IResult;

pub fn identifier(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_a("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_")(i)
}
