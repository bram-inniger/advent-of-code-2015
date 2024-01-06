use std::ops::Not;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve_1(strings: &[&str]) -> usize {
    strings
        .iter()
        .filter(|s| RE_THREE_VOWELS.is_match(s))
        .filter(|s| RE_DOUBLE_LETTER.is_match(s))
        .filter(|s| RE_SUB_STRINGS.is_match(s).not())
        .count()
}

pub fn solve_2(strings: &[&str]) -> usize {
    strings
        .iter()
        .map(|s| s.as_bytes())
        .filter(|a| has_two_repeating(a))
        .filter(|a| has_double_between(a))
        .count()
}

lazy_static! {
    static ref RE_THREE_VOWELS: Regex = Regex::new(r"^.*[aeiou].*[aeiou].*[aeiou].*$").unwrap();
    static ref RE_DOUBLE_LETTER: Regex = Regex::new(
        r"^.*(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz).*$"
    )
    .unwrap();
    static ref RE_SUB_STRINGS: Regex = Regex::new(r"^.*(ab|cd|pq|xy).*$").unwrap();
}

fn has_two_repeating(ascii: &[u8]) -> bool {
    for idx_1 in 0..(ascii.len() - 3) {
        for idx_2 in (idx_1 + 2)..(ascii.len() - 1) {
            if ascii[idx_1] == ascii[idx_2] && ascii[idx_1 + 1] == ascii[idx_2 + 1] {
                return true;
            }
        }
    }

    false
}

fn has_double_between(ascii: &[u8]) -> bool {
    //aba
    for idx in 0..(ascii.len() - 2) {
        if ascii[idx] == ascii[idx + 2] {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_05_part_01_sample() {
        let sample = vec![
            "ugknbfddgicrmopn",
            "aaa",
            "jchzalrnumimnmhp",
            "haegwjzuvuyypxyu",
            "dvszwmarrgswjxmb",
        ];

        assert_eq!(2, solve_1(&sample));
    }

    #[test]
    fn day_05_part_01_solution() {
        let input = include_str!("../../inputs/day_05.txt")
            .lines()
            .collect_vec();

        assert_eq!(236, solve_1(&input));
    }

    #[test]
    fn day_05_part_02_sample() {
        let sample = vec![
            "qjhvhtzxzqqjkmpb",
            "xxyxx",
            "uurcxstgmygtbstg",
            "ieodomkazucvgmuy",
        ];

        assert_eq!(2, solve_2(&sample));
    }

    #[test]
    fn day_05_part_02_solution() {
        let input = include_str!("../../inputs/day_05.txt")
            .lines()
            .collect_vec();

        assert_eq!(51, solve_2(&input));
    }
}
