pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 14)
}

fn solve(input: &str, count: usize) -> Option<usize> {
    let bytes = input.as_bytes();

    let max = bytes.len() - count;
    let mut i = 0;
    while i < max {
        let jump = calculate_forward_skip(&bytes[i..i + count]);
        if jump == 0 {
            return Some(i + count);
        }

        i += jump;
    }

    None
}

fn calculate_forward_skip(bytes: &[u8]) -> usize {
    let mut i = bytes.len();
    while i != 1 {
        let v = bytes[i - 1];
        if let Some(p) = bytes[0..i - 1].iter().position(|x| *x == v) {
            return p + 1;
        }

        i -= 1;
    }

    0
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
