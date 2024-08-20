use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

use itertools::Itertools;
use sscanf::scanf;

pub const TITLE: &str = "Balance Bots";

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
enum Target {
    Bot(i32),
    Output(i32),
}

impl Target {
    fn new(s: &str, i: i32) -> Self {
        match s {
            "bot" => Self::Bot(i),
            "output" => Self::Output(i),
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Bot {
    recieved_chip: i32,
    lo: Target,
    hi: Target,
}

impl FromStr for Bot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (recieved_chip, target_lo, lo, target_hi, hi) = scanf!(
            s,
            "bot {i32} gives low to {str} {i32} and high to {str} {i32}"
        )
        .unwrap();

        Ok(Self {
            recieved_chip,
            lo: Target::new(target_lo, lo),
            hi: Target::new(target_hi, hi),
        })
    }
}

fn get_instructions(input: &str) -> HashMap<i32, Bot> {
    input
        .lines()
        .filter(|l| l.starts_with("bot"))
        .map(|l| l.parse::<Bot>().unwrap())
        .map(|i| (i.recieved_chip, i))
        .collect::<HashMap<_, _>>()
}

fn get_initial_state(input: &str) -> HashMap<i32, Vec<i32>> {
    input
        .lines()
        .filter(|l| l.starts_with("value"))
        .filter_map(|l| scanf!(l, "value {i32} goes to bot {i32}").ok())
        .map(|(value, bot)| (bot, value))
        .into_group_map()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> i32 {
    let instructions = get_instructions(input);
    let mut state = get_initial_state(input);
    loop {
        let current = state.clone();
        // For every bot with 2 microchips, give it to the bot next in line based on its instruction.
        for (bot, microchips) in current.into_iter().filter(|(_, v)| v.len() == 2) {
            let (lo, hi) = (
                microchips[0].min(microchips[1]),
                microchips[0].max(microchips[1]),
            );
            let instruction = &instructions[&bot];
            state.remove(&bot);
            // Ignore output bins.
            if let Target::Bot(bot) = instruction.lo {
                state.entry(bot).or_default().push(lo);
            }
            if let Target::Bot(bot) = instruction.hi {
                state.entry(bot).or_default().push(hi);
            }
            if lo == 17 && hi == 61 {
                return bot;
            }
        }
    }
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> i32 {
    let instructions = get_instructions(input);
    let mut state = get_initial_state(input);
    let mut bins = HashMap::new();
    while !bins.contains_key(&0) || !bins.contains_key(&1) || !bins.contains_key(&2) {
        let current = state.clone();
        // For every bot with 2 microchips, give it to the bot next in line based on its instruction.
        for (bot, microchips) in current.into_iter().filter(|(_, v)| v.len() == 2) {
            let (lo, hi) = (
                microchips[0].min(microchips[1]),
                microchips[0].max(microchips[1]),
            );
            let instruction = &instructions[&bot];
            state.remove(&bot);
            match instruction.lo {
                Target::Bot(bot) => state.entry(bot).or_default().push(lo),
                Target::Output(bin) => {
                    bins.insert(bin, lo);
                }
            }
            match instruction.hi {
                Target::Bot(bot) => state.entry(bot).or_default().push(hi),
                Target::Output(bin) => {
                    bins.insert(bin, hi);
                }
            }
        }
    }
    bins[&0] * bins[&1] * bins[&2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 56);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 7847);
    }
}
