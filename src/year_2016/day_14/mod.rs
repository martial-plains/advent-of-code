use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Mutex,
    },
    thread,
};

use algorithms::hashbrown::HashMap;

use crate::shared::md5::hash;

pub const TITLE: &str = "One-Time Pad";

pub const INPUT: &str = "ihaygndm";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> i32 {
    let input = input.trim();

    let md5 = |n| {
        let (mut buffer, size) = format_string(input, n);
        hash(&mut buffer, size)
    };
    generate_pad(md5)
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> i32 {
    let input = input.trim();
    let md5 = |n| {
        let (mut buffer, size) = format_string(input, n);
        let mut result = hash(&mut buffer, size);

        for _ in 0..2016 {
            buffer[0..8].copy_from_slice(&to_ascii(result.0));
            buffer[8..16].copy_from_slice(&to_ascii(result.1));
            buffer[16..24].copy_from_slice(&to_ascii(result.2));
            buffer[24..32].copy_from_slice(&to_ascii(result.3));
            result = hash(&mut buffer, 32);
        }

        result
    };
    generate_pad(md5)
}

fn to_hash_chars(hash: &[u8]) -> [u8; 32] {
    let mut hash_chars = [0_u8; 32];
    for i in 0..32 {
        hash_chars[i] = if i % 2 == 0 {
            (hash[i / 2] & 0xF0) >> 4
        } else {
            hash[i / 2] & 0x0F
        };
    }
    hash_chars
}

struct Shared {
    done: AtomicBool,
    counter: AtomicI32,
}

struct Exclusive {
    threes: BTreeMap<i32, u32>,
    fives: BTreeMap<i32, u32>,
    found: BTreeSet<i32>,
}

fn format_string(prefix: &str, n: i32) -> ([u8; 64], usize) {
    let string = format!("{prefix}{n}");
    let size = string.len();

    let mut buffer = [0; 64];
    buffer[0..size].copy_from_slice(string.as_bytes());

    (buffer, size)
}

fn generate_pad(md5: impl Fn(i32) -> (u32, u32, u32, u32) + Copy + Sync) -> i32 {
    let shared = Shared {
        done: AtomicBool::new(false),
        counter: AtomicI32::new(0),
    };
    let exclusive = Exclusive {
        threes: BTreeMap::new(),
        fives: BTreeMap::new(),
        found: BTreeSet::new(),
    };
    let mutex = Mutex::new(exclusive);

    rayon::scope(|scope| {
        for _ in 0..rayon::current_num_threads() {
            scope.spawn(|_| check_keys(&shared, &mutex, md5));
        }
    });

    let exclusive = mutex.into_inner().unwrap();
    *exclusive.found.iter().nth(63).unwrap()
}

fn check_keys(
    shared: &Shared,
    mutex: &Mutex<Exclusive>,
    md5: impl Fn(i32) -> (u32, u32, u32, u32),
) {
    while !shared.done.load(Ordering::Relaxed) {
        let n = shared.counter.fetch_add(1, Ordering::Relaxed);
        let (a, b, c, d) = md5(n);

        let mut prev = u32::MAX;
        let mut same = 1;
        let mut three = 0;
        let mut five = 0;

        for mut word in [d, c, b, a] {
            for _ in 0..8 {
                let next = word & 0xf;

                if next == prev {
                    same += 1;
                } else {
                    same = 1;
                }

                if same == 3 {
                    three = 1 << next;
                }
                if same == 5 {
                    five |= 1 << next;
                }

                word >>= 4;
                prev = next;
            }
        }

        if three != 0 || five != 0 {
            let mut exclusive = mutex.lock().unwrap();
            let mut candidates = Vec::new();

            if three != 0 {
                exclusive.threes.insert(n, three);

                let fives = &exclusive.fives;
                for (&index, &mask) in fives.range(n + 1..n + 1000) {
                    if three & mask != 0 {
                        candidates.push(index);
                    }
                }
            }

            if five != 0 {
                exclusive.fives.insert(n, five);

                let threes = &exclusive.threes;
                for (&index, &mask) in threes.range(n - 1000..n - 1) {
                    if five & mask != 0 {
                        candidates.push(index);
                    }
                }
            }

            exclusive.found.extend(candidates);

            if exclusive.found.len() >= 64 {
                shared.done.store(true, Ordering::Relaxed);
            }
        }
    }
}

const fn to_ascii(n: u32) -> [u8; 8] {
    let mut n = n as u64;
    n = ((n << 16) & 0x0000_ffff_0000_0000) | (n & 0x0000_0000_0000_ffff);
    n = ((n << 8) & 0x00ff_0000_00ff_0000) | (n & 0x0000_00ff_0000_00ff);
    n = ((n << 4) & 0x0f00_0f00_0f00_0f00) | (n & 0x000f_000f_000f_000f);

    let mask = ((n + 0x0606_0606_0606_0606) >> 4) & 0x0101_0101_0101_0101;
    n = n + 0x3030_3030_3030_3030 + 0x27 * mask;
    n.to_be_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 15035);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 19968);
    }
}
