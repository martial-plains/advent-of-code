use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Mutex,
    },
    thread,
};

use algorithms::hashbrown::HashMap;
use regex::Regex;

use crate::shared::md5::hash;

pub const TITLE: &str = "Timing is Everything";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> i32 {
    let mut discs = Vec::new();
    let re =
        Regex::new(r"(?i)Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.")
            .unwrap();

    for capture in re.captures_iter(input) {
        let disc_number: usize = capture[1].parse().unwrap();
        let number_of_positions: usize = capture[2].parse().unwrap();
        let position_at_time: usize = capture[3].parse().unwrap();

        let disc = Disc::new(number_of_positions, position_at_time);
        discs.push(disc);
    }

    let mut time = -1;

    while !discs.iter().all(|disc| disc.current_position == 0) {
        time += 1;
        for (index, disc) in discs.iter_mut().enumerate() {
            for _ in 0..=index {
                disc.rotate_from_position(time as usize + 1 + index);
            }
        }
    }
    time
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> i32 {
    let mut discs = Vec::new();
    let re =
        Regex::new(r"(?i)Disc #(\d+) has (\d+) positions; at time=0, it is at position (\d+)\.")
            .unwrap();

    for capture in re.captures_iter(input) {
        let disc_number: usize = capture[1].parse().unwrap();
        let number_of_positions: usize = capture[2].parse().unwrap();
        let position_at_time: usize = capture[3].parse().unwrap();

        let disc = Disc::new(number_of_positions, position_at_time);
        discs.push(disc);
    }

    discs.push(Disc::new(11, 0));

    let mut time = -1;

    while !discs.iter().all(|disc| disc.current_position == 0) {
        time += 1;
        for (index, disc) in discs.iter_mut().enumerate() {
            for _ in 0..=index {
                disc.rotate_from_position(time as usize + 1 + index);
            }
        }
    }
    time
}

struct Disc {
    number_of_positions: usize,
    position_at_time: usize,
    current_position: usize,
}

impl Disc {
    pub const fn new(number_of_positions: usize, position_at_time: usize) -> Self {
        let current_position = position_at_time;

        Self {
            number_of_positions,
            position_at_time,
            current_position,
        }
    }

    pub fn rotate_from_position(&mut self, steps: usize) {
        self.current_position = (self.position_at_time + steps) % self.number_of_positions;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 16824);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 3_543_984);
    }
}
