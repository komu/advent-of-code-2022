use std::{cmp::Ordering, iter::Peekable, str::Bytes};

pub fn part_one(input: &str) -> Option<usize> {
    let result = input
        .trim()
        .split("\n\n")
        .enumerate()
        .filter(|(_, s)| {
            let (x, y) = s.split_once('\n').unwrap();
            compare_values(&mut PacketParser::new(x), &mut PacketParser::new(y)) == Ordering::Less
        })
        .map(|(i, _)| i + 1)
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let packets = input.trim().lines().filter(|s| !s.is_empty());

    let mut divider1_index = 1;
    let mut divider2_index = 2;

    for packet in packets {
        if compare_list_to_value(&mut PacketParser::new(packet), 3) == Ordering::Less {
            divider1_index += 1;
            divider2_index += 1;
        } else if compare_list_to_value(&mut PacketParser::new(packet), 6) == Ordering::Less {
            divider2_index += 1;
        }
    }

    Some(divider1_index * divider2_index)
}

struct PacketParser<'a> {
    iter: Peekable<Bytes<'a>>,
}

impl<'a> PacketParser<'a> {
    fn new(s: &'a str) -> Self {
        PacketParser {
            iter: s.bytes().peekable(),
        }
    }

    fn peek(&mut self) -> Option<u8> {
        self.iter.peek().copied()
    }

    fn consume(&mut self, c: u8) {
        assert_eq!(self.iter.next(), Some(c))
    }

    fn consume_if(&mut self, c: u8) -> bool {
        self.iter.next_if_eq(&c).is_some()
    }

    fn read_number(&mut self) -> u8 {
        let mut value = 0;

        while let Some(c) = self.iter.next_if(|x| x.is_ascii_digit()) {
            value *= 10;
            value += c - b'0';
        }

        value
    }
}

fn compare_values(l: &mut PacketParser, r: &mut PacketParser) -> Ordering {
    match (l.peek(), r.peek()) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(b'['), Some(b'[')) => compare_lists(l, r),
        (Some(b'['), _) => compare_list_to_value(l, r.read_number()),
        (_, Some(b'[')) => compare_list_to_value(r, l.read_number()).reverse(),
        _ => l.read_number().cmp(&r.read_number()),
    }
}

fn compare_lists(l: &mut PacketParser, r: &mut PacketParser) -> Ordering {
    l.consume(b'[');
    r.consume(b'[');

    loop {
        let left_ends = l.consume_if(b']');
        let right_ends = r.consume_if(b']');
        if left_ends || right_ends {
            if left_ends && right_ends {
                return Ordering::Equal;
            } else if left_ends {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }

        let result = compare_values(l, r);
        if result != Ordering::Equal {
            return result;
        }

        l.consume_if(b',');
        r.consume_if(b',');
    }
}

fn compare_list_to_value(list_it: &mut PacketParser, value: u8) -> Ordering {
    while list_it.consume_if(b'[') {}

    if list_it.consume_if(b']') {
        Ordering::Less
    } else {
        let result = list_it.read_number().cmp(&value);
        if result != Ordering::Equal {
            result
        } else if list_it.consume_if(b']') {
            Ordering::Equal
        } else {
            Ordering::Greater
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
