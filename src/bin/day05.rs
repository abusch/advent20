use std::collections::HashSet;

use anyhow::format_err;
use anyhow::Result;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Seat(u8, u8);

impl Seat {
    pub fn from_code(code: &str) -> Result<Seat> {
        let (row_code, column_code) = code.split_at(7);
        let row = Self::binary_search(row_code, 'F', 'B')?;
        let column = Self::binary_search(column_code, 'L', 'R')?;

        Ok(Seat(row, column))
    }

    fn binary_search(input: &str, lowc: char, highc: char) -> Result<u8> {
        let mut low = 0;
        let mut high = (1 << (input.len() as u8)) - 1;

        for c in input.chars() {
            let delta = (high - low) / 2 + 1;
            if c == lowc {
                high = high - delta;
            } else if c == highc {
                low = low + delta;
            } else {
                return Err(format_err!(
                    "invalid input: {}. Expecting {} or {}",
                    c,
                    lowc,
                    highc
                ));
            }

            if low == high {
                break;
            }
        }

        Ok(low)
    }

    pub fn id(&self) -> u16 {
        self.0 as u16 * 8 + self.1 as u16
    }
}

fn main() -> Result<()> {
    let input = advent20::input_string()?;

    let seats = input
        .lines()
        .map(|line| Seat::from_code(line))
        .collect::<Result<HashSet<Seat>>>()?;

    let max_id = seats.iter().map(Seat::id).max().unwrap();
    let min_id = seats.iter().map(Seat::id).min().unwrap();

    println!("part 1: {}", max_id);

    let mut all_seats = HashSet::with_capacity(128);
    for row in 0..128 {
        for column in 0..8 {
            let seat = Seat(row, column);
            if seat.id() >= min_id && seat.id() <= max_id {
                all_seats.insert(seat);
            }
        }
    }

    let seat_id = all_seats
        .difference(&seats)
        .map(Seat::id)
        .next()
        .ok_or(format_err!("No remaining seat!"))?;

    println!("part 2: {}", seat_id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let seat = Seat(44, 5);
        assert_eq!(357, seat.id());
    }

    #[test]
    fn test_seat_from_code() {
        let seat = Seat::from_code("FBFBBFFRLR").unwrap();

        assert_eq!(Seat(44, 5), seat);
    }
}
