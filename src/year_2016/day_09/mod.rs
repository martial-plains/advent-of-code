use anyhow::anyhow;

pub const TITLE: &str = "Explosives in Cyberspace";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
/// # Panics
/// Panics if input is empty
pub fn part1(input: &str) -> u64 {
    uncompressed_size(input.trim().as_bytes(), false).unwrap()
}

#[must_use]
/// # Panics
/// Panics if input is empty
pub fn part2(input: &str) -> u64 {
    uncompressed_size(input.trim().as_bytes(), true).unwrap()
}

fn uncompressed_size(text: &[u8], recursive: bool) -> anyhow::Result<u64> {
    let error_mapper_uf8 = |_| anyhow!("Invalid input");
    let error_mapper_parse = |_| anyhow!("Invalid input");
    let mut start_parenthesis_idx = None;
    let mut uncompressed_len = 0;

    let mut i = 0;
    while i < text.len() {
        let c = text[i];
        if c == b'(' {
            start_parenthesis_idx = Some(i);
        } else if c == b')' {
            if let Some(from) = start_parenthesis_idx {
                let inside_parenthesis = &text[from + 1..i];
                let parts = inside_parenthesis
                    .split(|&c| c == b'x')
                    .collect::<Vec<&[u8]>>();
                if parts.len() != 2 {
                    return Err(anyhow!("Invalid input"));
                }
                let chars_to_take = std::str::from_utf8(parts[0])
                    .map_err(error_mapper_uf8)?
                    .parse::<u64>()
                    .map_err(error_mapper_parse)?;
                let repetitions = std::str::from_utf8(parts[1])
                    .map_err(error_mapper_uf8)?
                    .parse::<u64>()
                    .map_err(error_mapper_parse)?;
                uncompressed_len += repetitions
                    * if recursive {
                        uncompressed_size(
                            &text[i + 1..i + 1 + usize::try_from(chars_to_take).unwrap()],
                            true,
                        )?
                    } else {
                        chars_to_take
                    };
                i += usize::try_from(chars_to_take).unwrap();
                start_parenthesis_idx = None;
            }
        } else if start_parenthesis_idx.is_none() {
            uncompressed_len += 1;
        }
        i += 1;
    }

    Ok(uncompressed_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 74532);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 11_558_231_665);
    }
}
