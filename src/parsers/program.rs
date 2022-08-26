use std::fmt::Error;

use nom::{combinator::eof, error::VerboseError, Finish, IResult};

use crate::{parsers::function_declaration::function_declaration, BoxError};

use super::{
    function_declaration::FunctionDeclaration,
    statements::{statements, Statement, Statements},
};

#[derive(Debug, Clone)]
pub struct Program {
    functions: Vec<FunctionDeclaration>,
    pub statements: Statements,
}

pub fn end_of_file(i: &str) -> IResult<&str, &str> {
    eof(i)
}

pub fn program<'a>(i: &'a str) -> IResult<&'a str, Program, VerboseError<&str>> {
    //let i = i.clone().as_str();
    let mut program = Program {
        functions: Vec::new(),
        statements: Statements { body: Vec::new() },
    };

    println!("Input {i}");
    let (remaining, mut statements) = statements(i)?;
    println!("Result: {statements:?}");
    println!("Rem: {remaining}");
    program.statements.body.append(&mut statements.body);
    //end_of_file(remaining)?;

    /*let functions = statements
    .body
    .iter()
    .filter(|statement| match statement {
        Statement::FunctionDeclaration(_) => true,
        _ => false,
    })
    .map(|statement| match statement {
        Statement::FunctionDeclaration(fd) => program.functions.push(fd.clone()),
        _ => {}
    });*/

    Ok((remaining, program))
}
