use advent20::input_string;
use anyhow::*;
use itertools::iproduct;

fn part1(input: &[u32]) -> Option<u32> {
    iproduct!(input, input)
        .filter(|(&x, &y)| x + y == 2020)
        .next()
        .map(|(x, y)| x * y)
}

fn part2(input: &[u32]) -> Option<u32> {
    iproduct!(input, input, input)
        .filter(|(&x, &y, &z)| x + y + z == 2020)
        .next()
        .map(|(x, y, z)| x * y * z)
}

fn main() -> Result<()> {
    let code = input_string()?;
    let nums = code
        .lines()
        .map(|l| l.parse::<u32>().context("Failed to parse input!"))
        .collect::<Result<Vec<_>>>()?;

    println!(
        "part 1: {}",
        part1(&nums).ok_or(format_err!("no result found!"))?
    );
    println!(
        "part 2: {}",
        part2(&nums).ok_or(format_err!("no result found!"))?
    );

    Ok(())
}
