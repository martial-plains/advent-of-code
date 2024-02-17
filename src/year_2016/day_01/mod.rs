use algorithms::higher_order_functions::Reductions;

use crate::shared::{
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

pub const TITLE: &str = "No Time for a Taxicab";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> i32 {
    let words = parse(input);
    let mut position = Point::ORIGIN;
    let mut direction = Point::UP;

    for step in words {
        direction = if step.starts_with('L') {
            direction.counter_clockwise()
        } else {
            direction.clockwise()
        };

        position += direction * step[1..].parse::<i32>().unwrap();
    }

    position.manhattan(Point::ORIGIN)
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> i32 {
    let words = parse(input);
    let mut position = Point::ORIGIN;
    let mut direction = Point::UP;
    let mut visited = FastSet::with_capacity(1000);

    for step in words {
        direction = if step.starts_with('L') {
            direction.counter_clockwise()
        } else {
            direction.clockwise()
        };

        let m = step[1..].parse::<i32>().unwrap();

        for _ in 0..m {
            position += direction;
            if !visited.insert(position) {
                return position.manhattan(Point::ORIGIN);
            }
        }
    }

    unreachable!()
}

fn parse(input: &str) -> Vec<String> {
    input.split(',').map(str::trim).map(str::to_owned).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 231);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part1(INPUT), 147);
    }
}
