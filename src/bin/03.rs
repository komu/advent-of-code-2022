use std::str::FromStr;

use aoc::helpers::parse_lines;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_lines::<Rucksack>(input)
            .map(|r| r.shared_item().priority())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_lines::<Rucksack>(input)
            .map(|r| r.items())
            .chunks(3)
            .into_iter()
            .map(|g| intersection(g).to_item().priority())
            .sum::<u32>(),
    )
}

#[derive(Clone, Copy, Debug)]
struct Item(u8);

#[derive(Clone, Copy)]
struct ItemSet(u64);

#[derive(Clone, Copy)]
struct Rucksack(ItemSet, ItemSet);

impl FromStr for ItemSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bits: u64 = 0;
        for item in s.chars().map(Item::from) {
            bits |= 1 << item.priority();
        }

        Ok(ItemSet(bits))
    }
}

fn intersection<I>(vals: I) -> ItemSet
where
    I: IntoIterator<Item = ItemSet>,
{
    vals.into_iter().reduce(|a, b| a.intersect(b)).unwrap()
}

impl ItemSet {
    fn union(self, rhs: ItemSet) -> ItemSet {
        ItemSet(self.0 | rhs.0)
    }

    fn intersect(self, rhs: ItemSet) -> ItemSet {
        ItemSet(self.0 & rhs.0)
    }

    fn to_item(self) -> Item {
        assert_ne!(self.0, 0);

        let mut priority = 0;

        let mut m = self.0;
        while m > 1 {
            priority += 1;
            m >>= 1;
        }

        assert_eq!(self.0, 1 << priority);
        Item(priority)
    }
}

impl Rucksack {
    fn shared_item(&self) -> Item {
        self.0.intersect(self.1).to_item()
    }

    fn items(&self) -> ItemSet {
        self.0.union(self.1)
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len() / 2;
        let left = s[..len].parse::<ItemSet>()?;
        let right = s[len..].parse::<ItemSet>()?;

        Ok(Rucksack(left, right))
    }
}

impl Item {
    fn priority(self) -> u32 {
        self.0 as u32
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        if ('a'..='z').contains(&c) {
            Item((c as u8) - b'a' + 1)
        } else if ('A'..='Z').contains(&c) {
            Item((c as u8) - b'A' + 27)
        } else {
            panic!("unexpected char '{}'", c)
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
