use advent20::input_string;
use itertools::iproduct;

fn part1(input: &[u32]) -> u32 {
    let (x, y) = iproduct!(input, input)
        .filter(|(&x, &y)| x + y == 2020)
        .next()
        .expect("did not find any result");

    x * y
}

fn part2(input: &[u32]) -> u32 {
    let (x, y, z) = iproduct!(input, input, input)
        .filter(|(&x, &y, &z)| x + y + z == 2020)
        .next()
        .expect("did not find any result");

    x * y * z
}

fn main() {
    let code = input_string();
    let nums = code
        .lines()
        .map(|l| l.parse::<u32>().expect("not a valid number"))
        .collect::<Vec<_>>();

    println!("part 1: {}", part1(&nums));
    println!("part 2: {}", part2(&nums));
}
