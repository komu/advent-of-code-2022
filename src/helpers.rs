/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use aoc::helpers::example_fn;`.
 */
use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> impl Iterator<Item = T> + '_ {
    input.lines().map(|s| s.parse::<T>().ok().unwrap())
}
