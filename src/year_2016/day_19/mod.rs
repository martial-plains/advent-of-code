use std::collections::VecDeque;

pub const TITLE: &str = "An Elephant Named Joseph";

pub const INPUT: &str = "3005290";

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    let amount = input.parse().unwrap();
    let mut elves = (0..amount).map(|v| v + 1).collect::<VecDeque<_>>();

    while elves.len() > 1 {
        // elf taking presents is added to the back of the list
        elves.rotate_left(1);
        // elf which has their presents taken is removed from the list
        elves.pop_front();
    }

    *elves.front().unwrap()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let amount: usize = input.parse().unwrap();
    // do not track the middle element, but divide the elements in two different lists
    let mut left = (0..amount / 2).map(|v| v + 1).collect::<VecDeque<_>>();
    let mut right = (amount / 2..amount).map(|v| v + 1).collect::<VecDeque<_>>();

    while left.len() + right.len() > 1 {
        let current = left.pop_front().unwrap();

        if left.len() == right.len() {
            left.pop_back();
        } else {
            right.pop_front();
        }

        right.push_back(current);
        // move the first right value to the back of the left values, as we are moving to the right
        left.push_back(right.pop_front().unwrap());
    }

    *left.front().unwrap()
}

fn next_index(start: usize, array: &[i32]) -> Option<usize> {
    let mut candidate = (start + 1) % array.len();
    for _ in 0..array.len() - 1 {
        if array[candidate] != 0 {
            return Some(candidate);
        }

        candidate = (candidate + 1) % array.len();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 1_816_277);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 1_410_967);
    }
}
