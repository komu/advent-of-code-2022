use aoc::point::CardinalDirection;
use aoc::shortest_path::{shortest_path_len, Graph};
use hashbrown::HashMap;
use std::iter::once;

pub fn part_one(input: &str) -> Option<u32> {
    Basin::parse(input, false).shortest_path()
}

pub fn part_two(input: &str) -> Option<u32> {
    Basin::parse(input, true).shortest_path()
}

type Minutes = u16;
type Coordinate = i8;

const DIRECTIONS: [(Coordinate, Coordinate); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Point = aoc::point::Point<Coordinate>;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct SearchState {
    pos: Point,
    minutes: Minutes,
    state: TripState,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TripState {
    Initial,
    VisitedEnd,
    VisitedStartAfterEnd,
}

#[derive(Debug)]
struct Blizzard {
    pos: Point,
    dir: CardinalDirection,
}

impl Blizzard {
    fn position(&self, minutes: Minutes, width: Coordinate, height: Coordinate) -> Point {
        let mut x = self.pos.x as i32;
        let mut y = self.pos.y as i32;
        let w = width as i32;
        let h = height as i32;
        let mw = (minutes as i32) % w;
        let mh = (minutes as i32) % h;

        match self.dir {
            CardinalDirection::E => x = (x + mw) % w,
            CardinalDirection::W => x = (x + w - mw) % w,
            CardinalDirection::S => y = (y + mh) % h,
            CardinalDirection::N => y = (y + h - mh) % h,
        };

        Point {
            x: x as Coordinate,
            y: y as Coordinate,
        }
    }
}

#[derive(Debug)]
struct Basin {
    start: Point,
    end: Point,
    width: Coordinate,
    height: Coordinate,
    blizzards_by_row: HashMap<Coordinate, Vec<Blizzard>>,
    blizzards_by_col: HashMap<Coordinate, Vec<Blizzard>>,
    go_back_to_start: bool,
}

impl Basin {
    fn parse(s: &str, go_back_to_start: bool) -> Self {
        let lines: Vec<_> = s.lines().filter(|l| l.starts_with('#')).collect();

        let mut blizzards_by_row = HashMap::<Coordinate, Vec<Blizzard>>::new();
        let mut blizzards_by_col = HashMap::<Coordinate, Vec<Blizzard>>::new();

        for (y, l) in lines.iter().skip(1).take(lines.len() - 2).enumerate() {
            let line = &l.as_bytes()[1..l.len() - 1];

            for (x, &c) in line.iter().enumerate() {
                if c != b'.' {
                    let blizzard = Blizzard {
                        pos: Point {
                            x: x as Coordinate,
                            y: y as Coordinate,
                        },
                        dir: CardinalDirection::for_code(c as char),
                    };

                    match blizzard.dir {
                        CardinalDirection::N | CardinalDirection::S => blizzards_by_col
                            .entry(blizzard.pos.x)
                            .or_default()
                            .push(blizzard),
                        CardinalDirection::W | CardinalDirection::E => blizzards_by_row
                            .entry(blizzard.pos.y)
                            .or_default()
                            .push(blizzard),
                    }
                }
            }
        }

        let width = (lines[0].len() - 2) as Coordinate;
        let height = (lines.len() - 2) as Coordinate;
        Basin {
            start: Point { x: 0, y: -1 },
            end: Point {
                x: width - 1,
                y: height,
            },
            width,
            height,
            blizzards_by_row,
            blizzards_by_col,
            go_back_to_start,
        }
    }

    fn is_empty(&self, p: Point, minutes: Minutes) -> bool {
        if p == self.start || p == self.end {
            true
        } else if p.x < 0 || p.y < 0 || p.x >= self.width || p.y >= self.height {
            false
        } else {
            if let Some(blizzards) = self.blizzards_by_col.get(&p.x) {
                if blizzards
                    .iter()
                    .any(|b| b.position(minutes, self.width, self.height) == p)
                {
                    return false;
                }
            }
            if let Some(blizzards) = self.blizzards_by_row.get(&p.y) {
                if blizzards
                    .iter()
                    .any(|b| b.position(minutes, self.width, self.height) == p)
                {
                    return false;
                }
            }
            return true;
        }
    }

    fn shortest_path(&self) -> Option<u32> {
        let start = SearchState {
            pos: self.start,
            minutes: 0,
            state: TripState::Initial,
        };
        shortest_path_len(self, start).map(|x| x.1)
    }
}

impl Graph for Basin {
    type Node = SearchState;

    fn is_solution(&self, node: &Self::Node) -> bool {
        node.pos == self.end
            && (!self.go_back_to_start || node.state == TripState::VisitedStartAfterEnd)
    }

    fn collect_neighbors(&self, node: &Self::Node, neighbors: &mut Vec<(Self::Node, u32)>) {
        let current_pos = node.pos;
        let current_minutes = node.minutes;
        let current_state = node.state;

        let wait_state = SearchState {
            pos: current_pos,
            minutes: current_minutes + 1,
            state: current_state,
        };

        let move_states = DIRECTIONS.iter().map(move |(dx, dy)| {
            use TripState::*;

            let pos = current_pos.towards(*dx, *dy);
            SearchState {
                pos,
                minutes: current_minutes + 1,
                state: match current_state {
                    Initial => {
                        if pos == self.end {
                            VisitedEnd
                        } else {
                            Initial
                        }
                    }
                    VisitedEnd => {
                        if pos == self.start {
                            VisitedStartAfterEnd
                        } else {
                            VisitedEnd
                        }
                    }
                    VisitedStartAfterEnd => VisitedStartAfterEnd,
                },
            }
        });

        let cs = move_states
            .chain(once(wait_state))
            .filter(|s| self.is_empty(s.pos, s.minutes))
            .map(|n| (n, 1));

        neighbors.extend(cs);
    }

    fn heuristic_distance(&self, node: &Self::Node) -> u32 {
        if self.go_back_to_start {
            let start_to_end = self.start.manhattan_distance(&self.end);
            (match node.state {
                TripState::Initial => 2 * start_to_end + node.pos.manhattan_distance(&self.end),
                TripState::VisitedEnd => start_to_end + node.pos.manhattan_distance(&self.start),
                TripState::VisitedStartAfterEnd => node.pos.manhattan_distance(&self.end),
            }) as u32
        } else {
            node.pos.manhattan_distance(&self.end) as u32
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 24);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blizzard_position_right() {
        let w = 7;
        let h = 5;
        let blizzard = Blizzard {
            pos: Point { x: 2, y: 4 },
            dir: CardinalDirection::E,
        };

        assert_eq!(Point { x: 2, y: 4 }, blizzard.position(0, w, h));
        assert_eq!(Point { x: 3, y: 4 }, blizzard.position(1, w, h));
        assert_eq!(Point { x: 4, y: 4 }, blizzard.position(2, w, h));
        assert_eq!(Point { x: 5, y: 4 }, blizzard.position(3, w, h));
        assert_eq!(Point { x: 6, y: 4 }, blizzard.position(4, w, h));
        assert_eq!(Point { x: 0, y: 4 }, blizzard.position(5, w, h));
        assert_eq!(Point { x: 1, y: 4 }, blizzard.position(6, w, h));
        assert_eq!(Point { x: 2, y: 4 }, blizzard.position(7, w, h));
        assert_eq!(Point { x: 3, y: 4 }, blizzard.position(8, w, h));
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
