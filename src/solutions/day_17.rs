use itertools::Itertools;
use std::str::FromStr;

pub fn solve_1(containers: &[&str], liters: u16) -> usize {
    let containers = containers
        .iter()
        .map(|c| u16::from_str(c).unwrap())
        .collect_vec();

    let mut current = Vec::with_capacity(containers.len());
    let mut acc = Vec::new();
    generate_combinations(containers.len(), &mut current, &mut acc);

    acc.iter()
        .filter(|cs| {
            cs.iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(idx, _)| containers[idx])
                .sum::<u16>()
                == liters
        })
        .count()
}

fn generate_combinations(to_add: usize, current: &mut Vec<bool>, acc: &mut Vec<Vec<bool>>) {
    if to_add == 0 {
        acc.push(current.clone());
    } else {
        current.push(false);
        generate_combinations(to_add - 1, current, acc);
        current.pop();

        current.push(true);
        generate_combinations(to_add - 1, current, acc);
        current.pop();
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_17_part_01_sample() {
        let sample = vec!["20", "15", "10", "5", "5"];

        assert_eq!(4, solve_1(&sample, 25));
    }

    #[test]
    fn day_17_part_01_solution() {
        let input = include_str!("../../inputs/day_17.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_638, solve_1(&input, 150));
    }
}
