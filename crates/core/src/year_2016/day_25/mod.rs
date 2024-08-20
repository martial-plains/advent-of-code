pub const TITLE: &str = "Clock Signal";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let first: u32 = unsigned(lines[1]);
    let second: u32 = unsigned(lines[2]);
    let offset = first * second;
    let mut result = 0;

    while result < offset {
        result = (result << 2) | 2;
    }

    result - offset
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub const fn part2(_: &str) -> usize {
    0
}

fn unsigned(value: &str) -> u32 {
    let mut bytes = value.bytes();
    let mut n = loop {
        let byte = bytes.next().unwrap();
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            break u32::from(digit);
        }
    };

    loop {
        let Some(byte) = bytes.next() else {
            break n;
        };
        let digit = byte.wrapping_sub(b'0');

        if digit < 10 {
            n = 10 * n + u32::from(digit);
        } else {
            break n;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 180);
    }
}
