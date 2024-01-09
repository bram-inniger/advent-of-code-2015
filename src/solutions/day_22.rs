use std::collections::VecDeque;
use std::ops::Not;
use std::str::FromStr;

use rustc_hash::FxHashMap;

pub fn solve_1(boss: &[&str]) -> i32 {
    let boss = Boss::new(boss);
    let book = Book::new();
    let game = Game::new(boss, &book);

    let mut to_play: VecDeque<Game> = VecDeque::new();
    to_play.push_back(game);

    let mut min_mana = i32::MAX;

    while let Some(game) = to_play.pop_front() {
        game.play().into_iter().for_each(|g| match g.state() {
            GameState::OnGoing => {
                if g.spent_mana < min_mana {
                    to_play.push_back(g)
                }
            }
            GameState::Won => min_mana = min_mana.min(g.spent_mana),
            GameState::Lost => {}
        });
    }

    min_mana
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Boss {
    hit_points: i32,
    damage: i32,
}

impl Boss {
    fn new(boss: &[&str]) -> Self {
        let hit_points = i32::from_str(&boss[0][12..]).unwrap();
        let damage = i32::from_str(&boss[1][8..]).unwrap();

        Self { hit_points, damage }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Game<'a> {
    actor: Actor,
    mana: i32,
    spent_mana: i32,
    player_hp: i32,
    boss_hp: i32,
    boss_damage: i32,
    book: &'a Book,
    effects: FxHashMap<&'a Spell, i32>,
}

impl<'a> Game<'a> {
    fn new(boss: Boss, book: &'a Book) -> Self {
        Self {
            actor: Actor::Player,
            mana: 500,
            spent_mana: 0,
            player_hp: 50,
            boss_hp: boss.hit_points,
            boss_damage: boss.damage,
            book,
            effects: Default::default(),
        }
    }

    fn play(&self) -> Vec<Self> {
        let mut player_mana = self.mana;
        let mut player_armor = 0;
        let mut boss_hp = self.boss_hp;

        // First deal with active effects
        self.effects.iter().for_each(|(&s, _)| match s {
            Spell::Shield { armor, .. } => player_armor = *armor,
            Spell::Poison { damage, .. } => boss_hp -= damage,
            Spell::Recharge { mana, .. } => player_mana += *mana,
            _ => unreachable!(),
        });

        // If the active effect kills the boss, return the winning game
        if boss_hp <= 0 {
            let mut game = self.clone();
            game.boss_hp = boss_hp;
            return vec![game];
        }

        // Decrease the timer of all ongoing effects, remove the ones that ended
        let effects: FxHashMap<&'a Spell, i32> = self
            .effects
            .iter()
            .map(|(&s, &t)| (s, t - 1))
            .filter(|(_, t)| t > &0)
            .collect();

        match self.actor {
            // Play the turn as the player, using any available spell (enough mana + not already in use)
            Actor::Player => self
                .book
                .spells
                .iter()
                .filter(|&s| player_mana >= s.cost())
                .filter(|&s| effects.contains_key(s).not())
                .map(|s| match s {
                    Spell::MagicMissile { cost, damage } => Game {
                        actor: Actor::Boss,
                        mana: player_mana - cost,
                        spent_mana: self.spent_mana + cost,
                        player_hp: self.player_hp,
                        boss_hp: boss_hp - damage,
                        boss_damage: self.boss_damage,
                        book: self.book,
                        effects: effects.clone(),
                    },
                    Spell::Drain { cost, damage, heal } => Game {
                        actor: Actor::Boss,
                        mana: player_mana - cost,
                        spent_mana: self.spent_mana + cost,
                        player_hp: self.player_hp + heal,
                        boss_hp: boss_hp - damage,
                        boss_damage: self.boss_damage,
                        book: self.book,
                        effects: effects.clone(),
                    },
                    _ => {
                        let mut effects = effects.clone();
                        effects.insert(s, s.turns());

                        Game {
                            actor: Actor::Boss,
                            mana: player_mana - s.cost(),
                            spent_mana: self.spent_mana + s.cost(),
                            player_hp: self.player_hp,
                            boss_hp,
                            boss_damage: self.boss_damage,
                            book: self.book,
                            effects,
                        }
                    }
                })
                .collect(),
            // Play the turn as the boss, dealing damage
            Actor::Boss => {
                let player_hp = self.player_hp - 1.max(self.boss_damage - player_armor);

                vec![Self {
                    actor: Actor::Player,
                    mana: player_mana,
                    spent_mana: self.spent_mana,
                    player_hp,
                    boss_hp,
                    boss_damage: self.boss_damage,
                    book: self.book,
                    effects,
                }]
            }
        }
    }

    fn state(&self) -> GameState {
        if self.player_hp > 0 && self.boss_hp > 0 {
            GameState::OnGoing
        } else if self.player_hp > 0 {
            GameState::Won
        } else {
            GameState::Lost
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Actor {
    Player,
    Boss,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum GameState {
    OnGoing,
    Won,
    Lost,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Book {
    spells: Vec<Spell>,
}

impl Book {
    fn new() -> Self {
        Self {
            spells: vec![
                Spell::MagicMissile {
                    cost: 53,
                    damage: 4,
                },
                Spell::Drain {
                    cost: 73,
                    damage: 2,
                    heal: 2,
                },
                Spell::Shield {
                    cost: 113,
                    armor: 7,
                    turns: 6,
                },
                Spell::Poison {
                    cost: 173,
                    damage: 3,
                    turns: 6,
                },
                Spell::Recharge {
                    cost: 229,
                    mana: 101,
                    turns: 5,
                },
            ],
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Spell {
    MagicMissile { cost: i32, damage: i32 },
    Drain { cost: i32, damage: i32, heal: i32 },
    Shield { cost: i32, armor: i32, turns: i32 },
    Poison { cost: i32, damage: i32, turns: i32 },
    Recharge { cost: i32, mana: i32, turns: i32 },
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile { cost, .. } => *cost,
            Spell::Drain { cost, .. } => *cost,
            Spell::Shield { cost, .. } => *cost,
            Spell::Poison { cost, .. } => *cost,
            Spell::Recharge { cost, .. } => *cost,
        }
    }
    fn turns(&self) -> i32 {
        match self {
            Spell::Shield { turns, .. } => *turns,
            Spell::Poison { turns, .. } => *turns,
            Spell::Recharge { turns, .. } => *turns,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day_22_part_01_sample() {
        // No sample inputs for part 1
    }

    #[test]
    fn day_22_part_01_solution() {
        let input = include_str!("../../inputs/day_22.txt")
            .lines()
            .collect_vec();

        assert_eq!(953, solve_1(&input));
    }
}
