use aoc::point::CardinalDirection;
use bitset::BitSet;
use itertools::Itertools;
use CardinalDirection::*;

pub fn part_one(input: &str) -> Option<u32> {
    Some(run(input, 10, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(run(input, 1_000_000, true))
}

pub fn run(input: &str, rounds: u32, round_count: bool) -> u32 {
    let mut elves = parse_elves(input);
    let mut elf_points = ElfSet::new(&elves);
    let mut proposals_counts = ProposalMap::new();
    let direction_choices = [
        [
            (N, MASK_NORTH),
            (S, MASK_SOUTH),
            (W, MASK_WEST),
            (E, MASK_EAST),
        ],
        [
            (S, MASK_SOUTH),
            (W, MASK_WEST),
            (E, MASK_EAST),
            (N, MASK_NORTH),
        ],
        [
            (W, MASK_WEST),
            (E, MASK_EAST),
            (N, MASK_NORTH),
            (S, MASK_SOUTH),
        ],
        [
            (E, MASK_EAST),
            (N, MASK_NORTH),
            (S, MASK_SOUTH),
            (W, MASK_WEST),
        ],
    ];
    let mut moves = Vec::<(Point, Point)>::with_capacity(elves.len());

    for r in 0..rounds {
        let directions = &direction_choices[r as usize % 4];

        // collect proposals
        proposals_counts.clear();
        for &elf in &elves {
            if let Some(proposal) = elf_points.proposal(elf, directions) {
                proposals_counts.propose(proposal);
            }
        }

        // act on proposals
        moves.clear();
        for elf in &mut elves {
            if let Some(proposal) = elf_points.proposal(*elf, directions) {
                if proposals_counts.has_single_proposal(proposal) {
                    moves.push((*elf, proposal));
                    *elf = proposal;
                }
            }
        }

        for &(old, new) in &moves {
            elf_points.remove(old);
            elf_points.add(new);
        }

        if moves.is_empty() && round_count {
            return r + 1;
        }
    }

    if round_count {
        panic!("no result");
    }

    let (w, h) = bounding_box_dimensions(&elves);
    w * h - elves.len() as u32
}

type Coord = i8;
type Point = aoc::point::Point<Coord>;

struct ProposalMap {
    proposals: Vec<u8>,
}

const MAX_POINT_INDEX: usize = 4 * 80000;
const INDEX_X_OFFSET: usize = 256;
const INDEX_Y_OFFSET: usize = 256;
const INDEX_STRIDE: usize = 512;

#[inline]
fn point_index(point: Point) -> usize {
    let y = (point.y as isize + INDEX_Y_OFFSET as isize) as usize;
    let x = (point.x as isize + INDEX_X_OFFSET as isize) as usize;
    y * INDEX_STRIDE + x
}

impl ProposalMap {
    fn new() -> Self {
        ProposalMap {
            proposals: vec![0; MAX_POINT_INDEX / 4],
        }
    }

    fn clear(&mut self) {
        self.proposals.fill(0);
    }

    fn propose(&mut self, point: Point) {
        let (array_index, bit_index) = self.index(point);

        let byte = self.proposals[array_index];
        let value = (byte >> bit_index) & 3;
        if value == 0 {
            self.proposals[array_index] = byte | (1 << bit_index);
        } else if value == 1 {
            self.proposals[array_index] = byte | (2 << bit_index);
        }
    }

    fn has_single_proposal(&self, point: Point) -> bool {
        let (array_index, bit_index) = self.index(point);

        ((self.proposals[array_index] >> bit_index) & 3) == 1
    }

    #[inline]
    fn index(&self, point: Point) -> (usize, usize) {
        let index = point_index(point);
        let array_index = index / 4;
        (array_index, 2 * (index % 4))
    }
}

struct ElfSet {
    points: BitSet,
}

const BIT_N: u8 = 0;
const BIT_S: u8 = 1;
const BIT_W: u8 = 2;
const BIT_E: u8 = 3;
const BIT_NW: u8 = 4;
const BIT_SW: u8 = 5;
const BIT_NE: u8 = 6;
const BIT_SE: u8 = 7;

const MASK_NORTH: u8 = (1 << BIT_N) | (1 << BIT_NE) | (1 << BIT_NW);
const MASK_SOUTH: u8 = (1 << BIT_S) | (1 << BIT_SE) | (1 << BIT_SW);
const MASK_EAST: u8 = (1 << BIT_E) | (1 << BIT_SE) | (1 << BIT_NE);
const MASK_WEST: u8 = (1 << BIT_W) | (1 << BIT_SW) | (1 << BIT_NW);

impl ElfSet {
    fn new(elves: &[Point]) -> Self {
        let mut result = ElfSet {
            points: BitSet::with_capacity(MAX_POINT_INDEX),
        };

        for &elf in elves {
            result.add(elf);
        }

        result
    }

    fn add(&mut self, point: Point) {
        self.points.set(point_index(point), true);
    }

    fn remove(&mut self, point: Point) {
        self.points.set(point_index(point), false);
    }

    fn proposal(&self, p: Point, directions: &[(CardinalDirection, u8)]) -> Option<Point> {
        let index = point_index(p);

        let n = self.points.test(index - INDEX_STRIDE) as u8;
        let s = self.points.test(index + INDEX_STRIDE) as u8;
        let w = self.points.test(index - 1) as u8;
        let e = self.points.test(index + 1) as u8;
        let nw = self.points.test(index - INDEX_STRIDE - 1) as u8;
        let sw = self.points.test(index + INDEX_STRIDE - 1) as u8;
        let ne = self.points.test(index - INDEX_STRIDE + 1) as u8;
        let se = self.points.test(index + INDEX_STRIDE + 1) as u8;

        let bits = (n << BIT_N)
            | (s << BIT_S)
            | (w << BIT_W)
            | (e << BIT_E)
            | (nw << BIT_NW)
            | (sw << BIT_SW)
            | (ne << BIT_NE)
            | (se << BIT_SE);

        if bits != 0 {
            for &(cd, mask) in directions {
                if bits & mask == 0 {
                    return Some(p + cd);
                }
            }
        }

        None
    }
}

fn bounding_box_dimensions(elves: &[Point]) -> (u32, u32) {
    let (x_min, x_max) = elves.iter().map(|e| e.x).minmax().into_option().unwrap();
    let (y_min, y_max) = elves.iter().map(|e| e.y).minmax().into_option().unwrap();

    (
        (x_max as i16 - x_min as i16 + 1) as u32,
        (y_max as i16 - y_min as i16 + 1) as u32,
    )
}

fn parse_elves(s: &str) -> Vec<Point> {
    let mut elves = Vec::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Point {
                    x: x as Coord,
                    y: y as Coord,
                });
            }
        }
    }

    elves
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
