use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

pub fn solve_1(reindeer: &[&str], time: u32) -> u32 {
    Race::new(reindeer).result(time).max_distance
}

pub fn solve_2(reindeer: &[&str], time: u32) -> u32 {
    Race::new(reindeer).result(time).max_points
}

#[derive(Debug)]
struct Race {
    reindeer: Vec<Reindeer>,
}

impl Race {
    fn new(reindeer: &[&str]) -> Self {
        let re = Regex::new(r"^(:?(\w+)) can fly (?<speed>\d+) km/s for (?<fly_time>\d+) seconds, but then must rest for (?<rest_time>\d+) seconds\.$").unwrap();
        let reindeer = reindeer.iter().map(|r| Reindeer::new(r, &re)).collect_vec();

        Self { reindeer }
    }

    fn result(&self, time: u32) -> RaceResult {
        let nr_reindeer = self.reindeer.len();
        let mut distances = vec![0; nr_reindeer];
        let mut points = vec![0; nr_reindeer];

        for t in 0..time {
            (0..self.reindeer.len())
                .filter(|&idx| {
                    let r = &self.reindeer[idx];
                    t % (r.fly_time + r.rest_time) < r.fly_time
                })
                .for_each(|idx| distances[idx] += &self.reindeer[idx].speed);

            let max_distance = *distances.iter().max().unwrap();

            (0..self.reindeer.len())
                .filter(|&idx| distances[idx] == max_distance)
                .for_each(|idx| points[idx] += 1);
        }

        let max_distance = *distances.iter().max().unwrap();
        let max_points = *points.iter().max().unwrap();

        RaceResult {
            max_distance,
            max_points,
        }
    }
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn new(reindeer: &str, re: &Regex) -> Reindeer {
        let caps = re.captures(reindeer).unwrap();

        let speed = u32::from_str(caps.name("speed").unwrap().as_str()).unwrap();
        let fly_time = u32::from_str(caps.name("fly_time").unwrap().as_str()).unwrap();
        let rest_time = u32::from_str(caps.name("rest_time").unwrap().as_str()).unwrap();

        Self {
            speed,
            fly_time,
            rest_time,
        }
    }
}

#[derive(Debug)]
struct RaceResult {
    max_distance: u32,
    max_points: u32,
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

    #[test]
    fn day_14_part_02_sample() {
        let sample = vec![
            "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
            "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        ];

        assert_eq!(689, solve_2(&sample, 1_000));
    }

    #[test]
    fn day_14_part_02_solution() {
        let input = include_str!("../../inputs/day_14.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_256, solve_2(&input, 2_503));
    }
}
