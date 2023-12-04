use algorithms::higher_order_functions::Reductions;

pub const TITLE: &str = "Not Quite Lisp";

pub fn part1(input: &str) -> isize {
    input
        .lines()
        .next()
        .map(|s| s.chars().map(convert))
        .unwrap()
        .sum()
}

pub fn part2(input: &str) -> isize {
    (input
        .lines()
        .next()
        .map(|s| s.chars().map(convert))
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn test_part1_example1() {
        let result = part1("(())");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let result = part1("()()");
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example3() {
        let result = part1("(((");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_example4() {
        let result = part1("(()(()(");
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part1_example5() {
        let result = part1("))(");
        assert_eq!(result, -1);
    }

    #[test]
    fn test_part1_example6() {
        let result = part1(")())())");
        assert_eq!(result, -3);
    }

    #[test]
    fn test_part1_puzzle() {
        let result = part1(INPUT);
        assert_eq!(result, 138);
    }

    #[test]
    fn test_part2_example1() {
        let result = part2(")");
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part2_example2() {
        let result = part2("()())");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2_puzzle() {
        let result = part2(INPUT);
        assert_eq!(result, 1771);
    }
}
