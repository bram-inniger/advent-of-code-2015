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

pub fn solve_2(instructions: &str) -> u16 {
    let mut position = 0;
    let mut floor = 0;

    for c in instructions.chars() {
        position += 1;
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        };

        if floor < 0 {
            return position;
        }
    }

    unreachable!()
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

    #[test]
    fn day_01_part_02_sample() {
        assert_eq!(1, solve_2(")"));
        assert_eq!(5, solve_2("()())"));
    }

    #[test]
    fn day_01_part_02_solution() {
        let input = include_str!("../../inputs/day_01.txt").trim();

        assert_eq!(1_795, solve_2(input));
    }
}
