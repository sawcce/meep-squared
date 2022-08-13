use nom::{branch::alt, IResult};

use super::{
    function_call::function_call, statements::Statement, string::string, variable::variable,
};

pub fn value(i: &str) -> IResult<&str, Statement> {
    alt((function_call, variable, string))(i)
}
