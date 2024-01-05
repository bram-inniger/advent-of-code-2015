use rustc_hash::FxHashSet;

pub fn solve_1(directions: &str) -> usize {
    let mut visited: FxHashSet<Coordinate> = FxHashSet::default();
    let mut current = Coordinate { x: 0, y: 0 };

    visited.insert(current);

    for d in directions.chars() {
        match d {
            '^' => current.y += 1,
            '>' => current.x += 1,
            'v' => current.y -= 1,
            '<' => current.x -= 1,
            _ => unreachable!(),
        }

        visited.insert(current);
    }

    visited.len()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_part_01_sample() {
        assert_eq!(2, solve_1(">"));
        assert_eq!(4, solve_1("^>v<"));
        assert_eq!(2, solve_1("^v^v^v^v^v"));
    }

    #[test]
    fn day_03_part_01_solution() {
        let input = include_str!("../../inputs/day_03.txt").trim();

        assert_eq!(2_592, solve_1(input));
    }
}
