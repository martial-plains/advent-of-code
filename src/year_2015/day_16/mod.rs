use std::collections::HashMap;

use anyhow::anyhow;
use regex::Regex;

pub const TITLE: &str = "Aunt Sue";

pub const INPUT: &str = include_str!("input.txt");

/// # Errors
///
/// This function will return an error if `input` is empty
pub fn part1(input: &str) -> anyhow::Result<usize> {
    let mut sues: Vec<Sue> = input
        .lines()
        .map(parse_line)
        .collect::<anyhow::Result<_>>()?;
    sues.retain(|sue| {
        for &(key, value) in &KNOWN_PROPS {
            if let Some(&sue_value) = sue.properties.get(key) {
                if sue_value != value {
                    return false;
                }
            }
        }
        true
    });

    if sues.len() != 1 {
        return Err(anyhow!("no sues matching the properties found"));
    }

    Ok(sues[0].index)
}

/// # Errors
///
/// This function will return an error if `input` is empty
pub fn part2(input: &str) -> anyhow::Result<usize> {
    let mut sues: Vec<Sue> = input
        .lines()
        .map(parse_line)
        .collect::<anyhow::Result<_>>()?;
    sues.retain(|sue| {
        for &(key, value) in &KNOWN_PROPS {
            if let Some(&sue_value) = sue.properties.get(key) {
                if !match key {
                    "cats" | "trees" => sue_value > value,
                    "pomeranians" | "goldfish" => sue_value < value,
                    _ => sue_value == value,
                } {
                    return false;
                }
            }
        }
        true
    });

    if sues.len() != 1 {
        return Err(anyhow!("no sues matching the properties found"));
    }

    Ok(sues[0].index)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Sue {
    index: usize,
    properties: HashMap<String, usize>,
}

fn parse_line(line: &str) -> anyhow::Result<Sue> {
    let main = Regex::new(r"^Sue (?P<index>\d+): (?P<props>.+)$").unwrap();
    let prop = Regex::new(r"(?P<key>[[:alpha:]]+): (?P<value>\d+)(, )?").unwrap();

    let captures = main.captures(line).ok_or(anyhow!("invalid format"))?;
    let index = captures["index"].parse().unwrap();
    let props = captures.name("props").unwrap().as_str();
    let properties = prop
        .captures_iter(props)
        .map(|capture| {
            Ok((
                capture.name("key").unwrap().as_str().to_owned(),
                capture["value"].parse()?,
            ))
        })
        .collect::<anyhow::Result<HashMap<String, usize>>>()?;

    anyhow::Ok(Sue { index, properties })
}

const KNOWN_PROPS: [(&str, usize); 10] = [
    ("children", 3),
    ("cats", 7),
    ("samoyeds", 2),
    ("pomeranians", 3),
    ("akitas", 0),
    ("vizslas", 0),
    ("goldfish", 5),
    ("trees", 3),
    ("cars", 2),
    ("perfumes", 1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT).unwrap(), 373);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT).unwrap(), 260);
    }
}
