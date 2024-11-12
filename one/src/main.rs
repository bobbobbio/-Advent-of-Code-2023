use advent::prelude::*;

#[part_one]
fn part_one(input: List<NotWhitespace, TermWith<NewLine>>) -> u64 {
    input
        .into_iter()
        .map(|i| {
            let nums = Vec::from_iter(i.0.chars().filter(|c| c.is_numeric()));
            format!(
                "{}{}",
                nums.first().unwrap_or(&'0'),
                nums.last().unwrap_or(&'0')
            )
            .parse::<u64>()
            .unwrap()
        })
        .sum()
}

fn find_number(s: &str, reverse: bool) -> u64 {
    let mut i = if reverse { s.len() - 1 } else { 0 };

    while i < s.len() {
        if let Ok(n) = s[i..(i + 1)].parse() {
            return n;
        }
        for j in (i + 3)..=s.len() {
            let substr = &s[i..j];
            let new = text2num::replace_numbers(substr, &text2num::Language::english(), 0.0);
            if let Ok(n) = new.parse() {
                return n;
            }
        }
        if reverse {
            i = i.wrapping_sub(1);
        } else {
            i += 1;
        }
    }
    panic!("{s:?} has no numbers, reverse={reverse}");
}

fn find_numbers(s: &str) -> (u64, u64) {
    (find_number(s, false), find_number(s, true))
}

#[part_two]
fn part_two(input: List<NotWhitespace, TermWith<NewLine>>) -> u64 {
    input
        .into_iter()
        .map(|i| {
            let (n1, n2) = find_numbers(&i.0);
            format!("{n1}{n2}").parse::<u64>().unwrap()
        })
        .sum()
}

harness!(part_1: 55172, part_2: 54925);
