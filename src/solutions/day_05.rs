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

lazy_static! {
    static ref RE_THREE_VOWELS: Regex = Regex::new(r"^.*[aeiou].*[aeiou].*[aeiou].*$").unwrap();
    static ref RE_DOUBLE_LETTER: Regex = Regex::new(
        r"^.*(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz).*$"
    )
    .unwrap();
    static ref RE_SUB_STRINGS: Regex = Regex::new(r"^.*(ab|cd|pq|xy).*$").unwrap();
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
}
