use aoc::helpers::{mut_refs, mut_refs3};

pub fn part_one(input: &str) -> Option<Num> {
    Some(mix2(input, 1, 1))
}

pub fn part_two(input: &str) -> Option<Num> {
    Some(mix2(input, 811589153, 10))
}

fn mix2(input: &str, key: Num, rounds: usize) -> Num {
    let numbers: Vec<Num> = input
        .lines()
        .map(|s| s.parse::<Num>().unwrap() * key)
        .collect();

    let mut indices: Vec<usize> = (0..numbers.len()).collect();

    for _ in 0..rounds {
        for (original_index, value) in numbers.iter().enumerate() {
            let old_index = indices.iter().position(|&x| x == original_index).unwrap();

            let new_index =
                (old_index as Num + value).rem_euclid(numbers.len() as Num - 1) as usize;

            indices.remove(old_index);
            indices.insert(new_index, original_index);
        }
    }

    let original_zero_index = numbers.iter().position(|&x| x == 0).unwrap();
    let zero = indices
        .iter()
        .position(|&i| i == original_zero_index)
        .unwrap();
    let x1 = numbers[indices[(zero + 1000) % numbers.len()]];
    let x2 = numbers[indices[(zero + 2000) % numbers.len()]];
    let x3 = numbers[indices[(zero + 3000) % numbers.len()]];
    x1 + x2 + x3
}

#[allow(unused)]
fn mix(input: &str, key: Num, times: u32) -> Num {
    let nums = input
        .lines()
        .map(|s| s.parse::<Num>().unwrap())
        .map(|n| n * key)
        .collect::<Vec<_>>();
    let mut list = NumList::new(&nums);

    for _ in 0..times {
        for i in 0..nums.len() {
            list.move_node(NodeRef(i));
        }
    }

    let vec = list.to_vec();
    let zero = vec.iter().position(|&n| n == 0).unwrap();
    let num1 = vec[(zero + 1000) % vec.len()];
    let num2 = vec[(zero + 2000) % vec.len()];
    let num3 = vec[(zero + 3000) % vec.len()];

    num1 + num2 + num3
}

type Num = i64;

#[derive(Clone, Copy, PartialEq, Eq)]
struct NodeRef(usize);

struct NumList {
    nodes: Vec<NumNode>,
    len: usize,
}

struct NumNode {
    value: Num,
    prev: NodeRef,
    next: NodeRef,
}

impl NumList {
    fn new(nums: &[Num]) -> NumList {
        let mut nodes = Vec::with_capacity(nums.len());

        for (i, &value) in nums.iter().enumerate() {
            nodes.push(NumNode {
                value,
                prev: if i > 0 {
                    NodeRef(i - 1)
                } else {
                    NodeRef(nums.len() - 1)
                },
                next: if i + 1 < nums.len() {
                    NodeRef(i + 1)
                } else {
                    NodeRef(0)
                },
            });
        }

        NumList {
            nodes,
            len: nums.len(),
        }
    }

    fn remove_node(&mut self, ptr: NodeRef) {
        let node = &self.nodes[ptr.0];
        let prev_ptr = node.prev;
        let next_ptr = node.next;

        let (prev, next) = mut_refs(&mut self.nodes, prev_ptr.0, next_ptr.0);
        prev.next = next_ptr;
        next.prev = prev_ptr;
    }

    fn insert_after(&mut self, prev_ref: NodeRef, inserted: NodeRef) {
        let next_ref = self.nodes[prev_ref.0].next;

        let (prev, node, next) = mut_refs3(&mut self.nodes, prev_ref.0, inserted.0, next_ref.0);
        node.prev = prev_ref;
        node.next = next_ref;
        prev.next = inserted;
        next.prev = inserted;
    }

    fn move_node(&mut self, ptr: NodeRef) {
        let mut delta = self.nodes[ptr.0].value;
        if delta == 0 {
            return;
        }

        self.remove_node(ptr);

        let mut new_prev = self.nodes[ptr.0].prev;
        let len = self.len as Num;
        delta %= len - 1;

        if delta > len / 2 {
            delta -= len - 1;
        } else if delta < -(len / 2) {
            delta += len - 1;
        }

        delta %= len - 1;
        if delta > 0 {
            for _ in 0..delta.abs() {
                new_prev = self.nodes[new_prev.0].next;
            }
        } else {
            for _ in 0..delta.abs() {
                new_prev = self.nodes[new_prev.0].prev;
            }
        }

        self.insert_after(new_prev, ptr);
    }

    fn to_vec(&self) -> Vec<Num> {
        let mut result = Vec::new();
        let start = NodeRef(0);
        let mut ptr = start;
        loop {
            let node = &self.nodes[ptr.0];

            result.push(node.value);

            ptr = node.next;
            if ptr == start {
                break;
            }
        }
        result
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 20);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
