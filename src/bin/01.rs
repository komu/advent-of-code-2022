use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    parse_elf_totals(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_elf_totals(input).sorted().rev().take(3).sum::<u32>())
}

fn parse_elf_totals(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        .split("\n\n")
        .map(|part| part.lines().map(|s| s.parse::<u32>().unwrap()).sum())
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
