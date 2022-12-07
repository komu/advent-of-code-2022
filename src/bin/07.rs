use std::str::FromStr;

use aoc::helpers::parse_lines;

pub fn part_one(input: &str) -> Option<u32> {
    let lines = parse_lines::<SessionLine>(input);

    let threshold = 100000;
    let mut sums = vec![0];
    let mut result = 0;

    for line in lines {
        match line {
            SessionLine::MoveUp => {
                let last_sum = *sums.last().unwrap();
                if last_sum <= threshold {
                    result += last_sum;
                }
                sums.pop();
            }
            SessionLine::MoveDown => {
                sums.push(0);
            }
            SessionLine::FileSize(len) => {
                for sum in &mut sums {
                    *sum += len;
                }
            }
            _ => {}
        }
    }

    for sum in sums {
        if sum <= threshold {
            result += sum;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = parse_lines::<SessionLine>(input).collect();

    let total = 70000000;
    let needs = 30000000;

    let mut used = 0;
    for line in &lines {
        if let SessionLine::FileSize(len) = line {
            used += len;
        }
    }

    let sufficient = needs - (total - used);
    let mut sums = vec![0];
    let mut result = u32::max_value();

    for line in lines {
        match line {
            SessionLine::MoveUp => {
                let last_sum = *sums.last().unwrap();
                if last_sum >= sufficient && last_sum <= result {
                    result = last_sum;
                }
                sums.pop();
            }
            SessionLine::MoveDown => {
                sums.push(0);
            }
            SessionLine::FileSize(len) => {
                for sum in &mut sums {
                    *sum += len;
                }
            }
            _ => {}
        }
    }

    for sum in sums {
        if sum >= sufficient && sum <= result {
            result = sum;
        }
    }

    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[derive(Debug)]
enum SessionLine {
    MoveTop,
    MoveUp,
    MoveDown,
    ListFiles,
    Dir,
    FileSize(u32),
}

impl FromStr for SessionLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        let r = if bytes[0].is_ascii_digit() {
            let (len, _) = s.split_once(' ').unwrap();
            Self::FileSize(len.parse().unwrap())
        } else if bytes[0] == b'd' {
            Self::Dir
        } else if bytes[2] == b'c' {
            if bytes[5] == b'/' {
                Self::MoveTop
            } else if bytes[5] == b'.' {
                Self::MoveUp
            } else {
                Self::MoveDown
            }
        } else {
            Self::ListFiles
        };
        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
