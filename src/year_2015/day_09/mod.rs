use std::collections::HashMap;

use permutohedron::Heap;

pub const TITLE: &str = "All in a Single Night";

/// # Panics
/// Panics if `input` is empty
#[must_use]
pub fn part1(input: &str) -> usize {
    let data: (Place, Routes) = transform(input);
    *route_lengths(data).iter().min().unwrap()
}

/// # Panics
/// Panics if `input` is empty
#[must_use]
pub fn part2(input: &str) -> usize {
    let data: (Place, Routes) = transform(input);
    *route_lengths(data).iter().max().unwrap()
}

type Edge = (String, String);
type Path = (Vec<String>, usize);

type Place = usize;

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
struct Connection {
    a: Place,
    b: Place,
}

impl Connection {
    fn new(a: Place, b: Place) -> Connection {
        if a <= b {
            Connection { a, b }
        } else {
            Connection { a: b, b: a }
        }
    }
}

type Routes = HashMap<Connection, usize>;

fn transform(input: &str) -> (Place, Routes) {
    let mut place_names = HashMap::new();
    let results = input
        .split('\n')
        .filter_map(|line| {
            let to_idx = line.find(" to ")?;
            let eq_idx = line.find(" = ")?;
            let fr = &line[0..to_idx];
            let to = &line[to_idx + 4..eq_idx];

            let l = place_names.len() as Place;
            let fr: Place = *place_names.entry(fr).or_insert(l);
            let l = place_names.len() as Place;
            let to: Place = *place_names.entry(to).or_insert(l);

            let dist: usize = line[eq_idx + 3..].parse().ok()?;
            Some((Connection::new(fr, to), dist))
        })
        .collect();
    (place_names.len() as Place, results)
}

fn route_lengths((place_count, routes): (Place, Routes)) -> Vec<usize> {
    let mut data = (0..place_count).collect::<Vec<_>>();
    let heap = Heap::new(&mut data);
    heap.map(|permutation| {
        permutation
            .iter()
            .take(permutation.len() - 1)
            .zip(permutation.iter().skip(1))
            .map(|(&from, &to)| {
                routes
                    .get(&Connection::new(from, to))
                    .expect("missing route")
            })
            .sum()
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 117);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 909);
    }
}
