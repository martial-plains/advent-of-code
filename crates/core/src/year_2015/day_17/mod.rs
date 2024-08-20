use std::{cmp::Ordering, collections::HashMap};

pub const TITLE: &str = "No Such Thing as Too Much";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if `input` is empty
#[must_use]
pub fn part1(input: &str) -> usize {
    combinations(input, 150).unwrap()
}

/// # Panics
///
/// Panics if `input` is empty
#[must_use]
pub fn part2(input: &str) -> usize {
    minimum_combinations(input, 150).unwrap()
}

fn combinations(input: &str, total: usize) -> anyhow::Result<usize> {
    fn visit(
        total: usize,
        idx: usize,
        sizes: &Vec<usize>,
        combinations: &mut usize,
        previous: usize,
    ) {
        let current = sizes[idx];
        match (previous + current).cmp(&total) {
            Ordering::Less => {
                for i in idx + 1..sizes.len() {
                    visit(total, i, sizes, combinations, previous + current);
                }
            }
            Ordering::Equal => {
                *combinations += 1;
            }
            Ordering::Greater => {}
        }
    }

    let mut sizes: Vec<usize> = input
        .lines()
        .map(|x| Ok(x.parse()?))
        .collect::<anyhow::Result<_>>()?;
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let mut combinations = 0;

    let upper = {
        let mut sum = 0;
        let mut i = sizes.len();
        loop {
            sum += sizes[i - 1];
            if sum >= total {
                break i;
            }
            i -= 1;
        }
    };

    for i in 0..upper {
        visit(total, i, &sizes, &mut combinations, 0);
    }

    anyhow::Ok(combinations)
}

fn minimum_combinations(input: &str, total: usize) -> anyhow::Result<usize> {
    fn visit(
        total: usize,
        idx: usize,
        sizes: &Vec<usize>,
        combinations: &mut HashMap<usize, usize>,
        previous: usize,
        count: usize,
    ) {
        let current = sizes[idx];
        match (previous + current).cmp(&total) {
            Ordering::Less => {
                for i in idx + 1..sizes.len() {
                    visit(total, i, sizes, combinations, previous + current, count + 1);
                }
            }
            Ordering::Equal => {
                *combinations.entry(count + 1).or_insert(0) += 1;
            }
            Ordering::Greater => {}
        }
    }

    let mut sizes: Vec<usize> = input
        .lines()
        .map(|x| Ok(x.parse()?))
        .collect::<anyhow::Result<_>>()?;
    sizes.sort_unstable_by(|a, b| b.cmp(a));
    let mut combinations = HashMap::new();

    let upper = {
        let mut sum = 0;
        let mut i = sizes.len();
        loop {
            sum += sizes[i - 1];
            if sum >= total {
                break i;
            }
            i -= 1;
        }
    };

    for i in 0..upper {
        visit(total, i, &sizes, &mut combinations, 0, 0);
    }

    Ok(combinations.into_iter().min_by_key(|v| v.0).unwrap().1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 654);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 57);
    }
}
