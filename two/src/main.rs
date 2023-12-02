#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;

#[derive(Debug, HasParser)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, HasParser)]
struct Round {
    count: u64,
    color: Color,
}

impl Round {
    fn is_possible(&self, r: u64, g: u64, b: u64) -> bool {
        match self.color {
            Color::Red => self.count <= r,
            Color::Green => self.count <= g,
            Color::Blue => self.count <= b,
        }
    }

    fn min_cubes(&self, r: &mut u64, g: &mut u64, b: &mut u64) {
        match self.color {
            Color::Red => *r = std::cmp::max(self.count, *r),
            Color::Green => *g = std::cmp::max(self.count, *g),
            Color::Blue => *b = std::cmp::max(self.count, *b),
        }
    }
}

#[derive(Debug, HasParser)]
struct Game {
    #[parse(before = "Game ", after = ":")]
    id: u64,
    rounds: List<List<Round, SepBy<CommaSpace>>, SepBy<SemiSpace>>,
}

impl Game {
    fn is_possible(&self, r: u64, g: u64, b: u64) -> bool {
        self.rounds
            .iter()
            .all(|rounds| rounds.iter().all(|round| round.is_possible(r, g, b)))
    }

    fn min_cubes_power(&self) -> u64 {
        let (mut r, mut g, mut b) = (0, 0, 0);
        self.rounds.iter().for_each(|rounds| {
            rounds
                .iter()
                .for_each(|round| round.min_cubes(&mut r, &mut g, &mut b))
        });
        r * g * b
    }
}

#[part_one]
fn part_one(games: List<Game, TermWith<NewLine>>) -> u64 {
    games
        .iter()
        .filter_map(|g| g.is_possible(12, 13, 14).then_some(g.id))
        .sum()
}

#[part_two]
fn part_two(games: List<Game, TermWith<NewLine>>) -> u64 {
    games.iter().map(|g| g.min_cubes_power()).sum()
}

harness!(part_1: 2317, part_2: 74804);
