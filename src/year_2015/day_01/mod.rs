use algorithms::higher_order_functions::Reductions;

struct Day01;

impl Day01 {
    const TITLE: &'static str = "Not Quite Lisp";

    pub fn part1(input: &str) -> isize {
        input
            .lines()
            .nth(0)
            .map(|s| s.chars().map(Self::convert))
            .unwrap()
            .sum()
    }

    pub fn part2(input: &str) -> isize {
        (input
            .lines()
            .nth(0)
            .map(|s| s.chars().map(Self::convert))
            .unwrap()
            .reductions(0, std::ops::Add::add)
            .enumerate()
            .find(|(_, floor)| *floor == -1)
            .unwrap()
            .0)
            .try_into()
            .unwrap()
    }

    fn convert(character: char) -> isize {
        match character {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Day01;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example1() {
        let result = Day01::part1("(())");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let result = Day01::part1("()()");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example3() {
        let result = Day01::part1("(((");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_example4() {
        let result = Day01::part1("(()(()(");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_example5() {
        let result = Day01::part1("))(");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_part1_example6() {
        let result = Day01::part1(")())())");
        assert_eq!(result, -3);
    }

    #[test]
    fn test_part1_puzzle() {
        let result = Day01::part1(INPUT);
        assert_eq!(result, 138);
    }

    #[test]
    fn test_part2_example1() {
        let result = Day01::part2(")");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_example2() {
        let result = Day01::part2("()())");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2_puzzle() {
        let result = Day01::part2(INPUT);
        assert_eq!(result, 1771);
    }
}
