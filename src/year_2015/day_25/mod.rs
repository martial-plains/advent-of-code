use anyhow::anyhow;

pub const TITLE: &str = "Let It Snow";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> u64 {
    let (row, column) = parse_input(input).unwrap();
    let (row, column) = (row - 1, column - 1); // Make 0 based

    let idx = get_grid_index(row, column) + 1;
    let mut nr = 20_151_125_u64;
    for _ in 1..idx {
        nr = (nr * 252_533) % 33_554_393;
    }

    nr
}

const fn get_grid_index(row: u64, column: u64) -> u64 {
    (row + column + 1) * (row + column) / 2 + column
}

fn parse_input(input: &str) -> anyhow::Result<(u64, u64)> {
    const PREFIX: &str =
        "To continue, please consult the code grid in the manual.  Enter the code at row ";
    const MID: &str = ", column ";
    const SUFFIX: &str = ".";

    if input.len() < PREFIX.len() + MID.len() + SUFFIX.len() + 2 {
        return Err(anyhow!("input too short"));
    }

    if &input[0..PREFIX.len()] != PREFIX {
        return Err(anyhow!("invalid prefix"));
    }
    let input = &input[PREFIX.len()..];
    let idx = input.find(MID).ok_or_else(|| anyhow!("no mid found"))?;
    let row = input[0..idx].parse()?;
    let input = &input[idx + MID.len()..];
    let idx = input
        .find(SUFFIX)
        .ok_or_else(|| anyhow!("no suffix found"))?;
    let column = input[0..idx].parse()?;
    let input = &input[idx + SUFFIX.len()..];
    if !input.is_empty() {
        return Err(anyhow!("input not empty"));
    }

    Ok((row, column))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 19_980_801);
    }
}
