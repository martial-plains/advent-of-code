use std::collections::HashSet;

use itertools::Itertools;

pub const TITLE: &str = "Doesn't He Have Intern-Elves For This?";

pub const INPUT: &str = include_str!("input.txt");

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(three_vowels)
        .filter(consecutive_characters)
        .filter(|s| !bad_strings(s))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(repeated_pair)
        .filter(repeated_either_side_of_one_character)
        .count()
}

fn three_vowels(string: &&str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    string.matches(|c| vowels.contains(&c)).count() >= 3
}

fn consecutive_characters(string: &&str) -> bool {
    string.chars().tuple_windows().any(|(a, b)| a == b)
}

fn bad_strings(string: &&str) -> bool {
    ["ab", "cd", "pq", "xy"]
        .into_iter()
        .map(|c| string.contains(c))
        .any(|second| second)
}

fn repeated_pair(string: &&str) -> bool {
    string
        .as_bytes()
        .windows(2)
        .map(|s| std::str::from_utf8(s).unwrap())
        .map(|s| string.match_indices(s).collect::<Vec<_>>())
        .filter(|r| r.len() > 1)
        .count()
        > 0
}

fn repeated_either_side_of_one_character(string: &&str) -> bool {
    string
        .as_bytes()
        .windows(3)
        .map(|s| std::str::from_utf8(s).unwrap())
        .any(|substring| substring.chars().next() == substring.chars().last())
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(str::as_bytes).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
        assert_eq!(part1("aaa"), 1);
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 258);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(part2("xxyxx"), 1);
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 53);
    }
}
