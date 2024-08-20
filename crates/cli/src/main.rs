use std::{fs, path::PathBuf};

use advent_of_code::solve;
use clap::Parser;

/// Represents the command-line interface (CLI) options for the Advent of Code application.
///
/// This struct is used to parse and store the input parameters provided by the user when running the
/// Advent of Code CLI application. It includes the year, day, and part of the challenge being solved,
/// as well as the path to the input file that contains the puzzle data.
///
/// # Attributes
///
/// - `year`: The year of the Advent of Code challenge. This should be a 4-digit number representing
///   the specific year's challenges.
///
/// - `day`: The day of the challenge within the Advent of Code event. Valid values are from 1 to 25,
///   corresponding to the days in December when the challenges are released.
///
/// - `part`: The part of the day's challenge to solve. Typically, there are two parts for each day's
///   challenge, with valid values being 1 or 2.
///
/// - `input_file`: The path to the input file containing the puzzle data. This file is expected to be
///   provided by the user and should match the format required for the specific challenge being solved.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The year of the Advent of Code challenge (e.g., 2024).
    pub year: u16,

    /// The day of the challenge within the Advent of Code event (1-25).
    pub day: u8,

    /// The part of the day's challenge to solve (1 or 2).
    pub part: u8,

    /// The path to the input file containing the puzzle data.
    pub input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let input = fs::read_to_string(args.input_file).unwrap();

    println!("{}", solve(args.year, args.day, args.part, &input))
}
