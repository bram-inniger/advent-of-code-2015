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
    let mut lights = Lights::new(lights);

    for _ in 0..steps {
        lights = lights.step();
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

    fn step(&self) -> Self {
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

        Self {
            grid,
            x_max: self.x_max,
            y_max: self.y_max,
        }
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
}
