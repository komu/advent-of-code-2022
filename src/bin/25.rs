pub fn part_one(input: &str) -> Option<String> {
    Some(int_to_snafu(input.lines().map(snafu_to_int).sum()))
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

fn snafu_to_int(s: &str) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| snafu_char_to_digit(c) * 5_i64.pow(i as u32))
        .sum()
}

fn int_to_snafu(mut num: i64) -> String {
    let mut str = String::with_capacity(16);

    for e in (0..22).rev() {
        let m = 5i64.pow(e);
        let digit = num.signum() * (num.abs() + m / 2) / m;

        if digit != 0 || !str.is_empty() {
            str.push(snafu_digit_to_char(digit));
            num -= digit * m;
        }
    }

    str
}

const SNAFU_DIGITS: [char; 5] = ['=', '-', '0', '1', '2'];

fn snafu_digit_to_char(c: i64) -> char {
    SNAFU_DIGITS[(c + 2) as usize]
}

fn snafu_char_to_digit(c: char) -> i64 {
    SNAFU_DIGITS
        .iter()
        .enumerate()
        .find(|&(_, &val)| val == c)
        .unwrap()
        .0 as i64
        - 2
}

fn main() {
    let input = &aoc::read_file("inputs", 25);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_to_snafu() {
        assert_eq!(int_to_snafu(1), "1");
        assert_eq!(int_to_snafu(2), "2");
        assert_eq!(int_to_snafu(3), "1=");
        assert_eq!(int_to_snafu(4), "1-");
        assert_eq!(int_to_snafu(5), "10");
        assert_eq!(int_to_snafu(6), "11");
        assert_eq!(int_to_snafu(7), "12");
        assert_eq!(int_to_snafu(8), "2=");
        assert_eq!(int_to_snafu(9), "2-");
        assert_eq!(int_to_snafu(10), "20");
        assert_eq!(int_to_snafu(15), "1=0");
        assert_eq!(int_to_snafu(20), "1-0");
        assert_eq!(int_to_snafu(2022), "1=11-2");
        assert_eq!(int_to_snafu(12345), "1-0---0");
        assert_eq!(int_to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(String::from("2=-1=0")));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 25);
        assert_eq!(part_two(&input), Some(0));
    }
}
