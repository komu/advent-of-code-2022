use std::str::FromStr;

use anyhow::anyhow;
use aoc::helpers::parse_lines;
use hashbrown::HashMap;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_lines::<Blueprint>(input).collect::<Vec<_>>();

    let result = blueprints.par_iter().map(|b| b.id * b.max_geodes(24)).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let blueprints = parse_lines::<Blueprint>(input).take(3).collect::<Vec<_>>();

    let result = blueprints.par_iter().map(|b| b.max_geodes(32)).product();
    Some(result)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Cost = u32;

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: Cost,
    clay_robot_ore_cost: Cost,
    obsidian_robot_ore_cost: Cost,
    obsidian_robot_clay_cost: Cost,
    geode_robot_ore_cost: Cost,
    geode_robot_obsidian_cost: Cost,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct SearchState {
    remaining_minutes: u8,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
}

impl SearchState {
    fn new(remaining_minutes: u8) -> Self {
        SearchState {
            remaining_minutes,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
        }
    }

    fn collect_materials(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geodes += self.geode_robots;
    }

    fn build_robot(&mut self, robot_type: Material, blueprint: &Blueprint) {
        match robot_type {
            Material::Ore => {
                self.ore -= blueprint.ore_robot_ore_cost;
                self.ore_robots += 1;
            }
            Material::Clay => {
                self.ore -= blueprint.clay_robot_ore_cost;
                self.clay_robots += 1;
            }
            Material::Obsidian => {
                self.ore -= blueprint.obsidian_robot_ore_cost;
                self.clay -= blueprint.obsidian_robot_clay_cost;
                self.obsidian_robots += 1;
            }
            Material::Geode => {
                self.ore -= blueprint.geode_robot_ore_cost;
                self.obsidian -= blueprint.geode_robot_obsidian_cost;
                self.geode_robots += 1;
            }
        }
    }

    fn can_build(&self, robot_type: Material, blueprint: &Blueprint) -> bool {
        match robot_type {
            Material::Ore => self.ore >= blueprint.ore_robot_ore_cost,
            Material::Clay => self.ore >= blueprint.clay_robot_ore_cost,
            Material::Obsidian => {
                self.ore >= blueprint.obsidian_robot_ore_cost
                    && self.clay >= blueprint.obsidian_robot_clay_cost
            }
            Material::Geode => {
                self.ore >= blueprint.geode_robot_ore_cost
                    && self.obsidian >= blueprint.geode_robot_obsidian_cost
            }
        }
    }
}

impl Blueprint {
    fn max_geodes(&self, minutes: u8) -> u32 {
        let mut cache = HashMap::new();
        let mut best = 0;
        self.recurse(SearchState::new(minutes), &mut cache, &mut best)
    }

    fn recurse(
        &self,
        mut state: SearchState,
        cache: &mut HashMap<SearchState, u32>,
        best: &mut u32,
    ) -> u32 {
        if state.remaining_minutes == 0 {
            if state.geodes > *best {
                *best = state.geodes;
            }
            return state.geodes;
        }

        let original_state = state.clone();
        if let Some(result) = cache.get(&original_state) {
            return *result;
        }

        state.remaining_minutes -= 1;

        let mut result = 0;
        if state.can_build(Material::Geode, self) {
            state.collect_materials();

            state.build_robot(Material::Geode, self);
            result = result.max(self.recurse(state, cache, best));
        } else if state.can_build(Material::Obsidian, self) {
            state.collect_materials();

            result = result.max(self.recurse(state.clone(), cache, best));

            state.build_robot(Material::Obsidian, self);
            result = result.max(self.recurse(state, cache, best));
        } else {
            let can_build_clay_robot = state.can_build(Material::Clay, self);
            let can_build_ore_robot = state.can_build(Material::Ore, self);

            state.collect_materials();

            result = result.max(self.recurse(state.clone(), cache, best));

            if can_build_clay_robot {
                let mut new_state = state.clone();
                new_state.build_robot(Material::Clay, self);
                result = result.max(self.recurse(new_state, cache, best));
            }

            if can_build_ore_robot {
                let mut new_state = state.clone();
                new_state.build_robot(Material::Ore, self);
                result = result.max(self.recurse(new_state, cache, best));
            }
        }

        cache.insert(original_state, result);

        result
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Blueprint (\d+): Each ore robot costs (\d+) ore\. Each clay robot costs (\d+) ore\. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian\."
            )
            .unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("no match for line '{}'", s))?;

        Ok(Blueprint {
            id: caps[1].parse()?,
            ore_robot_ore_cost: caps[2].parse()?,
            clay_robot_ore_cost: caps[3].parse()?,
            obsidian_robot_ore_cost: caps[4].parse()?,
            obsidian_robot_clay_cost: caps[5].parse()?,
            geode_robot_ore_cost: caps[6].parse()?,
            geode_robot_obsidian_cost: caps[7].parse()?,
        })
    }
}

fn main() {
    let input = &aoc::read_file("inputs", 19);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 19);
        assert_eq!(part_two(&input), None);
    }
}
