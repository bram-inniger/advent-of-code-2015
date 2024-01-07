use itertools::Itertools;
use std::str::FromStr;

use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(relations: &[&str]) -> i16 {
    let seating = Seating::new(relations);

    seating
        .people
        .iter()
        .permutations(seating.people.len())
        .map(|p| {
            (0..p.len())
                .map(|idx| {
                    let middle = *p[idx];

                    let left_idx = if idx == 0 { p.len() - 1 } else { idx - 1 };
                    let left = *p[left_idx];

                    let right_idx = if idx == p.len() - 1 { 0 } else { idx + 1 };
                    let right = *p[right_idx];

                    seating.relations[&(middle, left)] + seating.relations[&(middle, right)]
                })
                .sum::<i16>()
        })
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Seating<'a> {
    people: FxHashSet<&'a str>,
    relations: FxHashMap<(&'a str, &'a str), i16>,
}

impl<'a> Seating<'a> {
    fn new(relations: &[&'a str]) -> Self {
        let re = Regex::new(r"^(?<name_a>\w+) would (?<score>(gain|lose) \d+) happiness units by sitting next to (?<name_b>\w+)\.$").unwrap();

        let relations: FxHashMap<(&'a str, &'a str), i16> = relations
            .iter()
            .map(|r| {
                let caps = re.captures(r).unwrap();

                let name_a = caps.name("name_a").unwrap().as_str();
                let name_b = caps.name("name_b").unwrap().as_str();
                let score = caps.name("score").unwrap().as_str();
                let score = if score.starts_with("gain") {
                    i16::from_str(score.strip_prefix("gain ").unwrap()).unwrap()
                } else if score.starts_with("lose") {
                    -i16::from_str(score.strip_prefix("lose ").unwrap()).unwrap()
                } else {
                    unreachable!()
                };

                ((name_a, name_b), score)
            })
            .collect();
        let people: FxHashSet<&'a str> = relations.keys().map(|(p, _)| *p).collect();

        Self { people, relations }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_13_part_01_sample() {
        let sample = vec![
            "Alice would gain 54 happiness units by sitting next to Bob.",
            "Alice would lose 79 happiness units by sitting next to Carol.",
            "Alice would lose 2 happiness units by sitting next to David.",
            "Bob would gain 83 happiness units by sitting next to Alice.",
            "Bob would lose 7 happiness units by sitting next to Carol.",
            "Bob would lose 63 happiness units by sitting next to David.",
            "Carol would lose 62 happiness units by sitting next to Alice.",
            "Carol would gain 60 happiness units by sitting next to Bob.",
            "Carol would gain 55 happiness units by sitting next to David.",
            "David would gain 46 happiness units by sitting next to Alice.",
            "David would lose 7 happiness units by sitting next to Bob.",
            "David would gain 41 happiness units by sitting next to Carol.",
        ];

        assert_eq!(330, solve_1(&sample));
    }

    #[test]
    fn day_13_part_01_solution() {
        let input = include_str!("../../inputs/day_13.txt")
            .lines()
            .collect_vec();

        assert_eq!(664, solve_1(&input));
    }
}
