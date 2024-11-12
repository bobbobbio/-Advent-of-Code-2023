use advent::prelude::*;

type Nums = List<u64, StartsWith<Spaces>>;

#[derive(Debug, Clone)]
struct Card {
    id: u64,
    winning_nums: HashSet<u64>,
    your_nums: Nums,
}

#[derive(Debug, Clone)]
struct MultiCard {
    card: Card,
    count: u64,
}

impl Card {
    fn matching_nums(&self) -> u32 {
        self.your_nums
            .iter()
            .map(|n| self.winning_nums.contains(&n) as u32)
            .sum::<u32>()
    }

    fn points(&self) -> u64 {
        let matching_nums = self.matching_nums();
        if matching_nums > 0 {
            2_u64.pow(matching_nums - 1)
        } else {
            0
        }
    }

    fn win_cards(&self, cards: &mut BTreeMap<u64, MultiCard>) {
        let matching_nums = self.matching_nums();
        for j in 0..matching_nums {
            let n = self.id + j as u64 + 1;
            if let Some(mc) = cards.get_mut(&n) {
                mc.count += 1;
            }
        }
    }
}

impl HasParser for Card {
    #[into_parser]
    fn parser() -> _ {
        (
            string("Card")
                .with(spaces())
                .with(u64::parser())
                .skip(token(':')),
            Nums::parser().skip(string(" |")),
            Nums::parser(),
        )
            .map(|(id, your_nums, winning_nums)| Self {
                id,
                your_nums,
                winning_nums: winning_nums.into_iter().collect(),
            })
    }
}

#[part_one]
fn part_one(input: List<Card, TermWith<NewLine>>) -> u64 {
    input.iter().map(|c| c.points()).sum()
}

#[part_two]
fn part_two(input: List<Card, TermWith<NewLine>>) -> u64 {
    let mut input: BTreeMap<u64, MultiCard> = input
        .into_iter()
        .map(|card| (card.id, MultiCard { card, count: 1 }))
        .collect();
    let mut i = 1;
    let greatest_card_num = *input.keys().max().unwrap();
    while i < greatest_card_num {
        if let Some(mc) = input.get(&i).cloned() {
            for _ in 0..mc.count {
                mc.card.win_cards(&mut input);
            }
        }
        i += 1;
    }
    input.values().map(|c| c.count).sum()
}

harness!(part_1: 20117, part_2: 13768818);
