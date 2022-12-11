use anyhow::anyhow;
use itertools::Itertools;
use num::integer::lcm;
use std::{cmp::Reverse, str::FromStr};

pub fn part_one(input: &str) -> Option<usize> {
    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    let mut items: Vec<Vec<WorryLevel>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    let mut result = Vec::with_capacity(20);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let monkey_items = &mut items[i];

            inspections[i] += monkey_items.len();

            result.clear();
            for item in monkey_items.iter() {
                let new_item = monkey.operation.evaluate(*item) / 3;
                let new_monkey = if new_item % monkey.divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                result.push((new_monkey, new_item));
            }
            monkey_items.clear();

            for (monkey, item) in &result {
                items[*monkey].push(*item);
            }
        }
    }

    let v: Vec<_> = inspections
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .collect();

    Some(v[0] * v[1])
}

pub fn part_two(input: &str) -> Option<usize> {
    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    let mut items: Vec<Vec<WorryLevel>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    let mut result = Vec::with_capacity(20);
    let modulo = monkeys.iter().map(|m| m.divisible_by).reduce(lcm).unwrap();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let monkey_items = &mut items[i];
            inspections[i] += monkey_items.len();

            result.clear();
            for item in monkey_items.iter() {
                let new_item = monkey.operation.evaluate(*item) % modulo;
                let new_monkey = if new_item % monkey.divisible_by == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };
                result.push((new_monkey, new_item));
            }
            monkey_items.clear();

            for (monkey, item) in &result {
                items[*monkey].push(*item);
            }
        }
    }

    let v: Vec<_> = inspections
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .collect();
    Some(v[0] * v[1])
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

type WorryLevel = u64;
type MonkeyId = usize;

#[derive(Debug)]
struct Monkey {
    items: Vec<WorryLevel>,
    operation: Op,
    divisible_by: WorryLevel,
    if_true: MonkeyId,
    if_false: MonkeyId,
}

#[derive(Debug)]
enum Op {
    Multiply(WorryLevel),
    Add(WorryLevel),
    Square,
}

impl Op {
    fn evaluate(&self, x: WorryLevel) -> WorryLevel {
        match self {
            Op::Multiply(y) => x * y,
            Op::Add(y) => x + y,
            Op::Square => x * x,
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next().unwrap();
        let items: Vec<WorryLevel> = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.parse::<WorryLevel>().unwrap())
            .collect();
        let operation: Op = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = ")
            .unwrap()
            .parse()?;
        let divisible_by: WorryLevel = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()?;
        let if_true: MonkeyId = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()?;
        let if_false: MonkeyId = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()?;

        Ok(Monkey {
            items,
            operation,
            divisible_by,
            if_true,
            if_false,
        })
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old * old" {
            Ok(Self::Square)
        } else if let Some(v) = s.strip_prefix("old * ") {
            Ok(Self::Multiply(v.parse()?))
        } else if let Some(v) = s.strip_prefix("old + ") {
            Ok(Self::Add(v.parse()?))
        } else {
            Err(anyhow!("Invalid op '{}'", s))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
