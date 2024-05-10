use std::{
    collections::{HashMap, HashSet},
    default,
    time::Instant,
};

use anyhow::anyhow;

use itertools::Itertools;
use log::info;
use num::abs;

pub const TITLE: &str = "A Maze of Twisty Little Cubicles";

pub const INPUT: &str = "1362";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> isize {
    let target = Location { x: 31, y: 39 };
    let mut distance = HashMap::new();

    distance = dijkstra(Location { x: 1, y: 1 }, Some(target), isize::MAX);

    distance[&target]
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let max_distance = 50;
    let mut distance = HashMap::new();

    distance = dijkstra(Location { x: 1, y: 1 }, None, max_distance);

    distance.len()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    const fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    const fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    const fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    const fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn is_wall(&self) -> bool {
        let multiplied =
            (self.x * self.x) + (3 * self.x) + (2 * self.x * self.y) + self.y + (self.y * self.y);
        let sum = multiplied + INPUT.parse::<isize>().unwrap();

        let as_bit_string = format!("{sum:b}");
        let count_ones = as_bit_string
            .chars()
            .map(|c| c.to_digit(10).unwrap_or(0))
            .sum::<u32>();

        count_ones % 2 == 1
    }

    const fn within_bounds(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    fn distance(&self, other: Self) -> isize {
        abs(self.x - other.x) + abs(self.y - other.y)
    }
}

fn dijkstra(
    source: Location,
    target: Option<Location>,
    max_distance: isize,
) -> HashMap<Location, isize> {
    fn neighbors_of(location: Location) -> Vec<Location> {
        [
            location.up(),
            location.down(),
            location.left(),
            location.right(),
        ]
        .iter()
        .filter(|location| location.within_bounds() && !location.is_wall())
        .copied()
        .collect_vec()
    }

    let mut distance = HashMap::new();
    let mut previous = HashMap::new();
    let mut unvisited = HashSet::new();
    let mut visited = HashSet::new();

    distance.entry(source).insert_entry(0);
    unvisited.insert(source);

    while !unvisited.is_empty() {
        let current = *distance
            .iter()
            .filter(|(key, value)| unvisited.contains(key))
            .sorted_by(|a, b| a.1.cmp(b.1))
            .next()
            .unwrap()
            .0;

        if target.is_some_and(|value| value == current) {
            break;
        }

        visited.insert(unvisited.take(&current).unwrap());

        if distance[&current] == max_distance {
            continue;
        }

        for neighbor in neighbors_of(current) {
            if !visited.contains(&neighbor) {
                unvisited.insert(neighbor);
            }

            let current_distance = distance[&current];

            if let Some(neighbor_distance) = distance.get(&neighbor).copied() {
                if current_distance + 1 < neighbor_distance {
                    distance.entry(neighbor).insert_entry(current_distance + 1);
                    previous.entry(neighbor).insert_entry(current);
                }
            } else {
                distance.entry(neighbor).insert_entry(current_distance + 1);
                previous.entry(neighbor).insert_entry(current);
            }
        }
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 82);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 138);
    }
}
