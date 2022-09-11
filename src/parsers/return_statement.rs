use nom::{
    bytes::complete::tag,
    error::{ErrorKind, VerboseError, VerboseErrorKind},
    IResult,
};

use super::{
    statements::{statement, Statement},
    value::value,
    ws::ws,
};

pub fn return_statement(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, _) = ws(tag("return"))(i)?;
    let (remaining, returned) = value(remaining)?;

    match returned {
        Statement::Return(_) => {
            return Err(nom::Err::Error(VerboseError {
                errors: vec![(
                    remaining,
                    VerboseErrorKind::Context("Cannot return a return statement"),
                )],
            }))
        }
        _ => {}
    }

    Ok((remaining, Statement::Return(Box::new(returned))))
}
