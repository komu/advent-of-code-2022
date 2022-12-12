# 🎄 Advent of Code 2022 

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

This year, my focus is neither on cute nor readable, but raw performance. Goal is to measure
individual parts in microseconds instead of milliseconds and to solve every problem with
a total budget of 100 milliseconds.

## Results

| Day                                            | Part 1   | Part 2   | Notes |
| :--------------------------------------------: | -------: | -------: | :---- |
| [Day 1](https://adventofcode.com/2022/day/1)   |  77.66µs |  69.09µs | Process everything in single iteration, avoid sorting results in part 2 |
| [Day 2](https://adventofcode.com/2022/day/2)   |  81.87µs |  62.62µs | Use suitable representations to allow using modular arithmetic for comparisons |
| [Day 3](https://adventofcode.com/2022/day/3)   |  54.33µs |  51.12µs | Represent rucksack as a bitset |
| [Day 4](https://adventofcode.com/2022/day/4)   |  82.56µs |  69.46µs | - |
| [Day 5](https://adventofcode.com/2022/day/5)   |  40.67µs |  36.89µs | - |
| [Day 6](https://adventofcode.com/2022/day/6)   |   5.00µs |   7.09µs | Calculate forward skips to avoid processing most of the input |
| [Day 7](https://adventofcode.com/2022/day/7)   |  43.68µs |  60.68µs | Avoid actually building the tree |
| [Day 8](https://adventofcode.com/2022/day/8)   |  78.85µs | 161.86µs | Precalculate maximums for each side to speed up part 1 |
| [Day 9](https://adventofcode.com/2022/day/9)   | 352.48µs | 784.74µs | -     |
| [Day 10](https://adventofcode.com/2022/day/10) |   5.37µs |   9.09µs | -     |
| [Day 11](https://adventofcode.com/2022/day/11) |  25.28µs |   5.42ms | 😔    |
| [Day 12](https://adventofcode.com/2022/day/12) | 767.46µs | 985.24µs | Search using A*, use custom map for distances |

(Totally unscientific numbers from a single run, will improve these in the future.)

## Previous years

* [2021 in Kotlin](https://github.com/komu/advent-of-code-2021)
* [2015-2020 in Kotlin](https://github.com/komu/advent-of-code)
