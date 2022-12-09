use std::{fmt::Debug, str::FromStr};

use aoc::helpers::parse_lines;
use hashbrown::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 10))
}

fn solve(input: &str, size: usize) -> usize {
    let movements = parse_lines::<Movement>(input);
    let mut visited = HashSet::<Point>::new();

    let mut knots: Vec<Point> = Vec::with_capacity(size);
    for _ in 0..size {
        knots.push(Point { x: 0, y: 0 });
    }

    visited.insert(knots[size - 1]);

    for m in movements {
        for _ in 0..m.steps {
            knots[0] = knots[0].towards(m.direction);

            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let point = knots[i];

                let dx = prev.x - point.x;
                let dy = prev.y - point.y;
                knots[i] = Point {
                    x: prev.x - if dx.abs() >= dy.abs() { dx.signum() } else { 0 },
                    y: prev.y - if dy.abs() >= dx.abs() { dy.signum() } else { 0 },
                };
            }

            visited.insert(knots[size - 1]);
        }
    }

    visited.len()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}

impl Point {
    fn towards(&self, d: Direction) -> Point {
        Point {
            x: self.x + d.dx,
            y: self.y + d.dy,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Movement {
    direction: Direction,
    steps: u8,
}

impl FromStr for Movement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (d, s) = s.split_once(' ').unwrap();
        Ok(Movement {
            direction: d.parse()?,
            steps: s.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: i32,
    dy: i32,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "U" => Direction { dx: 0, dy: 1 },
            "D" => Direction { dx: 0, dy: -1 },
            "L" => Direction { dx: -1, dy: 0 },
            "R" => Direction { dx: 1, dy: 0 },
            _ => panic!("invalid direction {}", s),
        })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part_two(input), Some(36));
    }
}
