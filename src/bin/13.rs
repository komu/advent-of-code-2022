use std::{
    cmp::Ordering,
    iter::Peekable,
    str::{Chars, FromStr},
};

use anyhow::{anyhow, Context};

pub fn part_one(input: &str) -> Option<usize> {
    let pairs = input.trim().split("\n\n").map(|s| {
        let (l1, l2) = s.split_once('\n').unwrap();
        (l1.parse::<Value>().unwrap(), l2.parse::<Value>().unwrap())
    });

    let result = pairs
        .enumerate()
        .filter(|(_, (x, y))| x <= y)
        .map(|(i, _)| i + 1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets: Vec<_> = input
        .trim()
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Value>().unwrap())
        .collect();

    let divider1 = Value::List(vec![Value::List(vec![Value::Int(2)])]);
    let divider2 = Value::List(vec![Value::List(vec![Value::Int(6)])]);

    packets.push(divider1.clone());
    packets.push(divider2.clone());

    packets.sort();

    let i1 = 1 + packets.iter().position(|x| *x == divider1).unwrap();
    let i2 = 1 + packets.iter().position(|x| *x == divider2).unwrap();

    Some(i1 * i2)
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            (Value::List(x), Value::List(y)) => x.cmp(y),
            (Value::Int(x), Value::List(y)) => [Value::Int(*x)].as_slice().cmp(y.as_slice()),
            (Value::List(x), Value::Int(y)) => x.as_slice().cmp([Value::Int(*y)].as_slice()),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Int(u8),
    List(Vec<Value>),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(it: &mut Peekable<Chars>) -> Result<Value, anyhow::Error> {
            match it.peek() {
                Some(&'[') => parse_list(it),
                Some(&c) if c.is_ascii_digit() => parse_scalar(it),
                Some(&c) => Err(anyhow!("unexpected char '{}'", c)),
                None => Err(anyhow!("unexpected end")),
            }
        }

        fn parse_list(it: &mut Peekable<Chars>) -> Result<Value, anyhow::Error> {
            expect(it, '[')?;

            let mut result = Vec::new();

            if it.peek() != Some(&']') {
                result.push(parse_value(it)?);

                while it.next_if_eq(&',').is_some() {
                    result.push(parse_value(it)?);
                }
            }

            expect(it, ']')?;
            Ok(Value::List(result))
        }

        fn parse_scalar(it: &mut Peekable<Chars>) -> Result<Value, anyhow::Error> {
            let Some(c) = it.next() else {
                return Err(anyhow!("expected scalar, got end"));
            };

            if c.is_ascii_digit() {
                let mut num = c as u8 - b'0';

                while let Some(n) = it.next_if(|n| n.is_ascii_digit()) {
                    num *= 10;
                    num += n as u8 - b'0'
                }

                Ok(Value::Int(num))
            } else {
                Err(anyhow!("expected scalar, got '{}'", c))
            }
        }

        fn expect(it: &mut Peekable<Chars>, expected: char) -> Result<(), anyhow::Error> {
            match it.next() {
                Some(c) if expected == c => Ok(()),
                Some(c) => Err(anyhow!("expected '{}', got '{}'", expected, c)),
                None => Err(anyhow!("expected '{}', got end", expected)),
            }
        }

        let mut it = s.chars().peekable();
        let v = parse_value(&mut it).with_context(|| format!("while parsing line '{}'", s))?;

        if let Some(c) = it.next() {
            Err(anyhow!("expected end, got '{}' on line '{}'", c, s))
        } else {
            Ok(v)
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
