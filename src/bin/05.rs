use aoc::helpers::{mut_refs, parse_lines};
use itertools::Itertools;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        for _ in 0..mv.count {
            stacks.move_multiple(mv.from, mv.to, 1)
        }
    }

    Some(stacks.top_str())
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for mv in moves {
        stacks.move_multiple(mv.from, mv.to, mv.count);
    }

    Some(stacks.top_str())
}

fn parse_input(input: &str) -> (Stacks, impl Iterator<Item = Move> + '_) {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();
    let stacks = stacks_input.parse::<Stacks>().unwrap();
    let moves = parse_lines::<Move>(moves_input);

    (stacks, moves)
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Crate(u8);

impl Crate {
    fn from_char(c: char) -> Crate {
        Crate(c as u8)
    }

    fn to_char(self) -> char {
        self.0 as char
    }
}

struct Stacks {
    stacks: Vec<Vec<Crate>>,
}

impl Stacks {
    fn top_str(&self) -> String {
        self.stacks.iter().map(|s| s.last().unwrap().to_char()).join("")
    }

    fn move_multiple(&mut self, from: usize, to: usize, count: usize) {
        let (source, target) = mut_refs(&mut self.stacks, from, to);

        let offset = source.len() - count;
        target.extend(&source[offset..]);
        source.truncate(offset);
    }
}

impl FromStr for Stacks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks: Vec<Vec<Crate>> = Vec::new();

        for line in s.lines() {
            let chars: Vec<_> = line.chars().collect();
            if line.starts_with(" 1") {
                break;
            }

            for i in 0..usize::max_value() {
                let j = i * 4 + 1;
                if j >= line.len() {
                    break;
                }

                if i >= stacks.len() {
                    stacks.push(Vec::new());
                }

                if chars[j] != ' ' {
                    stacks[i].push(Crate::from_char(chars[j]));
                }
            }
        }

        for stack in stacks.iter_mut() {
            stack.reverse();
        }

        Ok(Stacks { stacks })
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let two_digit_count = s.len() == 19;
        let d = usize::from(two_digit_count);
        Ok(Move {
            count: s[5..6 + d].parse::<usize>().unwrap(),
            from: s[12 + d..13 + d].parse::<usize>().unwrap() - 1,
            to: s[17 + d..18 + d].parse::<usize>().unwrap() - 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
