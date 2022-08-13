mod parsers;
use std::fs::read_to_string;

use nom::{
    character::{self},
    combinator::{self},
    sequence::{self},
    IResult,
};

use crate::parsers::{
    closure::closure, function_declaration::function_declaration, statements::statements,
};

pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

fn main() -> std::result::Result<(), BoxError> {
    let code = read_to_string("./programs/hello_world.msq")?;
    let code = code.as_str();

    let x = statements(code).unwrap();
    println!("{:?}", x.0);
    println!("{:?}", x.1);
    Ok(())
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
