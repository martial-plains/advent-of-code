pub const TITLE: &str = "Firewall Rules";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    let ranges = parse(input);
    find_lowest_matching(&ranges)
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let ranges = parse(input);
    count_allowed(&ranges)
}

fn find_lowest_matching(ranges: &[(usize, usize)]) -> usize {
    let mut last_high = 0;

    for &(low, high) in ranges {
        if last_high < low {
            return last_high;
        }
        last_high = last_high.max(high + 1);
    }

    last_high
}

fn count_allowed(ranges: &[(usize, usize)]) -> usize {
    let mut allowed_count = 0;
    let mut last_high = 0;

    for &(low, high) in ranges {
        if last_high < low {
            allowed_count += low - last_high;
        }

        last_high = if high == usize::MAX {
            usize::MAX
        } else {
            last_high.max(high + 1)
        };
    }

    allowed_count
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            (low.parse().unwrap(), high.parse().unwrap())
        })
        .collect();
    ranges.sort_unstable();

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 17_348_574);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 104);
    }
}
