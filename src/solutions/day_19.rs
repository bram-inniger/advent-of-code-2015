use itertools::Itertools;
use rustc_hash::FxHashSet;

pub fn solve_1(machine: &str) -> usize {
    Machine::new(machine).generate_molecules().len()
}

#[derive(Debug)]
struct Machine<'a> {
    transformations: Vec<(&'a str, &'a str)>,
    molecule: &'a str,
}

impl<'a> Machine<'a> {
    fn new(machine: &'a str) -> Self {
        let split = machine.split("\n\n").collect_vec();

        let transformations = split[0]
            .split('\n')
            .map(|s| {
                let replacement = s.split(" => ").collect_vec();
                (replacement[0], replacement[1])
            })
            .collect();
        let molecule = split[1];

        Self {
            transformations,
            molecule,
        }
    }

    fn generate_molecules(&self) -> FxHashSet<String> {
        self.transformations
            .iter()
            .flat_map(|(from, to)| {
                self.molecule.match_indices(from).map(move |(idx, _)| {
                    format!(
                        "{}{}{}",
                        &self.molecule[..idx],
                        to,
                        &self.molecule[(idx + from.len())..]
                    )
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_19_part_01_sample() {
        let sample = "H => HO\n\
            H => OH\n\
            O => HH\n\
            \n\
            HOH";

        assert_eq!(4, solve_1(sample));

        let sample = "H => HO\n\
            H => OH\n\
            O => HH\n\
            \n\
            HOHOHO";

        assert_eq!(7, solve_1(sample));
    }

    #[test]
    fn day_19_part_01_solution() {
        let input = include_str!("../../inputs/day_19.txt").trim();

        assert_eq!(509, solve_1(input));
    }
}
