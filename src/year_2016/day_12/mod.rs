use std::default;

use algorithms::{hashbrown::HashMap, macros::hashmap};

use anyhow::anyhow;

use itertools::Itertools;

pub const TITLE: &str = "Leonardo's Monorail";

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Default, Clone)]
struct Computer {
    registers: HashMap<String, isize>,
}

impl Computer {
    fn new(a: isize, b: isize, c: isize, d: isize) -> Self {
        Self {
            registers: hashmap! {
                String::from("a") => a,
                String::from("b") => b,
                String::from("c") => c,
                String::from("d") => d,
            },
        }
    }

    fn copy_value(&mut self, value: isize, register: &str) {
        self.registers
            .entry(register.to_string())
            .and_modify(|inner| *inner = value);
    }

    fn copy(&mut self, from: &str, to: &str) {
        let new_val = self.registers[to];
        self.registers
            .entry(to.to_string())
            .and_modify(|inner| *inner = new_val);
    }

    fn decrement(&mut self, register: &str) {
        self.registers
            .entry(register.to_string())
            .and_modify(|inner| *inner -= 1)
            .or_default();
    }

    fn increment(&mut self, register: &str) {
        self.registers
            .entry(register.to_string())
            .and_modify(|inner| *inner += 1)
            .or_default();
    }
}

#[derive(Debug)]
struct Solver<'s> {
    computer: &'s mut Computer,
    instructions: Vec<&'s str>,
}

impl<'s> Solver<'s> {
    fn new(computer: &'s mut Computer, instructions: Vec<&'s str>) -> Self {
        Self {
            computer,
            instructions,
        }
    }

    fn solve(&mut self) {
        let mut i: isize = 0;

        while i < self.instructions.len() as isize {
            let components = self.instructions[i as usize].split(' ').collect::<Vec<_>>();

            if self.instructions[i as usize].starts_with("cpy") {
                if let Ok(value) = components[1].parse::<isize>() {
                    self.computer.copy_value(value, components[2]);
                } else {
                    let value = self.computer.registers[components[1]];
                    self.computer.copy_value(value, components[2]);
                }
            } else if self.instructions[i as usize].starts_with("jnz") {
                let value = components
                    .get(1)
                    .and_then(|s| s.parse().ok())
                    .unwrap_or_else(|| {
                        self.computer
                            .registers
                            .get(components[1])
                            .copied()
                            .unwrap_or_default()
                    });
                if value != 0 {
                    let jump = components[2].parse::<isize>().unwrap();
                    i += jump;
                    continue;
                }
            } else if self.instructions[i as usize].starts_with("inc") {
                self.computer.increment(components[1]);
            } else if self.instructions[i as usize].starts_with("dec") {
                self.computer.decrement(components[1]);
            } else {
                panic!(
                    "illegal instruction: {}",
                    INPUT.chars().nth(i as usize).unwrap()
                );
            }
            i += 1;
        }
    }
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> isize {
    let mut computer = Computer::new(0, 0, 0, 0);
    Solver::new(&mut computer, input.lines().collect_vec()).solve();

    computer
        .registers
        .into_iter()
        .sorted()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .1
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> isize {
    let mut computer = Computer::new(0, 0, 1, 0);
    Solver::new(&mut computer, input.lines().collect_vec()).solve();

    computer
        .registers
        .into_iter()
        .sorted()
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 318_077);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 9_227_731);
    }
}
