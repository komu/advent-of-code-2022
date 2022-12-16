use anyhow::anyhow;
use aoc::helpers::parse_lines;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{ops::RangeInclusive, str::FromStr};
use rayon::prelude::*;

type Point = aoc::point::Point<i32>;

pub fn part_one(input: &str) -> Option<u32> {
    part_one_y(input, 2000000)
}

pub fn part_two(input: &str) -> Option<u64> {
    part_two_max(input, 4_000_000)
}

fn part_one_y(input: &str, y: i32) -> Option<u32> {
    let sensors: Vec<_> = parse_lines::<SensorData>(input).collect();
    let beacons_on_line = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.y == y)
        .sorted()
        .dedup()
        .count();

    let mut ranges: Vec<_> = sensors.iter().flat_map(|s| s.range_on_line(y)).collect();
    ranges.sort_by_key(|r| *r.start());

    let mut count = 0;
    let mut prev_end = i32::min_value();

    for r in &ranges {
        let start = (*r.start()).max(prev_end + 1);
        let end = *r.end();
        if start <= end {
            count += (end - start + 1) as u32;
        }

        prev_end = end.max(prev_end);
    }

    Some(count - beacons_on_line as u32)
}

fn part_two_max(input: &str, max: i32) -> Option<u64> {
    let sensors: Vec<_> = parse_lines::<SensorData>(input).collect();

    (0..=max).into_par_iter().find_map_any(|y| {
        let mut ranges: Vec<_> = sensors.iter().flat_map(|s| s.range_on_line(y)).collect();
        ranges.sort_by_key(|r| *r.start());

        let mut prev_end = 0;
        for r in ranges.iter() {
            let last_start = *r.start();

            if prev_end + 1 < last_start && last_start > 0 && prev_end < max {
                let x = prev_end + 1;
                return Some(4000000 * (x as u64) + (y as u64));
            }
            prev_end = prev_end.max(*r.end());
        }

        None
    })
}

#[derive(Debug)]
struct SensorData {
    point: Point,
    beacon: Point,
    min_range: u32,
}

impl SensorData {
    fn new(point: Point, beacon: Point) -> Self {
        SensorData {
            min_range: point.manhattan_distance(&beacon),
            point,
            beacon,
        }
    }

    fn range_on_line(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let distance = y.abs_diff(self.point.y);
        if distance < self.min_range {
            let d = (self.min_range - distance) as i32;

            Some(self.point.x - d..=self.point.x + d)
        } else {
            None
        }
    }
}

impl FromStr for SensorData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("no match for line '{}'", s))?;
        let point = Point {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap(),
        };
        let beacon = Point {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap(),
        };
        Ok(SensorData::new(point, beacon))
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_one_y(&input, 10), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        assert_eq!(part_two_max(&input, 20), Some(56000011));
    }
}
