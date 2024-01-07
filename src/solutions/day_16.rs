use std::ops::Not;
use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;

pub fn solve_1(sues: &[&str]) -> u16 {
    let tape = read_tape();

    sues.iter()
        .map(|s| Sue::new(s))
        .find(|s| s.valid_exact(&tape))
        .unwrap()
        .number
}

pub fn solve_2(sues: &[&str]) -> u16 {
    let tape = read_tape();

    sues.iter()
        .map(|s| Sue::new(s))
        .find(|s| s.valid_ranges(&tape))
        .unwrap()
        .number
}

fn read_tape<'a>() -> FxHashMap<&'a str, u16> {
    [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .into_iter()
    .collect()
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Sue (?<number>\d+): (?<properties>.+)$").unwrap();
}

#[derive(Debug)]
struct Sue<'a> {
    number: u16,
    properties: FxHashMap<&'a str, u16>,
}

impl<'a> Sue<'a> {
    fn new(sue: &'a str) -> Self {
        let caps = RE.captures(sue).unwrap();

        let number = u16::from_str(caps.name("number").unwrap().as_str()).unwrap();
        let properties = caps
            .name("properties")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|p| {
                let split = p.split(": ").collect_vec();
                (split[0], u16::from_str(split[1]).unwrap())
            })
            .collect();

        Self { number, properties }
    }

    fn valid_exact(&self, tape: &FxHashMap<&str, u16>) -> bool {
        tape.iter()
            .all(|(&p, &v)| self.properties.contains_key(p).not() || self.properties[p] == v)
    }

    fn valid_ranges(&self, tape: &FxHashMap<&str, u16>) -> bool {
        tape.iter().all(|(&p, &v)| {
            self.properties.contains_key(p).not() || {
                match p {
                    "cats" | "trees" => self.properties[p] > v,
                    "pomeranians" | "goldfish" => self.properties[p] < v,
                    _ => self.properties[p] == v,
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_16_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_16_part_01_solution() {
        let input = include_str!("../../inputs/day_16.txt")
            .lines()
            .collect_vec();

        assert_eq!(40, solve_1(&input));
    }

    #[test]
    fn day_16_part_02_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_16_part_02_solution() {
        let input = include_str!("../../inputs/day_16.txt")
            .lines()
            .collect_vec();

        assert_eq!(241, solve_2(&input));
    }
}
