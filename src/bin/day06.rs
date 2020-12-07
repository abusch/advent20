use std::collections::HashSet;

use anyhow::Result;

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let mut groups = Vec::new();
    let mut current_group = None;

    for line in input.lines() {
        if line.is_empty() {
            groups.push(current_group.take().unwrap());
            continue;
        }

        let group = current_group.get_or_insert(HashSet::new());
        for c in line.chars() {
            group.insert(c);
        }
    }

    if let Some(group) = current_group.take() {
        groups.push(group);
    }


    let sum: usize = groups.iter()
        .map(HashSet::len)
        .sum();

    println!("part 1: {}", sum);

    Ok(())
}
