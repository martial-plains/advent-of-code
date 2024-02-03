pub const TITLE: &str = "Matchsticks";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
/// * Panics if an invalid pattern is given.
#[must_use]
pub fn part1(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| line.len() >= 2)
        .map(|line| {
            let trimmed = &line[1..line.len() - 1];

            let mut state = State::Regular;
            let mut result = Vec::with_capacity(line.len() - 2);
            for byte in trimmed.bytes() {
                let new_state: State = match state {
                    State::Regular => {
                        if byte == b'\\' {
                            State::EscapeDetected
                        } else {
                            result.push(byte);
                            State::Regular
                        }
                    }
                    State::EscapeDetected => {
                        if byte == b'x' {
                            State::Hex01
                        } else if byte == b'\\' || byte == b'"' {
                            result.push(byte);
                            State::Regular
                        } else {
                            panic!("invalid character escape sequence");
                        }
                    }
                    State::Hex01 => {
                        if byte.is_ascii_digit() {
                            State::Hex02(byte - b'0')
                        } else if byte.is_ascii_lowercase() {
                            State::Hex02(byte - (b'a' - 10))
                        } else {
                            panic!("expected hex escape sequence");
                        }
                    }
                    State::Hex02(first) => {
                        let second = if byte.is_ascii_digit() {
                            byte - b'0'
                        } else if byte.is_ascii_lowercase() {
                            byte - (b'a' - 10)
                        } else {
                            panic!("expected hex escape sequence");
                        };
                        result.push((first << 4) | second);
                        State::Regular
                    }
                };
                state = new_state;
            }

            (line.to_string(), result)
        })
        .map(|(source, parsed)| source.len() - parsed.len())
        .sum()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    input
        .split('\n')
        .filter(|line| line.len() >= 2)
        .map(|line| {
            let count = line.bytes().filter(|c| *c == b'"' || *c == b'\\').count();
            count + 2
        })
        .sum()
}

enum State {
    Regular,
    EscapeDetected,
    Hex01,
    Hex02(u8),
}

const DOUBLE_QUOTE: u8 = 0x22;
const BACKSLASH: u8 = 0x5c;
const X_LOWER: u8 = 0x78;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 1350);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 2085);
    }
}
