use crate::shared::md5::hash;

use std::{
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    thread,
};

use rayon::prelude::*;

pub const TITLE: &str = "The Ideal Stocking Stuffer";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn part1(input: &str) -> u32 {
    let shared = parse(input);
    shared.first.load(Ordering::Relaxed)
}

#[must_use]
pub fn part2(input: &str) -> u32 {
    let shared = parse(input);
    shared.second.load(Ordering::Relaxed)
}

struct Solution {
    prefix: String,
    done: AtomicBool,
    counter: AtomicU32,
    first: AtomicU32,
    second: AtomicU32,
}

fn parse(input: &str) -> Solution {
    let shared = Solution {
        prefix: input.trim().to_string(),
        done: AtomicBool::new(false),
        counter: AtomicU32::new(1000),
        first: AtomicU32::new(u32::MAX),
        second: AtomicU32::new(u32::MAX),
    };

    // Handle the first 999 numbers specially as the number of digits varies.
    (1..1000).into_par_iter().for_each(|n| {
        let (mut buffer, size) = format_string(&shared.prefix, n);
        check_hash(&mut buffer, size, n, &shared);
    });

    // Use as many cores as possible to parallelize the remaining search.
    rayon::scope(|scope| {
        for _ in 0..rayon::current_num_threads() {
            scope.spawn(|_| worker(&shared));
        }
    });

    shared
}

fn format_string(prefix: &str, n: u32) -> ([u8; 64], usize) {
    let string = format!("{prefix}{n}");
    let size = string.len();

    let mut buffer = [0; 64];
    buffer[0..size].copy_from_slice(string.as_bytes());

    (buffer, size)
}

fn check_hash(buffer: &mut [u8], size: usize, n: u32, shared: &Solution) {
    let (result, ..) = hash(buffer, size);

    if result & 0xffff_ff00 == 0 {
        shared.second.fetch_min(n, Ordering::Relaxed);
        shared.done.store(true, Ordering::Relaxed);
    } else if result & 0xffff_f000 == 0 {
        shared.first.fetch_min(n, Ordering::Relaxed);
    }
}

fn worker(shared: &Solution) {
    while !shared.done.load(Ordering::Relaxed) {
        let offset = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let (mut buffer, size) = format_string(&shared.prefix, offset);

        for n in 0..1000 {
            // Format macro is very slow, so update digits directly
            buffer[size - 3] = b'0' + u8::try_from(n / 100).unwrap();
            buffer[size - 2] = b'0' + ((n / 10) % 10) as u8;
            buffer[size - 1] = b'0' + (n % 10) as u8;

            check_hash(&mut buffer, size, offset + n, shared);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_examples() {
        assert_eq!(part1("abcdef"), 609_043);
        assert_eq!(part1("pqrstuv"), 1_048_970);
    }

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 254_575);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 1_038_736);
    }
}
