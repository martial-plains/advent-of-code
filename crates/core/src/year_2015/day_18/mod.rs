use std::{
    fmt::{self, Display, Formatter},
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::{anyhow, Error};

pub const TITLE: &str = "Like a GIF For Your Yard";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if `input` is invalid
#[must_use]
pub fn part1(input: &str) -> usize {
    let mut grid: LightGrid = input.parse().unwrap();
    for _ in 0..100 {
        grid = grid.transform_1();
    }
    grid.count_on()
}

/// # Panics
///
/// Panics if `input` is invalid
#[must_use]
pub fn part2(input: &str) -> usize {
    let mut grid: LightGrid = input.parse().unwrap();
    grid[(0, 0)] = true;
    grid[(99, 0)] = true;
    grid[(0, 99)] = true;
    grid[(99, 99)] = true;

    for _ in 0..100 {
        grid = grid.transform_2();
    }

    grid.count_on()
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct LightGrid {
    size: (usize, usize),
    data: Vec<bool>,
}

struct LightGridNeighbors {
    size: (usize, usize),
    idx: usize,
    base: (usize, usize),
}

impl LightGrid {
    const fn neighbors(&self, position: (usize, usize)) -> LightGridNeighbors {
        LightGridNeighbors {
            size: self.size,
            idx: 0,
            base: position,
        }
    }

    fn transform_1(&self) -> Self {
        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, &old_state)| {
                let position = (i % self.size.0, i / self.size.0);
                let neighbors_on = self
                    .neighbors(position)
                    .filter(|&position| self[position])
                    .count();
                if old_state {
                    neighbors_on == 2 || neighbors_on == 3
                } else {
                    neighbors_on == 3
                }
            })
            .collect();
        Self {
            size: self.size,
            data,
        }
    }

    fn transform_2(&self) -> Self {
        let mut new = self.transform_1();
        let (w, h) = self.size;
        new[(0, 0)] = true;
        new[(w - 1, 0)] = true;
        new[(0, h - 1)] = true;
        new[(w - 1, h - 1)] = true;
        new
    }

    fn count_on(&self) -> usize {
        self.data.iter().filter(|s| **s).count()
    }
}

impl Iterator for LightGridNeighbors {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = self.base;
        let (w, h) = self.size;
        loop {
            self.idx += 1;
            // 123
            // 4B5
            // 678
            match self.idx {
                1 if x > 0 && y > 0 => return Some((x - 1, y - 1)),
                2 if y > 0 => return Some((x, y - 1)),
                3 if x < w - 1 && y > 0 => return Some((x + 1, y - 1)),
                4 if x > 0 => return Some((x - 1, y)),
                5 if x < w - 1 => return Some((x + 1, y)),
                6 if x > 0 && y < h - 1 => return Some((x - 1, y + 1)),
                7 if y < h - 1 => return Some((x, y + 1)),
                8 if x < w - 1 && y < h - 1 => return Some((x + 1, y + 1)),
                1..=8 => {}

                _ => return None,
            }
        }
    }
}

impl Index<(usize, usize)> for LightGrid {
    type Output = bool;

    fn index(&self, (x, y): (usize, usize)) -> &bool {
        &self.data[self.size.0 * y + x]
    }
}
impl IndexMut<(usize, usize)> for LightGrid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut bool {
        &mut self.data[self.size.0 * y + x]
    }
}

impl Display for LightGrid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut res = String::with_capacity((self.size.0 + 1) * self.size.1);
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                res.push(if self[(x, y)] { '#' } else { '.' });
            }
            res.push('\n');
        }
        res.pop();
        write!(f, "{res}")
    }
}

impl FromStr for LightGrid {
    type Err = Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let lines: Vec<_> = s.lines().collect();
        if lines.is_empty() {
            return Err(anyhow!("empty string"));
        }
        let h = lines.len();
        let w = lines[0].len();
        if !lines.iter().skip(1).all(|x| x.len() == w) {
            return Err(anyhow!("inconsistent width"));
        }
        let mut data = Vec::with_capacity(w * h);
        for line in lines {
            for c in line.chars() {
                match c {
                    '#' => data.push(true),
                    '.' => data.push(false),
                    _ => return Err(anyhow!("invalid character, expected # or .")),
                }
            }
        }
        Ok(Self { size: (w, h), data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 814);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 924);
    }
}
