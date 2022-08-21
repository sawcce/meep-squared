use std::fmt::Error;

use nom::{combinator::eof, Finish, IResult};

use crate::BoxError;

use super::{
    function_declaration::FunctionDeclaration,
    statements::{statements, Statement, Statements},
    ws::ws,
};

struct Program {
    functions: Vec<FunctionDeclaration>,
    statements: Statements,
}

pub fn end_of_file(i: &str) -> IResult<&str, &str> {
    eof(i)
}

pub fn program(i: &str) -> Result<Program, BoxError> {
    //let i = i.clone().as_str();
    let mut program = Program {
        functions: Vec::new(),
        statements: Statements { body: Vec::new() },
    };

    let (remaining, statements) = statements(i)?;
    let a = end_of_file(remaining)?;

    let functions = statements
        .body
        .iter()
        .filter(|statement| match statement {
            Statement::FunctionDeclaration(_) => true,
            _ => false,
        })
        .map(|statement| match statement {
            Statement::FunctionDeclaration(fd) => program.functions.push(fd.clone()),
            _ => {}
        });

    Ok(program)
}
