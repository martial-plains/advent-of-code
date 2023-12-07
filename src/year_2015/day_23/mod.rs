use anyhow::anyhow;

pub const TITLE: &str = "Opening the Turing Lock";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> usize {
    solve(input, 0).unwrap()
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    solve(input, 1).unwrap()
}

fn access(regs: &mut [usize; 2], reg: Register) -> &mut usize {
    match reg {
        Register::A => &mut regs[0],
        Register::B => &mut regs[1],
    }
}

fn solve(input: &str, start_a: usize) -> anyhow::Result<usize> {
    let instructions = parse_input(input)?;
    let mut ip = 0isize;
    let regs = &mut [start_a, 0];

    while ip >= 0 && (usize::try_from(ip).unwrap()) < instructions.len() {
        // print!("[{:>6 } {:>6 }] {:>20 }", regs[0], regs[1], format!("{:?}", instructions[ip as usize]));
        match &instructions[usize::try_from(ip).unwrap()] {
            Instruction::Half(reg) => {
                *access(regs, *reg) /= 2;
                ip += 1;
            }
            Instruction::Tripple(reg) => {
                *access(regs, *reg) *= 3;
                ip += 1;
            }
            Instruction::Increment(reg) => {
                *access(regs, *reg) += 1;
                ip += 1;
            }
            Instruction::Jump(offset) => ip += offset,
            Instruction::JumpIfEven(reg, offset) => {
                if *access(regs, *reg) % 2 == 0 {
                    ip += offset;
                } else {
                    ip += 1;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if *access(regs, *reg) == 1 {
                    ip += offset;
                } else {
                    ip += 1;
                }
            }
        }
        // println!("     [{:>6 } {:>6 }]", regs[0], regs[1]);
    }
    Ok(regs[1])
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Half(Register),
    Tripple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Instruction>> {
    fn parse_register(s: &str) -> anyhow::Result<Register> {
        if s.len() == 1 {
            match s.as_bytes()[0] {
                b'a' => return Ok(Register::A),
                b'b' => return Ok(Register::B),
                _ => {}
            }
        }
        Err(anyhow!("invalid register"))
    }
    input
        .split('\n')
        .map(|line| {
            if line.len() < 5 {
                return Err(anyhow!("invalid instruction length"));
            }
            let constructor = match &line[0..4] {
                "hlf " => return Ok(Instruction::Half(parse_register(&line[4..])?)),
                "tpl " => return Ok(Instruction::Tripple(parse_register(&line[4..])?)),
                "inc " => return Ok(Instruction::Increment(parse_register(&line[4..])?)),
                "jmp " => return Ok(Instruction::Jump(line[4..].parse()?)),
                "jie " => Instruction::JumpIfEven,
                "jio " => Instruction::JumpIfOne,
                _ => return Err(anyhow!("invalid instruction")),
            };
            let mut parts = line[4..].split(", ");
            let reg = parse_register(parts.next().unwrap())?;
            let offset = parts.next().ok_or(anyhow!("expected offset"))?.parse()?;
            Ok(constructor(reg, offset))
        })
        .collect::<anyhow::Result<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 255);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 334);
    }
}
