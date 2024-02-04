use anyhow::anyhow;
use arrayvec::ArrayVec;
use lazy_static::lazy_static;

pub const TITLE: &str = "Wizard Simulator 20XX";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if unable to parse input
pub fn part1(input: &str) -> usize {
    let boss = parse_input(input).unwrap();
    let state = State {
        player: Player { hp: 50, mana: 500 },
        boss,
        effects: Effects {
            shield: 0,
            poison: 0,
            recharge: 0,
        },
    };

    // Use A* because I don't feel like coding up Dijkstra right now
    let mut astar = crate::shared::astar::AStar::new();
    let path = astar
        .solve(state, State::next_states, |_| 0, |state| state.boss.hp <= 0)
        .ok_or_else(|| anyhow!("no solution found"))
        .unwrap();
    path.last().unwrap().1
}

/// # Panics
///
/// Panics if unable to parse input
#[must_use]
pub fn part2(input: &str) -> usize {
    let boss = parse_input(input).unwrap();
    let state = State {
        player: Player { hp: 50, mana: 500 },
        boss,
        effects: Effects {
            shield: 0,
            poison: 0,
            recharge: 0,
        },
    };

    // Use A* because I don't feel like coding up Dijkstra right now
    let mut astar = crate::shared::astar::AStar::new();
    let path = astar
        .solve(
            state,
            |state| {
                let mut state = *state;
                state.player.hp -= 1;
                if state.player.hp <= 0 {
                    ArrayVec::new()
                } else {
                    State::next_states(&state)
                }
            },
            |_| 0,
            |state| state.boss.hp <= 0,
        )
        .ok_or_else(|| anyhow!("no solution found"))
        .unwrap();
    path.last().unwrap().1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    hp: i16,
    mana: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Boss {
    hp: i16,
    damage: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Effects {
    shield: u8,
    poison: u8,
    recharge: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    player: Player,
    boss: Boss,
    effects: Effects,
}

impl Effects {
    fn count_down(&mut self) {
        if self.shield != 0 {
            self.shield -= 1;
        }
        if self.poison != 0 {
            self.poison -= 1;
        }
        if self.recharge != 0 {
            self.recharge -= 1;
        }
    }
}

impl State {
    fn execute_boss_turn(&mut self) {
        // Perform effects
        if self.effects.poison != 0 {
            self.boss.hp -= 3;
        }
        if self.effects.recharge != 0 {
            self.player.mana += 101;
        }
        self.effects.count_down();

        // Perform boss attack
        if self.boss.hp <= 0 {
            return;
        }
        let player_has_shield = self.effects.shield != 0;
        let damage = if player_has_shield {
            (self.boss.damage - 7).max(1)
        } else {
            self.boss.damage
        };
        self.player.hp -= damage;
    }

    fn next_states(&self) -> ArrayVec<(Self, usize), 5> {
        let mut new_states = ArrayVec::<(Self, usize), 5>::new();
        let mut state = *self;

        // Apply effects
        if state.effects.poison != 0 {
            state.boss.hp -= 3;
            // Special case where boss before we can cast, 0 cost
            if state.boss.hp <= 0 {
                new_states.push((state, 0));
                return new_states;
            }
        }
        if state.effects.recharge != 0 {
            state.player.mana += 101;
        }
        state.effects.count_down();

        // Magic missle
        if state.player.mana >= 53 {
            let mut state = state;
            state.player.mana -= 53;
            state.boss.hp -= 4;
            state.execute_boss_turn();
            if state.player.hp > 0 {
                new_states.push((state, 53));
            }
        }

        // Shield
        if state.player.mana >= 113 && state.effects.shield == 0 {
            let mut state = state;
            state.player.mana -= 113;
            state.effects.shield = 6;
            state.execute_boss_turn();
            if state.player.hp > 0 {
                new_states.push((state, 113));
            }
        }

        // Poison
        if state.player.mana >= 173 && state.effects.poison == 0 {
            let mut state = state;
            state.player.mana -= 173;
            state.effects.poison = 6;
            state.execute_boss_turn();
            if state.player.hp > 0 {
                new_states.push((state, 173));
            }
        }

        // Recharge
        if state.player.mana >= 229 && state.effects.recharge == 0 {
            let mut state = state;
            state.player.mana -= 229;
            state.effects.recharge = 5;
            state.execute_boss_turn();
            if state.player.hp > 0 {
                new_states.push((state, 229));
            }
        }

        // Drain
        if state.player.mana >= 73 {
            state.player.mana -= 73;
            state.player.hp += 2;
            state.boss.hp -= 2;
            state.execute_boss_turn();
            if state.player.hp > 0 {
                new_states.push((state, 73));
            }
        }

        new_states
    }
}

fn parse_input(input: &str) -> anyhow::Result<Boss> {
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^Hit Points: (?P<hp>\d+)\r?\nDamage: (?P<damage>\d+)$").unwrap();
    };
    let captures = RE.captures(input).ok_or_else(|| anyhow!("invalid input"))?;
    let hp = captures["hp"].parse()?;
    let damage = captures["damage"].parse()?;

    Ok(Boss { hp, damage })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 953);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 1289);
    }
}
