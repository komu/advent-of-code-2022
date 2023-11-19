use std::cmp::Reverse;
use std::iter::once;

use hashbrown::HashMap;
use priority_queue::PriorityQueue;

use TripState::{Initial, VisitedEnd, VisitedStartAfterEnd};

pub fn part_one(input: &str) -> Option<u16> {
    Basin::parse(input, false).shortest_path()
}

pub fn part_two(input: &str) -> Option<u16> {
    Basin::parse(input, true).shortest_path()
}

type Minutes = i16;
type Coordinate = i8;

const DIRECTIONS: [(Coordinate, Coordinate); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

type Point = aoc::point::Point<Coordinate>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct SearchState {
    pos: Point,

    // bottom 2 bits is TripState, rest is minutes
    minutes_and_trip_state: u16,
}

const MAX_WIDTH: usize = 128;
const MAX_HEIGHT: usize = 32;
const MAX_MINUTES: usize = 1024;
const MAX_TRIP_STATE: usize = 3;
const TRIP_STATE_BITS: u8 = 2;

impl SearchState {
    fn new(pos: Point, minutes: Minutes, state: TripState) -> Self {
        SearchState {
            pos,
            minutes_and_trip_state: (minutes as u16) << TRIP_STATE_BITS | (state as u16),
        }
    }

    fn index(&self) -> usize {
        let x = (self.pos.x + 1) as usize;
        let y = (self.pos.y + 1) as usize;
        (x * MAX_HEIGHT + y) * MAX_MINUTES + (self.minutes_and_trip_state as usize)
    }

    fn wait(&self) -> SearchState {
        SearchState::new(self.pos, self.minutes() + 1, self.state())
    }

    #[inline]
    fn minutes(&self) -> Minutes {
        (self.minutes_and_trip_state >> TRIP_STATE_BITS) as Minutes
    }

    #[inline]
    fn state(&self) -> TripState {
        match self.minutes_and_trip_state & 3 {
            0 => Initial,
            1 => VisitedEnd,
            2 => VisitedStartAfterEnd,
            _ => panic!("invalid TripState"),
        }
    }
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
}

impl Blizzard {
    fn new(pos: Point) -> Self {
        Blizzard { pos }
    }

    #[inline]
    fn x_at_offset(&self, m: i16, w: i16) -> Coordinate {
        ((self.pos.x as i16 + m) % w) as Coordinate
    }

    #[inline]
    fn y_at_offset(&self, m: i16, h: i16) -> Coordinate {
        ((self.pos.y as i16 + m) % h) as Coordinate
    }
}

#[derive(Debug)]
struct Basin {
    start: Point,
    end: Point,
    width: Coordinate,
    height: Coordinate,
    blizzards_by_row: HashMap<Coordinate, (Vec<Blizzard>, Vec<Blizzard>)>,
    blizzards_by_col: HashMap<Coordinate, (Vec<Blizzard>, Vec<Blizzard>)>,
    go_back_to_start: bool,
}

impl Basin {
    fn parse(s: &str, go_back_to_start: bool) -> Self {
        let lines: Vec<_> = s.lines().filter(|l| l.starts_with('#')).collect();

        let mut blizzards_by_row = HashMap::<Coordinate, (Vec<Blizzard>, Vec<Blizzard>)>::new();
        let mut blizzards_by_col = HashMap::<Coordinate, (Vec<Blizzard>, Vec<Blizzard>)>::new();

        for (y, l) in lines.iter().skip(1).take(lines.len() - 2).enumerate() {
            let line = &l.as_bytes()[1..l.len() - 1];

            for (x, &c) in line.iter().enumerate() {
                if c != b'.' {
                    let pos = Point {
                        x: x as Coordinate,
                        y: y as Coordinate,
                    };

                    match c {
                        b'v' => blizzards_by_col
                            .entry(pos.x)
                            .or_insert_with(|| (Vec::new(), Vec::new()))
                            .0
                            .push(Blizzard::new(pos)),
                        b'^' => blizzards_by_col
                            .entry(pos.x)
                            .or_insert_with(|| (Vec::new(), Vec::new()))
                            .1
                            .push(Blizzard::new(pos)),
                        b'>' => blizzards_by_row
                            .entry(pos.y)
                            .or_insert_with(|| (Vec::new(), Vec::new()))
                            .0
                            .push(Blizzard::new(pos)),
                        b'<' => blizzards_by_row
                            .entry(pos.y)
                            .or_insert_with(|| (Vec::new(), Vec::new()))
                            .1
                            .push(Blizzard::new(pos)),
                        _ => panic!("invalid direction {c}"),
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
        if p.x < 0 || p.y < 0 || p.x >= self.width || p.y >= self.height {
            p == self.start || p == self.end
        } else {
            if let Some((forward, backward)) = self.blizzards_by_col.get(&p.x) {
                let h = self.height as i16;
                let m = minutes % h;
                let m2 = h - m % h;
                if forward.iter().any(|b| b.y_at_offset(m, h) == p.y) {
                    return false;
                }
                if backward.iter().any(|b| b.y_at_offset(m2, h) == p.y) {
                    return false;
                }
            }
            if let Some((forward, backward)) = self.blizzards_by_row.get(&p.y) {
                let w = self.width as i16;
                let m = minutes % w;
                let m2 = w - m % w;
                if forward.iter().any(|b| b.x_at_offset(m, w) == p.x) {
                    return false;
                }
                if backward.iter().any(|b| b.x_at_offset(m2, w) == p.x) {
                    return false;
                }
            }

            true
        }
    }

    fn shortest_path(&self) -> Option<u16> {
        let start = SearchState::new(self.start, 0, Initial);

        let mut g_score = vec![u16::MAX; MAX_HEIGHT * MAX_WIDTH * MAX_MINUTES * MAX_TRIP_STATE];
        let mut open_set = PriorityQueue::<SearchState, Reverse<u16>>::new();

        g_score[start.index()] = 0;
        let start_distance = self.heuristic_distance(&start);
        open_set.push(start, Reverse(start_distance));

        while let Some((current, _)) = open_set.pop() {
            let current_gscore = current.minutes() as u16;

            if current.pos == self.end
                && (!self.go_back_to_start || current.state() == VisitedStartAfterEnd)
            {
                return Some(current.minutes() as u16);
            };
            for neighbor in self.neighbors(&current) {
                let tentative_gscore = current_gscore + 1;
                if tentative_gscore < g_score[neighbor.index()] {
                    let neighbor_score = tentative_gscore + self.heuristic_distance(&neighbor);

                    g_score[neighbor.index()] = tentative_gscore;
                    open_set.push(neighbor, Reverse(neighbor_score));
                }
            }
        }

        None
    }

    fn neighbors(&self, node: &SearchState) -> impl Iterator<Item = SearchState> + '_ {
        let current_pos = node.pos;
        let next_minute = node.minutes() + 1;
        let current_state = node.state();

        let wait_state = node.wait();

        let move_states = DIRECTIONS.iter().map(move |(dx, dy)| {
            let pos = current_pos.towards(*dx, *dy);
            let next_state = match current_state {
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
            };

            SearchState::new(pos, next_minute, next_state)
        });

        move_states
            .chain(once(wait_state))
            .filter(|s| self.is_empty(s.pos, s.minutes()))
    }

    fn heuristic_distance(&self, node: &SearchState) -> u16 {
        let h = if self.go_back_to_start {
            let start_to_end = self.start.manhattan_distance(&self.end) as u16;
            match node.state() {
                Initial => 2 * start_to_end + node.pos.manhattan_distance(&self.end) as u16,
                VisitedEnd => start_to_end + node.pos.manhattan_distance(&self.start) as u16,
                VisitedStartAfterEnd => node.pos.manhattan_distance(&self.end) as u16,
            }
        } else {
            node.pos.manhattan_distance(&self.end) as u16
        };
        if h > 20 {
            h * 2
        } else {
            h
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
