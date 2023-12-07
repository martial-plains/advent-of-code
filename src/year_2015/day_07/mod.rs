use std::collections::HashMap;

pub const TITLE: &str = "Some Assembly Required";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn part1(input: &str) -> u16 {
    parse(input).0
}

#[must_use]
pub fn part2(input: &str) -> u16 {
    parse(input).1
}

enum Gate<'a> {
    Wire(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LeftShift(&'a str, u16),
    RightShift(&'a str, u16),
}

fn signal<'a>(
    key: &'a str,
    circuit: &HashMap<&'a str, Gate<'a>>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(result) = cache.get(key) {
        return *result;
    }

    let result = if key.chars().next().unwrap().is_ascii_digit() {
        key.parse().unwrap()
    } else {
        match circuit[key] {
            Gate::Wire(w) => signal(w, circuit, cache),
            Gate::Not(w) => !signal(w, circuit, cache),
            Gate::And(l, r) => signal(l, circuit, cache) & signal(r, circuit, cache),
            Gate::Or(l, r) => signal(l, circuit, cache) | signal(r, circuit, cache),
            Gate::LeftShift(w, n) => signal(w, circuit, cache) << n,
            Gate::RightShift(w, n) => signal(w, circuit, cache) >> n,
        }
    };

    cache.insert(key, result);
    result
}

/// # Panics
/// * Panics if `input` is empty
#[must_use]
fn parse(input: &str) -> (u16, u16) {
    let mut tokens = input.split_ascii_whitespace();
    let mut circuit = HashMap::new();

    while let (Some(first), Some(second)) = (tokens.next(), tokens.next()) {
        let gate = if first == "NOT" {
            let _third = tokens.next().unwrap();
            Gate::Not(second)
        } else if second == "->" {
            Gate::Wire(first)
        } else {
            let third = tokens.next().unwrap();
            let _fourth = tokens.next().unwrap();

            match second {
                "AND" => Gate::And(first, third),
                "OR" => Gate::Or(first, third),
                "LSHIFT" => Gate::LeftShift(first, third.parse().unwrap()),
                "RSHIFT" => Gate::RightShift(first, third.parse().unwrap()),
                _ => unreachable!(),
            }
        };

        let wire = tokens.next().unwrap();
        circuit.insert(wire, gate);
    }

    let mut cache = HashMap::new();
    let result1 = signal("a", &circuit, &mut cache);

    cache.clear();
    cache.insert("b", result1);
    let result2 = signal("a", &circuit, &mut cache);

    (result1, result2)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 46065);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 14134);
    }
}
