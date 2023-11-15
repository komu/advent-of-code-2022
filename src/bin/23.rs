use enum_iterator::{all, Sequence};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Sequence)]
enum Direction {
    N,
    S,
    W,
    E,
    NE,
    NW,
    SE,
    SW,
}

impl Direction {
    fn deltas(self) -> (i32, i32) {
        match self {
            Direction::N => (0, -1),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
            Direction::E => (1, 0),
            Direction::NE => (1, -1),
            Direction::NW => (-1, -1),
            Direction::SE => (1, 1),
            Direction::SW => (-1, 1),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum CardinalDirection {
    N,
    S,
    W,
    E,
}

impl CardinalDirection {
    fn deltas(self) -> (i32, i32) {
        match self {
            CardinalDirection::N => (0, -1),
            CardinalDirection::S => (0, 1),
            CardinalDirection::W => (-1, 0),
            CardinalDirection::E => (1, 0),
        }
    }

    fn adjacent(self) -> Vec<Direction> {
        match self {
            CardinalDirection::N => vec![Direction::N, Direction::NE, Direction::NW],
            CardinalDirection::S => vec![Direction::S, Direction::SE, Direction::SW],
            CardinalDirection::W => vec![Direction::W, Direction::NW, Direction::SW],
            CardinalDirection::E => vec![Direction::E, Direction::NE, Direction::SE],
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(run(input, 10, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run(input, 1_000_000, true))
}

pub fn run(input: &str, rounds: u32, round_count: bool) -> u32 {
    let mut elves = ElfMap::parse(input);

    let directions_vecs = vec![
        vec![CardinalDirection::N, CardinalDirection::S, CardinalDirection::W, CardinalDirection::E],
        vec![CardinalDirection::S, CardinalDirection::W, CardinalDirection::E, CardinalDirection::N],
        vec![CardinalDirection::W, CardinalDirection::E, CardinalDirection::N, CardinalDirection::S],
        vec![CardinalDirection::E, CardinalDirection::N, CardinalDirection::S, CardinalDirection::W],
    ];

    for r in 0..rounds {
        let directions = &directions_vecs[r as usize % 4];

        let mut proposals_counts = HashMap::<Point, usize>::new();

        // collect proposals
        for elf in &elves.elves {
            if let Some(proposal) = elves.proposal(elf, directions) {
                *proposals_counts.entry(proposal).or_insert(0) += 1;
            }
        }

        let mut next_gen = ElfMap::new();

        // act on proposals
        let mut moves = 0;
        for elf in &elves.elves {
            let new_pos = if let Some(proposal) = elves.proposal(elf, directions) {
                if let Some(1) = proposals_counts.get(&proposal) {
                    moves += 1;
                    proposal
                } else {
                    *elf
                }
            } else {
                *elf
            };
            next_gen.elves.insert(new_pos);
        }

        if moves == 0 && round_count {
            return r + 1;
        }
        elves = next_gen;
    }

    if round_count {
        panic!("no result");
    }

    let (x_min, x_max) = elves.elves.iter().map(|e| e.x).minmax().into_option().unwrap();
    let (y_min, y_max) = elves.elves.iter().map(|e| e.y).minmax().into_option().unwrap();

    let w = (x_max - x_min + 1) as u32;
    let h = (y_max - y_min + 1) as u32;
    w * h - elves.elves.len() as u32
}

type Point = aoc::point::Point<i32>;

#[derive(Debug)]
struct ElfMap {
    elves: HashSet<Point>,
}

impl ElfMap {
    fn proposal(&self, elf: &Point, directions: &[CardinalDirection]) -> Option<Point> {
        if self.is_alone(elf) {
            return None
        }
        for &cd in directions {
            if self.is_free(elf, cd) {
                let (dx, dy) = cd.deltas();
                return Some(elf.towards(dx, dy))
            }
        }
        None
    }
}

impl ElfMap {
    fn is_alone(&self, p: &Point) -> bool {
        for d in all::<Direction>() {
            let (dx, dy) = d.deltas();
            let p2 = p.towards(dx, dy);
            if self.elves.contains(&p2) {
                return false;
            }
        }
        true
    }

    fn is_free(&self, p: &Point, cd: CardinalDirection) -> bool {
        for d in cd.adjacent() {
            let (dx, dy) = d.deltas();
            let p2 = p.towards(dx, dy);
            if self.elves.contains(&p2) {
                return false;
            }
        }
        true
    }

    #[allow(dead_code)]
    fn dump(&self) {
        let (x_min, x_max) = self.elves.iter().map(|e| e.x).minmax().into_option().unwrap();
        let (y_min, y_max) = self.elves.iter().map(|e| e.y).minmax().into_option().unwrap();

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let p = Point { x, y };
                if self.elves.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
        println!();
    }
}

impl ElfMap {
    fn new() -> Self {
        ElfMap { elves: HashSet::new() }
    }

    fn parse(s: &str) -> Self {
        let mut elves = HashSet::new();

        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert(Point { x: x as i32, y: y as i32 });
                }
            }
        }

        ElfMap {
            elves
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 23);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
