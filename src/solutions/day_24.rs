use std::ops::Not;
use std::str::FromStr;
use itertools::Itertools;

pub fn solve_1(packages: &[&str]) -> u64 {
    let packages = packages.iter().map(|p| u64::from_str(p).unwrap()).collect_vec();

    smallest_groups(&packages)
        .iter()
        .map(|g| quantum_entanglement(g))
        .min()
        .unwrap()
}

fn smallest_groups(packages: &[u64]) -> Vec<Vec<&u64>> {
    let group_sum = packages.iter().sum::<u64>() / 3;

    for i in 1.. {
        let permutations = packages.iter()
            .permutations(i)
            .filter(|p| p.iter().copied().sum::<u64>() == group_sum)
            .collect_vec();

        if permutations.is_empty().not() {
            return permutations
        }
    }

    unreachable!()
}

fn quantum_entanglement(group: &[&u64]) -> u64 {
    group.iter().copied().product::<u64>()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_24_part_01_sample() {
        let sample = vec![
            "1",
            "2",
            "3",
            "4",
            "5",
            "7",
            "8",
            "9",
            "10",
            "11",
        ];

        assert_eq!(99, solve_1(&sample));
    }

    #[ignore = "slow brute force test"]
    #[test]
    fn day_24_part_01_solution() {
        let input = include_str!("../../inputs/day_24.txt")
            .lines()
            .collect_vec();

        assert_eq!(10_439_961_859, solve_1(&input));
    }
}
