use std::collections::HashMap;

use regex::Regex;

pub const TITLE: &str = "Reindeer Olympics";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is empty.
#[must_use]
pub fn part1(input: &str) -> usize {
    parse_reindeers(input)
        .unwrap()
        .into_values()
        .map(|mobility| compute_at_time(&mobility, 2503))
        .max()
        .unwrap()
}

/// # Panics
///
/// Panics if input is empty.
#[must_use]
pub fn part2(input: &str, time: usize) -> usize {
    let mut states = parse_reindeers(input)
        .unwrap()
        .into_values()
        .map(|mobility| State {
            mobility,
            points: 0,
            distance: 0,
        })
        .collect::<Vec<_>>();

    for current_time in 1..=time {
        // Update all distances
        for state in &mut states {
            state.distance = compute_at_time(&state.mobility, current_time);
        }

        // Award points
        let highest = states.iter().map(|state| state.distance).max().unwrap();
        for state in &mut states {
            if state.distance == highest {
                state.points += 1;
            }
        }
    }

    states.into_iter().map(|x| x.points).max().unwrap()
}

type Reindeers<'a> = HashMap<&'a str, Mobility>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mobility {
    speed: usize,
    duration: usize,
    rest_time: usize,
}

#[derive(Clone)]
struct State {
    mobility: Mobility,
    points: usize,
    distance: usize,
}

fn parse_reindeers(input: &str) -> anyhow::Result<Reindeers<'_>> {
    let re = Regex::new(r"(?m)^(?P<n>[[:alpha:]]+) can fly (?P<s>\d+) km/s for (?P<d>\d+) seconds, but then must rest for (?P<r>\d+) seconds\.$").unwrap();

    re.captures_iter(input)
        .map(|m| {
            let name = m.name("n").unwrap().as_str();
            let speed = m["s"].parse()?;
            let duration = m["d"].parse()?;
            let rest_time = m["r"].parse()?;
            Ok((
                name,
                Mobility {
                    speed,
                    duration,
                    rest_time,
                },
            ))
        })
        .collect()
}

fn compute_at_time(mobility: &Mobility, time: usize) -> usize {
    let cycle_time = mobility.duration + mobility.rest_time;
    let cycle_count = time / cycle_time;
    let remaining_time = time - cycle_count * cycle_time;
    let cycle_distance = cycle_count * (mobility.speed * mobility.duration);
    if remaining_time >= mobility.duration {
        cycle_distance + mobility.speed * mobility.duration
    } else {
        cycle_distance + mobility.speed * remaining_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 2640);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT, 2503), 1102);
    }
}
