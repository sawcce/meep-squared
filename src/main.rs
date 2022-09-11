mod interpreter;
mod parsers;

use interpreter::Compiler;
use interpreter::Engine;

use std::collections::HashMap;
use std::fs::read_to_string;

use nom::{character, combinator, sequence, IResult};

use crate::parsers::conditional::conditional_statement;

pub type BoxError<'a> = Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + 'a, // + std::marker::Send // needed for threads
              // + std::marker::Sync, // needed for threads
>;

fn main() {
    /*    let code = String::from("_ -> let a = b end");
    let x = closure(&code).unwrap();
    println!("{x:?}"); */
    let code = "if a -> end";
    conditional_statement(code).unwrap();

    let code = read_to_string("./programs/fib.msq").unwrap();

    let code = code.clone();
    let code = &code;

    println!("{code}");

    let scope = &mut vec![HashMap::new()];
    let mut compiler = Compiler::new(scope);
    compiler.compile(code);
    // println!("{:#?}", compiler.instructions.clone());

    let mut engine = Engine::new();
    engine.execute_program(compiler.instructions);
    engine.shout_memory();
}

#[cfg(test)]
pub mod test {
    use crate::parsers::args_list::args_list;

    #[test]
    pub fn arguments() {
        let string = r#"a, b, c"#;
        let res = args_list(string);
        println!("{res:?}");

        let string = r#"a"#;
        let res = args_list(string);
        println!("{res:?}");

        let string = r#"_"#;
        let res = args_list(string);
        println!("{res:?}");
    }
}

pub fn number(input: &str) -> IResult<&str, &str> {
    combinator::map(
        sequence::tuple((
            combinator::opt(character::complete::char('-')),
            nom::multi::many1(character::complete::digit1),
            combinator::opt(sequence::preceded(
                character::complete::char('.'),
                nom::multi::many1(character::complete::digit1),
            )),
        )),
        |_| "0",
    )(input)
}

/*"
import stdio.print, println

main -> println("hello world")

print ...args ->
    for each arg in args -> print(arg)
    print("\n")
end

"*/
