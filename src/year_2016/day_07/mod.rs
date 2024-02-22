use algorithms::higher_order_functions::Reductions;

use crate::shared::{
    hash::{FastSet, FastSetBuilder},
    point::Point,
};

pub const TITLE: &str = "Internet Protocol Version 7";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut inside = false;
    let mut positive = false;
    let mut negative = false;

    for w in input.as_bytes().windows(4) {
        if w[0].is_ascii_lowercase() {
            if is_palindrome(w) {
                if inside {
                    negative = true;
                } else {
                    positive = true;
                }
            }
        } else if w[0] == b'[' {
            inside = true;
        } else if w[0] == b']' {
            inside = false;
        } else {
            // Next line
            if positive && !negative {
                count += 1;
            }
            positive = false;
            negative = false;
        }
    }

    if positive && !negative {
        count += 1;
    }

    count
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut version = 0;
    let mut inside = false;
    let mut positive = false;
    let mut aba = [usize::MAX; 676];
    let mut bab = [usize::MAX; 676];

    for w in input.as_bytes().windows(3) {
        if w[1].is_ascii_lowercase() {
            if w[0] == w[2] && w[0] != w[1] {
                let (first, second) = bytes_to_indices(w);

                if inside {
                    // Reverse the order of letters
                    let index = 26 * second + first;
                    bab[index] = version;
                    positive |= aba[index] == version;
                } else {
                    let index = 26 * first + second;
                    aba[index] = version;
                    positive |= bab[index] == version;
                }
            }
        } else if w[1] == b'[' {
            inside = true;
        } else if w[1] == b']' {
            inside = false;
        } else {
            // Next line
            if positive {
                count += 1;
            }
            version += 1;
            positive = false;
        }
    }

    if positive {
        count += 1;
    }

    count
}

fn is_palindrome(w: &[u8]) -> bool {
    w[0] == w[3] && w[1] == w[2] && w[0] != w[1]
}

fn bytes_to_indices(w: &[u8]) -> (usize, usize) {
    let first = (w[0] - b'a') as usize;
    let second = (w[1] - b'a') as usize;
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 118);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 260);
    }
}
