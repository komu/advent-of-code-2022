use std::{ops::Range, str::FromStr};

use aoc::helpers::parse_lines;
use bitset::BitSet;
use enum_iterator::{all, Sequence};
use hashbrown::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let cubes = parse_lines::<Point>(input);

    let mut sides = SideSet::new();
    let mut result = 0;

    for cube in cubes {
        result += 6;
        for side in cube.sides() {
            if !sides.insert(&side) {
                result -= 2;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut top_cube = Point::new(0, 0, 0);
    let mut cubes = PointSet::new();

    for cube in parse_lines::<Point>(input).collect::<Vec<_>>() {
        cubes.insert(&cube);

        if cube.y > top_cube.y {
            top_cube = cube;
        }
    }

    //let mut seen = PointSet::new(); //HashSet::with_capacity(1500);
    let mut seen = HashSet::with_capacity(1500);
    let mut outer_sides = SideSet::new();

    let mut queue = vec![Point::new(top_cube.x, top_cube.y + 1, top_cube.z)];

    while let Some(p) = queue.pop() {
        let has_neighbors = p.has_neighbors(&cubes);

        for direction in all::<Direction>() {
            let neighbor = p.towards(direction);

            if cubes.contains(&neighbor) {
                outer_sides.insert(&p.side_towards(direction));
            } else if (has_neighbors || neighbor.has_neighbors(&cubes)) && seen.insert(neighbor) {
                queue.push(neighbor);
            }
        }
    }

    Some(outer_sides.len())
}

type Coord = i8;

struct PointSet {
    bits: BitSet,
    bounds: Range<Coord>,
}

impl PointSet {
    fn new() -> PointSet {
        let bounds = -3..24;
        let size = (bounds.end - bounds.start) as usize;
        PointSet {
            bits: BitSet::with_capacity(size * size * size),
            bounds,
        }
    }

    fn insert(&mut self, point: &Point) -> bool {
        let i = self.index(point);
        let old = self.bits.test(i);
        self.bits.set(i, true);
        !old
    }

    fn contains(&self, point: &Point) -> bool {
        self.bits.test(self.index(point))
    }

    fn index(&self, point: &Point) -> usize {
        let w = (self.bounds.end - self.bounds.start) as usize;
        let o = self.bounds.start;
        let x = (point.x - o) as usize;
        let y = (point.y - o) as usize;
        let z = (point.z - o) as usize;
        (y * w + x) * w + z
    }
}

struct SideSet {
    bits: BitSet,
    bounds: Range<Coord>,
}

impl SideSet {
    fn new() -> SideSet {
        let bounds = -2..24;
        let size = (bounds.end - bounds.start) as usize;
        SideSet {
            bits: BitSet::with_capacity(size * size * size * 6),
            bounds,
        }
    }

    fn insert(&mut self, side: &Side) -> bool {
        let w = (self.bounds.end - self.bounds.start) as usize;
        let o = self.bounds.start;
        let x = (side.x - o) as usize;
        let y = (side.y - o) as usize;
        let z = (side.z - o) as usize;
        let p = side.plane as usize;
        let index = ((y * w + x) * w + z) * 6 + p;

        let old = self.bits.test(index);
        self.bits.set(index, true);
        !old
    }

    fn len(&self) -> u32 {
        self.bits.count() as u32
    }
}

#[derive(Clone, Copy, Sequence)]
#[repr(u8)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Forward,
    Backward,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Point {
    fn new(x: Coord, y: Coord, z: Coord) -> Self {
        Point { x, y, z }
    }

    fn towards(&self, d: Direction) -> Point {
        use Direction::*;
        match d {
            Left => Point::new(self.x - 1, self.y, self.z),
            Right => Point::new(self.x + 1, self.y, self.z),
            Down => Point::new(self.x, self.y - 1, self.z),
            Up => Point::new(self.x, self.y + 1, self.z),
            Backward => Point::new(self.x, self.y, self.z - 1),
            Forward => Point::new(self.x, self.y, self.z + 1),
        }
    }

    fn side_towards(&self, d: Direction) -> Side {
        use Direction::*;
        match d {
            Left => Side::new(Axis::X, self.x, self.y, self.z),
            Right => Side::new(Axis::X, self.x + 1, self.y, self.z),
            Down => Side::new(Axis::Y, self.x, self.y, self.z),
            Up => Side::new(Axis::Y, self.x, self.y + 1, self.z),
            Backward => Side::new(Axis::Z, self.x, self.y, self.z),
            Forward => Side::new(Axis::Z, self.x, self.y, self.z + 1),
        }
    }

    fn has_neighbors(&self, cubes: &PointSet) -> bool {
        all::<Direction>().any(|d| cubes.contains(&self.towards(d)))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Side {
    plane: Axis,
    x: Coord,
    y: Coord,
    z: Coord,
}

impl Side {
    fn new(plane: Axis, x: Coord, y: Coord, z: Coord) -> Self {
        Side { plane, x, y, z }
    }
}

impl Point {
    fn sides(&self) -> [Side; 6] {
        [
            Side::new(Axis::X, self.x, self.y, self.z),
            Side::new(Axis::X, self.x + 1, self.y, self.z),
            Side::new(Axis::Y, self.x, self.y, self.z),
            Side::new(Axis::Y, self.x, self.y + 1, self.z),
            Side::new(Axis::Z, self.x, self.y, self.z),
            Side::new(Axis::Z, self.x, self.y, self.z + 1),
        ]
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(',');
        let x = splits.next().unwrap().parse()?;
        let y = splits.next().unwrap().parse()?;
        let z = splits.next().unwrap().parse()?;

        Ok(Point { x, y, z })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 18);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
