use std::cmp::min;

struct Day02;

impl Day02 {
    const TITLE: &'static str = "I Was Told There Would Be No Math";

    pub fn part1(input: &str) -> isize {
        let mapping = |v: Vec<isize>| {
            2 * (v[0] * v[1] + v[0] * v[2] + v[1] * v[2])
                + min(v[0] * v[1], min(v[0] * v[2], v[1] * v[2]))
        };
        input
            .lines()
            .map(|line| {
                line.split_terminator('x')
                    .flat_map(|s| s.chars().map(|c| c as isize).collect::<Vec<isize>>())
                    .collect::<Vec<isize>>()
            })
            .map(mapping)
            .sum()
    }

    pub fn part2(input: &str) -> isize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Day02;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example1() {
        let result = Day02::part1("2x3x4");
        assert_eq!(result, 58);
    }

    #[test]
    fn test_part1_example2() {
        let result = Day02::part1("1x1x10");
        assert_eq!(result, 43);
    }

    #[test]
    fn test_part1_puzzle() {
        let result = Day02::part1(INPUT);
        assert_eq!(result, 1_598_415);
    }

    #[test]
    fn test_part2_example1() {
        let result = Day02::part2("2x3x4");
        assert_eq!(result, 34);
    }

    #[test]
    fn test_part2_example2() {
        let result = Day02::part2("1x1x10");
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2_puzzle() {
        let result = Day02::part2(INPUT);
        assert_eq!(result, 3_812_909);
    }
}
