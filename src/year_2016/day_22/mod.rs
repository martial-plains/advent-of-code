use std::{array, collections::VecDeque};

use itertools::Itertools;

pub const TITLE: &str = "Grid Computing";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> i32 {
    let nodes = get_nodes(input);
    let mut sum = 0;
    for a in &nodes {
        for b in &nodes {
            if a.path != b.path && a.used != 0 && a.used <= b.avail {
                sum += 1;
            }
        }
    }

    sum
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let nodes = get_nodes(input);

    let bottom_right = nodes
        .iter()
        .map(|node| node.position)
        .max()
        .ok_or("No bottom right node")
        .unwrap();
    let dimensions = (bottom_right.0 + 1, bottom_right.1 + 1);
    let wall_threshold = nodes
        .iter()
        .filter(|node| node.position.1 == 0)
        .map(|node| node.size)
        .max()
        .ok_or("No wall threshold")
        .unwrap();
    let mut grid = vec![true; usize::from(dimensions.0) * usize::from(dimensions.1)];

    for node in &nodes {
        grid[usize::from(node.position.1) * usize::from(dimensions.0)
            + usize::from(node.position.0)] = node.size > wall_threshold;
    }

    let empty_pos = nodes
        .iter()
        .find(|node| node.used == 0 && node.size > 0)
        .ok_or("No empty node")
        .unwrap()
        .position;
    let payload_pos = (dimensions.0 - 1, 0);
    let dist_to_payload = dist(dimensions, grid.clone(), empty_pos, payload_pos).unwrap();
    let dist_to_home = dist(dimensions, grid, (payload_pos.0 - 1, payload_pos.1), (0, 0)).unwrap();
    (dist_to_payload + 5 * dist_to_home)
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
struct Node {
    path: String,
    position: (u8, u8),
    size: i32,
    used: i32,
    avail: i32,
    percentage_used: i32,
}

fn get_nodes(input: &str) -> Vec<Node> {
    let re = regex::Regex::new(r"(?P<path>[a-zA-Z\/\-0-9]+)\s+(?P<size>\d+)T\s+(?P<used>\d+)T\s+(?P<avail>\d+)T\s+(?P<percentageUsed>\d+)%").unwrap();

    let mut nodes = Vec::<Node>::new();
    for line in input.lines().skip(2) {
        if let Some(captures) = re.captures(line) {
            let path = captures.name("path").unwrap().as_str().to_string();
            let size = captures
                .name("size")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let used = captures
                .name("used")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let avail = captures
                .name("avail")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
            let percentage_used = captures
                .name("percentageUsed")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();

            let name_parts = path.split('-').collect::<Vec<_>>();
            let x = name_parts[1][1..].parse::<u8>().unwrap();
            let y = name_parts[2][1..].parse::<u8>().unwrap();

            nodes.push(Node {
                path,
                position: (x, y),
                size,
                used,
                avail,
                percentage_used,
            });
        }
    }

    nodes
}

fn dist(
    dimensions: (u8, u8),
    mut grid: Vec<bool>,
    start: (u8, u8),
    destination: (u8, u8),
) -> Result<usize, String> {
    let mut queue = VecDeque::new();
    queue.push_back((0, start));

    while let Some((path_length, position)) = queue.pop_front() {
        let grid_idx =
            usize::from(position.1) * usize::from(dimensions.0) + usize::from(position.0);

        if grid[grid_idx] {
            continue;
        }
        if position == destination {
            return Ok(path_length);
        }

        grid[grid_idx] = true;

        if position.0 != 0 {
            queue.push_back((path_length + 1, (position.0 - 1, position.1)));
        }
        if position.0 != dimensions.0 - 1 {
            queue.push_back((path_length + 1, (position.0 + 1, position.1)));
        }
        if position.1 != 0 {
            queue.push_back((path_length + 1, (position.0, position.1 - 1)));
        }
        if position.1 != dimensions.1 - 1 {
            queue.push_back((path_length + 1, (position.0, position.1 + 1)));
        }
    }

    Err(format!("No path found from {start:?} to {destination:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 1003);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 192);
    }
}
