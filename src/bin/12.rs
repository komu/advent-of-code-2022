use anyhow::anyhow;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, iter, str::FromStr};

pub fn part_one(input: &str) -> Option<u16> {
    let height_map = input.parse::<HeightMap>().unwrap();
    height_map.shortest_path_len(iter::once(height_map.start))
}

pub fn part_two(input: &str) -> Option<u16> {
    let height_map = input.parse::<HeightMap>().unwrap();
    height_map.shortest_path_len(height_map.candidate_starts())
}

type Height = u8;
type Point = aoc::point::Point<i16>;

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Height>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

const DIRECTIONS: [(i16, i16); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl HeightMap {
    fn elevation_at(&self, p: Point) -> Height {
        self.heights[self.index_of(p)]
    }

    #[inline]
    fn index_of(&self, p: Point) -> usize {
        p.y as usize * self.width + p.x as usize
    }

    fn candidate_starts(&self) -> impl Iterator<Item = Point> + '_ {
        let width = self.width as i16;
        self.heights
            .iter()
            .enumerate()
            .filter(|(_, h)| **h == 0)
            .map(move |(i, _)| Point {
                x: i as i16 % width,
                y: i as i16 / width,
            })
    }

    fn neighbors(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
        let x_range = 0..self.width as i16;
        let y_range = 0..self.height as i16;
        let max_elevation = self.elevation_at(p) + 1;

        DIRECTIONS
            .iter()
            .map(move |(dx, dy)| Point {
                x: p.x + dx,
                y: p.y + dy,
            })
            .filter(move |&n| {
                x_range.contains(&n.x)
                    && y_range.contains(&n.y)
                    && max_elevation >= self.elevation_at(n)
            })
    }

    fn shortest_path_len<I>(&self, start_points: I) -> Option<u16>
    where
        I: Iterator<Item = Point>,
    {
        let mut open_set = PriorityQueue::new();
        let mut g_score = vec![u16::MAX; self.heights.len()];

        for start in start_points {
            open_set.push(start, Reverse(self.heuristic_distance(start)));
            g_score[self.index_of(start)] = 0;
        }

        let end_index = self.index_of(self.end);
        while let Some((current, _)) = open_set.pop() {
            let current_index = self.index_of(current);
            let current_gscore = g_score[current_index];

            if current_index == end_index {
                return Some(current_gscore);
            }

            let tentative_gscore = current_gscore + 1;
            for neighbor in self.neighbors(current) {
                let neighbor_index = self.index_of(neighbor);
                if tentative_gscore < g_score[neighbor_index] {
                    g_score[neighbor_index] = tentative_gscore;

                    open_set.push(
                        neighbor,
                        Reverse(tentative_gscore + self.heuristic_distance(neighbor)),
                    );
                }
            }
        }

        None
    }

    fn heuristic_distance(&self, p: Point) -> u16 {
        p.manhattan_distance(&self.end)
    }
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();
        let mut height = 0;
        let mut width = 0;
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            width = line.len();
            height += 1;

            for (x, h) in line.bytes().enumerate() {
                match h {
                    b'S' => {
                        start = Point {
                            x: x as i16,
                            y: y as i16,
                        };
                        heights.push(0);
                    }
                    b'E' => {
                        end = Point {
                            x: x as i16,
                            y: y as i16,
                        };
                        heights.push(b'z' - b'a');
                    }
                    b'a'..=b'z' => {
                        heights.push(h - b'a');
                    }
                    _ => {
                        return Err(anyhow!("invalid height '{}'", h));
                    }
                }
            }
        }

        Ok(HeightMap {
            heights,
            width,
            height,
            start,
            end,
        })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
