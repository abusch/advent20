use anyhow::*;
use itertools::Itertools;

fn check(previous: &[u64], n: u64) -> Option<u64> {
    let found = previous
        .iter()
        .cartesian_product(previous.iter())
        .find(|(&a, &b)| (a != b) && (a + b == n));

    if found.is_none() {
        Some(n)
    } else {
        None
    }
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let nums = input
        .lines()
        .map(|s| s.parse::<u64>().map_err(|e| e.into()))
        .collect::<Result<Vec<_>>>()?;

    let weakness = nums
        .windows(26)
        .find_map(|window| check(&window[0..25], *window.last().unwrap()))
        .ok_or(format_err!("No solution found!"))?;

    println!("part 1: {}", weakness);

    let mut encryption_weakness = 0;
    for window_size in 2.. {
        let found = nums.windows(window_size)
            .find_map(|w| {
                if w.iter().sum::<u64>() == weakness {
                    let min = w.iter().min().unwrap();
                    let max = w.iter().max().unwrap();
                    Some(min + max)
                } else {
                    None
                }
            });
        if let Some(res) = found {
            encryption_weakness = res;
            break;
        }
    }

    println!("part 2: {}", encryption_weakness);

    Ok(())
}
