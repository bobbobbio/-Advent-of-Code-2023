use advent::prelude::*;
use std::cmp::min;

#[derive(Clone, Debug, HasParser, PartialEq, Eq)]
enum RockOrAsh {
    #[parse(string = "#")]
    Rock,
    #[parse(string = ".")]
    Ash,
}

#[derive(HasParser)]
struct Grid(List<List<RockOrAsh, Nil>, TermWith<NewLine>>);

impl Grid {
    fn rows(&self) -> impl Iterator<Item = impl DoubleEndedIterator<Item = RockOrAsh> + '_> + '_ {
        self.0.iter().map(|row| row.iter().cloned())
    }

    fn columns(
        &self,
    ) -> impl Iterator<Item = impl DoubleEndedIterator<Item = RockOrAsh> + '_> + '_ {
        (0..(self.0[0].len())).map(|c_i| self.0.iter().map(move |row| row[c_i].clone()))
    }
}

fn cmp_sub(
    row: &[RockOrAsh],
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
) -> u32 {
    let mut b = b_start;
    let mut a = a_start;
    let mut dist = 0;
    loop {
        if row[a] != row[b] {
            dist += 1;
        }
        if b == b_end || a == a_end {
            break;
        }
        b -= 1;
        a += 1;
    }

    dist
}

fn hash_map_build(r0: HashSet<usize>, r1: HashSet<usize>) -> HashMap<usize, usize> {
    let mut m = HashMap::new();
    for k in r0.iter() {
        m.insert(*k, 0);
    }
    for k in r1.iter() {
        m.insert(*k, 1);
    }
    m
}

fn hash_map_intersect(a: &mut Option<HashMap<usize, usize>>, b: HashMap<usize, usize>) {
    if let Some(a) = a {
        let mut intersection = HashMap::new();
        for (k, a_v) in a.iter() {
            if let Some(b_v) = b.get(k) {
                intersection.insert(*k, *a_v + *b_v);
            }
        }
        *a = intersection;
    } else {
        *a = Some(b);
    }
}

fn find_reflections(lines: Vec<Vec<RockOrAsh>>, smudge: bool) -> HashSet<usize> {
    let mut grid_refs: Option<HashMap<usize, usize>> = None;
    for line in &lines {
        let mut refs_0 = HashSet::new();
        let mut refs_1 = HashSet::new();
        for i in 0..(line.len() - 1) {
            match cmp_sub(line, i + 1, min(i * 2 + 1, line.len() - 1), i, 0) {
                0 => {
                    refs_0.insert(i);
                }
                1 => {
                    refs_1.insert(i);
                }
                _ => {}
            }
        }
        let m = hash_map_build(refs_0, refs_1);
        hash_map_intersect(&mut grid_refs, m);
    }
    grid_refs
        .unwrap_or_default()
        .into_iter()
        .filter_map(|(k, v)| (v == smudge as usize).then_some(k))
        .collect()
}

fn solve(grids: List<Grid, SepBy<NewLine>>, smudge: bool) -> usize {
    let mut vertical = vec![];
    let mut horizontal = vec![];
    for grid in grids {
        let v = find_reflections(grid.rows().map(Vec::from_iter).collect(), smudge);
        let h = find_reflections(grid.columns().map(Vec::from_iter).collect(), smudge);

        assert!(!(v.is_empty() && h.is_empty()));

        for v in v {
            vertical.push(v + 1);
        }
        for h in h {
            horizontal.push(h + 1);
        }
    }
    vertical.into_iter().sum::<usize>() + horizontal.into_iter().sum::<usize>() * 100
}

#[part_one]
fn part_one(grids: List<Grid, SepBy<NewLine>>) -> usize {
    solve(grids, false)
}

#[part_two]
fn part_two(grids: List<Grid, SepBy<NewLine>>) -> usize {
    solve(grids, true)
}

harness!(part_1: 37381, part_2: 28210);
