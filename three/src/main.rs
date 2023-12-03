#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;

#[derive(HasParser, Debug)]
enum Symbol {
    #[parse(string = "*")]
    Asterisk,
    #[parse(string = "#")]
    Pound,
    #[parse(string = "+")]
    Plus,
    #[parse(string = "$")]
    Dollar,
    #[parse(string = "/")]
    ForwardSlash,
    #[parse(string = "=")]
    Equal,
    #[parse(string = "@")]
    At,
    #[parse(string = "-")]
    Minus,
    #[parse(string = "%")]
    Percent,
    #[parse(string = "&")]
    Ampersand,
}

#[derive(HasParser, Debug)]
enum Entry {
    #[parse(string = ".")]
    Dot,
    Number(u64),
    Symbol(Symbol),
}

fn number_width(mut n: u64) -> usize {
    let mut i = 0;
    while n > 0 {
        i += 1;
        n /= 10;
    }
    i
}

impl Entry {
    fn size(&self) -> usize {
        match self {
            Self::Number(n) => number_width(*n),
            _ => 1,
        }
    }
}

#[derive(HasParser, Debug)]
struct Line(List<Entry, Nil>);

impl Line {
    fn get_entry(&self, index: usize) -> Option<&Entry> {
        let mut offset = 0;
        for e in &self.0 {
            offset += e.size();
            if offset > index {
                return Some(e);
            }
        }
        None
    }
}

fn adjacent(input: &List<Line, TermWith<NewLine>>, line_n: usize, x: usize) -> Vec<&Entry> {
    let entry_size = input[line_n].get_entry(x).unwrap().size();
    let mut entries: Vec<&Entry> = vec![];
    for a_line_n in [line_n.wrapping_sub(1), line_n, line_n + 1] {
        for j in x.saturating_sub(1)..(x + entry_size + 1) {
            if a_line_n < input.len() {
                if let Some(entry) = input[a_line_n].get_entry(j) {
                    if !entries.iter().any(|&e| std::ptr::eq(e, entry)) {
                        entries.push(entry);
                    }
                }
            }
        }
    }
    entries
}

#[part_one]
fn part_one(input: List<Line, TermWith<NewLine>>) -> u64 {
    let mut numbers = vec![];
    for line_n in 0..input.len() {
        let mut i: usize = 0;
        for e in &input[line_n].0 {
            if let Entry::Number(n) = &e {
                for entry in adjacent(&input, line_n, i) {
                    if matches!(entry, Entry::Symbol(_)) {
                        numbers.push(n);
                    }
                }
            }
            i += e.size();
        }
    }
    numbers.into_iter().sum()
}

#[part_two]
fn part_two(input: List<Line, TermWith<NewLine>>) -> u64 {
    let mut numbers = vec![];
    for line_n in 0..input.len() {
        let mut i: usize = 0;
        for e in &input[line_n].0 {
            if matches!(e, Entry::Symbol(Symbol::Asterisk)) {
                let mut gears = vec![];
                for entry in adjacent(&input, line_n, i) {
                    if let Entry::Number(n) = entry {
                        gears.push(n);
                    }
                }
                if gears.len() == 2 {
                    numbers.push(gears[0] * gears[1]);
                }
            }
            i += e.size();
        }
    }
    numbers.into_iter().sum()
}

harness!(part_1: 560670, part_2: 91622824);
