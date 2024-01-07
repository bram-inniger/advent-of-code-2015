use itertools::Itertools;
use rustc_hash::FxHashMap;

const NEIGHBOURS: [Coordinate; 8] = [
    Coordinate { x: 0, y: -1 },
    Coordinate { x: 1, y: -1 },
    Coordinate { x: 1, y: 0 },
    Coordinate { x: 1, y: 1 },
    Coordinate { x: 0, y: 1 },
    Coordinate { x: -1, y: 1 },
    Coordinate { x: -1, y: 0 },
    Coordinate { x: -1, y: -1 },
];

pub fn solve_1(lights: &[&str], steps: u8) -> usize {
    solve(lights, steps, false)
}

pub fn solve_2(lights: &[&str], steps: u8) -> usize {
    solve(lights, steps, true)
}

fn solve(lights: &[&str], steps: u8, stuck: bool) -> usize {
    let mut lights = Lights::new(lights);

    if stuck {
        lights.light_corners()
    }

    for _ in 0..steps {
        lights = lights.step(stuck);
    }

    lights
        .grid
        .values()
        .filter(|s| matches!(s, State::On))
        .count()
}

#[derive(Debug)]
struct Lights {
    grid: FxHashMap<Coordinate, State>,
    x_max: i8,
    y_max: i8,
}

impl Lights {
    fn new(lights: &[&str]) -> Self {
        let lights = lights.iter().map(|s| s.as_bytes()).collect_vec();
        let mut grid = FxHashMap::default();

        for y in 0..lights.len() {
            for x in 0..lights[0].len() {
                let state = match lights[y][x] {
                    b'#' => State::On,
                    b'.' => State::Off,
                    _ => unreachable!(),
                };
                let coordinate = Coordinate {
                    x: x as i8,
                    y: y as i8,
                };

                grid.insert(coordinate, state);
            }
        }

        let x_max = lights[0].len() as i8;
        let y_max = lights.len() as i8;

        Self { grid, x_max, y_max }
    }

    fn step(&self, stuck: bool) -> Self {
        let grid: FxHashMap<Coordinate, State> = self
            .grid
            .iter()
            .map(|(&c, &s)| {
                let lit = self.lit_neighbours(c);
                let state = match s {
                    State::On => {
                        if lit == 2 || lit == 3 {
                            State::On
                        } else {
                            State::Off
                        }
                    }
                    State::Off => {
                        if lit == 3 {
                            State::On
                        } else {
                            State::Off
                        }
                    }
                };

                (c, state)
            })
            .collect();

        let mut lights = Self {
            grid,
            x_max: self.x_max,
            y_max: self.y_max,
        };

        if stuck {
            lights.light_corners()
        }

        lights
    }

    fn lit_neighbours(&self, c: Coordinate) -> usize {
        NEIGHBOURS
            .iter()
            .filter(|n| {
                let neighbour = Coordinate {
                    x: c.x + n.x,
                    y: c.y + n.y,
                };
                matches!(self.grid.get(&neighbour), Some(State::On))
            })
            .count()
    }

    fn light_corners(&mut self) {
        [
            Coordinate { x: 0, y: 0 },
            Coordinate {
                x: self.x_max - 1,
                y: 0,
            },
            Coordinate {
                x: 0,
                y: self.y_max - 1,
            },
            Coordinate {
                x: self.x_max - 1,
                y: self.y_max - 1,
            },
        ]
        .into_iter()
        .for_each(|c| {
            self.grid.insert(c, State::On);
        });
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i8,
    y: i8,
}

#[derive(Debug, Copy, Clone)]
enum State {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_18_part_01_sample() {
        let sample = vec![".#.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.."];

        assert_eq!(4, solve_1(&sample, 4));
    }

    #[test]
    fn day_18_part_01_solution() {
        let input = include_str!("../../inputs/day_18.txt")
            .lines()
            .collect_vec();

        assert_eq!(814, solve_1(&input, 100));
    }

    #[test]
    fn day_18_part_02_sample() {
        let sample = vec!["##.#.#", "...##.", "#....#", "..#...", "#.#..#", "####.#"];

        assert_eq!(17, solve_2(&sample, 5));
    }

    #[test]
    fn day_18_part_02_solution() {
        let input = include_str!("../../inputs/day_18.txt")
            .lines()
            .collect_vec();

        assert_eq!(924, solve_2(&input, 100));
    }
}
