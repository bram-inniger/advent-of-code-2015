use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(reindeer: &[&str], time: u32) -> u32 {
    reindeer
        .iter()
        .map(|r| Reindeer::new(r))
        .map(|r| r.distance(time))
        .max()
        .unwrap()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(
            r"^(:?(\w+)) can fly (?<speed>\d+) km/s for (?<fly_time>\d+) seconds, but then must rest for (?<rest_time>\d+) seconds\.$"
        ).unwrap();
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn new(reindeer: &str) -> Reindeer {
        let caps = RE.captures(reindeer).unwrap();

        let speed = u32::from_str(caps.name("speed").unwrap().as_str()).unwrap();
        let fly_time = u32::from_str(caps.name("fly_time").unwrap().as_str()).unwrap();
        let rest_time = u32::from_str(caps.name("rest_time").unwrap().as_str()).unwrap();

        Self {
            speed,
            fly_time,
            rest_time,
        }
    }

    fn distance(&self, time: u32) -> u32 {
        let full_intervals = time / (self.fly_time + self.rest_time);
        let remaining_time = time % (self.fly_time + self.rest_time);

        (full_intervals * self.fly_time + remaining_time.min(self.fly_time)) * self.speed
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_14_part_01_sample() {
        let sample = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ];

        assert_eq!(1_120, solve_1(&sample, 1_000));
    }

    #[test]
    fn day_14_part_01_solution() {
        let input = include_str!("../../inputs/day_14.txt")
            .lines()
            .collect_vec();

        assert_eq!(2_660, solve_1(&input, 2_503));
    }
}
