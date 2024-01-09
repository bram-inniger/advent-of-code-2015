use std::str::FromStr;

use regex::Regex;

pub fn solve_1(message: &str) -> u64 {
    let code = Code::new(message);
    calculate_nth(index_of(&code))
}

fn index_of(code: &Code) -> u64 {
    // Derived on paper
    (code.row - 1) * (code.row) / 2
        + (code.col - 1) * (code.col) / 2
        + (code.col - 1) * code.row
        + 1
}

fn calculate_nth(n: u64) -> u64 {
    let mut result = 20_151_125;

    for _ in 1..n {
        result *= 252_533;
        result %= 33_554_393;
    }

    result
}

#[derive(Debug)]
struct Code {
    row: u64,
    col: u64,
}

impl Code {
    fn new(message: &str) -> Self {
        let re = Regex::new(r"To continue, please consult the code grid in the manual\. {2}Enter the code at row (?<row>\d+), column (?<col>\d+)\.").unwrap();
        let caps = re.captures(message).unwrap();

        Self {
            row: u64::from_str(caps.name("row").unwrap().as_str()).unwrap(),
            col: u64::from_str(caps.name("col").unwrap().as_str()).unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_25_part_01_sample() {
        let sample = "To continue, please consult the code grid in the manual.  Enter the code at row 4, column 2.";

        assert_eq!(32_451_966, solve_1(sample));
    }

    #[test]
    fn day_25_part_01_solution() {
        let input = include_str!("../../inputs/day_25.txt").trim();

        assert_eq!(2_650_453, solve_1(input));
    }
}
