use std::cmp::min;

pub const TITLE: &str = "I Was Told There Would Be No Math";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
/// * Panics if input is empty
/// * Panics if string isn't ascii safe
#[must_use]
pub fn part1(input: &str) -> isize {
    let mapping = |v: Vec<isize>| {
        2 * (v[0] * v[1] + v[0] * v[2] + v[1] * v[2])
            + min(v[0] * v[1], min(v[0] * v[2], v[1] * v[2]))
    };
    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|s: &str| s.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(mapping)
        .sum()
}

/// # Panics
/// * Panics if input is empty
/// * Panics if string isn't ascii safe
#[must_use]
pub fn part2(input: &str) -> isize {
    let mapping =
        |v: Vec<isize>| 2 * min(v[0] + v[1], min(v[0] + v[2], v[1] + v[2])) + v[0] * v[1] * v[2];

    input
        .lines()
        .map(|line| {
            line.split('x')
                .map(|s: &str| s.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(mapping)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let result = part1("2x3x4");
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1("1x1x10");
        assert_eq!(result, 43);
    }

    #[test]
    fn test_part1_puzzle() {
        let result = part1(INPUT);
        assert_eq!(result, 1_598_415);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2("2x3x4");
        assert_eq!(result, 34);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2("1x1x10");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2_puzzle() {
        let result = part2(INPUT);
        assert_eq!(result, 3_812_909);
    }
}
