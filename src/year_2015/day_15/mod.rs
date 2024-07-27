use anyhow::anyhow;
use regex::RegexBuilder;

pub const TITLE: &str = "Science for Hungry People";

pub const INPUT: &str = include_str!("input.txt");

/// # Panics
///
/// Panics if input is empty.
#[must_use]
pub fn part1(input: &str) -> isize {
    let ingredients = parse_ingredients(input);
    let mut max = 0;
    for_each_combination(&ingredients, |a, b, c, d| {
        let capacity = isize::max(
            0,
            a * ingredients[0].capacity
                + b * ingredients[1].capacity
                + c * ingredients[2].capacity
                + d * ingredients[3].capacity,
        );
        let durability = isize::max(
            0,
            a * ingredients[0].durability
                + b * ingredients[1].durability
                + c * ingredients[2].durability
                + d * ingredients[3].durability,
        );
        let flavor = isize::max(
            0,
            a * ingredients[0].flavor
                + b * ingredients[1].flavor
                + c * ingredients[2].flavor
                + d * ingredients[3].flavor,
        );
        let texture = isize::max(
            0,
            a * ingredients[0].texture
                + b * ingredients[1].texture
                + c * ingredients[2].texture
                + d * ingredients[3].texture,
        );
        // let calories   = isize::max(0, a * ingredients[0].calories   + b * ingredients[1].calories   + c * ingredients[2].calories   + d * ingredients[3].calories  );
        let value = capacity * durability * flavor * texture;
        if value > max {
            max = value;
        }
    })
    .unwrap();
    max
}

/// # Panics
///
/// Panics if input is empty.
#[must_use]
pub fn part2(input: &str) -> isize {
    let ingredients = parse_ingredients(input);
    let mut max = 0;
    for_each_combination(&ingredients, |a, b, c, d| {
        let capacity = isize::max(
            0,
            a * ingredients[0].capacity
                + b * ingredients[1].capacity
                + c * ingredients[2].capacity
                + d * ingredients[3].capacity,
        );
        let durability = isize::max(
            0,
            a * ingredients[0].durability
                + b * ingredients[1].durability
                + c * ingredients[2].durability
                + d * ingredients[3].durability,
        );
        let flavor = isize::max(
            0,
            a * ingredients[0].flavor
                + b * ingredients[1].flavor
                + c * ingredients[2].flavor
                + d * ingredients[3].flavor,
        );
        let texture = isize::max(
            0,
            a * ingredients[0].texture
                + b * ingredients[1].texture
                + c * ingredients[2].texture
                + d * ingredients[3].texture,
        );
        let calories = isize::max(
            0,
            a * ingredients[0].calories
                + b * ingredients[1].calories
                + c * ingredients[2].calories
                + d * ingredients[3].calories,
        );
        let value = capacity * durability * flavor * texture;
        if calories == 500 && value > max {
            max = value;
        }
    })
    .unwrap();
    max
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

fn parse_ingredients(input: &str) -> Vec<Ingredient> {
    let re = RegexBuilder::new(r"^(?P<n>[[:alpha:]]+): capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").multi_line(true).build().unwrap();
    re.captures_iter(input)
        .map(|m| Ingredient {
            capacity: m["capacity"].parse().unwrap(),
            durability: m["durability"].parse().unwrap(),
            flavor: m["flavor"].parse().unwrap(),
            texture: m["texture"].parse().unwrap(),
            calories: m["calories"].parse().unwrap(),
        })
        .collect()
}

fn for_each_combination<F>(ingredients: &[Ingredient], mut f: F) -> anyhow::Result<()>
where
    F: FnMut(isize, isize, isize, isize),
{
    if ingredients.len() != 4 {
        return Err(anyhow!("can only handle 4 ingredients"));
    }

    for a in 0..101 {
        let remainder = 100 - a;
        for b in 0..=remainder {
            let remainder = remainder - b;
            for c in 0..=remainder {
                let remainder = remainder - c;
                let d = remainder;
                f(a, b, c, d);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_puzzle() {
        assert_eq!(part1(INPUT), 222_870);
    }

    #[test]
    fn test_part2_puzzle() {
        assert_eq!(part2(INPUT), 117_936);
    }
}
