use itertools::iproduct;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<usize> {
    let forest: Forest = input.parse().unwrap();

    let size = forest.width * forest.height;
    let mut left_max = vec![0; size];
    let mut right_max = vec![0; size];
    let mut top_max = vec![0; size];
    let mut bottom_max = vec![0; size];

    for x in 1..forest.width - 1 {
        forest.build_maximums(&mut left_max, (0..forest.height - 1).map(|y| (x, y)));
        forest.build_maximums(&mut right_max, (1..forest.height).rev().map(|y| (x, y)));
    }

    for y in 1..forest.height - 1 {
        forest.build_maximums(&mut top_max, (0..forest.width - 1).map(|x| (x, y)));
        forest.build_maximums(&mut bottom_max, (1..forest.width).rev().map(|x| (x, y)));
    }

    let mut inner_count = 0;
    for y in 1..forest.height - 1 {
        for x in 1..forest.width - 1 {
            let i = y * forest.width + x;
            let height = forest.tree_height_at(i);
            if height > right_max[i]
                || height > left_max[i]
                || height > top_max[i]
                || height > bottom_max[i]
            {
                inner_count += 1;
            }
        }
    }

    let outer_count = 2 * forest.width + 2 * (forest.height - 2);
    Some(outer_count + inner_count)
}

pub fn part_two(input: &str) -> Option<usize> {
    let forest: Forest = input.parse().unwrap();

    let coords = iproduct!(1..forest.height - 1, 1..forest.width - 1);
    let score = coords
        .map(|(x, y)| forest.scenic_score(x, y))
        .max()
        .unwrap();

    Some(score)
}

struct Forest {
    tree_heights: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Forest {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Forest, Self::Err> {
        let mut tree_heights: Vec<u8> = Vec::with_capacity(100 * 100);
        let mut height = 0;

        for line in s.lines() {
            height += 1;

            for c in line.chars() {
                tree_heights.push(c.to_digit(10).unwrap() as u8);
            }
        }

        let width = tree_heights.len() / height;

        Ok(Forest {
            tree_heights,
            width,
            height,
        })
    }
}

impl Forest {
    fn tree_height(&self, x: usize, y: usize) -> u8 {
        self.tree_height_at(y * self.width + x)
    }

    fn tree_height_at(&self, index: usize) -> u8 {
        self.tree_heights[index]
    }

    fn build_maximums<I>(&self, max_heights: &mut [u8], coordinates: I)
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let mut previous_max: u8 = 0;
        for (x, y) in coordinates {
            max_heights[y * self.width + x] = previous_max;

            let tree_height = self.tree_height(x, y);
            if tree_height > previous_max {
                previous_max = tree_height;
            }
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let height = self.tree_height(x, y);
        if height < 5 {
            return 0;
        }

        let u = self.visible_trees(height, (1..y).rev().map(|y| (x, y)));
        let d = self.visible_trees(height, (y + 1..self.height - 1).map(|y| (x, y)));
        let r = self.visible_trees(height, (1..x).rev().map(|x| (x, y)));
        let l = self.visible_trees(height, (x + 1..self.width - 1).map(|x| (x, y)));

        u * d * r * l
    }

    fn visible_trees<I>(&self, height: u8, coords: I) -> usize
    where
        I: Iterator<Item = (usize, usize)>,
    {
        1 + coords
            .take_while(|(x, y)| self.tree_height(*x, *y) < height)
            .count()
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
