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

#[derive(Clone, Hash, PartialEq, Eq, HasParser)]
struct Grid(List<List<Entry, Nil>, TermWith<NewLine>>);

impl Grid {
    fn rows(&self) -> impl Iterator<Item = impl DoubleEndedIterator<Item = Entry> + '_> + '_ {
        self.0.iter().map(|row| row.iter().cloned())
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }

    fn columns(&self) -> impl Iterator<Item = impl DoubleEndedIterator<Item = Entry> + '_> + '_ {
        (0..(self.0[0].len())).map(|c_i| self.0.iter().map(move |row| row[c_i].clone()))
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn move_north(grid: &mut Grid) -> bool {
    let height = grid.height();

    let mut moved = false;
    for r in 1..height {
        let row = grid.0[r].clone();
        for (c, _) in row.into_iter().enumerate().filter(|(_, e)| *e == Entry::RoundRock) {
            if grid.0[r - 1][c] == Entry::Empty {
                grid.0[r - 1][c] = Entry::RoundRock;
                grid.0[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_south(grid: &mut Grid) -> bool {
    let height = grid.height();

    let mut moved = false;
    for r in (0..(height - 1)).rev() {
        let row = grid.0[r].clone();
        for (c, _) in row.into_iter().enumerate().filter(|(_, e)| *e == Entry::RoundRock) {
            if grid.0[r + 1][c] == Entry::Empty {
                grid.0[r + 1][c] = Entry::RoundRock;
                grid.0[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_east(grid: &mut Grid) -> bool {
    let width = grid.width();

    let mut moved = false;
    for c in (0..(width - 1)).rev() {
        let column: Vec<Entry> = grid.columns().skip(c).next().unwrap().collect();
        for (r, _) in column.into_iter().enumerate().filter(|(_, e)| *e == Entry::RoundRock) {
            if grid.0[r][c + 1] == Entry::Empty {
                grid.0[r][c + 1] = Entry::RoundRock;
                grid.0[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

fn move_west(grid: &mut Grid) -> bool {
    let width = grid.width();

    let mut moved = false;
    for c in 1..width {
        let column: Vec<Entry> = grid.columns().skip(c).next().unwrap().collect();
        for (r, _) in column.into_iter().enumerate().filter(|(_, e)| *e == Entry::RoundRock) {
            if grid.0[r][c - 1] == Entry::Empty {
                grid.0[r][c - 1] = Entry::RoundRock;
                grid.0[r][c] = Entry::Empty;
                moved = true;
            }
        }
    }
    moved
}

#[part_one]
fn part_one(mut grid: Grid) -> usize {
    let height = grid.height();

    while move_north(&mut grid) {}

    grid.rows()
        .enumerate()
        .map(|(i, row)| row.filter(|c| *c == Entry::RoundRock).count() * (height - i))
        .sum()
}

#[part_two]
fn part_two(mut grid: Grid) -> usize {
    let height = grid.height();

    let mut found = None;
    let mut grids: Vec<Grid> = vec![];
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

    found.unwrap().rows()
        .enumerate()
        .map(|(i, row)| row.filter(|c| *c == Entry::RoundRock).count() * (height - i))
        .sum()
}

harness!(part_1: 109661, part_2: 90176);
