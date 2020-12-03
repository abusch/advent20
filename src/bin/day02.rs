use std::str::FromStr;

use anyhow::Result;
use nom::sequence::tuple;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, anychar, char, digit1},
    combinator::map_res,
    IResult,
};

use advent20::input_string;

struct ParsedInput<'a> {
    pub min: usize,
    pub max: usize,
    pub c: char,
    pub pwd: &'a str,
}

fn verify_password(input: &ParsedInput) -> bool {
    let cnt = input.pwd.chars().filter(|&chr| chr == input.c).count();

    (cnt >= input.min) && (cnt <= input.max)
}

fn verify_password2(input: &ParsedInput) -> bool {
    let chars = input.pwd.chars().collect::<Vec<char>>();

    (chars[input.min - 1] == input.c && chars[input.max - 1] != input.c)
        || (chars[input.min - 1] != input.c && chars[input.max - 1] == input.c)
}

fn line_parser<'a>(input: &'a str) -> IResult<&'a str, ParsedInput<'a>, ()> {
    let (input, (min, _, max, _, c, _, pwd)) = tuple((
        map_res(digit1, u32::from_str),
        char('-'),
        map_res(digit1, u32::from_str),
        char(' '),
        anychar,
        tag(": "),
        alphanumeric1,
    ))(input)?;

    Ok((
        input,
        ParsedInput {
            min: min as usize,
            max: max as usize,
            c,
            pwd,
        },
    ))
}

fn parse_line(input: &str) -> Result<ParsedInput> {
    let (_, parsed_input) = line_parser(input)?;
    Ok(parsed_input)
}

fn main() -> Result<()> {
    let input = input_string()?;
    let parsed_lines = input.lines().map(parse_line).collect::<Result<Vec<_>>>()?;

    let count = parsed_lines.iter().filter(|&v| verify_password(v)).count();
    println!("part 1: {}", count);

    let count = parsed_lines.iter().filter(|&v| verify_password2(v)).count();
    println!("part 2: {}", count);

    Ok(())
}
