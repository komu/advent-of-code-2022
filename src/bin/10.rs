use std::str::FromStr;

use anyhow::anyhow;
use aoc::helpers::parse_lines;

pub fn part_one(input: &str) -> Option<i32> {
    let mut score = 0;

    run(input, |cycles, x| {
        if [20, 60, 100, 140, 180, 220].contains(&cycles) {
            score += x * cycles;
        }
    });

    Some(score)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut output = String::new();

    run(input, |cycles, x| {
        let xpos = (cycles - 1) % 40;
        if (xpos - x).abs() <= 1 {
            output.push('#');
        } else {
            output.push('.');
        }

        if xpos == 39 {
            output.push('\n');
        }
    });

    Some(output)
}

fn run<F>(input: &str, mut f: F)
where
    F: FnMut(i32, i32) -> (),
{
    let ops = parse_lines::<OpCode>(input);

    let mut x = 1;
    let mut current_cycle = 1;

    for op in ops {
        match op {
            OpCode::Noop => {
                f(current_cycle, x);
                current_cycle += 1;
            }
            OpCode::Addx(d) => {
                f(current_cycle, x);
                current_cycle += 1;
                f(current_cycle, x);
                current_cycle += 1;
                x += d;
            }
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[derive(Debug)]
enum OpCode {
    Noop,
    Addx(i32),
}

impl FromStr for OpCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(OpCode::Noop)
        } else if let Some(suffix) = s.strip_prefix("addx ") {
            Ok(OpCode::Addx(suffix.parse()?))
        } else {
            Err(anyhow!("invalid opcode '{}'", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);

        let output = part_two(&input);
        println!("{}", output.unwrap());
        assert_eq!(
            part_two(&input),
            Some(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
                .to_owned()
            )
        );
    }
}
