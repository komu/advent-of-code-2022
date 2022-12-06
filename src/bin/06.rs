pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 14)
}

fn solve(input: &str, count: usize) -> Option<usize> {
    let bytes = input.as_bytes();

    for i in 0..bytes.len() - count {
        if all_distinct(&bytes[i..i + count]) {
            return Some(i + count);
        }
    }

    None
}

fn all_distinct(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .enumerate()
        .all(|(i, v)| !bytes[i + 1..].contains(v))
}

fn main() {
    let input = &aoc::read_file("inputs", 6);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one(&"nppdvjthqldpwncqszvftbrmjlhg"), Some(6));
        assert_eq!(part_one(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(10));
        assert_eq!(part_one(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(19));
        assert_eq!(part_two(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(23));
        assert_eq!(part_two(&"nppdvjthqldpwncqszvftbrmjlhg"), Some(23));
        assert_eq!(part_two(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), Some(29));
        assert_eq!(part_two(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(26));
    }
}
