pub const TITLE: &str = "Squares With Three Sides";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part1(input: &str) -> usize {
    count(
        input
            .lines()
            .flat_map(|s| s.split(' '))
            .flat_map(str::parse),
    )
}

/// # Panics
/// Panics if input is empty
#[must_use]
pub fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .flat_map(|s| s.split(' '))
        .flat_map(str::parse)
        .collect::<Vec<_>>();
    let first = count(input.iter().copied().step_by(3));
    let second = count(input.iter().copied().skip(1).step_by(3));
    let third = count(input.iter().copied().skip(2).step_by(3));
    first + second + third
}

#[inline]
fn count(iter: impl Iterator<Item = u32>) -> usize {
    iter.array_chunks::<3>()
        .filter(|&[a, b, c]| a + b > c && a + c > b && b + c > a)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 993);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 1849);
    }
}
