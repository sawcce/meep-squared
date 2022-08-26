use nom::{bytes::complete::tag, error::VerboseError, sequence::delimited, IResult};

use super::{args_list::call_list, identifier::identifier, statements::Statement, ws::ws};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Statement>,
}

pub fn function_call(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let (remaining, name) = ws(identifier)(i)?;
    let (remaining, args) = delimited(tag("("), ws(call_list), tag(")"))(remaining)?;

    Ok((
        remaining,
        Statement::FunctionCall(FunctionCall {
            name: String::from(name),
            arguments: args,
        }),
    ))
}
