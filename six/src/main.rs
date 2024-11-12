use advent::prelude::*;

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Input {
    #[parse(before = "Time:")]
    time: List<u64, StartsWith<Spaces>>,
    #[parse(before = "Distance:")]
    distance: List<u64, StartsWith<Spaces>>,
}

#[part_one]
fn part_one(input: Input) -> u64 {
    input
        .time
        .iter()
        .zip(input.distance.iter())
        .map(|(time, distance)| {
            (0..*time)
                .filter(|hold| hold * (time - hold) > *distance)
                .count() as u64
        })
        .product()
}

#[part_two]
fn part_two(input: Input) -> u64 {
    let time: u64 = input
        .time
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    let distance: u64 = input
        .distance
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse()
        .unwrap();
    (0..time)
        .filter(|hold| hold * (time - hold) > distance)
        .count() as u64
}

harness!(part_1: 771628, part_2: 27363861);
