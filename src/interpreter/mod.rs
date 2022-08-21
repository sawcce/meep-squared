use std::rc::Rc;

use dashmap::DashMap;

use crate::parsers::statements::{statements, Statement};

struct Variable {
    id: String,
}

#[derive(Debug, Clone)]
struct Scope {
    values: DashMap<String, String>,
    parent: Option<Rc<Scope>>,
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
    scope: Scope,
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            scope: Scope {
                values: DashMap::new(),
                parent: None,
            },
            instructions: Vec::new(),
        }
    }

    pub fn compile(&mut self, code: &str) {
        let (_, statements) = statements(code).unwrap();
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
