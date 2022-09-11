use nom::{error::VerboseError, IResult};

use super::statements::Statement;

enum Operation {
    Add(Statement, Statement),
    Substraction(Statement, Statement),
    Equals(Statement, Statement),
}

// fn addition(i: &str) -> IResult<&str, Statement, VerboseError<&str>> {}
