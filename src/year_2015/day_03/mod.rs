use std::{cmp::min, fmt::Display, str::FromStr};

use crate::shared::{
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

pub const TITLE: &str = "Perfectly Spherical Houses in a Vacuum";

pub fn part1(input: &str) -> usize {
    let input = input
        .chars()
        .map(Direction::from)
        .map(Point::from)
        .collect::<Vec<Point>>();
    deliver(&input, |_| true)
}

pub fn part2(input: &str) -> usize {
    let input = input
        .chars()
        .map(Direction::from)
        .map(Point::from)
        .collect::<Vec<Point>>();
    deliver(&input, |i| i % 2 == 0)
}

fn deliver(input: &[Point], predicate: fn(usize) -> bool) -> usize {
    let mut santa = Point::ORIGIN;
    let mut robot = Point::ORIGIN;
    let mut set = FastSet::with_capacity(10_000);
    set.insert(Point::ORIGIN);

    for (index, point) in input.iter().enumerate() {
        if predicate(index) {
            santa += *point;
            set.insert(santa);
        } else {
            robot += *point;
            set.insert(robot);
        }
    }

    set.len()
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::North,
            '>' => Direction::East,
            'v' => Direction::South,
            '<' => Direction::West,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Self::UP,
            Direction::East => Self::RIGHT,
            Direction::South => Self::DOWN,
            Direction::West => Self::LEFT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_exmpale1() {
        let result = part1(">");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1("^>v<");
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1("^v^v^v^v^v");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_puzzle() {
        let result = part1(INPUT);
        assert_eq!(result, 2565);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2("^v");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2("^>v<");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_example3() {
        let result = part2("^v^v^v^v^v");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2_puzzle() {
        let result = part2(INPUT);
        assert_eq!(result, 2639);
    }
}
