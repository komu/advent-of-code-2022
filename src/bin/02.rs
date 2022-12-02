pub fn part_one(input: &str) -> Option<u32> {
    let rounds = input.lines().map(|s| s.split(' ').collect::<Vec<_>>());

    Some(
        rounds
            .map(|r| {
                let op = Shape::from_code(r[0]);
                let me = Shape::from_code(r[1]);

                me.score() + me.result(op).score()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let rounds = input.lines().map(|s| s.split(' ').collect::<Vec<_>>());

    Some(
        rounds
            .map(|r| {
                let op = Shape::from_code(r[0]);
                let result = Result::from_code(r[1]);

                op.shape_for_result(result).score() + result.score()
            })
            .sum(),
    )
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
        *[Self::Rock, Self::Paper, Self::Scissors]
            .iter()
            .find(|s| s.result(self) == result)
            .unwrap()
    }

    fn result(self, against: Shape) -> Result {
        if self == against {
            return Result::Draw;
        }

        let wins = match self {
            Self::Rock => against == Self::Scissors,
            Self::Paper => against == Self::Rock,
            Self::Scissors => against == Self::Paper,
        };

        if wins {
            Result::Win
        } else {
            Result::Lose
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
