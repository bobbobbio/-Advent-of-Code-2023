use advent::prelude::*;
use std::collections::HashSet;

#[derive(Debug, HasParser)]
enum Entry {
    #[parse(string = "#")]
    Galaxy,
    #[parse(string = ".")]
    Empty
}

fn solve(input: List<List<Entry, Nil>, TermWith<NewLine>>, expansion: usize) -> i128 {
    let mut galaxies = vec![];

    let mut empty_columns = HashSet::new();
    for x in 0..input[0].len() {
        if input.iter().all(|row| matches!(row[x], Entry::Empty)) {
            empty_columns.insert(x);
        }
    }

    let mut y = 0;
    for row in &input {
        let mut no_galaxies = true;
        let mut x = 0;
        for (i, e) in row.iter().enumerate() {
            if matches!(e, Entry::Galaxy) {
                galaxies.push((x, y));
                no_galaxies = false;
            }
            if empty_columns.contains(&i) {
                x += 1 * expansion;
            } else {
                x += 1;
            }
        }
        if no_galaxies {
            y += 1 * expansion;
        } else {
            y += 1;
        }
    }

    let mut distances = vec![];
    for (i, ga) in galaxies.iter().enumerate() {
        for gb in &galaxies[(i + 1)..] {
            let dx = i128::abs(ga.0 as i128 - gb.0 as i128);
            let dy = i128::abs(ga.1 as i128 - gb.1 as i128);
            distances.push(dx + dy);
        }
    }
    distances.into_iter().sum::<i128>()
}


#[part_one]
fn part_one(input: List<List<Entry, Nil>, TermWith<NewLine>>) -> i128 {
    solve(input, 2)
}

#[part_two]
fn part_two(input: List<List<Entry, Nil>, TermWith<NewLine>>) -> i128 {
    solve(input, 1000000)
}

harness!(part_1: 9543156, part_2: 625243292686);
