use std::cmp::Reverse;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    parse_elf_totals(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        parse_elf_totals(input)
            .map(Reverse)
            .k_smallest(3)
            .map(|r| r.0)
            .sum::<u32>(),
    )
}

fn parse_elf_totals(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().batching(|it| {
        let mut sum = 0;
        loop {
            match it.next() {
                None => {
                    if sum > 0 {
                        return Some(sum);
                    } else {
                        return None;
                    }
                }
                Some("") => return Some(sum),
                Some(s) => sum += s.parse::<u32>().unwrap(),
            }
        }
    })
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
