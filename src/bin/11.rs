use anyhow::anyhow;
use aoc::helpers::mut_refs3;
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

    for _ in 0..20 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let (monkey_items, true_items, false_items) =
                mut_refs3(&mut items, i, monkey.if_true, monkey.if_false);

            inspections[i] += monkey_items.len();

            for &item in monkey_items.iter() {
                let new_item = monkey.operation.eval_div3(item);
                if new_item % monkey.divisible_by == 0 {
                    true_items.push(new_item);
                } else {
                    false_items.push(new_item);
                };
            }
            monkey_items.clear();
        }
    }

    Some(monkey_business(&inspections))
}

pub fn part_two(input: &str) -> Option<usize> {
    let monkeys: Vec<_> = input
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect();
    let mut items: Vec<Vec<WorryLevel>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections: Vec<usize> = vec![0; monkeys.len()];
    let modulo = monkeys.iter().map(|m| m.divisible_by).reduce(lcm).unwrap();

    for _ in 0..10000 {
        for (i, monkey) in monkeys.iter().enumerate() {
            let (monkey_items, true_items, false_items) =
                mut_refs3(&mut items, i, monkey.if_true, monkey.if_false);

            inspections[i] += monkey_items.len();

            for &item in monkey_items.iter() {
                let new_item = monkey.operation.eval_mod(item, modulo);
                if new_item % monkey.divisible_by == 0 {
                    true_items.push(new_item);
                } else {
                    false_items.push(new_item);
                }
            }
            monkey_items.clear();
        }
    }

    Some(monkey_business(&inspections))
}

fn monkey_business(inspections: &[usize]) -> usize {
    let v: Vec<_> = inspections
        .iter()
        .map(Reverse)
        .k_smallest(2)
        .map(|r| r.0)
        .collect();
    v[0] * v[1]
}

type WorryLevel = u32;
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
    fn eval_div3(&self, x: WorryLevel) -> WorryLevel {
        let v = match self {
            Op::Multiply(y) => x * y,
            Op::Add(y) => x + y,
            Op::Square => x * x,
        };
        v / 3
    }

    fn eval_mod(&self, x: WorryLevel, modulo: WorryLevel) -> WorryLevel {
        match self {
            Op::Multiply(y) => (x * y) % modulo,
            Op::Add(y) => (x + y) % modulo,
            // The square might overflow u32 before taking modulo, so we need to widen it temporarily
            Op::Square => (((x as u64) * (x as u64)) % (modulo as u64)) as WorryLevel,
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

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
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
