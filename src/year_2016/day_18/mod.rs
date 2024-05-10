use itertools::Itertools;

pub const TITLE: &str = "Like a Rogue";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    expand_map(&input.chars().map(|item| item == TRAP).collect_vec(), 40)
        .iter()
        .flatten()
        .filter(|flag| !*flag)
        .count()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    expand_map(
        &input.chars().map(|item| item == TRAP).collect_vec(),
        400_000,
    )
    .iter()
    .flatten()
    .filter(|flag| !*flag)
    .count()
}

const TRAP: char = '^';

const fn is_trap(position: usize, previous_row: &[bool]) -> bool {
    if position >= previous_row.len() {
        return false;
    }
    previous_row[position]
}

const fn tile_for_row(position: usize, previous_row: &[bool]) -> bool {
    let left = is_trap(position.wrapping_sub(1), previous_row);
    let center = is_trap(position, previous_row);
    let right = is_trap(position + 1, previous_row);

    !left && right || left && !right
}

fn expand_map(first_row: &[bool], rows: usize) -> Vec<Vec<bool>> {
    let count = first_row.len();
    let mut map = Vec::new();
    map.push(first_row.to_vec());

    for r in 1..rows {
        let previous_row = &map[r - 1];
        let new_row: Vec<bool> = (0..count).map(|i| tile_for_row(i, previous_row)).collect();
        map.push(new_row);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 1951);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 20_002_936);
    }
}
