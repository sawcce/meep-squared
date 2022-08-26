mod interpreter;
mod parsers;

use interpreter::Compiler;

use parsers::program::program;
use std::fs::read_to_string;

use nom::{
    character::{self},
    combinator::{self},
    error::convert_error,
    sequence::{self},
    Finish, IResult,
};

pub type BoxError<'a> = Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + 'a, // + std::marker::Send // needed for threads
              // + std::marker::Sync, // needed for threads
>;

fn main() {
    /*    let code = String::from("_ -> let a = b end");
    let x = closure(&code).unwrap();
    println!("{x:?}"); */

    let code = read_to_string("./programs/hello_world.msq").unwrap();
    println!("{code}");

    let code = code.clone();
    let code = &code;

    let mut compiler = Compiler::new();
    compiler.compile(code);
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
