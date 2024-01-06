use regex::Regex;

pub fn solve_1(strings: &[&str]) -> usize {
    let re = Regex::new(r"(\\x[0-9a-f]{2})").unwrap();

    let code_characters: usize = strings.iter().map(|s| s.len()).sum();
    let memory_characters: usize = strings
        .iter()
        .map(|s| &s[1..s.len() - 1])
        .map(|s| s.replace("\\\\", "\\"))
        .map(|s| s.replace("\\\"", "\""))
        .map(|s| re.replace_all(&s, "#").to_string())
        .map(|s| s.len())
        .sum();

    code_characters - memory_characters
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_08_part_01_sample() {
        let sample = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "\"\\x27\""];

        assert_eq!(12, solve_1(&sample));
    }

    #[test]
    fn day_08_part_01_solution() {
        let input = include_str!("../../inputs/day_08.txt")
            .lines()
            .collect_vec();

        assert_eq!(1_333, solve_1(&input));
    }
}
