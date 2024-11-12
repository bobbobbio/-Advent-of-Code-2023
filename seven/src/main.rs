use advent::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Clone, Debug, Hash, HasParser, PartialOrd, Ord, PartialEq, Eq)]
enum Card {
    #[parse(string = "O")]
    Joker,
    #[parse(string = "2")]
    Two,
    #[parse(string = "3")]
    Three,
    #[parse(string = "4")]
    Four,
    #[parse(string = "5")]
    Five,
    #[parse(string = "6")]
    Six,
    #[parse(string = "7")]
    Seven,
    #[parse(string = "8")]
    Eight,
    #[parse(string = "9")]
    Nine,
    #[parse(string = "T")]
    T,
    #[parse(string = "J")]
    J,
    #[parse(string = "Q")]
    Q,
    #[parse(string = "K")]
    K,
    #[parse(string = "A")]
    A,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Debug, HasParser, PartialOrd, Ord, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn to_kind_no_joker(&self) -> Kind {
        let mut card_counts: HashMap<Card, u64> = HashMap::new();
        for c in &self.cards {
            *card_counts.entry(c.clone()).or_insert(0) += 1;
        }
        let mut sorted_counts: Vec<_> = card_counts.iter().collect();
        sorted_counts.sort_by_key(|(_, c)| **c);
        let most_count = *sorted_counts.last().unwrap().1;
        let second_most_count = if sorted_counts.len() > 1 {
            *sorted_counts[sorted_counts.len() - 2].1
        } else {
            0
        };

        match (most_count, second_most_count) {
            (5, _) => Kind::FiveOfAKind,
            (4, _) => Kind::FourOfAKind,
            (3, 2) => Kind::FullHouse,
            (3, _) => Kind::ThreeOfAKind,
            (2, 2) => Kind::TwoPair,
            (2, _) => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }

    fn to_kind(&self) -> Kind {
        if let Some(joker_index) = self.cards.iter().position(|c| c == &Card::Joker) {
            Card::iter()
                .filter(|c| c != &Card::Joker)
                .map(|c| {
                    let mut new_hand = self.clone();
                    new_hand.cards[joker_index] = c;
                    new_hand.to_kind()
                })
                .max()
                .unwrap()
        } else {
            self.to_kind_no_joker()
        }
    }
}

#[part_one]
fn part_one(input: List<Hand, TermWith<NewLine>>) -> u64 {
    let mut hands: Vec<_> = input.iter().map(|h| (h.to_kind(), h)).collect();
    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(n, (_k, h))| (n as u64 + 1) * h.bid)
        .sum()
}

#[part_two]
fn part_two(input: List<Hand, TermWith<NewLine>>) -> u64 {
    let new_hands: Vec<_> = input
        .iter()
        .map(|h| {
            let mut new_hand = h.clone();
            new_hand.cards = new_hand
                .cards
                .into_iter()
                .map(|c| if c == Card::J { Card::Joker } else { c })
                .collect();
            new_hand
        })
        .collect();
    let mut hands: Vec<_> = new_hands.iter().map(|h| (h.to_kind(), h)).collect();

    hands.sort();
    hands
        .iter()
        .enumerate()
        .map(|(n, (_k, h))| (n as u64 + 1) * h.bid)
        .sum()
}

harness!(part_1: 252656917, part_2: 253499763);
