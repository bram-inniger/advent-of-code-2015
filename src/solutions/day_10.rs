use crate::util::BASE_10;
use itertools::Itertools;

pub fn solve_1(start: &str, nr_steps: u8) -> usize {
    let mut number = start
        .chars()
        .map(|c| c.to_digit(BASE_10).unwrap())
        .collect_vec();

    (0..nr_steps).for_each(|_| number = expand(&number));

    number.len()
}

fn expand(number: &[u32]) -> Vec<u32> {
    let mut expanded = vec![];

    let mut digit = number[0];
    let mut occurrences = 1;

    for &current in &number[1..] {
        if current == digit {
            occurrences += 1;
        } else {
            expanded.push(occurrences);
            expanded.push(digit);

            digit = current;
            occurrences = 1;
        }
    }

    expanded.push(occurrences);
    expanded.push(digit);

    expanded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_10_part_01_sample() {
        let sample = "1";

        assert_eq!(6, solve_1(sample, 5));
    }

    #[test]
    fn day_10_part_01_solution() {
        let input = include_str!("../../inputs/day_10.txt").trim();

        assert_eq!(252_594, solve_1(input, 40));
    }
}
