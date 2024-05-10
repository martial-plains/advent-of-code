use algorithms::higher_order_functions::Reductions;

use crate::shared::{
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

pub const TITLE: &str = "Signals and Noise";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn calculate_frequency(input: &str) -> Vec<[usize; 26]> {
    let width = input.lines().next().unwrap().len();
    let mut freq = vec![[0; 26]; width];

    for (i, b) in input.bytes().filter(u8::is_ascii_lowercase).enumerate() {
        freq[i % width][(b - b'a') as usize] += 1;
    }

    freq
}

#[must_use]
pub fn part1(input: &str) -> String {
    let freq = calculate_frequency(input);

    freq.iter()
        .map(|freq| {
            freq.iter()
                .enumerate()
                .filter(|(_, f)| **f > 0)
                .max_by_key(|(_, f)| **f)
                .unwrap()
        })
        .map(|(index, _)| ((index as u8) + b'a') as char)
        .collect()
}

#[must_use]
pub fn part2(input: &str) -> String {
    let freq = calculate_frequency(input);

    freq.iter()
        .map(|freq| {
            freq.iter()
                .enumerate()
                .filter(|(_, f)| **f > 0)
                .min_by_key(|(_, f)| **f)
                .unwrap()
        })
        .map(|(index, _)| ((index as u8) + b'a') as char)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), "qoclwvah");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "ryrgviuv");
    }
}
