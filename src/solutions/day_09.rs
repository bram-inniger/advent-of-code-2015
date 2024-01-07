use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHashMap, FxHashSet};

pub fn solve_1(distances: &[&str]) -> u16 {
    let graph = Graph::new(distances);

    graph
        .nodes
        .iter()
        .permutations(graph.nodes.len())
        .map(|p| {
            (1..p.len())
                .map(|idx| graph.vertices[&(*p[idx - 1], *p[idx])])
                .sum()
        })
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Graph<'a> {
    nodes: FxHashSet<&'a str>,
    vertices: FxHashMap<(&'a str, &'a str), u16>,
}

impl<'a> Graph<'a> {
    fn new(distances: &[&'a str]) -> Self {
        let re = Regex::new(r"^(?<city_a>\w+) to (?<city_b>\w+) = (?<distance>\d+)$").unwrap();

        let vertices: FxHashMap<(&'a str, &'a str), u16> = distances
            .iter()
            .flat_map(|d| {
                let caps = re.captures(d).unwrap();

                let city_a = caps.name("city_a").unwrap().as_str();
                let city_b = caps.name("city_b").unwrap().as_str();
                let distance = u16::from_str(caps.name("distance").unwrap().as_str()).unwrap();

                [((city_a, city_b), distance), ((city_b, city_a), distance)]
            })
            .collect();
        let nodes = vertices.keys().map(|(a, _)| *a).collect();

        Self { vertices, nodes }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_09_part_01_sample() {
        let sample = vec![
            "London to Dublin = 464",
            "London to Belfast = 518",
            "Dublin to Belfast = 141",
        ];

        assert_eq!(605, solve_1(&sample));
    }

    #[test]
    fn day_09_part_01_solution() {
        let input = include_str!("../../inputs/day_09.txt")
            .lines()
            .collect_vec();

        assert_eq!(251, solve_1(&input));
    }
}
