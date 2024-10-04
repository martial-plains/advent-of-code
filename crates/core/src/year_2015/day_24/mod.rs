use anyhow::anyhow;

pub const TITLE: &str = "It Hangs in the Balance";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> u64 {
    solve(input, 3).unwrap()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> u64 {
    solve(input, 4).unwrap()
}

fn difference<A, B, I>(a: A, mut b: B) -> Difference<A, B, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    I: Clone + Eq,
{
    let n = b.next();
    Difference(a, b, n)
}

struct Difference<A, B, I>(A, B, Option<I>);

impl<A, B, I> Iterator for Difference<A, B, I>
where
    A: Iterator<Item = I>,
    B: Iterator<Item = I>,
    I: Clone + Eq,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
        loop {
            let item = self.0.next()?;
            match self.2.as_ref() {
                None => return Some(item),
                Some(other) => {
                    if other == &item {
                        self.2 = self.1.next();
                    } else {
                        return Some(item);
                    }
                }
            }
        }
    }
}

fn compute_quantum_entanglement(slice: &[u64]) -> u64 {
    slice.iter().fold(1, |acc, nr| acc * (*nr))
}

fn solve(input: &str, buckets: u64) -> anyhow::Result<u64> {
    let mut nrs = input
        .lines()
        .map(|nr| Ok(nr.parse()?))
        .collect::<anyhow::Result<Vec<u64>>>()?;
    nrs.sort_unstable_by(|a, b| b.cmp(a));

    let total_weight = nrs.iter().sum::<u64>();
    if total_weight % buckets != 0 {
        return Err(anyhow!("sum of the numbers has to be a multiple of 3"));
    }

    let mut combinations = Vec::new();
    let mut combinations_other_two = Vec::new();
    let mut backtrack_stack = Vec::new();
    let mut remainder = Vec::new();

    let weight_per_bucket = total_weight / buckets;
    for number_count in 1..nrs.len().div_ceil(usize::try_from(buckets).unwrap()) {
        fn backtrack(
            stack: &mut Vec<u64>,
            combinations: &mut Vec<Vec<u64>>,
            nrs: &Vec<u64>,
            weight_per_bucket: u64,
            number_count: usize,
            start_idx: usize,
        ) {
            if stack.len() == number_count {
                if stack.iter().sum::<u64>() == weight_per_bucket {
                    combinations.push(stack.clone());
                }
                return;
            }
            for i in start_idx..nrs.len() {
                stack.push(nrs[i]);
                backtrack(
                    stack,
                    combinations,
                    nrs,
                    weight_per_bucket,
                    number_count,
                    i + 1,
                );
                stack.pop();
            }
        }

        if nrs[0..number_count].iter().sum::<u64>() < weight_per_bucket {
            continue;
        }

        combinations.clear();

        backtrack(
            &mut backtrack_stack,
            &mut combinations,
            &nrs,
            weight_per_bucket,
            number_count,
            0,
        );
        assert!(backtrack_stack.is_empty());
        if combinations.is_empty() {
            continue;
        }

        combinations.sort_unstable_by(|a, b| {
            compute_quantum_entanglement(a).cmp(&compute_quantum_entanglement(b))
        });

        for combination in &mut combinations {
            combination.sort_unstable_by(|a, b| b.cmp(a));
            remainder.clear();
            remainder.extend(difference(nrs.iter().copied(), combination.iter().copied()));

            let mut is_valid = false;
            for j in 1..(remainder.len() + usize::try_from(buckets).unwrap() - 2)
                / (usize::try_from(buckets).unwrap() - 1)
            {
                combinations_other_two.clear();
                backtrack(
                    &mut backtrack_stack,
                    &mut combinations_other_two,
                    &remainder,
                    weight_per_bucket,
                    j,
                    0,
                );
                if !combinations_other_two.is_empty() {
                    is_valid = true;
                    break;
                }
            }
            if !is_valid {
                continue;
            }
            return Ok(compute_quantum_entanglement(combination));
        }
    }

    Err(anyhow!("no solution found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 11_846_773_891);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 80_393_059);
    }
}
