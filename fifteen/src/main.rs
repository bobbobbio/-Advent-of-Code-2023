use advent::prelude::*;
use std::fmt;

fn hash(s: String) -> u32 {
    let mut v = 0;
    for c in s.chars() {
        v += (c as u8) as u32;
        v *= 17;
        v %= 256;
    }
    v
}

#[derive(Debug)]
struct Step(String);

impl HasParser for Step {
    #[into_parser]
    fn parser() -> _ {
        many(satisfy(|c| c != ',' && c != '\n')).map(Self)
    }
}

#[part_one]
fn part_one(input: List<Step, SepBy<Comma>>) -> u32 {
    input.into_iter().map(|s| hash(s.0)).sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Label(String);

impl HasParser for Label {
    #[into_parser]
    fn parser() -> _ {
        many1(alpha_num()).map(Self)
    }
}

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, HasParser)]
#[parse(sep_by = "=")]
struct Lens {
    label: Label,
    focal_length: u32
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", &self.label, self.focal_length)
    }
}

#[derive(HasParser)]
enum Operation {
    Add(Lens),
    Remove(#[parse(after = "-")] Label),
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add(lens) => write!(f, "{lens}"),
            Self::Remove(label) => write!(f, "{label}-"),
        }
    }
}

#[derive(Debug, Default)]
struct Bucket(Vec<Lens>);

impl fmt::Display for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self.0.iter();
        if let Some(l) = iter.next() {
            write!(f, "{l}")?;
        }
        for l in iter {
            write!(f, ", {l}")?;
        }
        write!(f, "]")
    }
}

impl Bucket {
    fn add(&mut self, lens: Lens) {
        if let Some(pos) = self.0.iter().position(|l| l.label == lens.label) {
            self.0[pos] = lens;
        } else {
            self.0.push(lens);
        }
    }

    fn remove(&mut self, label: Label) {
        self.0.retain(|lens| lens.label != label)
    }
}

#[part_two]
fn part_two(input: List<Operation, SepBy<Comma>>) -> u32 {
    let mut buckets: [_; 256] = std::array::from_fn(|_| Bucket::default());

    for op in input {
        match op {
            Operation::Add(lens) => {
                let hash = hash(lens.label.0.clone()) as usize;
                buckets[hash].add(lens);
            }
            Operation::Remove(label) => {
                let hash = hash(label.0.clone()) as usize;
                buckets[hash].remove(label);
            }
        }
    }

    let mut focusing_power = 0;
    for (bucket_number, bucket) in buckets.iter().enumerate() {
        for (slot, lens) in bucket.0.iter().enumerate() {
            focusing_power += (bucket_number + 1) * (slot + 1) * lens.focal_length as usize;
        }
    }
    focusing_power as u32
}

harness!(part_1: 516657, part_2: 210906);
