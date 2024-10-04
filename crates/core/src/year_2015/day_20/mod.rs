use std::sync::LazyLock;

pub const TITLE: &str = "Infinite Elves and Infinite Houses";

pub const INPUT: &str = "36000000";

/// # Panics
///
/// Panics if `input` is invalid
#[must_use]
pub fn part1(input: &str) -> usize {
    let input: u64 = input.parse().unwrap();
    for house_number in 1.. {
        // The amount of presents is the sum of all the positive integer divisors.
        // Multiplied by 10.
        if fold_divisors(house_number, |acc, div| acc + div) * 10 >= input {
            return house_number;
        }
    }
    unreachable!()
}

/// # Panics
///
/// Panics if `input` is invalid
#[must_use]
pub fn part2(input: &str) -> u64 {
    let input: u64 = input.parse().unwrap();

    for house_number in 1u64.. {
        // The amount of presents is the sum of all the positive integer divisors
        // where those divisors (the elves) haven't previously visited 50 house
        // numbers. Multiplied by 11.
        if fold_divisors(usize::try_from(house_number).unwrap(), |acc, div| {
            if house_number.div_ceil(div) <= 50 {
                acc + div
            } else {
                acc
            }
        }) * 11
            >= input
        {
            return house_number;
        }
    }

    unreachable!()
}

static SIEVE: LazyLock<primal::Sieve> = LazyLock::new(|| primal::Sieve::new(100_000_000));

/// Iterates over all the positive integer divisors of a number (by doing
/// prime factorization and combinations). It then applies function f over
/// an accumulated value (starting at 0) for each divisor.
fn fold_divisors<F>(house_number: usize, f: F) -> u64
where
    F: Fn(u64, u64) -> u64,
{
    fn accumulate<F>(
        sum: &mut u64,
        factors: &Vec<(usize, usize)>,
        factor_idx: usize,
        multiplier: u64,
        f: &F,
    ) where
        F: Fn(u64, u64) -> u64,
    {
        if factor_idx >= factors.len() {
            *sum = f(*sum, multiplier);
            return;
        }
        let (factor, exponent) = factors[factor_idx];
        let factor = factor as u64;
        let mut extra_multiplier = 1;
        for _ in 0..=exponent {
            accumulate(
                sum,
                factors,
                factor_idx + 1,
                multiplier * extra_multiplier,
                f,
            );
            extra_multiplier *= factor;
        }
    }

    let factors = SIEVE.factor(house_number).unwrap();

    let mut sum = 0;
    accumulate(&mut sum, &factors, 0, 1, &f);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 831_600);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 884_520);
    }
}
