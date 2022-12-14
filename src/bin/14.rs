use std::{ops::RangeInclusive, str::FromStr};

use itertools::Itertools;

type Coordinate = i16;
type Point = aoc::point::Point<Coordinate>;

pub fn part_one(input: &str) -> Option<u32> {
    let scan: Scan = input.parse().unwrap();

    let mut grid = Grid::new(scan.x_range(), scan.height());

    grid.add_paths(&scan.paths);

    let mut fill_count = 0;
    let start = Point { x: 500, y: 0 };

    let mut point = start;
    let mut path = vec![];
    loop {
        let down = point.towards(0, 1);
        let dl = point.towards(-1, 1);
        let dr = point.towards(1, 1);

        if down.y == grid.height {
            break;
        }

        if grid.is_empty(&down) {
            path.push(point);
            point = down;
        } else if grid.is_empty(&dl) {
            path.push(point);
            point = dl;
        } else if grid.is_empty(&dr) {
            path.push(point);
            point = dr;
        } else {
            fill_count += 1;
            grid.fill(&point);
            point = path.pop().unwrap();
        }
    }

    Some(fill_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let scan: Scan = input.parse().unwrap();

    let x_range = scan.x_range();
    let mut grid = Grid::new(
        x_range.start() - scan.height()..=x_range.end() + scan.height(),
        scan.height() + 1,
    );

    grid.floor = true;
    grid.add_paths(&scan.paths);

    let mut fill_count = 0;
    let mut path = vec![];
    let mut point = Point { x: 500, y: 0 };
    loop {
        let down = point.towards(0, 1);
        let dl = point.towards(-1, 1);
        let dr = point.towards(1, 1);

        if grid.is_empty(&down) {
            path.push(point);
            point = down;
        } else if grid.is_empty(&dl) {
            path.push(point);
            point = dl;
        } else if grid.is_empty(&dr) {
            path.push(point);
            point = dr;
        } else {
            fill_count += 1;
            grid.fill(&point);
            if let Some(up) = path.pop() {
                point = up;
            } else {
                break;
            }
        }
    }

    Some(fill_count)
}

struct Grid {
    filled: Vec<bool>,
    x_offset: Coordinate,
    width: usize,
    height: Coordinate,
    floor: bool,
}

impl Grid {
    fn new(x_range: RangeInclusive<Coordinate>, height: Coordinate) -> Self {
        let width = (x_range.end() - x_range.start() + 1) as usize;
        Grid {
            filled: vec![false; width * height as usize],
            x_offset: *x_range.start(),
            width,
            height,
            floor: false,
        }
    }

    fn is_empty(&self, p: &Point) -> bool {
        (!self.floor || p.y < self.height) && !self.filled[self.index(p.x, p.y)]
    }

    fn add_paths(&mut self, paths: &[Vec<Point>]) {
        for path in paths {
            for (a, b) in path.iter().tuple_windows() {
                if a.x == b.x {
                    for y in a.y.min(b.y)..=a.y.max(b.y) {
                        self.fill(&Point { x: a.x, y });
                    }
                } else {
                    for x in a.x.min(b.x)..=a.x.max(b.x) {
                        self.fill(&Point { x, y: a.y });
                    }
                }
            }
        }
    }

    fn fill(&mut self, p: &Point) {
        let i = self.index(p.x, p.y);
        self.filled[i] = true;
    }

    fn index(&self, x: Coordinate, y: Coordinate) -> usize {
        y as usize * self.width + ((x - self.x_offset) as usize)
    }
}

struct Scan {
    paths: Vec<Vec<Point>>,
}

impl Scan {
    fn x_range(&self) -> RangeInclusive<Coordinate> {
        let (min, max) = self
            .paths
            .iter()
            .flatten()
            .map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        min - 1..=max + 1
    }

    fn height(&self) -> Coordinate {
        1 + self.paths.iter().flatten().map(|p| p.y).max().unwrap()
    }
}

impl FromStr for Scan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|s| s.split(" -> ").map(|s| s.parse().unwrap()).collect());
        Ok(Scan {
            paths: lines.collect(),
        })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
