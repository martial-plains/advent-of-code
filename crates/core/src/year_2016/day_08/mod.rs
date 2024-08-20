use itertools::Itertools;

pub const TITLE: &str = "Two-Factor Authentication";

pub const INPUT: &str = include_str!("input.txt");

#[must_use]
/// # Panics
/// Panics if input is empty
pub fn part1(input: &str) -> usize {
    let mut display = [[false; 50]; 6];

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "rect" {
            let dims: Vec<usize> = words[1].split('x').map(|s| s.parse().unwrap()).collect();
            let width = dims[0];
            let height = dims[1];
            for display in display.iter_mut().take(height) {
                for display in display.iter_mut().take(width) {
                    *display = true;
                }
            }
        } else if words[0] == "rotate" {
            let row = words[1] == "row";
            let pos: usize = words[2].split('=').nth(1).unwrap().parse().unwrap();
            let by: usize = words[4].parse().unwrap();
            for _ in 0..by {
                if row {
                    let tmp = display[pos][49];
                    for i in (1..50).rev() {
                        display[pos][i] = display[pos][i - 1];
                    }
                    display[pos][0] = tmp;
                } else {
                    let tmp = display[5][pos];
                    for i in (1..6).rev() {
                        display[i][pos] = display[i - 1][pos];
                    }
                    display[0][pos] = tmp;
                }
            }
        }
    }
    display.iter().flatten().filter(|&b| *b).count()
}

#[must_use]
/// # Panics
/// Panics if input is empty
pub fn part2(input: &str) -> String {
    let mut display = [[false; 50]; 6];

    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if words[0] == "rect" {
            let dims: Vec<usize> = words[1].split('x').map(|s| s.parse().unwrap()).collect();
            let width = dims[0];
            let height = dims[1];
            for display in display.iter_mut().take(height) {
                for display in display.iter_mut().take(width) {
                    *display = true;
                }
            }
        } else if words[0] == "rotate" {
            let row = words[1] == "row";
            let pos: usize = words[2].split('=').nth(1).unwrap().parse().unwrap();
            let by: usize = words[4].parse().unwrap();
            for _ in 0..by {
                if row {
                    let tmp = display[pos][49];
                    for i in (1..50).rev() {
                        display[pos][i] = display[pos][i - 1];
                    }
                    display[pos][0] = tmp;
                } else {
                    let tmp = display[5][pos];
                    for i in (1..6).rev() {
                        display[i][pos] = display[i - 1][pos];
                    }
                    display[0][pos] = tmp;
                }
            }
        }
    }

    let mut result = display
        .iter()
        .map(|bools| {
            bools
                .iter()
                .map(|flag| if *flag { "#" } else { "." })
                .collect::<String>()
        })
        .join("\n");

    result.insert(0, '\n');

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 123);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "\n.##..####.###..#..#.###..####.###....##.###...###.\n#..#.#....#..#.#..#.#..#....#.#..#....#.#..#.#....\n#..#.###..###..#..#.#..#...#..###.....#.#..#.#....\n####.#....#..#.#..#.###...#...#..#....#.###...##..\n#..#.#....#..#.#..#.#....#....#..#.#..#.#.......#.\n#..#.#....###...##..#....####.###...##..#....###..");
    }
}
