use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

use permutohedron::Heap;
use regex::{Regex, RegexBuilder};

pub const TITLE: &str = "Knights of the Dinner Table";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// * Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> usize {
    let input = &input;
    compute_total_happiness(parse_happiness(input).unwrap())
        .try_into()
        .unwrap()
}

/// # Panics
///
/// * Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> isize {
    let (mut people, mut happiness) = parse_happiness(input).unwrap();
    for person in &people {
        happiness.insert(("", person), 0);
        happiness.insert((person, ""), 0);
    }
    people.insert("");
    compute_total_happiness((people, happiness))
}

type People<'a> = HashSet<&'a str>;
type Happiness<'a> = HashMap<(&'a str, &'a str), isize>;

fn parse_happiness<'a>(input: &'a str) -> anyhow::Result<(People<'a>, Happiness<'a>)> {
    let re = RegexBuilder::new(r"^(?P<f>[[:alpha:]]+) would (?P<n>gain|lose) (?P<a>\d+) happiness units by sitting next to (?P<t>[[:alpha:]]+)\.").multi_line(true).build().unwrap();

    let happiness: Happiness<'a> = re
        .captures_iter(input)
        .map(|m| {
            let from = m.name("f").unwrap().as_str();
            let is_negative = &m["n"] == "lose";
            let amount: isize = m["a"].parse().unwrap();
            let to = m.name("t").unwrap().as_str();
            ((from, to), if is_negative { -amount } else { amount })
        })
        .collect();

    if happiness.is_empty() {
        return Err(anyhow!("expected any input"));
    }

    let mut people = HashSet::new();
    for (f, t) in happiness.keys() {
        people.insert(*f);
        if !happiness.contains_key(&(t, f)) {
            return Err(anyhow!("happiness should be specified both ways"));
        }
    }

    Ok((people, happiness))
}

fn compute_total_happiness<'a>((people, happiness): (People<'a>, Happiness<'a>)) -> isize {
    let mut people = people.into_iter().collect::<Vec<_>>();
    let heap = Heap::new(&mut people);
    heap.map(|permutation| {
        permutation
            .iter()
            .zip(
                permutation
                    .iter()
                    .skip(1)
                    .chain(std::iter::once(&permutation[0])),
            )
            .filter_map(|(a, b)| {
                if a == b {
                    None
                } else {
                    Some(happiness.get(&(a, b)).unwrap() + happiness.get(&(b, a)).unwrap())
                }
            })
            .sum()
    })
    .max()
    .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 709);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 668);
    }
}
