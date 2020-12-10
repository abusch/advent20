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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Op {
    Acc,
    Jmp,
    Nop,
}

impl std::fmt::Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::Acc => "acc",
            Self::Jmp => "jmp",
            Self::Nop => "nop",
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Inst {
    op: Op,
    offset: i32,
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:+}", self.op, self.offset)
    }
}

fn parse_inst(input: &str) -> IResult<&str, Inst> {
    map(
        tuple((
            alt((tag("acc"), tag("jmp"), tag("nop"))),
            tag(" "),
            alt((char('-'), char('+'))),
            map_res(digit1, |s: &str| s.parse::<i32>()),
        )),
        |(op, _, sign, mut offset)| {
            if sign == '-' {
                offset *= -1;
            }
            let op = match op {
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                "nop" => Op::Nop,
                _ => unreachable!(),
            };
            Inst { op, offset }
        },
    )(input)
}

fn check_prg_termination(prg: &[Inst]) -> Option<i32> {
    let mut acc = 0i32;
    let mut addr = 0i32;
    let mut visited_addr = HashSet::new();
    loop {
        if visited_addr.contains(&addr) {
            // infinite loop
            break None;
        } else if addr == prg.len() as i32 {
            break Some(acc);
        } else {
            visited_addr.insert(addr);
        }
        let i = &prg[addr as usize];
        match i.op {
            Op::Acc => {
                acc += i.offset;
                addr += 1;
            }
            Op::Jmp => addr += i.offset,
            Op::Nop => addr += 1,
        }
    }
}

fn mutate_prg<F>(prg: &[Inst], addr: i32, mutation: F) -> Vec<Inst>
where
    F: Fn(&mut Inst)
{
    let mut new_prg = prg.to_vec();
    if let Some(i) = new_prg.get_mut(addr as usize) {
        mutation(i);
    }

    new_prg
}

fn parse_program(input: &str) -> Result<Vec<Inst>> {
    input
        .lines()
        .map(|line| {
            parse_inst(line)
                .finish()
                .map(|v| v.1)
                .map_err(|e| format_err!("Failed to parse instruction: {}", e.to_string()))
        })
        .collect::<Result<Vec<_>>>()
}

fn part2(prg: &[Inst]) -> Option<i32> {
    let nop_addrs: Vec<_> = prg
        .iter()
        .enumerate()
        .filter_map(|(addr, i)| if i.op == Op::Nop { Some(addr) } else { None })
        .collect();
    let jmp_addrs: Vec<_> = prg
        .iter()
        .enumerate()
        .filter_map(|(addr, i)| if i.op == Op::Jmp { Some(addr) } else { None })
        .collect();

    nop_addrs
        .iter()
        .find_map(|addr| check_prg_termination(&mutate_prg(&prg, *addr as i32, |mut i| i.op = Op::Jmp)))
        .or_else(|| {
            jmp_addrs
                .iter()
                .find_map(|addr| check_prg_termination(&mutate_prg(&prg, *addr as i32, |mut i| i.op = Op::Nop)))
        })
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let prg = parse_program(&input)?;
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
        let i = &prg[addr as usize];
        match i.op {
            Op::Acc => {
                acc += i.offset;
                addr += 1;
            }
            Op::Jmp => addr += i.offset,
            Op::Nop => addr += 1,
        }
    }

    println!("part 1: {}", acc);

    let res = part2(&prg).unwrap();

    println!("part 2: {}", res);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let prg = parse_program(input).unwrap();
        println!("prg:");
        prg.iter().for_each(|i| println!("{}", i));

        println!("prg2:");
        let prg2 = mutate_prg(&prg, 2, |mut i| i.op = Op::Nop);
        prg2.iter().for_each(|i| println!("{}", i));

        assert_eq!(Some(8), part2(&prg));
    }
}
