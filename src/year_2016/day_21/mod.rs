use std::string::ToString;

pub const TITLE: &str = "Scrambled Letters and Hash";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> String {
    let mut password = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
    scramble(input, &mut password).unwrap();
    password.iter().map(|&b| b as char).collect::<String>()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> String {
    let mut password = [b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h'];
    let desired = [b'f', b'b', b'g', b'd', b'c', b'e', b'a', b'h'];
    let mut answer = None;
    all_permutations(&mut password, &mut |permutation| {
        let mut copy = [0, 0, 0, 0, 0, 0, 0, 0];
        copy.copy_from_slice(permutation);
        scramble(input, &mut copy).unwrap();
        if copy == desired {
            answer = Some(permutation.iter().map(|&b| b as char).collect::<String>());
        }
        Ok(())
    })
    .unwrap();

    answer
        .ok_or_else(|| "No solution found".to_string())
        .unwrap()
}

fn scramble(input: &str, password: &mut [u8]) -> Result<(), String> {
    let error_mapper = |_| "Invalid input";
    for line in input.lines() {
        let words = line.split(' ').collect::<Vec<_>>();
        match words.first() {
            Some(&"swap") => {
                if words[1] == "position" {
                    let x = words[2].parse::<usize>().map_err(error_mapper).unwrap();
                    let y = words[5].parse::<usize>().map_err(error_mapper).unwrap();
                    password.swap(x, y);
                } else {
                    // Swap letters
                    let x = words[2].as_bytes()[0];
                    let y = words[5].as_bytes()[0];
                    for c in password.iter_mut() {
                        let orig = *c;
                        *c = if orig == x {
                            y
                        } else if orig == y {
                            x
                        } else {
                            orig
                        };
                    }
                }
            }
            Some(&"rotate") => {
                let rotation = if words[1] == "based" {
                    let letter = words[6].as_bytes()[0];
                    if let Some((idx, _)) =
                        password.iter().enumerate().find(|&(_idx, &c)| c == letter)
                    {
                        i32::try_from((1 + idx + usize::from(idx >= 4)) % password.len()).unwrap()
                    } else {
                        return Err(format!(
                            "Unable to find letter for rotation: '{}'",
                            letter as char
                        ));
                    }
                } else {
                    words[2].parse::<i32>().map_err(error_mapper).unwrap()
                        * if words[1] == "left" { -1 } else { 1 }
                };

                if rotation < 0 {
                    password.rotate_left((-rotation) as usize);
                } else {
                    password.rotate_right(rotation as usize);
                }
            }
            Some(&"reverse") => {
                let x = words[2].parse::<usize>().map_err(error_mapper).unwrap();
                let y = words[4].parse::<usize>().map_err(error_mapper).unwrap();
                password[x..=y].reverse();
            }
            Some(&"move") => {
                let x = words[2].parse::<usize>().map_err(error_mapper).unwrap();
                let y = words[5].parse::<usize>().map_err(error_mapper).unwrap();
                let mut buffer: Vec<u8> = password.to_vec();
                let removed_letter = buffer.remove(x);
                buffer.insert(y, removed_letter);
                password.clone_from_slice(&buffer);
            }
            _ => {
                return Err("Invalid input".to_string());
            }
        }
    }
    Ok(())
}

fn all_permutations<F, T>(sequence: &mut [T], on_permutation: &mut F) -> Result<(), String>
where
    F: FnMut(&[T]) -> Result<(), String>,
{
    let size = sequence.len();
    all_permutations_internal(sequence, size, on_permutation)
}

fn all_permutations_internal<F, T>(
    sequence: &mut [T],
    size: usize,
    on_permutation: &mut F,
) -> Result<(), String>
where
    F: FnMut(&[T]) -> Result<(), String>,
{
    if size == 1 {
        return on_permutation(sequence);
    }

    for i in 0..size {
        all_permutations_internal(sequence, size - 1, on_permutation)?;

        if size % 2 == 1 {
            // If size is odd, swap first and last element.
            sequence.swap(0, size - 1);
        } else {
            // If size is even, swap ith and last element.
            sequence.swap(i, size - 1);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), "gbhcefad");
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), "gahedfcb");
    }
}
