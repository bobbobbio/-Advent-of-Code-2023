use advent::prelude::*;
use std::fmt;
use std::cmp::max;

#[derive(Copy, Clone, HasParser)]
enum Tile {
    #[parse(string = ".")]
    Empty,
    #[parse(string = "\\")]
    MirrorLeft,
    #[parse(string = "/")]
    MirrorRight,
    #[parse(string = "|")]
    VerticalSplit,
    #[parse(string = "-")]
    HorizontalSplit,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::MirrorLeft => write!(f, "\\"),
            Self::MirrorRight => write!(f, "/"),
            Self::VerticalSplit => write!(f, "|"),
            Self::HorizontalSplit => write!(f, "-"),
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, c: usize, r: usize, w: usize, h: usize) -> Option<(usize, usize)> {
        match self {
            Self::Up => {
                if r == 0 {
                    return None;
                }
                Some((c, r - 1))
            }
            Self::Down => {
                if r + 1 == h {
                    return None;
                }
                Some((c, r + 1))
            }
            Self::Left => {
                if c == 0 {
                    return None;
                }
                Some((c - 1, r))
            }
            Self::Right => {
                if c + 1 == w {
                    return None;
                }
                Some((c + 1, r))
            }
        }
    }

    fn mirror_left(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Right => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
        }
    }

    fn mirror_right(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Right => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
        }
    }
}

#[derive(Clone)]
struct Cell {
    tile: Tile,
    energy: HashSet<Direction>,
}

impl From<Tile> for Cell {
    fn from(tile: Tile) -> Self {
        Self {
            tile,
            energy: Default::default(),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.energy.is_empty() {
            write!(f, "#")
        } else {
            write!(f, "{}", self.tile)
        }
    }
}

fn energize(board: &mut Grid<Cell>, c: usize, r: usize, d: Direction) -> bool {
    board[r][c].energy.insert(d)
}

fn board_from_input(input: Grid<Tile>) -> Grid<Cell> {
    Grid::new(Vec::from_iter(
        input
            .rows()
            .map(|r| Vec::from_iter(r.iter().cloned().map(Cell::from))),
    )).unwrap()
}

fn solve(mut board: Grid<Cell>, s_c: usize, s_r: usize, s_d: Direction) -> u32 {
    let mut stack = vec![];
    stack.push(((s_c, s_r), s_d));

    while let Some(((c, r), d)) = stack.pop() {
        if !energize(&mut board, c, r, d) {
            continue;
        }
        let tile = board[r][c].tile;

        let mut directions = vec![];
        match tile {
            Tile::Empty => directions.push(d),
            Tile::MirrorLeft => directions.push(d.mirror_left()),
            Tile::MirrorRight => directions.push(d.mirror_right()),
            Tile::VerticalSplit => {
                match d {
                    Direction::Left | Direction::Right => {
                        directions.push(Direction::Up);
                        directions.push(Direction::Down);
                    },
                    Direction::Up | Direction::Down => {
                        directions.push(d);
                    }
                }
            }
            Tile::HorizontalSplit => {
                match d {
                    Direction::Left | Direction::Right => {
                        directions.push(d);
                    },
                    Direction::Up | Direction::Down => {
                        directions.push(Direction::Left);
                        directions.push(Direction::Right);
                    }
                }
            }
        }

        for d in directions {
            let Some((c, r)) = d.apply(c, r, board.width(), board.height()) else {
                continue;
            };

            if board[r][c].energy.contains(&d) {
                continue;
            }

            stack.push(((c, r), d));
        }
    }

    board.rows().map(|r| r.iter().map(|c| !c.energy.is_empty() as u32).sum::<u32>()).sum()
}

#[part_one]
fn part_one(input: Grid<Tile>) -> u32 {
    let board = board_from_input(input);
    solve(board, 0, 0, Direction::Right)
}

#[part_two]
fn part_two(input: Grid<Tile>) -> u32 {
    let board = board_from_input(input);
    let mut max_energy = 0;

    for c in 0..board.width() {
        max_energy = max(max_energy, solve(board.clone(), c, 0, Direction::Down));
    }
    for c in 0..board.width() {
        max_energy = max(max_energy, solve(board.clone(), c, board.height() - 1, Direction::Up));
    }
    for r in 0..board.height() {
        max_energy = max(max_energy, solve(board.clone(), 0, r, Direction::Right));
    }
    for r in 0..board.height() {
        max_energy = max(max_energy, solve(board.clone(), board.width() - 1, r, Direction::Up));
    }

    max_energy
}

harness!(part_1: 7498, part_2: 7846);
