use std::{fs, path::PathBuf};

use advent_of_code::solve;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    year: u16,
    day: u8,
    part: u8,
    input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let input = fs::read_to_string(args.input_file).unwrap();

    println!("{}", solve(args.year, args.day, args.part, &input))
}
