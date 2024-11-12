use advent::prelude::*;

#[derive(HasParser, Debug)]
struct RangeMap {
    dest: u64,
    src: u64,
    len: u64,
}

impl RangeMap {
    fn map(&self, value: MultiRange) -> (MultiRange, MultiRange) {
        let mut mapped = vec![];
        let mut unmapped = vec![];
        for range in value.ranges {
            let range_end = range.start + range.len;
            if range.start < self.src {
                if range_end <= self.src {
                    unmapped.push(range);
                } else {
                    unmapped.push(Range {
                        start: range.start,
                        len: self.src - range.start,
                    });
                    mapped.push(Range {
                        start: self.dest,
                        len: range_end - self.src,
                    });
                }
            } else {
                let self_end = self.src + self.len;
                if range.start >= self_end {
                    unmapped.push(range);
                } else {
                    if range_end <= self_end {
                        mapped.push(Range {
                            start: self.dest + range.start - self.src,
                            len: range.len,
                        });
                    } else {
                        mapped.push(Range {
                            start: self.dest + range.start - self.src,
                            len: self_end - range.start,
                        });
                        unmapped.push(Range {
                            start: self_end,
                            len: range_end - self_end,
                        });
                    }
                }
            }
        }
        assert!(!(mapped.is_empty() && unmapped.is_empty()));
        (mapped.into(), unmapped.into())
    }
}

#[derive(HasParser, Debug)]
#[parse(sep_by = "\n")]
struct Almanac {
    #[parse(before = "seeds: ", after = "\n")]
    seeds: List<u64, SepBy<Space>>,
    #[parse(before = "seed-to-soil map:\n")]
    seed_to_soil: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "soil-to-fertilizer map:\n")]
    soil_to_fertilizer: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "fertilizer-to-water map:\n")]
    fertilizer_to_water: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "water-to-light map:\n")]
    water_to_light: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "light-to-temperature map:\n")]
    light_to_temperature: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "temperature-to-humidity map:\n")]
    temperature_to_humidity: List<RangeMap, TermWith<NewLine>>,
    #[parse(before = "humidity-to-location map:\n")]
    humidity_to_location: List<RangeMap, TermWith<NewLine>>,
}

impl Almanac {
    fn maps(&self) -> Vec<&List<RangeMap, TermWith<NewLine>>> {
        vec![
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temperature,
            &self.temperature_to_humidity,
            &self.humidity_to_location,
        ]
    }
}

fn map_value(mut unmapped: MultiRange, m: &List<RangeMap, TermWith<NewLine>>) -> MultiRange {
    let mut mapped = MultiRange::default();
    for r in m {
        let (new_mapped, new_unmapped) = r.map(unmapped);
        mapped.merge(new_mapped);

        unmapped = new_unmapped;
        if unmapped.is_empty() {
            break;
        }
    }
    mapped.merge(unmapped);
    mapped
}

#[part_one]
fn part_one(input: Almanac) -> u64 {
    input
        .seeds
        .iter()
        .map(|seed| {
            let mut value = MultiRange::from_value(*seed);
            for m in input.maps() {
                value = map_value(value, m);
            }
            value
        })
        .min()
        .unwrap()
        .ranges[0]
        .start
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: u64,
    len: u64,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MultiRange {
    ranges: Vec<Range>,
}

impl MultiRange {
    fn from_value(v: u64) -> Self {
        Self {
            ranges: vec![Range { start: v, len: 1 }],
        }
    }

    fn from_range(r: Range) -> Self {
        Self { ranges: vec![r] }
    }

    fn merge(&mut self, other: Self) {
        self.ranges.extend(other.ranges);
        self.ranges.sort()
    }

    fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }
}

impl From<Vec<Range>> for MultiRange {
    fn from(mut ranges: Vec<Range>) -> Self {
        ranges.sort();
        Self { ranges }
    }
}

#[part_two]
fn part_two(input: Almanac) -> u64 {
    input
        .seeds
        .chunks(2)
        .map(|seed_range| {
            let mut value_range = MultiRange::from_range(Range {
                start: seed_range[0],
                len: seed_range[1],
            });
            for m in input.maps() {
                value_range = map_value(value_range, m)
            }
            value_range
        })
        .min()
        .unwrap()
        .ranges[0]
        .start
}

harness!(part_1: 196167384, part_2: 125742456);
