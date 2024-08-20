use anyhow::anyhow;

pub const TITLE: &str = "Corporate Policy";

pub const INPUT: &str = "hepxcrrq";

/// # Panics
///
/// Panics if `input` is invalid.
#[must_use]
pub fn part1(input: &str) -> String {
    let mut pw = validate_input(input).unwrap();
    cycle_password_until_valid(&mut pw);
    unsafe { String::from_utf8_unchecked(pw) }
}

/// # Panics
///
/// Panics if `input` is invalid.
#[must_use]
pub fn part2(input: &str) -> String {
    let mut pw = validate_input(input).unwrap();
    cycle_password_until_valid(&mut pw);
    cycle_password_until_valid(&mut pw);
    unsafe { String::from_utf8_unchecked(pw) }
}

fn rule_abc(pw: &[u8]) -> bool {
    if pw.len() < 3 {
        return false;
    }
    for i in 0..pw.len() - 3 {
        if pw[i] + 1 == pw[i + 1] && pw[i] + 2 == pw[i + 2] {
            return true;
        }
    }
    false
}

fn rule_iol(pw: &[u8]) -> bool {
    pw.iter().all(|c| !matches!(*c, b'i' | b'o' | b'l'))
}

fn rule_two_pairs(pw: &[u8]) -> bool {
    if pw.len() < 4 {
        return false;
    }
    for i in 0..pw.len() - 3 {
        if pw[i] == pw[i + 1] {
            for j in i + 2..pw.len() - 1 {
                if pw[j] == pw[j + 1] && pw[i] != pw[j] {
                    return true;
                }
            }
            return false;
        }
    }
    false
}

fn cycle_password(pw: &mut [u8]) {
    fn cycle_char(idx: usize, pw: &mut [u8]) {
        if pw[idx] == b'z' {
            pw[idx] = b'a';
            cycle_char(idx - 1, pw);
        } else {
            pw[idx] += 1;
        }
    }
    cycle_char(pw.len() - 1, pw);
}

fn cycle_password_until_valid(pw: &mut [u8]) {
    cycle_password(pw);
    while !rule_abc(pw) || !rule_iol(pw) || !rule_two_pairs(pw) {
        cycle_password(pw);
    }
}

fn validate_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let pw = input.as_bytes().to_vec();
    for c in &pw {
        if !(&b'a'..=&b'z').contains(&c) {
            return Err(anyhow!("invalid input in puzzle"));
        }
    }
    Ok(pw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), "hepxxyzz");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "heqaabcc");
    }
}
