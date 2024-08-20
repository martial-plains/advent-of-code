use itertools::Itertools;
use serde_json::Value;

pub const TITLE: &str = "JSAbacusFramework.io";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
pub fn part1(input: &str) -> isize {
    extract_numbers(input).sum()
}

/// # Panics
///
/// Panics if unable to convert input to JSON
#[must_use]
pub fn part2(input: &str) -> isize {
    let mut json = serde_json::from_str(input).unwrap();

    prune_red(&mut json);
    part1(&serde_json::to_string(&json).unwrap())
}

fn prune_red(json: &mut Value) {
    match json {
        Value::Object(map) => {
            for (_, value) in map.iter_mut() {
                if let Some(s) = value.as_str() {
                    if s == "red" {
                        map.clear();
                        return;
                    }
                }
            }
            for (_, value) in map.iter_mut() {
                prune_red(value);
            }
        }
        Value::Array(array) => {
            for value in array.iter_mut() {
                prune_red(value);
            }
        }
        _ => {}
    }
}

fn extract_numbers(input: &str) -> impl Iterator<Item = isize> + '_ {
    input.chars().peekable().batching(|it| loop {
        match it.next() {
            Some(x) if x == '-' || x.is_ascii_digit() => {
                let is_negative = if x == '-' {
                    if let Some(&x) = it.peek() {
                        if !x.is_ascii_digit() {
                            continue;
                        }
                    }
                    true
                } else {
                    false
                };

                let mut nr = if is_negative {
                    0
                } else {
                    x as isize - '0' as isize
                };
                for x in it.by_ref() {
                    if !x.is_ascii_digit() {
                        break;
                    }
                    nr = nr * 10 + (x as isize) - ('0' as isize);
                }
                return Some(if is_negative { -nr } else { nr });
            }
            Some(_) => {}
            None => break None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 111_754);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 65402);
    }
}
