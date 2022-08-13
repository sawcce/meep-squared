use nom::{bytes::complete::tag, sequence::delimited, IResult};

use super::{args_list::call_list, identifier::identifier, statements::Statement, ws::ws};

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Statement>,
}

pub fn function_call(i: &str) -> IResult<&str, Statement> {
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
