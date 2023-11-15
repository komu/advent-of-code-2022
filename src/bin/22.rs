pub fn part_one(input: &str) -> Option<u32> {
    let map = MonkeyMap::parse(input);
    let wrap_strategy = WrapStrategy::Simple {
        width: map.width,
        height: map.height,
    };
    Some(run(map, wrap_strategy))
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = MonkeyMap::parse(input);
    let wraps = if map.width < 20 {
        example_wrap_definitions()
    } else {
        real_wrap_definitions()
    };
    Some(run(map, WrapStrategy::Complex(wraps)))
}

fn run(map: MonkeyMap, wrap_strategy: WrapStrategy) -> u32 {
    let mut facing = Facing::Right;
    let mut position = map.start_point();

    for s in &map.path {
        match *s {
            Instruction::Forward(steps) => {
                for _ in 0..steps {
                    if let Some((new_point, new_facing)) =
                        map.move_towards(position, facing, &wrap_strategy)
                    {
                        position = new_point;
                        facing = new_facing;
                    } else {
                        break;
                    }
                }
            }
            Instruction::Turn(t) => facing.turn(t),
        }
    }

    let row = position.y + 1;
    let col = position.x + 1;
    1000 * (row as u32) + 4 * (col as u32) + (facing as u32)
}

type Point = aoc::point::Point<i16>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl From<u8> for Facing {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!(),
        }
    }
}

impl Facing {
    fn turn(&mut self, turn: Turn) {
        *self = Facing::from(match turn {
            Turn::Left => (*self as u8 + 3) % 4,
            Turn::Right => (*self as u8 + 1) % 4,
        });
    }

    fn deltas(self) -> (i16, i16) {
        match self {
            Facing::Right => (1, 0),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Up => (0, -1),
        }
    }

    fn opposite(self) -> Facing {
        match self {
            Facing::Right => Facing::Left,
            Facing::Down => Facing::Up,
            Facing::Left => Facing::Right,
            Facing::Up => Facing::Down,
        }
    }
}

fn map_point((a1, _a2): (Point, Point), (b1, b2): (Point, Point), p: Point) -> Point {
    let d = (p.x - a1.x) + (p.y - a1.y);
    let b_dx = (b2.x - b1.x).signum();
    let b_dy = (b2.y - b1.y).signum();

    Point {
        x: b1.x + d * b_dx,
        y: b1.y + d * b_dy,
    }
}

#[derive(Clone, Copy)]
struct Boundary {
    start: Point,
    end: Point,
    facing: Facing,
}

impl Boundary {
    fn left(x: i16, y: i16, len: i16) -> Boundary {
        Boundary {
            start: Point { x, y },
            end: Point { x, y: y + len - 1 },
            facing: Facing::Right,
        }
    }

    fn right(x: i16, y: i16, len: i16) -> Boundary {
        Boundary {
            start: Point { x, y },
            end: Point { x, y: y + len - 1 },
            facing: Facing::Left,
        }
    }

    fn top(x: i16, y: i16, len: i16) -> Boundary {
        Boundary {
            start: Point { x, y },
            end: Point { x: x + len - 1, y },
            facing: Facing::Down,
        }
    }

    fn bottom(x: i16, y: i16, len: i16) -> Boundary {
        Boundary {
            start: Point { x, y },
            end: Point { x: x + len - 1, y },
            facing: Facing::Up,
        }
    }

    fn reverse(&self) -> Boundary {
        Boundary {
            start: self.end,
            end: self.start,
            facing: self.facing,
        }
    }

    fn shift_start(&self) -> Boundary {
        match self.facing {
            Facing::Left => *self,
            Facing::Up => *self,
            Facing::Right => Boundary {
                start: self.start.towards(-1, 0),
                end: self.end.towards(-1, 0),
                facing: self.facing,
            },
            Facing::Down => Boundary {
                start: self.start.towards(0, -1),
                end: self.end.towards(0, -1),
                facing: self.facing,
            },
        }
    }

    fn shift_target(&self) -> Boundary {
        match self.facing {
            Facing::Right => *self,
            Facing::Down => *self,
            Facing::Left => Boundary {
                start: self.start.towards(-1, 0),
                end: self.end.towards(-1, 0),
                facing: self.facing,
            },
            Facing::Up => Boundary {
                start: self.start.towards(0, -1),
                end: self.end.towards(0, -1),
                facing: self.facing,
            },
        }
    }
}

fn real_wrap_definitions() -> Vec<WrapDefinition> {
    let len = 50;
    let a = Boundary::top(50, 0, len);
    let b = Boundary::top(100, 0, len);
    let c = Boundary::left(50, 0, len);
    let d = Boundary::right(150, 0, len);
    let e = Boundary::bottom(100, 50, len);
    let f = Boundary::left(50, 50, len);
    let g = Boundary::right(100, 50, len);
    let h = Boundary::top(0, 100, len);
    let i = Boundary::left(0, 100, len);
    let j = Boundary::right(100, 100, len);
    let k = Boundary::bottom(50, 150, len);
    let l = Boundary::left(0, 150, len);
    let m = Boundary::right(50, 150, len);
    let n = Boundary::bottom(0, 200, len);
    vec![
        WrapDefinition::new(a, l),
        WrapDefinition::new(l, a),
        WrapDefinition::new(b, n),
        WrapDefinition::new(n, b),
        WrapDefinition::new(c, i.reverse()),
        WrapDefinition::new(i, c.reverse()),
        WrapDefinition::new(d, j.reverse()),
        WrapDefinition::new(j, d.reverse()),
        WrapDefinition::new(e, g),
        WrapDefinition::new(g, e),
        WrapDefinition::new(f, h),
        WrapDefinition::new(h, f),
        WrapDefinition::new(k, m),
        WrapDefinition::new(m, k),
    ]
}

fn example_wrap_definitions() -> Vec<WrapDefinition> {
    let len = 4;
    vec![
        WrapDefinition::new(
            Boundary::right(12, 4, len),
            Boundary::top(12, 8, len).reverse(),
        ),
        WrapDefinition::new(
            Boundary::bottom(8, 12, len),
            Boundary::bottom(0, 8, len).reverse(),
        ),
        WrapDefinition::new(Boundary::top(4, 4, len), Boundary::left(8, 0, len)),
    ]
}

struct WrapDefinition {
    from: Boundary,
    to: Boundary,
}

impl WrapDefinition {
    fn new(from: Boundary, to: Boundary) -> Self {
        WrapDefinition {
            from: from.shift_start(),
            to: to.shift_target(),
        }
    }

    fn wrap(&self, p: Point) -> (Point, Facing) {
        (
            map_point(
                (self.from.start, self.from.end),
                (self.to.start, self.to.end),
                p,
            ),
            self.to.facing,
        )
    }

    fn matches(&self, p: Point, d: Facing) -> bool {
        let x_range = self.from.start.x..=self.from.end.x;
        let y_range = self.from.start.y..=self.from.end.y;

        x_range.contains(&p.x) && y_range.contains(&p.y) && self.from.facing == d.opposite()
    }
}

enum WrapStrategy {
    Simple { width: i16, height: i16 },
    Complex(Vec<WrapDefinition>),
}

impl WrapStrategy {
    fn wrap(&self, mut p: Point, facing: Facing) -> (Point, Facing) {
        match self {
            &WrapStrategy::Simple { width, height } => {
                if p.x < 0 {
                    p.x = width - 1;
                } else if p.x >= width {
                    p.x = 0;
                } else if p.y < 0 {
                    p.y = height - 1;
                } else if p.y >= height {
                    p.y = 0;
                } else {
                    let (dx, dy) = facing.deltas();
                    p = p.towards(dx, dy);
                }
                (p, facing)
            }
            WrapStrategy::Complex(wraps) => {
                for wrap in wraps {
                    if wrap.matches(p, facing) {
                        return wrap.wrap(p);
                    }
                }
                panic!("unregistered wrap at {:?}, {:?}", p, facing);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
enum Instruction {
    Forward(u16),
    Turn(Turn),
}

impl Instruction {
    fn parse_list(suffix: &str) -> Vec<Instruction> {
        let mut result = Vec::new();

        let mut steps = 0;
        for c in suffix.chars() {
            if c.is_ascii_digit() {
                steps = steps * 10 + c.to_digit(10).unwrap() as u16;
            } else {
                if steps != 0 {
                    result.push(Instruction::Forward(steps));
                    steps = 0;
                }

                result.push(Instruction::Turn(match c {
                    'L' => Turn::Left,
                    'R' => Turn::Right,
                    _ => panic!("invalid turn '{c}'"),
                }));
            }
        }

        if steps != 0 {
            result.push(Instruction::Forward(steps));
        }

        result
    }
}

struct MonkeyMap<'a> {
    grid: Vec<&'a [u8]>,
    width: i16,
    height: i16,
    path: Vec<Instruction>,
}

impl<'a> MonkeyMap<'a> {
    fn parse(s: &'a str) -> Self {
        let mut grid = Vec::new();

        let mut width = 0;
        let (prefix, suffix) = s.split_once("\n\n").unwrap();
        for line in prefix.lines() {
            width = width.max(line.as_bytes().len() as i16);
            grid.push(line.as_bytes());
        }

        let height = grid.len() as i16;

        MonkeyMap {
            grid,
            width,
            height,
            path: Instruction::parse_list(suffix.trim_end()),
        }
    }

    fn start_point(&self) -> Point {
        Point {
            x: self.grid[0].iter().position(|&c| c == b'.').unwrap() as i16,
            y: 0,
        }
    }

    fn move_towards(
        &self,
        mut p: Point,
        mut facing: Facing,
        wrap_strategy: &WrapStrategy,
    ) -> Option<(Point, Facing)> {
        let (dx, dy) = facing.deltas();

        p = p.towards(dx, dy);

        while self.get(p) == b' ' {
            (p, facing) = wrap_strategy.wrap(p, facing);
        }

        if self.get(p) == b'.' {
            Some((p, facing))
        } else {
            None
        }
    }

    fn get(&self, p: Point) -> u8 {
        if p.y >= 0 && p.y < self.grid.len() as i16 && p.x >= 0 {
            let row = self.grid[p.y as usize];
            if p.x < row.len() as i16 {
                return row[p.x as usize];
            }
        }
        b' '
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 22);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mapping_points() {
        // (12,4)-(12,7) gets mapped to (8,12)-(8,15)
        let a1 = Point { x: 12, y: 4 };
        let a2 = Point { x: 12, y: 7 };
        let b1 = Point { x: 12, y: 8 };
        let b2 = Point { x: 15, y: 8 };
        let a = (a1, a2);
        let b = (b1, b2);

        assert_eq!(
            Point { x: 12, y: 8 },
            map_point(a, b, Point { x: 12, y: 4 })
        );
        assert_eq!(
            Point { x: 13, y: 8 },
            map_point(a, b, Point { x: 12, y: 5 })
        );
        assert_eq!(
            Point { x: 14, y: 8 },
            map_point(a, b, Point { x: 12, y: 6 })
        );
        assert_eq!(
            Point { x: 15, y: 8 },
            map_point(a, b, Point { x: 12, y: 7 })
        );
    }

    #[test]
    fn test_mapping_points_reversed() {
        // (12,4)-(12,7) gets mapped to (8,15)-(8,12)
        let a1 = Point { x: 12, y: 4 };
        let a2 = Point { x: 12, y: 7 };
        let b1 = Point { x: 15, y: 8 };
        let b2 = Point { x: 12, y: 8 };
        let a = (a1, a2);
        let b = (b1, b2);

        assert_eq!(
            Point { x: 15, y: 8 },
            map_point(a, b, Point { x: 12, y: 4 })
        );
        assert_eq!(
            Point { x: 14, y: 8 },
            map_point(a, b, Point { x: 12, y: 5 })
        );
        assert_eq!(
            Point { x: 13, y: 8 },
            map_point(a, b, Point { x: 12, y: 6 })
        );
        assert_eq!(
            Point { x: 12, y: 8 },
            map_point(a, b, Point { x: 12, y: 7 })
        );
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(5031));
    }
}
