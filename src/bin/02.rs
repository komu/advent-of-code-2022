pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|r| {
            let op = Shape::from_code(r.0);
            let me = Shape::from_code(r.1);

            me.score() + me.result(op).score()
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|r| {
            let op = Shape::from_code(r.0);
            let result = Result::from_code(r.1);

            op.shape_for_result(result).score() + result.score()
        })
        .sum();
    Some(result)
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Result {
    Lose,
    Draw,
    Win,
}

impl Shape {
    fn from_code(str: &str) -> Shape {
        match str {
            "A" | "X" => Shape::Rock,
            "B" | "Y" => Shape::Paper,
            "C" | "Z" => Shape::Scissors,
            _ => panic!("invalid code '{}'", str),
        }
    }

    fn score(self) -> u32 {
        (self as u32) + 1
    }

    fn shape_for_result(self, result: Result) -> Self {
        match ((self as u8) + (result as u8) + 2) % 3 {
            0 => Self::Rock,
            1 => Self::Paper,
            2 => Self::Scissors,
            _ => panic!(),
        }
    }

    fn result(self, against: Shape) -> Result {
        match (((self as u8) + 3) - (against as u8)) % 3 {
            0 => Result::Draw,
            1 => Result::Win,
            2 => Result::Lose,
            _ => panic!(),
        }
    }
}

impl Result {
    fn from_code(str: &str) -> Self {
        match str {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid code '{}'", str),
        }
    }

    fn score(self) -> u32 {
        (self as u32) * 3
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
