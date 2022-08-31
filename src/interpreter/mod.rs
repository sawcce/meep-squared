use colored::*;
use std::{
    cell::Ref,
    collections::HashMap,
    fmt, io,
    ops::Deref,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use dashmap::DashMap;
use nom::{error::convert_error, Finish};

use crate::parsers::{
    conditional::Conditional,
    function_declaration::FunctionDeclaration,
    number::Number,
    program::program,
    statements::{Statement, Statements},
    variable::Assignement,
};

#[derive(Debug, Clone)]

pub enum Value {
    String(String),
    Int32(i32),
    Float32(f32),
    Closure {
        argument_count: u8,
        instructions: Vec<Instruction>,
    },
    CopyVar(Id, Box<Option<Value>>),
    Boolean(bool),
    /// Field used for instructions that need to be evaluated (operations, function calls) but that
    /// are still considered values in their compiled context
    LazyEval(Box<Instruction>),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::String(val1) => match other {
                Value::String(val2) => val1 == val2,
                _ => false,
            },
            Value::Int32(val1) => match other {
                Value::Int32(val2) => val1 == val2,
                _ => false,
            },

            Value::Float32(val1) => match other {
                Value::Float32(val2) => val1 == val2,
                _ => false,
            },

            Value::Boolean(val1) => match other {
                Value::Boolean(val2) => val1 == val2,
                _ => false,
            },
            _ => {
                println!(
                    "Runtime Error: Did not expect this kind of value in equals, must be an error"
                );
                panic!()
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Boolean(value) => write!(f, "{value}"),
            Value::String(content) => write!(f, "{content}"),
            Value::Float32(num) => write!(f, "{num}"),
            Value::Int32(num) => write!(f, "{num}"),
            Value::Closure { instructions, .. } => {
                write!(
                    f,
                    "Closure ({})",
                    match instructions.first().unwrap() {
                        Instruction::ExtCall(x) => format!("ExtCall {:?}", x),
                        _ => format!("{} instructions", instructions.len()),
                    }
                )
            }
            //Value::CopyVar(id, val) => write!(f, "{id}"),
            _ => Ok(()),
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
    Return(Value),
    Conditional(ConditionalInstruction),
}

#[derive(Debug, Clone)]
struct ConditionalInstruction {
    main: (Value, Vec<Instruction>),
    alternates: Option<Vec<(Value, Instruction)>>,
    fallback: Option<Vec<Instruction>>,
}

fn say_time(name: &str, instant: Instant) {
    let fmt_str = format!(
        "✅ Task finished > {} done in: {}s",
        name,
        instant.elapsed().as_secs_f32().to_string()
    );
    println!("{}", fmt_str.green());
}

/// An enum used for variable scoping, tells the compiler wether this is a reference
/// to an argument or a memory slot (id)
#[derive(Debug, Clone)]
enum ReferenceType {
    Id(Id),
    /// Says that it's an argument, stores the Id of the function it belongs to, and the index of
    /// the argument
    Argument(Id, u8),
}

#[derive(Debug)]
/// A struct that is responsible for parsing a program and generating a list of instructions that
/// will be fed to the engine
pub struct Compiler<'a> {
    scope: &'a mut Vec<HashMap<String, (Id, bool)>>,
    pub instructions: Vec<Instruction>,
}

const STD_PREFIX: &str = "msq_std::";
const PRINT_ID: &str = "msq_std::print()";
const ADD_ID: &str = "msq_std::add()";
const EQUALS_ID: &str = "msq_std::equals()";
const SMALLER_ID: &str = "msq_std::smaller()";
const DATE_ID: &str = "msq_std::date()";
const INPUT_ID: &str = "msq_std::input()";

fn print(args: Vec<Value>) -> Option<Value> {
    for arg in args {
        print!("{}", arg);
    }
    println!("");
    None
}

fn date(args: Vec<Value>) -> Option<Value> {
    Some(Value::Int32(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i32,
    ))
}

fn smaller(args: Vec<Value>) -> Option<Value> {
    if args.len() != 2 {
        println!("Runtime error: Expected two arguments for function 'smaller'");
        panic!()
    }

    match args.first().unwrap() {
        Value::Int32(value1) => match args.last().unwrap() {
            Value::Float32(value2) => return Some(Value::Boolean(value1 < &(*value2 as i32))),
            Value::Int32(value2) => return Some(Value::Boolean(value1 < value2)),
            _ => {
                panic!()
            }
        },
        Value::Float32(value1) => match args.last().unwrap() {
            Value::Float32(value2) => return Some(Value::Boolean(value1 < value2)),
            Value::Int32(value2) => return Some(Value::Boolean(value1 < &(*value2 as f32))),
            _ => {
                panic!()
            }
        },
        _ => {
            panic!()
        }
    }
}

fn input(args: Vec<Value>) -> Option<Value> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Runtime error: Unable to read user input");

    Some(Value::String(input))
}

fn add(args: Vec<Value>) -> Option<Value> {
    let value = args.first().unwrap();

    match value {
        Value::Int32(..) => {
            let mut sum = 0i32;

            for num in args {
                match num {
                    Value::Int32(value) => sum += value,
                    _ => panic!("Can only add an Int with an Int"),
                }
            }
            return Some(Value::Int32(sum));
        }
        Value::Float32(..) => {
            let mut sum = 0f32;

            for num in args {
                match num {
                    Value::Float32(value) => sum += value,
                    _ => panic!("Can only add a Float with a Float"),
                }
            }
            return Some(Value::Float32(sum));
        }
        _ => panic!(),
    };
}

fn equals(args: Vec<Value>) -> Option<Value> {
    if args.len() != 2 {
        println!("Runtime error: Only expected 2 arguments for equals function");
        panic!()
    }

    let is_equals = args.first().unwrap() == args.last().unwrap();

    Some(Value::Boolean(is_equals))
}

impl<'a> Compiler<'a> {
    pub fn new(scope: &'a mut Vec<HashMap<String, (Id, bool)>>) -> Self {
        Self {
            scope,
            instructions: Vec::new(),
        }
    }

    fn add_default_func(&mut self, name: &str, id: &str, instructions: Vec<Instruction>) {
        self.scope
            .first_mut()
            .unwrap()
            .insert(name.to_string(), (id.to_string(), false));

        self.instructions.push(Instruction::VariableDeclaration(
            id.to_string(),
            false,
            Value::Closure {
                instructions,
                argument_count: 0,
            },
        ));
    }

    fn prepare_defaults(&mut self) {
        self.scope_in();

        self.add_default_func("print", PRINT_ID, vec![Instruction::ExtCall(print)]);
        self.add_default_func("add", ADD_ID, vec![Instruction::ExtCall(add)]);
        self.add_default_func("equals", EQUALS_ID, vec![Instruction::ExtCall(equals)]);
        self.add_default_func("smaller", SMALLER_ID, vec![Instruction::ExtCall(smaller)]);
        self.add_default_func("date", DATE_ID, vec![Instruction::ExtCall(date)]);
        self.add_default_func("input", INPUT_ID, vec![Instruction::ExtCall(input)]);

        self.scope_in();
    }

    pub fn compile(&mut self, code: &str) {
        let parse_start = Instant::now();
        let result = program(code);
        println!("{:#?}", result);
        say_time("Parsing", parse_start);

        if result.clone().is_err() {
            let x = result.finish().err().unwrap();
            let y = convert_error(code, x);

            println!("Error: {y}");
            return;
        }

        let result_program = result.unwrap();

        let compile_start = Instant::now();
        self.prepare_defaults();
        let instructions = &mut self.generate_instruction(result_program.1.statements);
        self.instructions.append(instructions);
        self.instructions.push(Instruction::ExecuteFunction(
            result_program.1.main_id,
            Vec::new(),
        ));

        say_time("Compiling", compile_start);
        println!();
        // println!("Instructions: {:?}", self.instructions);
    }

    fn generate_instruction(&mut self, statements: Statements) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        for statement in statements.body.iter() {
            match statement {
                Statement::Return(content) => {
                    instructions.push(Instruction::Return(self.eval(*content.clone())))
                }
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
                        self.resolve_variable(fc.clone().name).unwrap().0,
                        fc.arguments
                            .iter()
                            .map(|arg| self.eval(arg.clone()))
                            .collect(),
                    ));
                }
                Statement::Conditional(statement) => {
                    instructions.push(self.conditional(statement.clone()))
                }
                _ => {}
            }
        }

        return instructions;
    }

    fn conditional(&mut self, conditional: Conditional) -> Instruction {
        let first_condition = self.eval(conditional.main_condition.0);

        self.scope_in();

        let first_body = self.generate_instruction(conditional.main_condition.1);

        self.scope_out();

        let alternates: Vec<(Value, Vec<Instruction>)> = Vec::new();

        for alternate in conditional.alternates {
            let condition = self.eval(alternate.0);
            self.scope_in();
            let body = self.generate_instruction(alternate.1);
            self.scope_out();
        }

        let mut fallback: Option<Vec<Instruction>> = None;

        if conditional.fallback.is_some() {
            let fb = conditional.fallback.unwrap();
            self.scope_in();
            fallback = Some(self.generate_instruction(fb));
            self.scope_out();
        }

        Instruction::Conditional(ConditionalInstruction {
            main: (first_condition, first_body),
            alternates: Some(Vec::new()),
            fallback,
        })
    }

    fn function_declaration(&mut self, declaration: FunctionDeclaration) -> Vec<Instruction> {
        let mut instructions: Vec<Instruction> = Vec::new();
        /*self.scope
        .get_mut(0)
        .unwrap()
        .insert(declaration.name.clone(), "how".to_string()); */

        self.scope
            .last_mut()
            .unwrap()
            .insert(declaration.name.clone(), (declaration.id.clone(), false));

        self.scope_in();

        for (index, argument) in declaration.closure.arguments.iter().enumerate() {
            self.scope.last_mut().unwrap().insert(
                argument.clone(),
                ((format!("{}-{}", declaration.id.clone(), index)), false),
            );
        }

        let child_instructions = self.generate_instruction(declaration.closure.body);
        /*self.scope
        .last_mut()
        .unwrap()
        .insert(declaration.name, declaration.id.clone()); */
        println!("{:?}", declaration.closure.arguments);

        instructions.push(Instruction::VariableDeclaration(
            declaration.id,
            false,
            Value::Closure {
                instructions: child_instructions,
                argument_count: declaration.closure.arguments.len() as u8,
            },
        ));
        self.scope_out();

        return instructions;
    }

    fn variable_assignement(&mut self, declaration: Assignement) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let is_declaration = declaration.id.is_some();

        if is_declaration {
            let _ = &self.scope.last_mut().unwrap().insert(
                declaration.name.clone(),
                (declaration.id.clone().unwrap(), declaration.mutable.clone()),
            );

            let value = declaration.value.deref().clone();
            let value = self.eval(value);
            instructions.push(Instruction::VariableDeclaration(
                declaration.id.unwrap(),
                declaration.mutable,
                value,
            ));

            return instructions;
        }

        let (id, mutable) = self.resolve_variable(declaration.name.clone()).unwrap();

        if !mutable {
            println!(
                "Trying to assign to constant variable: {}",
                declaration.name
            );
            panic!();
        }

        let value = declaration.value.deref().clone();
        let value = self.eval(value);

        instructions.push(Instruction::VariableAssignement(id, value));

        return instructions;
    }

    fn resolve_variable(&self, name: String) -> Option<(Id, bool)> {
        for scope in self.scope.clone().iter().rev() {
            let entry = scope.get(&name);

            if entry.is_some() {
                return Some(entry.unwrap().clone());
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
            Statement::Number(number) => match number {
                Number::Int(value) => Value::Int32(value),
                Number::Float(value) => Value::Float32(value),
            },
            Statement::Closure(_) => todo!(),
            Statement::Variable(name) => {
                println!("Resolving: {name}");
                println!("{:#?}", self.scope);
                Value::CopyVar(self.resolve_variable(name).unwrap().0, Box::new(None))
            }
            Statement::Boolean(value) => Value::Boolean(value),
            Statement::FunctionCall(call) => {
                Value::LazyEval(Box::new(Instruction::ExecuteFunction(
                    self.resolve_variable(call.name).unwrap().0,
                    call.arguments
                        .iter()
                        .map(|arg| self.eval(arg.clone()))
                        .collect(),
                )))
            }
            _ => Value::String(String::new()),
        }
    }
}

pub struct Engine {
    context: HashMap<(Id, u8), Value>,
    instructions: Vec<Instruction>,
    memory: DashMap<String, Entry>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            context: HashMap::new(),
            instructions: Vec::new(),
            memory: DashMap::new(),
        }
    }

    pub fn execute_program(&mut self, instructions: Vec<Instruction>) {
        let execute_start = Instant::now();
        self.execute(instructions);
        say_time("Executing", execute_start);
        println!();
    }

    fn execute(&mut self, instructions: Vec<Instruction>) -> Option<Value> {
        for instruction in instructions.iter() {
            match instruction.clone() {
                Instruction::Return(value) => {
                    return Some(self.eval(value));
                }
                Instruction::Test(_) => todo!(),
                Instruction::VariableDeclaration(id, mutable, value) => {
                    self.variable_declaration(id, mutable, value);
                }
                Instruction::VariableAssignement(id, value) => self.variable_assignement(id, value),
                Instruction::PopVariable(_) => todo!(),
                Instruction::ExecuteFunction(id, args) => match self.function_call(id, args) {
                    Some(value) => return Some(value),
                    None => {}
                },
                Instruction::Conditional(instruction) => match self.conditional(instruction) {
                    Some(value) => return Some(value),
                    None => {}
                },
                Instruction::ExtCall(_) => todo!(),
            }
        }

        None
    }

    fn variable_declaration(&mut self, id: String, mutable: bool, value: Value) {
        let value = self.eval(value);

        self.memory.insert(id, Entry { mutable, value });
    }

    fn conditional(&mut self, instruction: ConditionalInstruction) -> Option<Value> {
        let val = self.eval(instruction.main.0);

        match val {
            Value::Boolean(is_true) => {
                if is_true {
                    let value = self.execute(instruction.main.1);
                    if value.is_some() {
                        return value;
                    }
                }
            }
            _ => {}
        }

        None
    }

    fn variable_assignement(&mut self, id: String, value: Value) {
        self.memory.get_mut(&id).unwrap().value = self.eval(value);
    }

    pub fn function_call(&mut self, id: Id, arguments: Vec<Value>) -> Option<Value> {
        let mem = &self.memory.clone();
        let func = mem.get(&id).unwrap().clone();
        let func = func.value;

        let (argument_count, instructions) = match func {
            Value::Closure {
                argument_count,
                instructions,
            } => (argument_count, instructions),
            _ => {
                println!("Runtime Error: Trying to call a function at memory slot '{}', but no function was found there!", {id});
                panic!();
            }
        };

        let arguments = arguments.iter().map(|arg| self.eval(arg.clone())).collect();

        if instructions.len() == 1 {
            match instructions.first().unwrap() {
                Instruction::ExtCall(callee) => {
                    return callee(arguments);
                }
                _ => {}
            }
        }

        for (index, argument) in arguments.iter().enumerate() {
            self.variable_declaration(format!("{}-{}", id, index), false, argument.clone());
        }

        let return_value = self.execute(instructions);

        for (index, argument) in arguments.iter().enumerate() {
            self.pop_value(format!("{}-{}", id, index));
        }

        return_value
    }

    fn pop_value(&mut self, id: Id) {
        //self.memory.remove(&id);
    }

    // Evaluates non-evaluated Values (function calls, var copies...)
    fn eval(&mut self, value: Value) -> Value {
        match value {
            // Value::Closure { instructions } => todo!(),
            Value::CopyVar(id, _) => {
                self.shout_memory();
                self.memory.get(&id).unwrap().clone().value.clone()
            }

            Value::LazyEval(contents) => self.execute(vec![(*contents.clone())]).unwrap(),
            value => value,
        }
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
