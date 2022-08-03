mod parsers;
use parsers::variable::variable;

use nom::{
    branch, bytes,
    character::{self, complete::multispace0},
    combinator::{self, value},
    error::ParseError,
    sequence::{self, delimited},
    IResult,
};

pub type BoxError = std::boxed::Box<
    dyn std::error::Error // must implement Error to satisfy ?
        + std::marker::Send // needed for threads
        + std::marker::Sync, // needed for threads
>;

struct Program {
    name: String,
}

fn main() -> std::result::Result<(), BoxError> {
    let string = r#"hello = "wo\"rld""#;
    println!("{string}");
    let res = variable(string);
    println!("{:?}", res);

    Ok(())
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

fn not_whitespace(i: &str) -> IResult<&str, &str> {
    nom::bytes::complete::is_not(" \t")(i)
}

/*"
import stdio.print, println

main -> println("hello world")

print ...args ->
    for each arg in args -> print(arg)
    print("\n")
end

"*/
