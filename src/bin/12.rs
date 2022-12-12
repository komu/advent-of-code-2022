use anyhow::anyhow;
use priority_queue::PriorityQueue;
use std::{cmp::Reverse, str::FromStr};

pub fn part_one(input: &str) -> Option<u32> {
    let height_map = input.parse::<HeightMap>().unwrap();
    height_map.shortest_path_len(vec![height_map.start])
}

pub fn part_two(input: &str) -> Option<u32> {
    let height_map = input.parse::<HeightMap>().unwrap();
    height_map.shortest_path_len(height_map.candidate_starts())
}

type Height = u8;

#[derive(Debug)]
struct HeightMap {
    heights: Vec<Height>,
    width: usize,
    start: Point,
    end: Point,
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl HeightMap {
    fn elevation_at(&self, p: Point) -> Height {
        self.heights[self.index_of(p)]
    }

    fn index_of(&self, p: Point) -> usize {
        p.y as usize * self.width as usize + p.x as usize
    }

    fn candidate_starts(&self) -> Vec<Point> {
        let width = self.width as i32;
        self.heights
            .iter()
            .enumerate()
            .filter(|(_, h)| **h == 0)
            .map(|(i, _)| Point {
                x: i as i32 % width,
                y: i as i32 / width,
            })
            .collect()
    }

    fn neighbors(&self, p: Point) -> Vec<Point> {
        let x_range = 0..self.width as i32;
        let y_range = 0..((self.heights.len() / self.width as usize) as i32);
        let max_elevation = self.elevation_at(p) + 1;

        DIRECTIONS
            .iter()
            .map(|(dx, dy)| (p.x + dx, p.y + dy))
            .filter(|(x, y)| x_range.contains(x) && y_range.contains(y))
            .map(|(x, y)| Point { x, y })
            .filter(|n| max_elevation >= self.elevation_at(*n))
            .collect()
    }

    fn shortest_path_len(&self, start_points: Vec<Point>) -> Option<u32> {
        let mut open_set = PriorityQueue::new();
        let mut g_score = vec![u32::max_value(); self.heights.len()];

        for &start in &start_points {
            open_set.push(start, Reverse(self.heuristic_distance(start)));
            g_score[self.index_of(start)] = 0;
        }

        while let Some((current, _)) = open_set.pop() {
            let current_gscore = g_score[self.index_of(current)];
            if current == self.end {
                return Some(current_gscore);
            }

            let tentative_gscore = current_gscore + 1;
            for neighbor in self.neighbors(current) {
                if tentative_gscore < g_score[self.index_of(neighbor)] {
                    g_score[self.index_of(neighbor)] = tentative_gscore;

                    open_set.push(
                        neighbor,
                        Reverse(tentative_gscore + self.heuristic_distance(neighbor)),
                    );
                }
            }
        }

        None
    }

    fn heuristic_distance(&self, p: Point) -> u32 {
        p.manhattan_distance(self.end) as u32
    }
}

impl FromStr for HeightMap {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = Vec::new();
        let mut width = 0;
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        for (y, line) in s.lines().enumerate() {
            width = line.len();

            for (x, h) in line.bytes().enumerate() {
                match h {
                    b'S' => {
                        start = Point {
                            x: x as i32,
                            y: y as i32,
                        };
                        heights.push(0);
                    }
                    b'E' => {
                        end = Point {
                            x: x as i32,
                            y: y as i32,
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
            start,
            end,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, p: Point) -> u32 {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
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
