use anyhow::Result;
use bitvec::prelude::*;

fn char_index(c: char) -> usize {
    let ascii_code = c as u8;
    let a = 'a' as u8;

    (ascii_code - a) as usize
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let mut groups = Vec::new();
    let mut current_group = None;

    for line in input.lines() {
        if line.is_empty() {
            groups.push(current_group.take().unwrap());
            continue;
        }

        let group = current_group.get_or_insert(Vec::new());
        let mut answers = bitarr![0; 26];
        for c in line.chars() {
            answers.set(char_index(c), true);
        }

        group.push(answers);
    }

    if let Some(group) = current_group.take() {
        groups.push(group);
    }

    let sum: usize = groups
        .iter()
        .map(|answers| {
            answers
                .iter()
                .fold(bitvec![0; 26], |mut acc, bv| {
                    acc |= bv.to_bitvec();
                    acc
                })
                .count_ones()
        })
        .sum();

    println!("part 1: {}", sum);

    let sum: usize = groups
        .iter()
        .map(|answers| {
            answers
                .iter()
                .fold(bitvec![1; 26], |mut acc, bv| {
                    acc &= bv.to_bitvec();
                    acc
                })
                .count_ones()
        })
        .sum();

    println!("part 2: {}", sum);

    Ok(())
}
