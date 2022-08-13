use nom::{
    branch::alt,
    combinator::{self, map},
    IResult,
};

use super::{
    function_call::function_call, identifier::identifier, statements::Statement, string::string,
    variable::variable,
};

pub fn value(i: &str) -> IResult<&str, Statement> {
    alt((
        function_call,
        map(identifier, |name| Statement::Variable(name)),
        string,
    ))(i)
}
