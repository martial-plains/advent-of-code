# Advent of Code

Welcome to my **Advent of Code** workspace! 🎄 This repository contains my solutions to the Advent of Code challenges, implemented in Rust. It is organized into two main crates:

- **Core Library**: A collection of Rust functions that solve each daily challenge.
- **CLI App**: A simple command-line interface to run solutions for any day and part.

## Repository Structure

```
advent-of-code/
├── crates/
│   ├── core/        # Core library with all solutions
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── cli/         # Command-line interface for running solutions
│   │   ├── src/
│   │   └── Cargo.toml
├── Cargo.toml       # Workspace configuration
└── README.md

```

## Core Library

The `core` crate contains the solutions for each day's puzzle. Each day's solution is implemented as a separate function.

You'll find solutions for each day within the `core/src/` folder.

## CLI App

The `cli` crate provides a simple command-line interface to run the solutions for any given day. Usage is straightforward:

```bash
$ cargo run --release -- <year> <day> <part> <input_file>
```

For example, to run day 1 of 2015, part 1 with an input file:

```bash
$ cargo run --release -- 2015 1 1 input/day1.txt
```

The app will call the corresponding function from the core library to process the input and print the result.

## Contributing

Feel free to open issues or submit pull requests if you find any bugs or want to improve the code.

## License

This project is licensed under the MIT License.
