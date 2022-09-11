use nom::{branch::alt, combinator::map, error::VerboseError, IResult};

use super::number::number;
use super::{
    boolean::boolean, function_call::function_call, identifier::identifier, statements::Statement,
    string::string,
};

pub fn value(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {
    let res = alt((
        function_call,
        string,
        number,
        boolean,
        map(identifier, |name| Statement::Variable(name)),
    ))(i)?;

    Ok(res)
}
