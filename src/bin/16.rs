use std::{cmp::Reverse, hash::Hash, str::FromStr};

use aoc::helpers::parse_lines;
use hashbrown::HashMap;
use itertools::iproduct;

pub fn part_one(input: &str) -> Option<u16> {
    let valves = ValveMap::new(input);
    let answer = valves.search(30);
    answer.iter().copied().max()
}

pub fn part_two(input: &str) -> Option<u16> {
    let valves = ValveMap::new(input);
    let answer = valves.search(26);

    let threshold = answer.iter().copied().max().unwrap() / 2;
    let answers = answer
        .iter()
        .copied()
        .enumerate()
        .map(|(k, v)| (k, v))
        .filter(|(_, v)| *v > threshold)
        .collect::<Vec<_>>();

    let mut best = 0;
    for (i, (k1, v1)) in answers.iter().enumerate() {
        for (k2, v2) in &answers[i + 1..] {
            if k1 & k2 == 0 {
                best = best.max(v1 + v2);
            }
        }
    }
    Some(best)
}

struct ValveMap {
    start_id: ValveId,
    valves: Vec<Valve>,
    distances: Vec<u8>,
    keys: usize,
}

impl ValveMap {
    fn new(input: &str) -> ValveMap {
        let mut valve_map = HashMap::<ValveId, Valve>::new();
        let mut id_mapping = ValveIdMapping::new();
        let mut valves = parse_lines::<ValveData>(input).collect::<Vec<_>>();

        // Move non-zero valves to front so that they'll get small ids and
        // we can use them for indexing
        valves.sort_by_key(|v| Reverse(v.rate));
        for v in &valves {
            id_mapping.get_id(&v.id);
        }

        for v in &valves {
            let v = v.to_valve(&mut id_mapping);
            valve_map.insert(v.id, v);
        }

        let keys = valve_map.keys().map(|k| k.0).max().unwrap() as usize + 1;
        let mut distances = vec![u8::MAX; keys * keys];

        // Initial distances
        for valve in valve_map.values() {
            for tunnel in &valve.tunnels {
                distances[valve.id.0 as usize * keys + tunnel.0 as usize] = 1;
            }
        }

        // Self-distances to zero
        for i in 0..keys {
            distances[i * keys + i] = 0;
        }

        // Floyd-Warshall
        for (k, i, j) in iproduct!(0..keys, 0..keys, 0..keys) {
            let ij = i * keys + j;
            let ik = i * keys + k;
            let kj = k * keys + j;

            let sum = distances[ik].saturating_add(distances[kj]);
            if distances[ij] > sum {
                distances[ij] = sum;
            }
        }

        let valves = valve_map
            .drain()
            .filter(|v| v.1.rate != 0)
            .map(|v| v.1)
            .collect();

        let start_id = id_mapping.get_id("AA");

        ValveMap {
            start_id,
            valves,
            distances,
            keys,
        }
    }

    fn search(&self, remaining_minutes: u8) -> Vec<u16> {
        let remaining_flow = self.valves.iter().map(|v| v.rate).sum::<u16>();
        let max_id = self.valves.iter().map(|v| v.id.0).max().unwrap();

        let mut answer = vec![0; 2 << max_id];
        self.recurse(
            self.start_id,
            remaining_minutes,
            ValveSet::empty(),
            remaining_flow,
            0,
            &mut answer,
        );
        answer
    }

    fn recurse(
        &self,
        v: ValveId,
        remaining_minutes: u8,
        state: ValveSet,
        remaining_flow: u16,
        flow: u16,
        answer: &mut Vec<u16>,
    ) {
        let best = answer[state.0 as usize].max(flow);
        answer[state.0 as usize] = best;

        if flow + remaining_flow * (remaining_minutes as u16 - 1) < best {
            return;
        }

        for u in &self.valves {
            if !state.contains(u.id) {
                let cost = self.distance_between(v, u.id) + 1;
                if cost < remaining_minutes {
                    let new_remaining_minutes = remaining_minutes - cost;

                    self.recurse(
                        u.id,
                        new_remaining_minutes,
                        state.add(u.id),
                        remaining_flow - u.rate,
                        flow + (new_remaining_minutes as u16) * u.rate,
                        answer,
                    );
                }
            }
        }
    }

    fn distance_between(&self, i: ValveId, j: ValveId) -> u8 {
        self.distances[i.0 as usize * self.keys + j.0 as usize]
    }
}

struct Valve {
    id: ValveId,
    rate: u16,
    tunnels: Vec<ValveId>,
}

struct ValveData {
    id: String,
    rate: u16,
    tunnels: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct ValveSet(u64);

impl ValveSet {
    fn empty() -> Self {
        ValveSet(0)
    }

    #[must_use]
    fn add(&self, valve: ValveId) -> ValveSet {
        ValveSet(self.0 | (1 << valve.0))
    }

    #[must_use]
    fn contains(self, valve: ValveId) -> bool {
        self.0 & (1 << valve.0) != 0
    }
}

impl ValveData {
    fn to_valve(&self, id_mapping: &mut ValveIdMapping) -> Valve {
        Valve {
            id: id_mapping.get_id(&self.id),
            rate: self.rate,
            tunnels: self.tunnels.iter().map(|t| id_mapping.get_id(t)).collect(),
        }
    }
}

impl FromStr for ValveData {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s[6..8].to_owned();
        let (rate, rest) = &s[23..].split_once(';').unwrap();
        let tunnels = rest
            .split_once("valve")
            .unwrap()
            .1
            .split_once(' ')
            .unwrap()
            .1;

        Ok(ValveData {
            id,
            rate: rate.parse().unwrap(),
            tunnels: tunnels.split(", ").map(|s| s.to_owned()).collect(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ValveId(u8);

struct ValveIdMapping(HashMap<String, ValveId>);

impl ValveIdMapping {
    fn new() -> ValveIdMapping {
        ValveIdMapping(HashMap::new())
    }

    fn get_id(&mut self, k: &str) -> ValveId {
        if let Some(id) = self.0.get(k) {
            *id
        } else {
            let id = ValveId(self.0.len() as u8);
            self.0.insert(k.to_owned(), id);
            id
        }
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 16);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
