use nom::{
    branch::alt,
    combinator::{self, map},
    error::{ParseError, VerboseError},
    IResult,
};

use super::variable::variable;
use super::{
    function_call::function_call, identifier::identifier, statements::Statement, string::string,
};

pub fn value(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let res = alt((
        function_call,
        map(identifier, |name| Statement::Variable(name)),
        string,
    ))(i)?;

    Ok(res)
}
