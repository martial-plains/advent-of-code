use anyhow::anyhow;
use lazy_static::lazy_static;

pub const TITLE: &str = "RPG Simulator 20XX";

pub const INPUT: &str = include_str!("input.txt");

/// # Errors
///
/// This function will return an error if no loadout exists that would let the player defeat the boss.
pub fn part1(input: &str) -> anyhow::Result<usize> {
    let boss = parse_input(input)?;
    let mut loadouts = loadouts().collect::<Vec<_>>();
    loadouts.sort_unstable_by(|a, b| a.cost.cmp(&b.cost));

    for loadout in loadouts {
        if does_player_win_fight(&boss, &loadout) {
            return Ok(loadout.cost);
        }
    }

    Err(anyhow!(
        "no loadout exists that would let the player defeat the boss",
    ))
}

/// # Errors
///
/// This function will return an error if no loadout exists that would let the player defeat the boss.
pub fn part2(input: &str) -> anyhow::Result<usize> {
    let boss = parse_input(input)?;
    loadouts()
        .filter(|loadout| !does_player_win_fight(&boss, loadout))
        .map(|loadout| loadout.cost)
        .max()
        .ok_or_else(|| anyhow!("no loadout exists where the boss wins"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Unit {
    hp: usize,
    damage: usize,
    armor: usize,
}

// Combinatorics:
// 5 weapons
// 6 armor (no armor is the 6th option)
// Rings:
//   1 - no ring
//   6 - 1 ring
//   6 nCr 2 - 2 rings = 15 (6 * 5 / 2)
// 22 rings
//
// Total combinations:
// 5 * 6 * 22 = 660
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Loadout {
    cost: usize,
    damage: usize,
    armor: usize,
}

const WEAPONS: [(usize, usize); 5] = [
    /* Dagger      */ (8, 4),
    /* Shortsword */ (10, 5),
    /* Warhammer  */ (25, 6),
    /* Longsword  */ (40, 7),
    /* Greataxe   */ (74, 8),
];
const ARMOR: [(usize, usize); 6] = [
    /* None         */ (0, 0),
    /* Leather     */ (13, 1),
    /* Chainmail   */ (31, 2),
    /* Splintmail  */ (53, 3),
    /* Bandedmail  */ (75, 4),
    /* Platemail  */ (102, 5),
];
const RINGS: [(usize, usize, usize); 6] = [
    /* Damage +1   */ (25, 1, 0),
    /* Damage +2   */ (50, 2, 0),
    /* Damage +3  */ (100, 3, 0),
    /* Defense +1  */ (20, 0, 1),
    /* Defense +2  */ (40, 0, 2),
    /* Defense +3  */ (80, 0, 3),
];

fn loadouts() -> impl Iterator<Item = Loadout> + Clone {
    fn loadout_from_index(idx: usize) -> Loadout {
        let ring_idx = idx % 22;
        let idx = idx / 22;
        let armor_idx = idx % 6;
        let weapon_idx = idx / 6;

        let mut damage = WEAPONS[weapon_idx].1;
        let mut armor = ARMOR[armor_idx].1;
        let mut cost = WEAPONS[weapon_idx].0 + ARMOR[armor_idx].0;

        let (ring1, ring2) = match ring_idx {
            0 => (None, None),
            1..=6 => (Some(ring_idx - 1), None),
            7..=11 => (Some(0), Some(ring_idx - (7 - 1))),
            12..=15 => (Some(1), Some(ring_idx - (12 - 2))),
            16..=18 => (Some(2), Some(ring_idx - (16 - 3))),
            19..=20 => (Some(3), Some(ring_idx - (19 - 4))),
            21..=21 => (Some(4), Some(5)),
            _ => unreachable!(),
        };
        let (ring1, ring2) = (
            ring1.map_or((0, 0, 0), |idx| RINGS[idx]),
            ring2.map_or((0, 0, 0), |idx| RINGS[idx]),
        );
        cost += ring1.0 + ring2.0;
        damage += ring1.1 + ring2.1;
        armor += ring1.2 + ring2.2;

        Loadout {
            cost,
            damage,
            armor,
        }
    }
    (0..660).map(loadout_from_index)
}

const fn calculate_effective_damage(from: &Unit, to: &Unit) -> usize {
    if from.damage <= to.armor {
        1
    } else {
        from.damage - to.armor
    }
}

const fn does_player_win_fight(boss: &Unit, loadout: &Loadout) -> bool {
    let player = Unit {
        hp: 100,
        damage: loadout.damage,
        armor: loadout.armor,
    };
    let edmg_player = calculate_effective_damage(&player, boss);
    let edmg_boss = calculate_effective_damage(boss, &player);

    let turns_to_kill_boss = (boss.hp + edmg_player - 1) / edmg_player;
    let boss_damage_before_death = edmg_boss * (turns_to_kill_boss - 1);

    // Player survived the damage the boss would deal
    boss_damage_before_death < player.hp
}

fn parse_input(input: &str) -> anyhow::Result<Unit> {
    use regex::Regex;
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Hit Points: (?P<hp>\d+)\r?\nDamage: (?P<damage>\d+)\r?\nArmor: (?P<armor>\d+)$"
        )
        .unwrap();
    };
    let captures = RE.captures(input).ok_or_else(|| anyhow!("invalid input"))?;
    let hp = captures["hp"].parse()?;
    let damage = captures["damage"].parse()?;
    let armor = captures["armor"].parse()?;

    Ok(Unit { hp, damage, armor })
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT).unwrap(), 111);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT).unwrap(), 188);
    }
}
