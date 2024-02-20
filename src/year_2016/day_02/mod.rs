use algorithms::higher_order_functions::Reductions;

use crate::shared::{
    grid::Grid,
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

pub const TITLE: &str = "Bathroom Security";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> String {
    let digits = Grid::parse("123\n456\n789");
    let mut position = Point::ORIGIN;
    let mut result = String::new();

    for line in input.lines() {
        for b in line.bytes() {
            let next = position + Point::from(b);
            if next.x.abs() <= 1 && next.y.abs() <= 1 {
                position = next;
            }
        }
        result.push(digits[position + Point::new(1, 1)] as char);
    }

    result
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> String {
    let digits = Grid::parse("##1##\n#234#\n56789\n#ABC#\n##D##");
    let mut position = Point::new(-2, 0);
    let mut result = String::new();

    for line in input.lines() {
        for b in line.bytes() {
            let next = position + Point::from(b);
            if next.manhattan(Point::ORIGIN) <= 2 {
                position = next;
            }
        }
        result.push(digits[position + Point::new(2, 2)] as char);
    }

    result
}

fn parse(input: &str) -> Vec<String> {
    input.split(',').map(str::trim).map(str::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), "92435");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "C1A88");
    }
}
