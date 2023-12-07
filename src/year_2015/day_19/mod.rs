use std::collections::{HashMap, HashSet};

use anyhow::anyhow;

pub const TITLE: &str = "Medicine for Rudolph";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is invalid
#[must_use]
pub fn part1(input: &str) -> usize {
    let (rules, molecule) = parse_input(input).unwrap();

    let mut combinations = HashSet::new();

    for (from, to) in rules {
        for i in 0..=(molecule.len() - from.len()) {
            if &molecule[i..i + from.len()] == from {
                let mut substitution =
                    String::with_capacity(molecule.len() - from.len() + to.len());
                substitution.push_str(&molecule[0..i]);
                substitution.push_str(to);
                substitution.push_str(&molecule[i + from.len()..]);
                combinations.insert(substitution);
            }
        }
    }

    combinations.len()
}

/// # Panics
///
/// Panics if input is invalid
#[must_use]
pub fn part2(input: &str) -> usize {
    let (rules, molecule) = parse_input(input).unwrap();

    let mut atom_map = HashMap::new();
    atom_map.insert("e", 0);
    let mut rules = rules
        .into_iter()
        .map(|(from, into)| {
            let from = string_to_molecule(from, &mut atom_map)?;
            if from.len() != 1 {
                return Err(anyhow!("mapping can only be from one atom to a molecule",));
            }
            let into = string_to_molecule(into, &mut atom_map)?;
            Ok((into, from[0]))
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .unwrap();
    rules.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
    let molecule = string_to_molecule(molecule, &mut atom_map).unwrap();

    let mut astar = crate::shared::astar::AStar::new();
    astar
        .solve(
            molecule,
            |molecule| {
                let mut candidates = arrayvec::ArrayVec::<Molecule, 32>::new();
                for (into, from) in &rules {
                    let Some(idx) = find_substr(molecule.as_slice(), into) else {
                        continue;
                    };

                    let mut new_molecule = Molecule::with_capacity(molecule.len() + 1 - into.len());

                    new_molecule.extend_from_slice(&molecule[0..idx]);
                    new_molecule.push(*from);
                    new_molecule.extend_from_slice(&molecule[idx + into.len()..]);
                    candidates.push(new_molecule);
                }
                candidates.into_iter().map(|a| (a, 1))
            },
            |molecule| molecule.len().max(1) - 1,
            |molecule| molecule.len() == 1 && molecule[0] == 0,
        )
        .map(|path| path.last().unwrap().1)
        .ok_or(anyhow!("no solution found for input"))
        .unwrap()
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<(&str, &str)>, &str)> {
    let mut iter = input.lines();
    let mut rules = Vec::new();
    loop {
        let line = iter.next().ok_or(anyhow!("unexpected end of input"))?;
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().ok_or(anyhow!("expected input atom"))?;
        let into = parts.next().ok_or(anyhow!("expected output atom"))?;
        if parts.next().is_some() {
            return Err(anyhow!("expected end of line"));
        }
        rules.push((from, into));
    }
    let molecule = iter.next().ok_or(anyhow!("unexpected end of input"))?;
    if iter.next().is_some() {
        return Err(anyhow!("expected end of input"));
    }
    Ok((rules, molecule))
}

type Atom = u8;
type Molecule = Vec<Atom>;

/// Turns a string into a molecule, which is a more efficient in-memory representation
/// The maximum amount of unique atoms (including the electron) is 255.
fn string_to_molecule<'s>(
    s: &'s str,
    atom_map: &mut HashMap<&'s str, Atom>,
) -> anyhow::Result<Molecule> {
    let mut molecule = Molecule::new();
    let mut idx = 0;
    let mut bytes = s.bytes().peekable();
    while let Some(byte) = bytes.next() {
        if !byte.is_ascii_alphabetic() {
            return Err(anyhow!("non-alphabetic character in molecules"));
        }
        let atom_length = if byte == b'e' {
            if s != "e" {
                return Err(anyhow!("electron isn't allowed within a complex molecule",));
            }
            1
        } else {
            if !byte.is_ascii_uppercase() {
                return Err(anyhow!(
                    "expected atom to start with an uppercase character",
                ));
            }
            let mut len = 1;
            while let Some(next_char) = bytes.peek().copied() {
                if next_char.is_ascii_lowercase() {
                    len += 1;
                    bytes.next();
                    continue;
                }
                break;
            }
            len
        };
        let slice = &s[idx..idx + atom_length];
        idx += atom_length;
        let atom_map_len = atom_map.len();
        if atom_map_len == 256 {
            return Err(anyhow!("too many unique atoms"));
        }
        let atom = *atom_map
            .entry(slice)
            .or_insert(Atom::try_from(atom_map_len).unwrap());
        molecule.push(atom);
    }
    Ok(molecule)
}

fn find_substr(target: &[u8], substr: &[u8]) -> Option<usize> {
    target
        .windows(substr.len())
        .position(|window| window == substr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 518);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 200);
    }
}
