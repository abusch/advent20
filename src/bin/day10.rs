use cached::proc_macro::cached;
use cached::UnboundCache;

use anyhow::*;

fn parse_input(input: &str) -> Result<Vec<u64>> {
    input
        .lines()
        .map(|s| s.parse::<u64>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()
}

#[cached(
    type = "UnboundCache<String, u64>",
    create= "{ UnboundCache::new() }",
    convert=r#"{format!("{}_{}", first, adapters.len())}"#
)]
fn num_arrangements(first: u64, adapters: &[u64]) -> u64 {
    // dbg!(first);
    // eprintln!("adapters: {:#?}", adapters);
    if adapters.is_empty() {
        return 1;
    }

    let candidates = adapters
        .iter()
        .enumerate()
        .take_while(|(_i, v)| *v - first <= 3)
        .collect::<Vec<_>>();
    let mut num = 0;
    for (i, c) in &candidates {
        num += num_arrangements(**c, &adapters[(*i as usize + 1)..]);
    }
    num
}

fn jolts(input: &str) -> Result<Vec<u64>> {
    let mut adapters = parse_input(&input)?;
    adapters.sort();
    let max = adapters.iter().max().expect("no adapters!").clone();
    adapters.insert(0, 0);
    adapters.push(max + 3);

    Ok(adapters)
}


fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let adapters = jolts(&input)?;

    let differences = adapters.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    let diff_1 = differences.iter().filter(|&&v| v == 1).count();
    let diff_3 = differences.iter().filter(|&&v| v == 3).count();

    println!("part 1: {}", diff_1 * diff_3);

    let num = num_arrangements(0, &adapters[1..]);
    println!("part 2: {}", num);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
        let adapters = jolts(input).unwrap();
        assert_eq!(19208, num_arrangements(0, &adapters[1..]));
    }
}
