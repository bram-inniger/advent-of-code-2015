use std::str::FromStr;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(instructions: &[&str]) -> usize {
    let mut grid = [[false; 1000]; 1000];
    let instructions = instructions
        .iter()
        .map(|i| Instruction::new(i))
        .collect_vec();

    #[allow(clippy::needless_range_loop)]
    for i in instructions {
        for x in i.x_min..=i.x_max {
            for y in i.y_min..=i.y_max {
                match i.action {
                    Action::TurnOn => grid[y][x] = true,
                    Action::TurnOff => grid[y][x] = false,
                    Action::Toggle => grid[y][x] = !grid[y][x],
                }
            }
        }
    }

    grid.iter()
        .map(|lights| lights.iter().filter(|&&light| light).count())
        .sum()
}

pub fn solve_2(instructions: &[&str]) -> i32 {
    let mut grid = vec![vec![0i32; 1000]; 1000];
    let instructions = instructions
        .iter()
        .map(|i| Instruction::new(i))
        .collect_vec();

    #[allow(clippy::needless_range_loop)]
    for i in instructions {
        for x in i.x_min..=i.x_max {
            for y in i.y_min..=i.y_max {
                match i.action {
                    Action::TurnOn => grid[y][x] += 1,
                    Action::TurnOff => grid[y][x] = i32::max(grid[y][x] - 1, 0),
                    Action::Toggle => grid[y][x] += 2,
                }
            }
        }
    }

    grid.iter().map(|lights| lights.iter().sum::<i32>()).sum()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(
            r"^(?<action>(turn on)|(turn off)|(toggle)) (?<x_min>\d+),(?<y_min>\d+) through (?<x_max>\d+),(?<y_max>\d+).*$"
        ).unwrap();
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Instruction {
    action: Action,
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Instruction {
    fn new(instruction: &str) -> Instruction {
        let caps = RE.captures(instruction).unwrap();

        let action = match caps.name("action").unwrap().as_str() {
            "turn on" => Action::TurnOn,
            "turn off" => Action::TurnOff,
            "toggle" => Action::Toggle,
            _ => unreachable!(),
        };
        let x_min = usize::from_str(caps.name("x_min").unwrap().as_str()).unwrap();
        let y_min = usize::from_str(caps.name("y_min").unwrap().as_str()).unwrap();
        let x_max = usize::from_str(caps.name("x_max").unwrap().as_str()).unwrap();
        let y_max = usize::from_str(caps.name("y_max").unwrap().as_str()).unwrap();

        Instruction {
            action,
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_06_part_01_sample() {
        let sample = vec![
            "turn on 0,0 through 999,999",
            "toggle 0,0 through 999,0",
            "turn off 499,499 through 500,500",
        ];

        assert_eq!(998_996, solve_1(&sample));
    }

    #[test]
    fn day_06_part_01_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(377_891, solve_1(&input));
    }

    #[test]
    fn day_06_part_02_sample() {
        let sample = vec!["turn on 0,0 through 0,0", "toggle 0,0 through 999,999"];

        assert_eq!(2_000_001, solve_2(&sample));
    }

    #[test]
    fn day_06_part_02_solution() {
        let input = include_str!("../../inputs/day_06.txt")
            .lines()
            .collect_vec();

        assert_eq!(14_110_788, solve_2(&input));
    }
}
