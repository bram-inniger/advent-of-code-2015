use std::str::FromStr;

pub fn solve_1(nr_presents: &str) -> usize {
    let nr_presents = usize::from_str(nr_presents).unwrap();
    let nr_houses = 1_000_000;

    let mut houses: Vec<usize> = vec![0; nr_houses];

    for house in 1..nr_houses {
        for idx in (house..nr_houses).step_by(house) {
            houses[idx] += house * 10;
        }
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, &nr_p)| nr_p >= nr_presents)
        .map(|(house, _)| house)
        .unwrap()
}

pub fn solve_2(nr_presents: &str) -> usize {
    let nr_presents = usize::from_str(nr_presents).unwrap();
    let nr_houses = 1_000_000;

    let mut houses: Vec<usize> = vec![0; nr_houses];

    for house in 1..nr_houses {
        for idx in (house..nr_houses.min(51 * house)).step_by(house) {
            houses[idx] += house * 11;
        }
    }

    houses
        .iter()
        .enumerate()
        .find(|(_, &nr_p)| nr_p >= nr_presents)
        .map(|(house, _)| house)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_20_part_01_sample() {
        assert_eq!(1, solve_1("10"));
        assert_eq!(4, solve_1("70"));
    }

    #[test]
    fn day_20_part_01_solution() {
        let input = include_str!("../../inputs/day_20.txt").trim();

        assert_eq!(776_160, solve_1(input));
    }

    #[test]
    fn day_20_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_20_part_02_solution() {
        let input = include_str!("../../inputs/day_20.txt").trim();

        assert_eq!(786_240, solve_2(input));
    }
}
