use std::str::FromStr;

use itertools::Itertools;

pub fn solve_1(presents: &[&str]) -> u32 {
    presents
        .iter()
        .map(|p| Present::new(p))
        .map(|p| p.surface() + p.slack())
        .sum()
}

pub fn solve_2(presents: &[&str]) -> u32 {
    presents
        .iter()
        .map(|p| Present::new(p))
        .map(|p| p.shortest_distance() + p.volume())
        .sum()
}

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn new(present: &str) -> Self {
        let split = present
            .split('x')
            .map(|s| u32::from_str(s).unwrap())
            .collect_vec();

        match split[..] {
            [l, w, h] => Self { l, w, h },
            _ => unreachable!(),
        }
    }

    fn surface(&self) -> u32 {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l
    }

    fn slack(&self) -> u32 {
        [self.l * self.w, self.l * self.h, self.w * self.h]
            .into_iter()
            .min()
            .unwrap()
    }

    fn shortest_distance(&self) -> u32 {
        [self.l + self.w, self.l + self.h, self.w + self.h]
            .into_iter()
            .map(|d| 2 * d)
            .min()
            .unwrap()
    }

    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_02_part_01_sample() {
        let sample = vec!["2x3x4", "1x1x10"];

        assert_eq!(101, solve_1(&sample));
    }

    #[test]
    fn day_02_part_01_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_606_483, solve_1(&input));
    }

    #[test]
    fn day_02_part_02_sample() {
        let sample = vec!["2x3x4", "1x1x10"];

        assert_eq!(48, solve_2(&sample));
    }

    #[test]
    fn day_02_part_02_solution() {
        let input = include_str!("../../inputs/day_02.txt")
            .lines()
            .collect_vec();

        assert_eq!(3_842_356, solve_2(&input));
    }
}
