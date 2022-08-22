use nom::{error::ParseError, IResult};

use super::{
    closure::{closure, Closure},
    identifier::identifier,
    statements::Statement,
    ws::ws,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub closure: Closure,
}

pub fn function_declaration<E: ParseError<&str>>(i: &str) -> IResult<&str, Statement, E> {
    let (remaining, name) = ws(identifier)(i)?;
    let (remaining, closure) = ws(closure)(remaining)?;
    Ok((
        remaining,
        Statement::FunctionDeclaration(FunctionDeclaration { name, closure }),
    ))
}
