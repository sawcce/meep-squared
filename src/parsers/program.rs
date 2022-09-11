use std::fmt::Error;

use nom::{
    combinator::{all_consuming, eof},
    error::VerboseError,
    Finish, IResult,
};

use crate::{parsers::function_declaration::function_declaration, BoxError};

use super::{
    function_declaration::FunctionDeclaration,
    statements::{statements, Statement, Statements},
};

#[derive(Debug, Clone)]
pub struct Program {
    pub main_id: String,
    functions: Vec<FunctionDeclaration>,
    pub statements: Statements,
}

pub fn end_of_file(i: &str) -> IResult<&str, &str> {
    eof(i)
}

pub fn program<'a>(i: &'a str) -> IResult<&'a str, Program, VerboseError<&str>> {
    //let i = i.clone().as_str();
    let mut program = Program {
        main_id: "".to_string(),
        functions: Vec::new(),
        statements: Statements { body: Vec::new() },
    };

    // println!("Input {i}");
    let (remaining, mut statements) = all_consuming(statements)(i)?;
    // println!("Result: {statements:?}");
    // println!("Rem: {remaining}");
    program.statements.body.append(&mut statements.body);

    for statement in program.statements.body.iter() {
        match statement {
            Statement::FunctionDeclaration(declaration) => {
                if declaration.name == "main".to_string() {
                    program.main_id = declaration.id.clone();
                }
            }
            _ => {}
        }
    }

    Ok((remaining, program))
}
