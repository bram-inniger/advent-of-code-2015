use itertools::Itertools;
use std::str::FromStr;

use regex::Regex;

pub fn solve_1(ingredients: &[&str]) -> i64 {
    Recipe::new(ingredients).perfect(false)
}

pub fn solve_2(ingredients: &[&str]) -> i64 {
    Recipe::new(ingredients).perfect(true)
}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<Ingredient>,
}

impl Recipe {
    fn new(ingredients: &[&str]) -> Self {
        let re =
            Regex::new(r"^(:?\w+): capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>-?\d+)$")
                .unwrap();
        let mut ingredients = ingredients
            .iter()
            .map(|i| Ingredient::new(i, &re))
            .collect_vec();

        // for the sample code case, we add dummy ingredients to keep the code short
        if ingredients.len() == 2 {
            let bad = Ingredient {
                capacity: -999,
                durability: -999,
                flavor: -999,
                texture: -999,
                calories: -999,
            };
            ingredients.push(bad);
            ingredients.push(bad);
        }

        Self { ingredients }
    }

    fn perfect(&self, calories: bool) -> i64 {
        let mut perfect = 0;

        for a in 0..=100 {
            for b in 0..=100 - a {
                for c in 0..=100 - a - b {
                    let d = 100 - a - b - c;
                    perfect = perfect.max(Self::score(self, &vec![a, b, c, d], calories));
                }
            }
        }

        perfect
    }

    fn score(&self, proportions: &Vec<i64>, calories: bool) -> i64 {
        let total_capacity = (0..proportions.len())
            .map(|idx| self.ingredients[idx].capacity * proportions[idx])
            .sum::<i64>()
            .max(0);
        let total_durability = (0..proportions.len())
            .map(|idx| self.ingredients[idx].durability * proportions[idx])
            .sum::<i64>()
            .max(0);
        let total_flavor = (0..proportions.len())
            .map(|idx| self.ingredients[idx].flavor * proportions[idx])
            .sum::<i64>()
            .max(0);
        let total_texture = (0..proportions.len())
            .map(|idx| self.ingredients[idx].texture * proportions[idx])
            .sum::<i64>()
            .max(0);
        let total_calories = (0..proportions.len())
            .map(|idx| self.ingredients[idx].calories * proportions[idx])
            .sum::<i64>()
            .max(0);

        if calories && total_calories != 500 {
            0
        } else {
            total_capacity * total_durability * total_flavor * total_texture
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn new(ingredient: &str, re: &Regex) -> Self {
        let caps = re.captures(ingredient).unwrap();

        let capacity = i64::from_str(caps.name("capacity").unwrap().as_str()).unwrap();
        let durability = i64::from_str(caps.name("durability").unwrap().as_str()).unwrap();
        let flavor = i64::from_str(caps.name("flavor").unwrap().as_str()).unwrap();
        let texture = i64::from_str(caps.name("texture").unwrap().as_str()).unwrap();
        let calories = i64::from_str(caps.name("calories").unwrap().as_str()).unwrap();

        Self {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_15_part_01_sample() {
        let sample = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ];

        assert_eq!(62_842_880, solve_1(&sample));
    }

    #[test]
    fn day_15_part_01_solution() {
        let input = include_str!("../../inputs/day_15.txt")
            .lines()
            .collect_vec();

        assert_eq!(13_882_464, solve_1(&input));
    }

    #[test]
    fn day_15_part_02_sample() {
        let sample = vec![
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8",
            "Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        ];

        assert_eq!(57_600_000, solve_2(&sample));
    }

    #[test]
    fn day_15_part_02_solution() {
        let input = include_str!("../../inputs/day_15.txt")
            .lines()
            .collect_vec();

        assert_eq!(11_171_160, solve_2(&input));
    }
}
