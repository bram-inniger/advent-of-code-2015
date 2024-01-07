use std::ops::Not;

use itertools::Itertools;

pub fn solve(password: &str) -> String {
    let mut password = Password::new(password);
    password.increment();

    while password.is_valid().not() {
        password.increment();
    }

    String::from_utf8(password.text).unwrap()
}

#[derive(Debug)]
struct Password {
    text: Vec<u8>,
}

impl Password {
    fn new(password: &str) -> Self {
        Password {
            text: password.chars().map(|c| c as u8).collect_vec(),
        }
    }

    fn increment(&mut self) {
        for c in self.text.iter_mut().rev() {
            if c == &b'z' {
                *c = b'a';
            } else {
                *c += 1;
                return;
            }
        }
    }

    fn is_valid(&self) -> bool {
        Self::straight(self) && Self::no_confusion(self) && Self::pairs(self)
    }

    fn straight(&self) -> bool {
        (2..self.text.len())
            .filter(|&idx| self.text[idx] >= b'c')
            .any(|idx| {
                self.text[idx - 2] == self.text[idx] - 2 && self.text[idx - 1] == self.text[idx] - 1
            })
    }

    fn no_confusion(&self) -> bool {
        self.text.contains(&b'i').not()
            && self.text.contains(&b'o').not()
            && self.text.contains(&b'l').not()
    }

    fn pairs(&self) -> bool {
        (1..self.text.len())
            .filter(|&idx| self.text[idx - 1] == self.text[idx])
            .map(|idx| self.text[idx])
            .unique()
            .count()
            >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_11_part_01_sample() {
        assert_eq!("abcdffaa", solve("abcdefgh"));
        assert_eq!("ghjaabcc", solve("ghijklmn"));
    }

    #[test]
    fn day_11_part_01_solution() {
        let input = include_str!("../../inputs/day_11.txt").trim();

        assert_eq!("vzbxxyzz", solve(input));
    }

    #[test]
    fn day_11_part_02_sample() {
        // No sample inputs for part 2
    }

    #[test]
    fn day_11_part_02_solution() {
        let input = include_str!("../../inputs/day_11.txt").trim();

        assert_eq!("vzcaabcc", solve(&solve(input)));
    }
}
