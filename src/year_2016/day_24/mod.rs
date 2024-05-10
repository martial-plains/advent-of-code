use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

pub const TITLE: &str = "Air Duct Spelunking";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut walls = Vec::new();
    let mut goals = Vec::new();
    for (j, row) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (i, c) in row.chars().enumerate() {
            if c == '#' {
                row_vec.push(true);
            } else if c.is_ascii_digit() {
                goals.push((j, i));
                row_vec.push(false);
            } else {
                row_vec.push(false);
            }
        }
        walls.push(row_vec);
    }
    let dists = get_dists_map(&walls, &goals);

    shortest_path_wrapper(&dists, true)
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut walls = Vec::new();
    let mut goals = Vec::new();
    for (j, row) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (i, c) in row.chars().enumerate() {
            if c == '#' {
                row_vec.push(true);
            } else if c.is_ascii_digit() {
                goals.push((j, i));
                row_vec.push(false);
            } else {
                row_vec.push(false);
            }
        }
        walls.push(row_vec);
    }
    let dists = get_dists_map(&walls, &goals);

    shortest_path_wrapper(&dists, false)
}

type Pos = (usize, usize);

fn get_dists_to_all(walls: &[Vec<bool>], from: Pos, to: &Vec<Pos>) -> Vec<usize> {
    let mut left = to.len();
    let mut dist = vec![vec![usize::MAX; walls[0].len()]; walls.len()];
    let mut todo = VecDeque::new();
    dist[from.0][from.1] = 0;
    todo.push_back(from);
    while left > 0 && !todo.is_empty() {
        if let Some(t) = todo.pop_front() {
            if to.contains(&t) {
                left -= 1;
            }
            let (x, y) = t;
            let d = dist[x][y] + 1;
            if x > 0 && !walls[x - 1][y] && dist[x - 1][y] > d {
                dist[x - 1][y] = d;
                todo.push_back((x - 1, y));
            }
            if x + 1 < walls.len() && !walls[x + 1][y] && dist[x + 1][y] > d {
                dist[x + 1][y] = d;
                todo.push_back((x + 1, y));
            }
            if y > 0 && !walls[x][y - 1] && dist[x][y - 1] > d {
                dist[x][y - 1] = d;
                todo.push_back((x, y - 1));
            }
            if y + 1 < walls[0].len() && !walls[x][y + 1] && dist[x][y + 1] > d {
                dist[x][y + 1] = d;
                todo.push_back((x, y + 1));
            }
        }
    }
    let mut res = Vec::new();
    for dst in to {
        res.push(dist[dst.0][dst.1]);
    }
    res
}

fn get_dists_map(walls: &[Vec<bool>], goals: &Vec<Pos>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    for &start in goals {
        ret.push(get_dists_to_all(walls, start, goals));
    }
    ret
}

fn shortest_path(
    dists: &Vec<Vec<usize>>,
    cur: usize,
    visited: usize,
    mem: &mut Vec<HashMap<usize, usize>>,
    is_part1: bool,
) -> usize {
    let goal_mask = (1 << dists.len()) - 1;
    if visited == goal_mask {
        if is_part1 {
            0
        } else {
            dists[cur][0]
        }
    } else {
        if let Some(val) = mem[cur].get(&visited) {
            return *val;
        }
        let mut best = usize::MAX;
        for i in 0..dists.len() {
            if (visited & (1 << i)) == 0 {
                let new_visited = visited | (1 << i);
                let dist = shortest_path(dists, i, new_visited, mem, is_part1);
                if dist != usize::MAX && dist + dists[cur][i] < best {
                    best = dist + dists[cur][i];
                }
            }
        }
        mem[cur].insert(visited, best);
        best
    }
}

fn shortest_path_wrapper(dists: &Vec<Vec<usize>>, is_part1: bool) -> usize {
    let mut mem = vec![HashMap::new(); dists.len()];
    shortest_path(dists, 0, 1, &mut mem, is_part1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 412);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 664);
    }
}
