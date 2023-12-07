use anyhow::anyhow;

pub const TITLE: &str = "Elves Look, Elves Say";

pub const INPUT: &str = "3113322113";

#[must_use]
/// # Panics
///
/// Panics if `input` is empty.
pub fn part1(input: &str) -> usize {
    apply_n(input, 40).unwrap()
}

#[must_use]
/// # Panics
///
/// Panics if `input` is empty.
pub fn part2(input: &str) -> usize {
    apply_n(input, 50).unwrap()
}

fn apply_n(input: &str, times: usize) -> anyhow::Result<usize> {
    let mut s = input.to_owned();
    for _ in 0..times {
        s = look_and_say(&s)?;
    }
    anyhow::Ok(s.len())
}

fn look_and_say(input: &str) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut result = String::new();
    let mut chars = input.chars();
    let mut curr = chars
        .next()
        .ok_or(anyhow!("look_and_say requires at least 1 character"))?;
    let mut count = 1;
    for next in chars {
        if next == curr {
            count += 1;
        } else {
            write!(result, "{count}{curr}")?;
            curr = next;
            count = 1;
        }
    }
    write!(result, "{count}{curr}")?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 329_356);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 4_666_278);
    }
}
