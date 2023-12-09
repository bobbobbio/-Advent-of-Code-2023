#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use advent::prelude::*;

fn extrapolate(input: List<i64, SepBy<Space>>) -> Vec<Vec<i64>> {
    let mut lines: Vec<Vec<i64>> = vec![];
    lines.push(input.into());

    while !matches!(&lines.last(), Some(l) if l.iter().all(|v| *v == 0)) {
        lines.push(
            lines
                .last()
                .unwrap()
                .windows(2)
                .map(|s| s[1] - s[0])
                .collect(),
        );
    }
    lines
}

fn next_in_seq(input: List<i64, SepBy<Space>>) -> i64 {
    let lines = extrapolate(input);
    lines.iter().map(|l| l.last().unwrap()).sum()
}

fn prev_in_seq(input: List<i64, SepBy<Space>>) -> i64 {
    let lines = extrapolate(input);
    lines
        .iter()
        .rev()
        .map(|l| l.first().unwrap())
        .fold(0, |a, e| e - a)
}

#[part_one]
fn part_one(input: List<List<i64, SepBy<Space>>, TermWith<NewLine>>) -> i64 {
    input.into_iter().map(next_in_seq).sum()
}

#[part_two]
fn part_two(input: List<List<i64, SepBy<Space>>, TermWith<NewLine>>) -> i64 {
    input.into_iter().map(prev_in_seq).sum()
}

harness!(part_1: 1702218515, part_2: 925);
