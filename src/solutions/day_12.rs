use std::str::FromStr;

use regex::Regex;

pub fn solve_1(json: &str) -> i32 {
    Regex::new(r"(-?\d+)")
        .unwrap()
        .find_iter(json)
        .map(|m| m.as_str())
        .map(|n| i32::from_str(n).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_12_part_01_sample() {
        assert_eq!(6, solve_1(r#"[1,2,3]"#));
        assert_eq!(6, solve_1(r#"{"a":2,"b":4}"#));
        assert_eq!(3, solve_1(r#"[[[3]]]"#));
        assert_eq!(3, solve_1(r#"{"a":{"b":4},"c":-1}"#));
        assert_eq!(0, solve_1(r#"{"a":[-1,1]}"#));
        assert_eq!(0, solve_1(r#"[-1,{"a":1}]"#));
        assert_eq!(0, solve_1(r#"[]"#));
        assert_eq!(0, solve_1(r#"{}"#));
    }

    #[test]
    fn day_12_part_01_solution() {
        let input = include_str!("../../inputs/day_12.txt").trim();

        assert_eq!(119_433, solve_1(input));
    }
}
