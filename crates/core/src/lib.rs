#![feature(iter_array_chunks, let_chains, trait_alias)]
#![warn(clippy::pedantic, clippy::nursery, missing_debug_implementations)]
#![allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
pub mod year_2015;
pub mod year_2016;

pub mod shared;

/// # Panics
///
/// This function will panic if:
/// - The input is invalid.
/// - If the `year`, `day` or `part` is not implemented yet.
#[must_use]
pub fn solve(year: u16, day: u8, part: u8, input: &str) -> String {
    match year {
        2015 => solve_2015(day, part, input),
        2016 => solve_2016(day, part, input),
        _ => "Unimplemented year".to_string(),
    }
}

fn solve_2015(day: u8, part: u8, input: &str) -> String {
    match (day, part) {
        (1, 1) => year_2015::day_01::part1(input).to_string(),
        (1, 2) => year_2015::day_01::part2(input).to_string(),
        (2, 1) => year_2015::day_02::part1(input).to_string(),
        (2, 2) => year_2015::day_02::part2(input).to_string(),
        (3, 1) => year_2015::day_03::part1(input).to_string(),
        (3, 2) => year_2015::day_03::part2(input).to_string(),
        (4, 1) => year_2015::day_04::part1(input).to_string(),
        (4, 2) => year_2015::day_04::part2(input).to_string(),
        (5, 1) => year_2015::day_05::part1(input).to_string(),
        (5, 2) => year_2015::day_05::part2(input).to_string(),
        (6, 1) => year_2015::day_06::part1(input).to_string(),
        (6, 2) => year_2015::day_06::part2(input).to_string(),
        (7, 1) => year_2015::day_07::part1(input).to_string(),
        (7, 2) => year_2015::day_07::part2(input).to_string(),
        (8, 1) => year_2015::day_08::part1(input).to_string(),
        (8, 2) => year_2015::day_08::part2(input).to_string(),
        (9, 1) => year_2015::day_09::part1(input).to_string(),
        (9, 2) => year_2015::day_09::part2(input).to_string(),
        (10, 1) => year_2015::day_10::part1(input).to_string(),
        (10, 2) => year_2015::day_10::part2(input).to_string(),
        (11, 1) => year_2015::day_11::part1(input),
        (11, 2) => year_2015::day_11::part2(input),
        (12, 1) => year_2015::day_12::part1(input).to_string(),
        (12, 2) => year_2015::day_12::part2(input).to_string(),
        (13, 1) => year_2015::day_13::part1(input).to_string(),
        (13, 2) => year_2015::day_13::part2(input).to_string(),
        (14, 1) => year_2015::day_14::part1(input).to_string(),
        (14, 2) => year_2015::day_14::part2(input).to_string(),
        (15, 1) => year_2015::day_15::part1(input).to_string(),
        (15, 2) => year_2015::day_15::part2(input).to_string(),
        (16, 1) => year_2015::day_16::part1(input).unwrap().to_string(),
        (16, 2) => year_2015::day_16::part2(input).unwrap().to_string(),
        (17, 1) => year_2015::day_17::part1(input).to_string(),
        (17, 2) => year_2015::day_17::part2(input).to_string(),
        (18, 1) => year_2015::day_18::part1(input).to_string(),
        (18, 2) => year_2015::day_18::part2(input).to_string(),
        (19, 1) => year_2015::day_19::part1(input).to_string(),
        (19, 2) => year_2015::day_19::part2(input).to_string(),
        (20, 1) => year_2015::day_20::part1(input).to_string(),
        (20, 2) => year_2015::day_20::part2(input).to_string(),
        (21, 1) => year_2015::day_21::part1(input).unwrap().to_string(),
        (21, 2) => year_2015::day_21::part2(input).unwrap().to_string(),
        (22, 1) => year_2015::day_22::part1(input).to_string(),
        (22, 2) => year_2015::day_22::part2(input).to_string(),
        (23, 1) => year_2015::day_23::part1(input).to_string(),
        (23, 2) => year_2015::day_23::part2(input).to_string(),
        (24, 1) => year_2015::day_24::part1(input).to_string(),
        (24, 2) => year_2015::day_24::part2(input).to_string(),
        (25, 1) => year_2015::day_25::part1(input).to_string(),
        (25, 2) => year_2015::day_25::part2(input).to_string(),
        _ => unimplemented!(),
    }
}

fn solve_2016(day: u8, part: u8, input: &str) -> String {
    match (day, part) {
        (1, 1) => year_2016::day_01::part1(input).to_string(),
        (1, 2) => year_2016::day_01::part2(input).to_string(),
        (2, 1) => year_2016::day_02::part1(input),
        (2, 2) => year_2016::day_02::part2(input),
        (3, 1) => year_2016::day_03::part1(input).to_string(),
        (3, 2) => year_2016::day_03::part2(input).to_string(),
        (4, 1) => year_2016::day_04::part1(input).to_string(),
        (4, 2) => year_2016::day_04::part2(input).to_string(),
        (5, 1) => year_2016::day_05::part1(input),
        (5, 2) => year_2016::day_05::part2(input),
        (6, 1) => year_2016::day_06::part1(input),
        (6, 2) => year_2016::day_06::part2(input),
        (7, 1) => year_2016::day_07::part1(input).to_string(),
        (7, 2) => year_2016::day_07::part2(input).to_string(),
        (8, 1) => year_2016::day_08::part1(input).to_string(),
        (8, 2) => year_2016::day_08::part2(input),
        (9, 1) => year_2016::day_09::part1(input).to_string(),
        (9, 2) => year_2016::day_09::part2(input).to_string(),
        (10, 1) => year_2016::day_10::part1(input).to_string(),
        (10, 2) => year_2016::day_10::part2(input).to_string(),
        (11, 1) => year_2016::day_11::part1(input).to_string(),
        (11, 2) => year_2016::day_11::part2(input).to_string(),
        (12, 1) => year_2016::day_12::part1(input).to_string(),
        (12, 2) => year_2016::day_12::part2(input).to_string(),
        (13, 1) => year_2016::day_13::part1(input).to_string(),
        (13, 2) => year_2016::day_13::part2(input).to_string(),
        (14, 1) => year_2016::day_14::part1(input).to_string(),
        (14, 2) => year_2016::day_14::part2(input).to_string(),
        (15, 1) => year_2016::day_15::part1(input).to_string(),
        (15, 2) => year_2016::day_15::part2(input).to_string(),
        (16, 1) => year_2016::day_16::part1(input).to_string(),
        (16, 2) => year_2016::day_16::part2(input).to_string(),
        (17, 1) => year_2016::day_17::part1(input),
        (17, 2) => year_2016::day_17::part2(input).to_string(),
        (18, 1) => year_2016::day_18::part1(input).to_string(),
        (18, 2) => year_2016::day_18::part2(input).to_string(),
        (19, 1) => year_2016::day_19::part1(input).to_string(),
        (19, 2) => year_2016::day_19::part2(input).to_string(),
        (20, 1) => year_2016::day_20::part1(input).to_string(),
        (20, 2) => year_2016::day_20::part2(input).to_string(),
        (21, 1) => year_2016::day_21::part1(input),
        (21, 2) => year_2016::day_21::part2(input),
        (22, 1) => year_2016::day_22::part1(input).to_string(),
        (22, 2) => year_2016::day_22::part2(input).to_string(),
        (23, 1) => year_2016::day_23::part1(input).to_string(),
        (23, 2) => year_2016::day_23::part2(input).to_string(),
        (24, 1) => year_2016::day_24::part1(input).to_string(),
        (24, 2) => year_2016::day_24::part2(input).to_string(),
        (25, 1) => year_2016::day_25::part1(input).to_string(),
        (25, 2) => year_2016::day_25::part2(input).to_string(),
        _ => unimplemented!(),
    }
}
