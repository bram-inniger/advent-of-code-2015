use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHashMap;

pub fn solve_1(circuit: &[&str]) -> u16 {
    solve(circuit, false)
}

pub fn solve_2(circuit: &[&str]) -> u16 {
    solve(circuit, true)
}

pub fn solve(circuit: &[&str], override_b: bool) -> u16 {
    let circuit = circuit
        .iter()
        .map(|c| Component::new(c))
        .map(|c| (c.destination(), c))
        .collect();
    let mut cache: FxHashMap<&str, u16> = FxHashMap::default();

    if override_b {
        let a = signal(&circuit, &mut cache, "a");
        cache.clear();
        cache.insert("b", a);
    }

    signal(&circuit, &mut cache, "a")
}

fn signal<'a>(
    ci: &FxHashMap<&'a str, Component<'a>>,
    ch: &mut FxHashMap<&'a str, u16>,
    wire: &'a str,
) -> u16 {
    if let Some(value) = ch.get(wire) {
        return *value;
    }
    let result = if let Ok(value) = u16::from_str(wire) {
        value
    } else {
        match ci[wire] {
            Component::Wire { source, .. } => signal(ci, ch, source),
            Component::And {
                source_a, source_b, ..
            } => signal(ci, ch, source_a) & signal(ci, ch, source_b),
            Component::Or {
                source_a, source_b, ..
            } => signal(ci, ch, source_a) | signal(ci, ch, source_b),
            Component::Not { source, .. } => !signal(ci, ch, source),
            Component::LShift { source, value, .. } => signal(ci, ch, source) << value,
            Component::RShift { source, value, .. } => signal(ci, ch, source) >> value,
        }
    };

    ch.insert(wire, result);

    result
}

lazy_static! {
    static ref RE_WIRE: Regex = Regex::new(r"^(?<source>\w+) -> (?<destination>\w+)$").unwrap();
    static ref RE_AND: Regex =
        Regex::new(r"^(?<source_a>\w+) AND (?<source_b>\w+) -> (?<destination>\w+)$").unwrap();
    static ref RE_OR: Regex =
        Regex::new(r"^(?<source_a>\w+) OR (?<source_b>\w+) -> (?<destination>\w+)$").unwrap();
    static ref RE_NOT: Regex = Regex::new(r"^NOT (?<source>\w+) -> (?<destination>\w+)$").unwrap();
    static ref RE_LSHIFT: Regex =
        Regex::new(r"^(?<source>\w+) LSHIFT (?<value>\d+) -> (?<destination>\w+)$").unwrap();
    static ref RE_RSHIFT: Regex =
        Regex::new(r"^(?<source>\w+) RSHIFT (?<value>\d+) -> (?<destination>\w+)$").unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Component<'a> {
    Wire {
        source: &'a str,
        destination: &'a str,
    },
    And {
        source_a: &'a str,
        source_b: &'a str,
        destination: &'a str,
    },
    Or {
        source_a: &'a str,
        source_b: &'a str,
        destination: &'a str,
    },
    Not {
        source: &'a str,
        destination: &'a str,
    },
    LShift {
        source: &'a str,
        value: u16,
        destination: &'a str,
    },
    RShift {
        source: &'a str,
        value: u16,
        destination: &'a str,
    },
}

impl<'a> Component<'a> {
    fn new(component: &'a str) -> Self {
        match component {
            c if RE_WIRE.is_match(c) => {
                let caps = RE_WIRE.captures(c).unwrap();

                let source = caps.name("source").unwrap().as_str();
                let destination = caps.name("destination").unwrap().as_str();

                Component::Wire {
                    source,
                    destination,
                }
            }
            c if RE_AND.is_match(c) => {
                let caps = RE_AND.captures(c).unwrap();

                let source_a = caps.name("source_a").unwrap().as_str();
                let source_b = caps.name("source_b").unwrap().as_str();
                let destination = caps.name("destination").unwrap().as_str();

                Component::And {
                    source_a,
                    source_b,
                    destination,
                }
            }
            c if RE_OR.is_match(c) => {
                let caps = RE_OR.captures(c).unwrap();

                let source_a = caps.name("source_a").unwrap().as_str();
                let source_b = caps.name("source_b").unwrap().as_str();
                let destination = caps.name("destination").unwrap().as_str();

                Component::Or {
                    source_a,
                    source_b,
                    destination,
                }
            }
            c if RE_NOT.is_match(c) => {
                let caps = RE_NOT.captures(c).unwrap();

                let source = caps.name("source").unwrap().as_str();
                let destination = caps.name("destination").unwrap().as_str();

                Component::Not {
                    source,
                    destination,
                }
            }
            c if RE_LSHIFT.is_match(c) => {
                let caps = RE_LSHIFT.captures(c).unwrap();

                let source = caps.name("source").unwrap().as_str();
                let value = u16::from_str(caps.name("value").unwrap().as_str()).unwrap();
                let destination = caps.name("destination").unwrap().as_str();

                Component::LShift {
                    source,
                    value,
                    destination,
                }
            }
            c if RE_RSHIFT.is_match(c) => {
                let caps = RE_RSHIFT.captures(c).unwrap();

                let source = caps.name("source").unwrap().as_str();
                let value = u16::from_str(caps.name("value").unwrap().as_str()).unwrap();
                let destination = caps.name("destination").unwrap().as_str();

                Component::RShift {
                    source,
                    value,
                    destination,
                }
            }
            _ => unreachable!(),
        }
    }

    fn destination(&self) -> &'a str {
        match self {
            Component::Wire { destination, .. } => destination,
            Component::And { destination, .. } => destination,
            Component::Or { destination, .. } => destination,
            Component::Not { destination, .. } => destination,
            Component::LShift { destination, .. } => destination,
            Component::RShift { destination, .. } => destination,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_07_part_01_sample() {
        let sample = vec![
            "123 -> x",
            "456 -> y",
            "x AND y -> d",
            "x OR y -> e",
            "x LSHIFT 2 -> a", // 'f' in the sample code, changed to 'a' for the implementation
            "y RSHIFT 2 -> g",
            "NOT x -> h",
            "NOT y -> i",
        ];

        assert_eq!(492, solve_1(&sample));
    }

    #[test]
    fn day_07_part_01_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(46_065, solve_1(&input));
    }

    #[test]
    fn day_07_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_07_part_02_solution() {
        let input = include_str!("../../inputs/day_07.txt")
            .lines()
            .collect_vec();

        assert_eq!(14_134, solve_2(&input));
    }
}
