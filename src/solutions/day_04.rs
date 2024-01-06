use md5::{Digest, Md5};

pub fn solve_1(key: &str) -> u32 {
    let mut number = 0;

    loop {
        let mut hasher = Md5::new();
        hasher.update(format!("{}{}", key, number));
        let result = hex::encode(hasher.finalize());

        if result.starts_with("00000") {
            return number
        }

        number += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_part_01_sample() {
        assert_eq!(609_043, solve_1("abcdef"));
        assert_eq!(1_048_970, solve_1("pqrstuv"));
    }

    #[test]
    fn day_04_part_01_solution() {
        let input = include_str!("../../inputs/day_04.txt").trim();

        assert_eq!(282_749, solve_1(input));
    }
}
