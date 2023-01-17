use std::str::FromStr;

use aoc::helpers::parse_lines;
use hashbrown::HashMap;

pub fn part_one(input: &str) -> Option<Num> {
    let exps: HashMap<_, _> = parse_lines::<Assignment>(input).map(|a| (a.id, a.exp)).collect();

    Some(evaluate(&ExpId::from("root"), &exps))
}

pub fn part_two(input: &str) -> Option<Num> {
    let exps: HashMap<_, _> = parse_lines::<Assignment>(input).map(|a| (a.id, a.exp)).collect();

    let (l, r) = match exps.get(&ExpId::from("root")) {
        Some(Exp::BinOp(l, _, r)) => (l, r),
        _ => panic!("invalid root")
    };

    let l = rewrite(l, &exps);
    let r = rewrite(r, &exps);
    if let Some(c) = r.evaluate() {
        invert(c, &l)
    } else if let Some(c) = l.evaluate() {
        invert(c, &r)
    } else {
        panic!("neither expression is constant");
    }
}

fn invert(x: Num, exp: &Exp2) -> Option<Num> {
    use Exp2::*;
    use Op::*;

    match exp {
        Human => Some(x),
        Constant(_) => None,
        BinOp(l, o, r) => {
            if let Some(a) = l.evaluate() {
                match o {
                    Add => invert(x - a, r),
                    Sub => invert(a - x, r),
                    Mul => invert(x / a, r),
                    Div => invert(a / x, r),
                }
            } else if let Some(a) = r.evaluate() {
                match o {
                    Add => invert(x - a, l),
                    Sub => invert(x + a, l),
                    Mul => invert(x / a, l),
                    Div => invert(a * x, l),
                }
            } else {
                panic!("neither side of expression constant");
            }
        }
    }
}

fn rewrite(key: &ExpId, exps: &HashMap<ExpId, Exp>) -> Exp2 {
    if key == &ExpId::from("humn") {
        return Exp2::Human
    }
    let exp = exps.get(key).unwrap();
    match exp {
        Exp::Constant(v) => Exp2::Constant(*v),
        Exp::BinOp(l, o, r) => {
            let lhs = rewrite(l, exps);
            let rhs = rewrite(r, exps);
            match (&lhs, &rhs) {
                (Exp2::Constant(l), Exp2::Constant(r)) => Exp2::Constant(o.eval(*l, *r)),
                _ => Exp2::BinOp(Box::new(lhs), *o, Box::new(rhs))
            }
        }
    }
}

fn evaluate(key: &ExpId, exps: &HashMap<ExpId, Exp>) -> Num {
    let exp = exps.get(key).unwrap();
    match exp {
        Exp::Constant(v) => *v,
        Exp::BinOp(l, o, r) => {
            let lhs = evaluate(&l, exps);
            let rhs = evaluate(&r, exps);
            o.eval(lhs, rhs)
        }
    }
}

type Num = i64;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct ExpId(u32);

impl From<&str> for ExpId {
    fn from(s: &str) -> Self {
        let s = s.as_bytes();
        assert_eq!(4, s.len());

        ExpId((s[0] as u32) | ((s[1] as u32) << 8)  | ((s[2] as u32) << 16) | ((s[3] as u32) << 24))
    }
}

struct Assignment {
    id: ExpId,
    exp: Exp,
}

enum Exp {
    Constant(Num),
    BinOp(ExpId, Op, ExpId)
}

#[derive(Debug)]
enum Exp2 {
    Human,
    Constant(Num),
    BinOp(Box<Exp2>, Op, Box<Exp2>)
}

impl Exp2 {

    fn evaluate(&self) -> Option<Num> {
        match self {
            Exp2::Human => None,
            Exp2::Constant(c) => Some(*c),
            Exp2::BinOp(l, o, r) => {
                let lhs = l.evaluate()?;
                let rhs = r.evaluate()?;

                Some(o.eval(lhs, rhs))
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Add, Sub, Mul, Div
}

impl Op {
    fn eval(self, lhs: Num, rhs: Num) -> Num {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '-' => Op::Sub,
            '*' => Op::Mul,
            '/' => Op::Div,
            c => panic!("unknown char {c}")
        }
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bs = s.as_bytes();
        let id = ExpId::from(&s[0..4]);

        let exp = if bs[6].is_ascii_digit() {
            Exp::Constant(s[6..].parse().unwrap())
        } else {
            let lhs = ExpId::from(&s[6..10]);
            let rhs = ExpId::from(&s[13..]);
            let op = Op::from(bs[11] as char);

            Exp::BinOp(lhs, op, rhs)
        };

        Ok(Assignment { id, exp })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 21);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
