use std::collections::HashSet;

use anyhow::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::tuple,
    Finish, IResult,
};

#[derive(Debug)]
pub enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop,
}

fn parse_inst(input: &str) -> IResult<&str, Inst> {
    map(
        tuple((
            alt((tag("acc"), tag("jmp"), tag("nop"))),
            tag(" "),
            alt((char('-'), char('+'))),
            map_res(digit1, |s: &str| s.parse::<i32>()),
        )),
        |(op, _, sign, mut num)| {
            if sign == '-' {
                num *= -1;
            }
            match op {
                "acc" => Inst::Acc(num),
                "jmp" => Inst::Jmp(num),
                "nop" => Inst::Nop,
                _ => unreachable!(),
            }
        },
    )(input)
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let instructions = input
        .lines()
        .map(|line| {
            parse_inst(line)
                .finish()
                .map(|v| v.1)
                .map_err(|e| format_err!("Failed to parse instruction: {}", e.to_string()))
        })
        .collect::<Result<Vec<_>>>()?;

    // dbg!(instructions);

    let mut acc = 0i32;
    let mut addr = 0i32;
    let mut visited_addr = HashSet::new();
    loop {
        if visited_addr.contains(&addr) {
            break;
        } else {
            visited_addr.insert(addr);
        }
        let i = &instructions[addr as usize];
        match i {
            Inst::Acc(n) => {
                acc += n;
                addr += 1;
            }
            Inst::Jmp(n) => addr += n,
            Inst::Nop => addr += 1,
        }
    }

    println!("part 1: {}", acc);

    Ok(())
}
