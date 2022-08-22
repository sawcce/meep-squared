//mod interpreter;
mod parsers;

use std::{fs::read_to_string, sync::Arc};

use nom::{
    character::{self},
    combinator::{self},
    error::{convert_error, VerboseError},
    sequence::{self},
    IResult,
};
use parsers::program::program;

pub type BoxError<'a> = Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + 'a, // + std::marker::Send // needed for threads
              // + std::marker::Sync, // needed for threads
>;

fn main() {
    let code = read_to_string("./programs/hello_world.msq").unwrap();
    println!("{code}");

    let prog = program::<VerboseError<&str>>(&(code.clone()));

    match prog {
        Ok(program_) => {
            println!("{program_:?}")
        }
        Err(err) => {
            let a = convert_error(code.clone(), err);
            println!("{a}");
        }
    }
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
