use std::rc::Rc;

use dashmap::DashMap;

use crate::parsers::statements::{statements, Statement};

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
        let (_, statements) = program(code).unwrap();
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
