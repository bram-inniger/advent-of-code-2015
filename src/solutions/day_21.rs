use itertools::{iproduct, Itertools};
use std::str::FromStr;

const PLAYER_HP: i16 = 100;

pub fn solve_1(boss: &[&str]) -> i16 {
    let boss = Boss::new(boss);
    let shop = Shop::new();

    shop.inventories()
        .iter()
        .filter(|inv| boss.wins(inv))
        .map(|inv| inv.iter().map(|i| i.cost).sum::<i16>())
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Boss {
    hit_points: i16,
    damage: i16,
    armor: i16,
}

impl Boss {
    fn new(boss: &[&str]) -> Self {
        let hit_points = i16::from_str(&boss[0][12..]).unwrap();
        let damage = i16::from_str(&boss[1][8..]).unwrap();
        let armor = i16::from_str(&boss[2][7..]).unwrap();

        Self {
            hit_points,
            damage,
            armor,
        }
    }

    fn wins(&self, inventory: &[&Item]) -> bool {
        let player_damage = 1.max(inventory.iter().map(|i| i.damage).sum::<i16>() - self.armor);
        let boss_damage = 1.max(self.damage - inventory.iter().map(|i| i.armor).sum::<i16>());

        let win_turns = (self.hit_points + player_damage - 1) / player_damage;
        let lose_turns = (PLAYER_HP + boss_damage - 1) / boss_damage;

        win_turns <= lose_turns
    }
}

#[derive(Debug)]
struct Shop<'a> {
    weapons: Vec<Item<'a>>,
    armor: Vec<Item<'a>>,
    rings: Vec<Item<'a>>,
}

impl<'a> Shop<'a> {
    fn new() -> Self {
        Self {
            weapons: vec![
                Item::attack("Dagger", 4, 8),
                Item::attack("Shortsword", 5, 10),
                Item::attack("Warhammer", 6, 25),
                Item::attack("Longsword", 7, 40),
                Item::attack("Greataxe", 8, 74),
            ],
            armor: vec![
                Item::defense("Leather", 1, 13),
                Item::defense("Chainmail", 2, 31),
                Item::defense("Splintmail", 3, 53),
                Item::defense("Bandedmail", 4, 75),
                Item::defense("Platemail", 5, 102),
            ],
            rings: vec![
                Item::attack("Damage +1", 1, 25),
                Item::attack("Damage +2", 2, 50),
                Item::attack("Damage +3", 3, 100),
                Item::defense("Defense +1", 1, 20),
                Item::defense("Defense +2", 2, 40),
                Item::defense("Defense +3", 3, 80),
            ],
        }
    }

    fn inventories(&self) -> Vec<Vec<&Item<'a>>> {
        let one_weapon = self.weapons.iter().permutations(1).collect_vec();
        let zero_armor = self.armor.iter().permutations(0).collect_vec();
        let one_armor = self.armor.iter().permutations(1).collect_vec();
        let zero_rings = self.rings.iter().permutations(0).collect_vec();
        let one_ring = self.rings.iter().permutations(1).collect_vec();
        let two_rings = self.rings.iter().permutations(2).collect_vec();

        let weapon_choices = one_weapon;
        let armor_choices = zero_armor.into_iter().chain(one_armor).collect_vec();
        let ring_choices = zero_rings
            .into_iter()
            .chain(one_ring)
            .chain(two_rings)
            .collect_vec();

        iproduct!(weapon_choices, armor_choices, ring_choices)
            .map(|(w, a, r)| w.into_iter().chain(a).chain(r).collect_vec())
            .collect()
    }
}

#[derive(Debug)]
struct Item<'a> {
    _name: &'a str,
    cost: i16,
    damage: i16,
    armor: i16,
}

impl<'a> Item<'a> {
    fn attack(_name: &'a str, value: i16, cost: i16) -> Self {
        Self {
            _name,
            cost,
            damage: value,
            armor: 0,
        }
    }

    fn defense(_name: &'a str, value: i16, cost: i16) -> Self {
        Self {
            _name,
            cost,
            damage: 0,
            armor: value,
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_21_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_21_part_01_solution() {
        let input = include_str!("../../inputs/day_21.txt")
            .lines()
            .collect_vec();

        assert_eq!(78, solve_1(&input));
    }
}
