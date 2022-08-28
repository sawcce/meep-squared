use std::{collections::HashMap, fmt, ops::Deref, rc::Rc};

use dashmap::DashMap;
use nom::{error::convert_error, Finish};

use crate::parsers::{
    function_declaration::FunctionDeclaration,
    program::program,
    statements::{statements, Statement, Statements},
    variable::Assignement,
};

struct Variable {
    id: String,
}

#[derive(Debug, Clone)]
struct Scope {
    pub values: DashMap<String, Id>,
}

impl Scope {
    fn new() -> Self {
        Scope {
            values: DashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]

pub enum Value {
    String(String),
    Number(i32),
    Closure { instructions: Vec<Instruction> },
    CopyVar(Id),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(content) => write!(f, "{content}"),
            Value::Number(num) => write!(f, "{num}"),
            Value::Closure { instructions } => {
                write!(
                    f,
                    "Closure ({})",
                    match instructions.first().unwrap() {
                        Instruction::ExtCall(x) => format!("ExtCall {:?}", x),
                        _ => format!("{} instructions", instructions.len()),
                    }
                )
            }
            Value::CopyVar(id) => write!(f, "{id}"),
        }
    }
}

#[derive(Debug, Clone)]
struct Entry {
    mutable: bool,
    value: Value,
}

type Id = String;

#[derive(Clone, Debug)]
/// An enum containing all the instructions the Engine can execute
pub enum Instruction {
    Test(String),
    /// A variable declaration taking its ID, wether it is mutable or not and the default value.
    VariableDeclaration(Id, bool, Value),
    VariableAssignement(Id, Value),
    PopVariable(Id),
    ExecuteFunction(Id, Vec<Value>),
    ExtCall(fn(Vec<Value>) -> Option<Value>),
}

#[derive(Debug)]
/// A struct that is responsible for parsing a program and generating a list of instructions that
/// will be fed to the engine
pub struct Compiler<'a> {
    scope: &'a mut Vec<HashMap<String, Id>>,
    pub instructions: Vec<Instruction>,
}

const STD_PREFIX: &str = "msq_std::";
const PRINT_ID: &str = "msq_std::print()";
const DATE_ID: &str = "msq_std::date()";

fn print(args: Vec<Value>) -> Option<Value> {
    for arg in args {
        print!("{}", arg);
    }
    println!("");
    None
}

fn date(args: Vec<Value>) -> Option<Value> {
    None
}

impl<'a> Compiler<'a> {
    pub fn new(scope: &'a mut Vec<HashMap<String, Id>>) -> Self {
        Self {
            scope,
            instructions: Vec::new(),
        }
    }

    fn prepare_defaults(&mut self) {
        self.scope_in();
        self.scope
            .first_mut()
            .unwrap()
            .insert("print".to_string(), PRINT_ID.to_string());

        self.instructions.push(Instruction::VariableDeclaration(
            PRINT_ID.to_string(),
            false,
            Value::Closure {
                instructions: vec![Instruction::ExtCall(print)],
            },
        ));

        self.scope_in();
    }

    pub fn compile(&mut self, code: &str) {
        let result = program(code);
        // println!("{result:#?}");

        if result.clone().is_err() {
            let x = result.finish().err().unwrap();
            let y = convert_error(code, x);

            println!("Error: {y}");
            return;
        }

        let result_program = result.unwrap();
        // println!("{:?}", result_program.clone().1.statements);
        self.prepare_defaults();
        let instructions = &mut self.generate_instruction(result_program.1.statements);
        self.instructions.append(instructions);
        self.instructions.push(Instruction::ExecuteFunction(
            result_program.1.main_id,
            Vec::new(),
        ));
        // println!("Instructions: {:?}", self.instructions);
    }

    fn generate_instruction(&mut self, statements: Statements) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for statement in statements.body.iter() {
            match statement {
                Statement::FunctionDeclaration(declaration) => {
                    instructions.extend_from_slice(
                        self.function_declaration(declaration.clone()).as_slice(),
                    );
                }
                Statement::Assignement(var) => {
                    instructions
                        .extend_from_slice(self.variable_assignement(var.clone()).as_slice());
                }
                Statement::FunctionCall(fc) => {
                    instructions.push(Instruction::ExecuteFunction(
                        self.resolve_variable(fc.clone().name).unwrap(),
                        fc.arguments
                            .iter()
                            .map(|arg| self.eval(arg.clone()))
                            .collect(),
                    ));
                }
                _ => {}
            }
        }

        return instructions;
    }

    fn function_declaration(&mut self, declaration: FunctionDeclaration) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        let _ = &self
            .scope
            .get_mut(0)
            .unwrap()
            .insert(declaration.name.clone(), "how".to_string());

        self.scope
            .last_mut()
            .unwrap()
            .insert(declaration.name.clone(), declaration.id.clone());

        self.scope_in();

        let child_instructions = self.generate_instruction(declaration.closure.body);
        self.scope
            .last_mut()
            .unwrap()
            .insert(declaration.name, declaration.id.clone());

        instructions.push(Instruction::VariableDeclaration(
            declaration.id,
            false,
            Value::Closure {
                instructions: child_instructions,
            },
        ));
        self.scope_out();

        return instructions;
    }

    fn variable_assignement(&mut self, declaration: Assignement) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let is_declaration = declaration.id.is_some();

        if is_declaration {
            let _ = &self
                .scope
                .last_mut()
                .unwrap()
                .insert(declaration.name.clone(), declaration.id.clone().unwrap());

            let value = declaration.clone().value.deref().clone();
            let value = self.eval(value);
            instructions.push(Instruction::VariableAssignement(
                declaration.id.unwrap(),
                value,
            ));

            return instructions;
        }

        return instructions;
    }

    fn resolve_variable(&self, name: String) -> Option<String> {
        for scope in self.scope.clone().iter().rev() {
            let entry = scope.get(&name);

            if entry.is_some() {
                return Some(entry.unwrap().to_string());
            }
        }
        None
    }

    /// Used when entering a nested scope, creates said scope in the scope stack.
    /// Use scope_out for the opposite
    fn scope_in(&mut self) {
        self.scope.push(HashMap::new());
    }

    /// Used when exiting a nested scope to its parent. Pops the last scope of the scope stack.
    fn scope_out(&mut self) {
        self.scope.pop();
    }

    fn eval(&self, statement: Statement) -> Value {
        match statement {
            Statement::String(value) => Value::String(value),
            Statement::Closure(_) => todo!(),
            Statement::Variable(id) => Value::CopyVar(id),
            _ => Value::String(String::new()),
        }
    }
}

pub struct Engine {
    instructions: Vec<Instruction>,
    memory: DashMap<String, Entry>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            instructions: Vec::new(),
            memory: DashMap::new(),
        }
    }

    pub fn execute(&mut self, instructions: Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction.clone() {
                Instruction::Test(_) => todo!(),
                Instruction::VariableDeclaration(id, mutable, value) => {
                    self.variable_declaration(id, mutable, value);
                }
                Instruction::VariableAssignement(id, value) => self.variable_assignement(id, value),
                Instruction::PopVariable(_) => todo!(),
                Instruction::ExecuteFunction(id, args) => self.function_call(id, args),
                Instruction::ExtCall(_) => todo!(),
            }
        }
    }

    fn variable_declaration(&mut self, id: String, mutable: bool, value: Value) {
        self.memory.insert(id, Entry { mutable, value });
    }

    fn variable_assignement(&mut self, id: String, value: Value) {
        self.memory.get_mut(&id).unwrap().value = value;
    }

    pub fn function_call(&mut self, id: Id, args: Vec<Value>) {
        let mem = &self.memory.clone();
        let func = mem.get(&id).unwrap().clone();
        let func = func.value;

        let instructions = match func {
            Value::Closure { instructions } => instructions,
            _ => {
                println!("Runtime Error: Trying to call a function at memory slot '{}', but no function was found there!", {id});
                panic!();
            }
        };

        if instructions.len() == 1 {
            match instructions.first().unwrap() {
                Instruction::ExtCall(callee) => {
                    callee(args);
                    return;
                }
                _ => {}
            }
        }

        self.execute(instructions);
    }

    pub fn shout_memory(&self) {
        println!("| Memory shout!");
        println!("| {: <40}| {: <12}| {}", "ID", "Mutable", "Value");
        for (key, entry) in self.memory.clone().into_iter() {
            println!(
                "| {: <40}| {: <12}| {}",
                key,
                if entry.mutable {
                    "Mutable"
                } else {
                    "Not Mutable"
                },
                entry.value
            );
        }
    }
}
