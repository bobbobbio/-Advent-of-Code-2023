use advent::prelude::*;
use std::mem;

#[derive(Clone, Debug, Hash, HasParser, PartialEq, Eq)]
enum MaybeWorkingSpring {
    #[parse(string = "#")]
    Operational,
    #[parse(string = "?")]
    Unknown,
}

#[derive(Clone, Debug, HasParser)]
enum MaybeWorkingSpringOrDamaged {
    MaybeWorking(MaybeWorkingSpring),
    #[parse(string = ".")]
    Damaged,
}

#[derive(Debug, HasParser)]
struct SpringLine {
    cells: List<MaybeWorkingSpringOrDamaged, Nil>,
    sums: List<u128, SepBy<Comma>>,
}

fn split_springs(
    mut springs: List<MaybeWorkingSpringOrDamaged, Nil>,
) -> VecDeque<VecDeque<MaybeWorkingSpring>> {
    springs.reverse();
    let mut line = VecDeque::new();
    let mut part = VecDeque::new();
    while let Some(s) = springs.pop() {
        match s {
            MaybeWorkingSpringOrDamaged::MaybeWorking(s) => {
                part.push_back(s);
            }
            MaybeWorkingSpringOrDamaged::Damaged => {
                if !part.is_empty() {
                    line.push_back(mem::take(&mut part));
                }
            }
        }
    }
    if !part.is_empty() {
        line.push_back(mem::take(&mut part));
    }
    line
}

#[derive(Default)]
struct Cache {
    c: HashMap<(VecDeque<VecDeque<MaybeWorkingSpring>>, VecDeque<u128>), u128>,
}

impl Cache {
    fn get(&self, key: &(VecDeque<VecDeque<MaybeWorkingSpring>>, VecDeque<u128>)) -> Option<&u128> {
        self.c.get(key)
    }

    fn enter(&mut self, key: &(VecDeque<VecDeque<MaybeWorkingSpring>>, VecDeque<u128>), a: u128) {
        self.c.insert((key.0.clone(), key.1.clone()), a);
    }
}

fn count_arrangements(
    cache: &mut Cache,
    k: &mut (VecDeque<VecDeque<MaybeWorkingSpring>>, VecDeque<u128>),
) -> u128 {
    if let Some(a) = cache.get(k) {
        return *a;
    }

    if let Some(seg) = k.0.pop_front() {
        if let Some(i) = seg.iter().position(|e| *e == MaybeWorkingSpring::Unknown) {
            let mut new_seg = seg.clone();
            new_seg[i] = MaybeWorkingSpring::Operational;
            k.0.push_front(new_seg);
            let mut arrangements = count_arrangements(cache, k);
            k.0.pop_front();

            let mut left = seg.clone();
            let mut right = left.split_off(i);
            right.pop_front();

            let mut pushed = 0;
            if !right.is_empty() {
                k.0.push_front(right);
                pushed += 1;
            }
            if !left.is_empty() {
                k.0.push_front(left);
                pushed += 1;
            }
            arrangements += count_arrangements(cache, k);

            for _ in 0..pushed {
                k.0.pop_front();
            }
            k.0.push_front(seg);

            cache.enter(k, arrangements);
            arrangements
        } else {
            let Some(s) = k.1.pop_front() else {
                k.0.push_front(seg);
                cache.enter(k, 0);
                return 0;
            };
            if seg.len() != s as usize {
                k.1.push_front(s);
                k.0.push_front(seg);
                cache.enter(k, 0);
                return 0;
            }

            let arrangements = count_arrangements(cache, k);
            k.1.push_front(s);
            k.0.push_front(seg);
            arrangements
        }
    } else {
        let arrangements = u128::from(k.1.is_empty());
        cache.enter(k, arrangements);
        arrangements
    }
}

fn solve(input: List<SpringLine, TermWith<NewLine>>) -> u128 {
    let mut arrangements = 0;
    let mut cache = Cache::default();
    for input_line in input {
        let line = split_springs(input_line.cells.clone());
        let sums: VecDeque<u128> = input_line.sums.clone().into_iter().collect();
        arrangements += count_arrangements(&mut cache, &mut (line, sums));
    }
    arrangements
}

#[part_one]
fn part_one(input: List<SpringLine, TermWith<NewLine>>) -> u128 {
    solve(input)
}

#[part_two]
fn part_two(mut input: List<SpringLine, TermWith<NewLine>>) -> u128 {
    for line in &mut input {
        let c = line.cells.clone();
        for _ in 0..4 {
            line.cells.push(MaybeWorkingSpringOrDamaged::MaybeWorking(
                MaybeWorkingSpring::Unknown,
            ));
            line.cells.extend(c.clone());
        }

        let s = line.sums.clone();
        for _ in 0..4 {
            line.sums.extend(s.clone());
        }
    }

    solve(input)
}

harness!(part_1: 7674, part_2: 4443895258186);
