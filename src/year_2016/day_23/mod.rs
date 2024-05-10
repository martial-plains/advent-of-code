pub const TITLE: &str = "Safe Cracking";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part1(input: &str) -> i64 {
    let mut prog = read_program(input);
    let mut state = CpuState {
        regs: [7, 0, 0, 0],
        ..Default::default()
    };
    run_program(&mut prog, &mut state);
    state.regs[0]
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> i64 {
    let mut prog = read_program(input);
    let mut state = CpuState {
        regs: [12, 0, 0, 0],
        ..Default::default()
    };
    run_program(&mut prog, &mut state);
    state.regs[0]
}

use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum InstrOp {
    Nop,
    IncR,
    DecR,
    CpyRr,
    CpyIr,
    JnzIi,
    JnzIr,
    JnzRi,
    JnzRr,
    TglI,
    TglR,
}

#[derive(Debug, Clone, Copy)]
struct Instr {
    op: InstrOp,
    args: [i64; 2],
}

#[derive(Debug, Default)]
struct CpuState {
    pc: usize,
    regs: [i64; 4],
}

type Program = Vec<Instr>;

fn toggle_instr(prog: &mut Program, instr: usize) {
    static NEXT: [InstrOp; 11] = [
        InstrOp::Nop,
        InstrOp::DecR,
        InstrOp::IncR,
        InstrOp::JnzRr,
        InstrOp::JnzIr,
        InstrOp::Nop,
        InstrOp::CpyIr,
        InstrOp::Nop,
        InstrOp::CpyRr,
        InstrOp::Nop,
        InstrOp::IncR,
    ];
    if instr < prog.len() {
        prog[instr].op = NEXT[prog[instr].op as usize];
    }
}

fn run_program(prog: &mut Program, state: &mut CpuState) {
    while state.pc < prog.len() {
        let instr = prog[state.pc];
        match instr.op {
            InstrOp::Nop => {}
            InstrOp::IncR => state.regs[usize::try_from(instr.args[0]).unwrap()] += 1,
            InstrOp::DecR => state.regs[usize::try_from(instr.args[0]).unwrap()] -= 1,
            InstrOp::CpyRr => {
                state.regs[usize::try_from(instr.args[1]).unwrap()] =
                    state.regs[usize::try_from(instr.args[0]).unwrap()];
            }
            InstrOp::CpyIr => state.regs[usize::try_from(instr.args[1]).unwrap()] = instr.args[0],
            InstrOp::JnzIi => {
                if instr.args[0] != 0 {
                    state.pc = usize::try_from(state.pc as i64 + instr.args[1] - 1).unwrap();
                }
            }
            InstrOp::JnzIr => {
                if instr.args[0] != 0 {
                    state.pc = usize::try_from(
                        state.pc as i64 + state.regs[usize::try_from(instr.args[1]).unwrap()] - 1,
                    )
                    .unwrap();
                }
            }
            InstrOp::JnzRi => {
                if state.regs[usize::try_from(instr.args[0]).unwrap()] != 0 {
                    state.pc = usize::try_from(state.pc as i64 + instr.args[1] - 1).unwrap();
                }
            }
            InstrOp::JnzRr => {
                if state.regs[usize::try_from(instr.args[0]).unwrap()] != 0 {
                    state.pc = usize::try_from(
                        state.pc as i64 + state.regs[usize::try_from(instr.args[1]).unwrap()] - 1,
                    )
                    .unwrap();
                }
            }
            InstrOp::TglI => toggle_instr(prog, state.pc + usize::try_from(instr.args[0]).unwrap()),
            InstrOp::TglR => toggle_instr(
                prog,
                state.pc
                    + usize::try_from(state.regs[usize::try_from(instr.args[0]).unwrap()]).unwrap(),
            ),
        }
        state.pc += 1;
    }
}

fn read_program(input: &str) -> Program {
    let mut program = Vec::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        let instr = match words.next() {
            Some("cpy") => cpy(&mut words),
            Some("jnz") => jnz(&mut words),
            Some("inc") => inc(&mut words),
            Some("dec") => dec(&mut words),
            Some("tgl") => tgl(words),
            _ => continue,
        };
        program.push(instr);
    }
    program
}

fn tgl(mut words: std::str::SplitWhitespace) -> Instr {
    let arg = words.next().unwrap();
    if arg.chars().all(|c| c.is_ascii_alphabetic()) {
        let arg = arg.chars().next().unwrap() as i64 - 'a' as i64;
        Instr {
            op: InstrOp::TglR,
            args: [arg, 0],
        }
    } else {
        let arg = arg.parse().unwrap();
        Instr {
            op: InstrOp::TglI,
            args: [arg, 0],
        }
    }
}

fn dec(words: &mut std::str::SplitWhitespace) -> Instr {
    let reg = words.next().unwrap().chars().next().unwrap() as i64 - 'a' as i64;
    Instr {
        op: InstrOp::DecR,
        args: [reg, 0],
    }
}

fn inc(words: &mut std::str::SplitWhitespace) -> Instr {
    let reg = words.next().unwrap().chars().next().unwrap() as i64 - 'a' as i64;
    Instr {
        op: InstrOp::IncR,
        args: [reg, 0],
    }
}

fn jnz(words: &mut std::str::SplitWhitespace) -> Instr {
    let arg1 = words.next().unwrap();
    let arg2 = words.next().unwrap();
    if arg1.chars().all(|c| c.is_ascii_alphabetic()) {
        let arg1 = arg1.chars().next().unwrap() as i64 - 'a' as i64;
        if arg2.chars().all(|c| c.is_ascii_alphabetic()) {
            let arg2 = arg2.chars().next().unwrap() as i64 - 'a' as i64;
            Instr {
                op: InstrOp::JnzRr,
                args: [arg1, arg2],
            }
        } else {
            let arg2 = arg2.parse().unwrap();
            Instr {
                op: InstrOp::JnzRi,
                args: [arg1, arg2],
            }
        }
    } else {
        let arg1 = arg1.parse().unwrap();
        if arg2.chars().all(|c| c.is_ascii_alphabetic()) {
            let arg2 = arg2.chars().next().unwrap() as i64 - 'a' as i64;
            Instr {
                op: InstrOp::JnzIr,
                args: [arg1, arg2],
            }
        } else {
            let arg2 = arg2.parse().unwrap();
            Instr {
                op: InstrOp::JnzIi,
                args: [arg1, arg2],
            }
        }
    }
}

fn cpy(words: &mut std::str::SplitWhitespace) -> Instr {
    let arg1 = words.next().unwrap();
    let arg2 = words.next().unwrap();
    if arg1.chars().all(|c| c.is_ascii_alphabetic()) {
        let arg1 = arg1.chars().next().unwrap() as i64 - 'a' as i64;
        if arg2.chars().all(|c| c.is_ascii_alphabetic()) {
            let arg2 = arg2.chars().next().unwrap() as i64 - 'a' as i64;
            Instr {
                op: InstrOp::CpyRr,
                args: [arg1, arg2],
            }
        } else {
            let arg2 = arg2.parse().unwrap();
            Instr {
                op: InstrOp::CpyIr,
                args: [arg1, arg2],
            }
        }
    } else {
        let arg1 = arg1.parse().unwrap();
        let arg2 = arg2.chars().next().unwrap() as i64 - 'a' as i64;
        Instr {
            op: InstrOp::CpyIr,
            args: [arg1, arg2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 12330);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 479_008_890);
    }
}
