use advent::prelude::*;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn x_delta(&self) -> isize {
        match self {
            Self::Left => -1,
            Self::Right => 1,
            _ => 0,
        }
    }

    fn y_delta(&self) -> isize {
        match self {
            Self::Up => -1,
            Self::Down => 1,
            _ => 0,
        }
    }

    fn new_xy(&self, x: usize, y: usize) -> (usize, usize) {
        (
            (x as isize + self.x_delta()) as usize,
            (y as isize + self.y_delta()) as usize,
        )
    }

    #[allow(dead_code)]
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn walk(d: Direction, c: char) -> Option<Direction> {
    use Direction::*;
    Some(match (d, c) {
        (Right, 'J') => Up,
        (Down, 'J') => Left,
        (Down, 'L') => Right,
        (Left, 'L') => Up,
        (Left, 'F') => Down,
        (Up, 'F') => Right,
        (Up, '7') => Left,
        (Right, '7') => Down,
        (Up, '|') => Up,
        (Down, '|') => Down,
        (Left, '-') => Left,
        (Right, '-') => Right,
        _ => return None,
    })
}

struct Grid {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: Vec<Vec<char>>) -> Self {
        Grid {
            width: data[0].len(),
            height: data.len(),
            data,
        }
    }

    fn squares(&self) -> impl Iterator<Item = (usize, usize, char)> + '_ {
        self.data
            .iter()
            .map(|l| l.iter().enumerate())
            .enumerate()
            .map(|(row, iter)| iter.map(move |(column, c)| (column, row, *c)))
            .flatten()
    }

    fn square(&self, x: usize, y: usize) -> char {
        self.data[y][x]
    }

    fn set_square(&mut self, x: usize, y: usize, c: char) {
        self.data[y][x] = c;
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, Direction, char)> {
        let mut pos = vec![];
        for d in Direction::iter() {
            let new_x = (x as isize + d.x_delta()) as usize;
            let new_y = (y as isize + d.y_delta()) as usize;
            if new_x > self.width || new_y > self.height {
                continue;
            }
            pos.push((new_x, new_y, d, self.square(new_x, new_y)));
        }
        pos
    }

    fn walk_til_loop_inner(
        &self,
        x: usize,
        y: usize,
        prev_d: Direction,
        seen: &mut HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        if seen.contains(&(x, y)) {
            return vec![];
        }
        seen.insert((x, y));

        let c = self.square(x, y);
        let new_dir = walk(prev_d, c).unwrap();

        let (new_x, new_y) = new_dir.new_xy(x, y);
        let mut path = self.walk_til_loop_inner(new_x, new_y, new_dir, seen);
        path.push((x, y));
        path
    }

    fn walk_til_loop(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut seen = HashSet::new();
        let (nx, ny, d) = self
            .neighbors(x, y)
            .into_iter()
            .filter_map(|(nx, ny, d, c)| walk(d, c).map(|_| (nx, ny, d)))
            .next()
            .unwrap();
        seen.insert((x, y));

        let mut path = self.walk_til_loop_inner(nx, ny, d, &mut seen);
        path.push((x, y));
        path
    }

    fn replace_start(&mut self, x: usize, y: usize) {
        let mut directions: Vec<_> = self
            .neighbors(x, y)
            .into_iter()
            .filter_map(|(_, _, d, c)| walk(d, c).map(|_| d))
            .collect();
        directions.sort();

        use Direction::*;
        let c = match (directions[0], directions[1]) {
            (Left, Right) => '-',
            (Up, Down) => '|',
            (Left, Down) => '7',
            (Right, Down) => 'F',
            (Right, Up) => 'L',
            (Left, Up) => 'J',
            _ => panic!(),
        };
        self.set_square(x, y, c);
    }

    fn enclosed(&self, path: &HashSet<(usize, usize)>) -> u64 {
        let mut enclosed = false;
        let mut num_enclosed = 0;
        let mut wall_char = None;
        for (x, y, c) in self.squares() {
            if x == 0 {
                enclosed = false;
            }
            if path.contains(&(x, y)) {
                match (wall_char, c) {
                    (_, '|') => enclosed = !enclosed,
                    (_, 'F') | (_, 'L') => wall_char = Some(c),
                    (Some('F'), 'J') => enclosed = !enclosed,
                    (Some('L'), '7') => enclosed = !enclosed,
                    _ => (),
                }
            } else if enclosed {
                num_enclosed += 1;
            }
        }
        num_enclosed
    }
}

#[part_one]
fn part_one(input: List<NotWhitespace, TermWith<NewLine>>) -> u64 {
    let grid = Grid::new(input.into_iter().map(|s| s.0.chars().collect()).collect());
    let (start_x, start_y, _) = grid.squares().find(|(_, _, c)| *c == 'S').unwrap();
    let path = grid.walk_til_loop(start_x, start_y);
    path.len() as u64 / 2
}

#[part_two]
fn part_two(input: List<NotWhitespace, TermWith<NewLine>>) -> u64 {
    let mut grid = Grid::new(input.into_iter().map(|s| s.0.chars().collect()).collect());
    let (start_x, start_y, _) = grid.squares().find(|(_, _, c)| *c == 'S').unwrap();
    grid.replace_start(start_x, start_y);
    let path: HashSet<_> = grid.walk_til_loop(start_x, start_y).into_iter().collect();
    grid.enclosed(&path)
}

harness!(part_1: 6860, part_2: 343);
