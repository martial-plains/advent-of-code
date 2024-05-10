use std::{
    collections::VecDeque,
    hash::Hash,
    io::{Bytes, Read, Write},
    ops::Range,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use md5::{digest::generic_array::GenericArray, Digest, Md5};

pub const TITLE: &str = "Two Steps Forward";

pub const INPUT: &str = "ioramepc";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> String {
    find_shortest_path(*GOAL, input, &BOUNDS)
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    find_longest_path(*GOAL, input, &BOUNDS).len()
}

lazy_static! {
    static ref GOAL: Coordinate = Coordinate::new(3, 3);
    static ref BOUNDS: Bounds = Bounds::new(0..4, 0..4);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

/// Two dimensional coordinate
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    const fn step(self, direction: Direction, steps: u32) -> Self {
        let steps = steps as i32;

        match direction {
            Direction::Right => Self::new(self.x + steps, self.y),
            Direction::Left => Self::new(self.x - steps, self.y),
            Direction::Up => Self::new(self.x, self.y - steps),
            Direction::Down => Self::new(self.x, self.y + steps),
        }
    }

    fn step_in_bounds(self, direction: Direction, steps: u32, bounds: &Bounds) -> Option<Self> {
        let coordinate = self.step(direction, steps);
        (bounds.x_values.contains(&coordinate.x) && bounds.y_values.contains(&coordinate.y))
            .then_some(coordinate)
    }
}

#[derive(Clone)]
struct Bounds {
    x_values: Range<i32>,
    y_values: Range<i32>,
}

impl Bounds {
    const fn new(x_values: Range<i32>, y_values: Range<i32>) -> Self {
        Self { x_values, y_values }
    }
}

fn find_shortest_path(goal: Coordinate, passcode: &str, bounds: &Bounds) -> String {
    let mut queue = VecDeque::from([(Coordinate::default(), String::new())]);
    while let Some((coordinate, path)) = queue.pop_front() {
        if coordinate == goal {
            return path;
        }

        for (coordinate, next_direction_char) in
            get_valid_adjacent(coordinate, bounds, passcode, &path)
        {
            queue.push_back((coordinate, format!("{path}{next_direction_char}")));
        }
    }

    unreachable!()
}

fn find_longest_path(goal: Coordinate, passcode: &str, bounds: &Bounds) -> String {
    let mut queue = VecDeque::from([(Coordinate::default(), String::new())]);
    let mut longest_path = String::new();

    while let Some((coordinate, path)) = queue.pop_front() {
        if coordinate == goal {
            longest_path.clone_from(&path);
            continue;
        }

        for (coordinate, next_direction_char) in
            get_valid_adjacent(coordinate, bounds, passcode, &path)
        {
            queue.push_back((coordinate, format!("{path}{next_direction_char}")));
        }
    }

    longest_path
}

fn get_valid_adjacent(
    coordinate: Coordinate,
    bounds: &Bounds,
    passcode: &str,
    path: &str,
) -> Vec<(Coordinate, char)> {
    let mut directions = vec![
        (Direction::Right, 'R'),
        (Direction::Up, 'U'),
        (Direction::Left, 'L'),
        (Direction::Down, 'D'),
    ];

    remove_locked_directions(&mut directions, passcode, path);
    transform_to_valid_coordinates(&directions, coordinate, bounds)
}

fn remove_locked_directions(directions: &mut Vec<(Direction, char)>, passcode: &str, path: &str) {
    let to_hash = format!("{passcode}{path}");
    let locks = hash(&to_hash).chars().take(4).collect_vec();
    let open = 'b'..='f';

    directions.retain(|(direction, _)| match direction {
        Direction::Up => open.contains(&locks[0]),
        Direction::Down => open.contains(&locks[1]),
        Direction::Left => open.contains(&locks[2]),
        Direction::Right => open.contains(&locks[3]),
    });
}

fn transform_to_valid_coordinates(
    directions: &[(Direction, char)],
    coordinate: Coordinate,
    bounds: &Bounds,
) -> Vec<(Coordinate, char)> {
    directions
        .iter()
        .map(|(direction, ch)| (coordinate.step_in_bounds(*direction, 1, bounds), ch))
        .filter(|(coordinate, _)| coordinate.is_some())
        .map(|(coordinate, ch)| (coordinate.unwrap(), *ch))
        .collect()
}

fn hash(to_hash: &str) -> String {
    let mut md5 = Md5::new();
    md5.update(to_hash);

    let digest = md5.finalize();
    format!("{digest:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), "RDDRULDDRR");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 766);
    }
}
