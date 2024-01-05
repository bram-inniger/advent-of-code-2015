pub fn solve_1(instructions: &str) -> i16 {
    instructions
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_part_01_sample() {
        assert_eq!(0, solve_1("(())"));
        assert_eq!(0, solve_1("()()"));
        assert_eq!(3, solve_1("((("));
        assert_eq!(3, solve_1("(()(()("));
        assert_eq!(3, solve_1("))((((("));
        assert_eq!(-1, solve_1("())"));
        assert_eq!(-1, solve_1("))("));
        assert_eq!(-3, solve_1(")))"));
        assert_eq!(-3, solve_1(")())())"));
    }

    #[test]
    fn day_01_part_01_solution() {
        let input = include_str!("../../inputs/day_01.txt").trim();

        assert_eq!(74, solve_1(input));
    }
}
