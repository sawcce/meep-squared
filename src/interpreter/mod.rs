use std::rc::Rc;

use dashmap::DashMap;
use nom::Finish;

use crate::parsers::{
    program::program,
    statements::{statements, Statement, Statements},
};

struct Variable {
    id: String,
}

#[derive(Debug, Clone)]
struct Scope {
    values: DashMap<String, String>,
}

#[derive(Debug, Clone)]

enum Value {
    String(String),
}

type Id = String;

#[derive(Clone, Debug)]
enum Instruction {
    Test(String),
    VariableAssignement(Id, Value),
    PopVariable(Id),
    ExecuteFunction(Id, Vec<Value>),
}

#[derive(Clone, Debug)]
pub struct Compiler {
    scope: Vec<Scope>,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scope: vec![Scope {
                values: DashMap::new(),
            }],
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, code: &str) {
        let result = program(code);

        if result.clone().is_err() {
            let x = result.finish().err().unwrap();
            return;
        }

        let result_program = result.unwrap();
        println!("{:?}", result_program.clone().1.statements);
        self.generate_instruction(result_program.1.statements);
        println!("Instructions: {:?}", self.instructions);
    }

    fn generate_instruction(&mut self, statements: Statements) {
        for statement in statements.body.into_iter() {
            match statement {
                Statement::FunctionDeclaration(decl) => {
                    self.instructions.push(Instruction::Test(format!(
                        "Function Declaration: {}",
                        decl.name
                    )));
                    self.generate_instruction(decl.closure.body);
                    self.instructions.push(Instruction::Test(format!(
                        "End Function Declaration: {}",
                        decl.name
                    )));
                }
                Statement::Assignement(var) => self.instructions.push(Instruction::Test(format!(
                    "Var: {}, Val: {:?}",
                    var.name, var.value
                ))),
                Statement::FunctionCall(fc) => self.instructions.push(Instruction::Test(format!(
                    "Function Call, Name: {}, Args: {:?}",
                    fc.name, fc.arguments
                ))),
                _ => {}
            }
        }
    }

    fn eval(self, statement: Statement) -> Value {
        match statement {
            Statement::String(value) => Value::String(value),
            Statement::Closure(_) => todo!(),
            Statement::Variable(_) => todo!(),
            _ => Value::String(String::new()),
        }
    }
}
