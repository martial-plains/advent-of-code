use itertools::Itertools;

pub const TITLE: &str = "Dragon Checksum";

pub const INPUT: &str = "10001001100000001";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    let input = input
        .chars()
        .map(|character| character == '1')
        .collect_vec();

    input
        .dragon_curve(272)
        .checksum()
        .iter()
        .map(|flag| if *flag { '1' } else { '0' })
        .join("")
        .parse()
        .unwrap()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let input = input
        .chars()
        .map(|character| character == '1')
        .collect_vec();

    input
        .dragon_curve(35_651_584)
        .checksum()
        .iter()
        .map(|flag| if *flag { '1' } else { '0' })
        .join("")
        .parse()
        .unwrap()
}

trait DragonCurve {
    fn dragon_curve(&self, length: usize) -> Vec<bool>;
    fn checksum(&self) -> Vec<bool>;
}

impl DragonCurve for Vec<bool> {
    fn dragon_curve(&self, length: usize) -> Vec<bool> {
        let mut dragon_curve = self.clone();
        dragon_curve.push(false);
        dragon_curve.extend(self.iter().rev().map(|&b| !b));

        if dragon_curve.len() < length {
            return dragon_curve.dragon_curve(length);
        }

        dragon_curve.truncate(length);
        dragon_curve
    }

    fn checksum(&self) -> Vec<bool> {
        let mut checksum = Self::with_capacity(self.len() / 2);
        for idx in (0..self.len()).step_by(2) {
            checksum.push(self[idx] == self[idx + 1]);
        }

        if checksum.len() % 2 == 0 {
            return checksum.checksum();
        }

        checksum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 10_101_001_010_100_001);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 10_100_001_110_101_001);
    }
}
