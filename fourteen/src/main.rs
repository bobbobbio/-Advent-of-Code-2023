use advent::prelude::*;
use std::fmt;

#[derive(Copy, Clone, Debug, Hash, HasParser, PartialEq, Eq)]
enum Entry {
    #[parse(string = "#")]
    SquareRock,
    #[parse(string = "O")]
    RoundRock,
    #[parse(string = ".")]
    Empty,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SquareRock => write!(f, "#"),
            Self::RoundRock => write!(f, "O"),
            Self::Empty => write!(f, "."),
        }
    }
}

fn move_north(grid: &mut Grid<Entry>) -> bool {
    let height = grid.height();

    let mut moved = false;
    for r in 1..height {
        let row = grid[r].clone();
        for (c, _) in row
            .into_iter()
            .enumerate()
            .filter(|(_, e)| *e == Entry::RoundRock)
        {
            if grid[r - 1][c] == Entry::Empty {
                grid[r - 1][c] = Entry::RoundRock;
                grid[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_south(grid: &mut Grid<Entry>) -> bool {
    let height = grid.height();

    let mut moved = false;
    for r in (0..(height - 1)).rev() {
        let row = grid[r].clone();
        for (c, _) in row
            .into_iter()
            .enumerate()
            .filter(|(_, e)| *e == Entry::RoundRock)
        {
            if grid[r + 1][c] == Entry::Empty {
                grid[r + 1][c] = Entry::RoundRock;
                grid[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_east(grid: &mut Grid<Entry>) -> bool {
    let width = grid.width();

    let mut moved = false;
    for c in (0..(width - 1)).rev() {
        let column: Vec<Entry> = grid.column(c).into_iter().copied().collect();
        for (r, _) in column
            .into_iter()
            .enumerate()
            .filter(|(_, e)| *e == Entry::RoundRock)
        {
            if grid[r][c + 1] == Entry::Empty {
                grid[r][c + 1] = Entry::RoundRock;
                grid[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_west(grid: &mut Grid<Entry>) -> bool {
    let width = grid.width();

    let mut moved = false;
    for c in 1..width {
        let column: Vec<Entry> = grid.column(c).into_iter().copied().collect();
        for (r, _) in column
            .into_iter()
            .enumerate()
            .filter(|(_, e)| *e == Entry::RoundRock)
        {
            if grid[r][c - 1] == Entry::Empty {
                grid[r][c - 1] = Entry::RoundRock;
                grid[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

#[part_one]
fn part_one(mut grid: Grid<Entry>) -> usize {
    let height = grid.height();

    while move_north(&mut grid) {}

    grid.rows()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == Entry::RoundRock).count() * (height - i))
        .sum()
}

#[part_two]
fn part_two(mut grid: Grid<Entry>) -> usize {
    let height = grid.height();

    let mut found = None;
    let mut grids: Vec<Grid<Entry>> = vec![];
    let mut grid_map = HashMap::new();
    let total_cycles = 1_000_000_000;
    for i in 0..total_cycles {
        while move_north(&mut grid) {}
        while move_west(&mut grid) {}
        while move_south(&mut grid) {}
        while move_east(&mut grid) {}

        if let Some(g_i) = grid_map.insert(grid.clone(), grids.len()) {
            let cycle_length = grids.len() - g_i;
            let rem = (total_cycles - i - 1) % cycle_length;
            found = Some(grids[g_i + rem].clone());
            break;
        }
        grids.push(grid.clone());
    }

    found
        .unwrap()
        .rows()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|c| **c == Entry::RoundRock).count() * (height - i))
        .sum()
}

harness!(part_1: 109661, part_2: 90176);
