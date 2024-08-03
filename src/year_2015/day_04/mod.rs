#![allow(clippy::many_single_char_names)]

use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

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
    let (result, ..) = {
        let mut buffer: &mut [u8] = buffer;
        let end = buffer.len() - 8;
        let bits = size * 8;

        buffer[size] = 0x80;
        buffer[end..].copy_from_slice(&bits.to_le_bytes());

        let mut m = [0; 16];
        let mut a0: u32 = 0x6745_2301;
        let mut b0: u32 = 0xefcd_ab89;
        let mut c0: u32 = 0x98ba_dcfe;
        let mut d0: u32 = 0x1032_5476;

        while !buffer.is_empty() {
            let (prefix, suffix) = buffer.split_at_mut(64);
            buffer = suffix;

            for (i, chunk) in prefix.chunks_exact(4).enumerate() {
                m[i] = u32::from_le_bytes(chunk.try_into().unwrap());
            }

            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;

            a = round1(a, b, c, d, m[0], 7, 0xd76a_a478);
            d = round1(d, a, b, c, m[1], 12, 0xe8c7_b756);
            c = round1(c, d, a, b, m[2], 17, 0x2420_70db);
            b = round1(b, c, d, a, m[3], 22, 0xc1bd_ceee);
            a = round1(a, b, c, d, m[4], 7, 0xf57c_0faf);
            d = round1(d, a, b, c, m[5], 12, 0x4787_c62a);
            c = round1(c, d, a, b, m[6], 17, 0xa830_4613);
            b = round1(b, c, d, a, m[7], 22, 0xfd46_9501);
            a = round1(a, b, c, d, m[8], 7, 0x6980_98d8);
            d = round1(d, a, b, c, m[9], 12, 0x8b44_f7af);
            c = round1(c, d, a, b, m[10], 17, 0xffff_5bb1);
            b = round1(b, c, d, a, m[11], 22, 0x895c_d7be);
            a = round1(a, b, c, d, m[12], 7, 0x6b90_1122);
            d = round1(d, a, b, c, m[13], 12, 0xfd98_7193);
            c = round1(c, d, a, b, m[14], 17, 0xa679_438e);
            b = round1(b, c, d, a, m[15], 22, 0x49b4_0821);

            a = round2(a, b, c, d, m[1], 5, 0xf61e_2562);
            d = round2(d, a, b, c, m[6], 9, 0xc040_b340);
            c = round2(c, d, a, b, m[11], 14, 0x265e_5a51);
            b = round2(b, c, d, a, m[0], 20, 0xe9b6_c7aa);
            a = round2(a, b, c, d, m[5], 5, 0xd62f_105d);
            d = round2(d, a, b, c, m[10], 9, 0x0244_1453);
            c = round2(c, d, a, b, m[15], 14, 0xd8a1_e681);
            b = round2(b, c, d, a, m[4], 20, 0xe7d3_fbc8);
            a = round2(a, b, c, d, m[9], 5, 0x21e1_cde6);
            d = round2(d, a, b, c, m[14], 9, 0xc337_07d6);
            c = round2(c, d, a, b, m[3], 14, 0xf4d5_0d87);
            b = round2(b, c, d, a, m[8], 20, 0x455a_14ed);
            a = round2(a, b, c, d, m[13], 5, 0xa9e3_e905);
            d = round2(d, a, b, c, m[2], 9, 0xfcef_a3f8);
            c = round2(c, d, a, b, m[7], 14, 0x676f_02d9);
            b = round2(b, c, d, a, m[12], 20, 0x8d2a_4c8a);

            a = round3(a, b, c, d, m[5], 4, 0xfffa_3942);
            d = round3(d, a, b, c, m[8], 11, 0x8771_f681);
            c = round3(c, d, a, b, m[11], 16, 0x6d9d_6122);
            b = round3(b, c, d, a, m[14], 23, 0xfde5_380c);
            a = round3(a, b, c, d, m[1], 4, 0xa4be_ea44);
            d = round3(d, a, b, c, m[4], 11, 0x4bde_cfa9);
            c = round3(c, d, a, b, m[7], 16, 0xf6bb_4b60);
            b = round3(b, c, d, a, m[10], 23, 0xbebf_bc70);
            a = round3(a, b, c, d, m[13], 4, 0x289b_7ec6);
            d = round3(d, a, b, c, m[0], 11, 0xeaa1_27fa);
            c = round3(c, d, a, b, m[3], 16, 0xd4ef_3085);
            b = round3(b, c, d, a, m[6], 23, 0x0488_1d05);
            a = round3(a, b, c, d, m[9], 4, 0xd9d4_d039);
            d = round3(d, a, b, c, m[12], 11, 0xe6db_99e5);
            c = round3(c, d, a, b, m[15], 16, 0x1fa2_7cf8);
            b = round3(b, c, d, a, m[2], 23, 0xc4ac_5665);

            a = round4(a, b, c, d, m[0], 6, 0xf429_2244);
            d = round4(d, a, b, c, m[7], 10, 0x432a_ff97);
            c = round4(c, d, a, b, m[14], 15, 0xab94_23a7);
            b = round4(b, c, d, a, m[5], 21, 0xfc93_a039);
            a = round4(a, b, c, d, m[12], 6, 0x655b_59c3);
            d = round4(d, a, b, c, m[3], 10, 0x8f0c_cc92);
            c = round4(c, d, a, b, m[10], 15, 0xffef_f47d);
            b = round4(b, c, d, a, m[1], 21, 0x8584_5dd1);
            a = round4(a, b, c, d, m[8], 6, 0x6fa8_7e4f);
            d = round4(d, a, b, c, m[15], 10, 0xfe2c_e6e0);
            c = round4(c, d, a, b, m[6], 15, 0xa301_4314);
            b = round4(b, c, d, a, m[13], 21, 0x4e08_11a1);
            a = round4(a, b, c, d, m[4], 6, 0xf753_7e82);
            d = round4(d, a, b, c, m[11], 10, 0xbd3a_f235);
            c = round4(c, d, a, b, m[2], 15, 0x2ad7_d2bb);
            b = round4(b, c, d, a, m[9], 21, 0xeb86_d391);

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        (a0.to_be(), b0.to_be(), c0.to_be(), d0.to_be())
    };

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

#[inline]
const fn round1(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & c) | (!b & d);
    common(f, a, b, m, s, k)
}

#[inline]
const fn round2(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & d) | (c & !d);
    common(f, a, b, m, s, k)
}

#[inline]
const fn round3(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = b ^ c ^ d;
    common(f, a, b, m, s, k)
}

#[inline]
const fn round4(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = c ^ (b | !d);
    common(f, a, b, m, s, k)
}

#[inline]
const fn common(f: u32, a: u32, b: u32, m: u32, s: u32, k: u32) -> u32 {
    f.wrapping_add(a)
        .wrapping_add(k)
        .wrapping_add(m)
        .rotate_left(s)
        .wrapping_add(b)
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
