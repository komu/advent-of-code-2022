use std::{ops::RangeInclusive, str::FromStr};

use aoc::helpers::parse_lines;

pub fn part_one(input: &str) -> Option<usize> {
    parse_lines::<RangePair>(input)
        .filter(|RangePair(r1, r2)| fully_contains(r1, r2) || fully_contains(r2, r1))
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_lines::<RangePair>(input)
        .filter(|RangePair(r1, r2)| overlaps(r1, r2))
        .count()
        .into()
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

struct RangePair(RangeInclusive<u32>, RangeInclusive<u32>);

impl FromStr for RangePair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r1, r2) = s.split_once(',').unwrap();
        Ok(RangePair(parse_range(r1), parse_range(r2)))
    }
}

fn parse_range(s: &str) -> RangeInclusive<u32> {
    let (min, max) = s.split_once('-').unwrap();
    min.parse::<u32>().unwrap()..=max.parse::<u32>().unwrap()
}

fn fully_contains<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.contains(r2.start()) && r1.contains(r2.end())
}

fn overlaps<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.start() <= r2.end() && r2.start() <= r1.end()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
