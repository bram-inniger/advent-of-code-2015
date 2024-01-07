use std::str::FromStr;

use itertools::Itertools;

pub fn solve_1(containers: &[&str], liters: u16) -> usize {
    let containers = parse_containers(containers);
    let acc = generate_combinations(&containers);

    fitting(liters, &containers, &acc).count()
}

pub fn solve_2(containers: &[&str], liters: u16) -> usize {
    let containers = parse_containers(containers);
    let combinations = generate_combinations(&containers);

    let min_containers = fitting(liters, &containers, &combinations)
        .map(|cs| cs.iter().filter(|&&b| b).count())
        .min()
        .unwrap();
    let smallest_combinations = combinations
        .into_iter()
        .filter(|cs| cs.iter().filter(|&&b| b).count() == min_containers)
        .collect_vec();

    fitting(liters, &containers, &smallest_combinations).count()
}

fn parse_containers(containers: &[&str]) -> Vec<u16> {
    containers
        .iter()
        .map(|c| u16::from_str(c).unwrap())
        .collect_vec()
}

fn generate_combinations(containers: &Vec<u16>) -> Vec<Vec<bool>> {
    let mut current = Vec::with_capacity(containers.len());
    let mut acc = Vec::new();
    generate_combinations_rec(containers.len(), &mut current, &mut acc);
    acc
}

fn generate_combinations_rec(to_add: usize, current: &mut Vec<bool>, acc: &mut Vec<Vec<bool>>) {
    if to_add == 0 {
        acc.push(current.clone());
    } else {
        current.push(false);
        generate_combinations_rec(to_add - 1, current, acc);
        current.pop();

        current.push(true);
        generate_combinations_rec(to_add - 1, current, acc);
        current.pop();
    }
}

fn fitting<'a>(
    liters: u16,
    containers: &'a [u16],
    acc: &'a [Vec<bool>],
) -> impl Iterator<Item = &'a Vec<bool>> + 'a {
    acc.iter().filter(move |cs| {
        cs.iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(idx, _)| containers[idx])
            .sum::<u16>()
            == liters
    })
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

    #[test]
    fn day_17_part_02_sample() {
        let sample = vec!["20", "15", "10", "5", "5"];

        assert_eq!(3, solve_2(&sample, 25));
    }

    #[test]
    fn day_17_part_02_solution() {
        let input = include_str!("../../inputs/day_17.txt")
            .lines()
            .collect_vec();

        assert_eq!(17, solve_2(&input, 150));
    }
}
