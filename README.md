# 🎄 Advent of Code 2022 

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

This year, my focus is not on short or cute, but raw performance. Goal is to measure
individual parts in microseconds instead of milliseconds and to solve every problem with
a total budget of 100 milliseconds.

## Results

|                    Day                     | Solution               |   Part 1 |   Part 2 | Notes                                                                          |
|:------------------------------------------:|:-----------------------|---------:|---------:|:-------------------------------------------------------------------------------|
|  [1](https://adventofcode.com/2022/day/1)  | [01.rs](src/bin/01.rs) |  77.66µs |  69.09µs | Process everything in single iteration, avoid sorting results in part 2        |
|  [2](https://adventofcode.com/2022/day/2)  | [02.rs](src/bin/02.rs) |  81.87µs |  62.62µs | Use suitable representations to allow using modular arithmetic for comparisons |
|  [3](https://adventofcode.com/2022/day/3)  | [03.rs](src/bin/03.rs) |  54.33µs |  51.12µs | Represent rucksack as a bitset                                                 |
|  [4](https://adventofcode.com/2022/day/4)  | [04.rs](src/bin/04.rs) |  82.56µs |  69.46µs | -                                                                              |
|  [5](https://adventofcode.com/2022/day/5)  | [05.rs](src/bin/05.rs) |  40.67µs |  36.89µs | -                                                                              |
|  [6](https://adventofcode.com/2022/day/6)  | [06.rs](src/bin/06.rs) |   5.00µs |   7.09µs | Calculate forward skips to avoid processing most of the input                  |
|  [7](https://adventofcode.com/2022/day/7)  | [07.rs](src/bin/07.rs) |  43.68µs |  60.68µs | Avoid actually building the tree                                               |
|  [8](https://adventofcode.com/2022/day/8)  | [08.rs](src/bin/08.rs) |  78.85µs | 161.86µs | Precalculate maximums for each side to speed up part 1                         |
|  [9](https://adventofcode.com/2022/day/9)  | [09.rs](src/bin/09.rs) | 352.48µs | 784.74µs | -                                                                              |
| [10](https://adventofcode.com/2022/day/10) | [10.rs](src/bin/10.rs) |   5.37µs |   9.09µs | -                                                                              |
| [11](https://adventofcode.com/2022/day/11) | [11.rs](src/bin/11.rs) |  25.28µs |   5.42ms | 😔                                                                             |
| [12](https://adventofcode.com/2022/day/12) | [12.rs](src/bin/12.rs) | 767.46µs | 985.24µs | Search using A*, use custom map for distances                                  |
| [13](https://adventofcode.com/2022/day/13) | [13.rs](src/bin/13.rs) |  22.63µs |  17.51µs | Avoid building trees, parse data only as far as needed                         |
| [14](https://adventofcode.com/2022/day/14) | [14.rs](src/bin/14.rs) | 170.18µs | 365.68µs | Backtrack on the paths instead of starting all over                            |
| [15](https://adventofcode.com/2022/day/15) | [15.rs](src/bin/15.rs) | 456.34µs | 122.09ms | 😔                                                                             |
| [16](https://adventofcode.com/2022/day/16) | [16.rs](src/bin/16.rs) |   7.86ms |   8.43ms | 😔                                                                             |
| [17](https://adventofcode.com/2022/day/17) | [17.rs](src/bin/17.rs) | 221.94µs | 326.42µs | Keep previous states in cache to find the period of the process                |
| [18](https://adventofcode.com/2022/day/18) | [18.rs](src/bin/18.rs) | 160.66µs | 437.68µs | -                                                                              |
| [19](https://adventofcode.com/2022/day/19) | [19.rs](src/bin/19.rs) |    2.73s |   16.44s | 😔                                                                             |
| [20](https://adventofcode.com/2022/day/20) | [20.rs](src/bin/20.rs) |   6.72ms |  78.69ms | 😔                                                                             |
| [21](https://adventofcode.com/2022/day/21) | [21.rs](src/bin/21.rs) | 325.30µs | 235.40µs | -                                                                              |
| [22](https://adventofcode.com/2022/day/22) | [22.rs](src/bin/22.rs) | 139.31µs | 127.73µs | -                                                                              |
| [23](https://adventofcode.com/2022/day/23) | [23.rs](src/bin/23.rs) |  15.51ms | 831.17ms | 😔                                                                             |
| [24](https://adventofcode.com/2022/day/24) | [24.rs](src/bin/24.rs) |    9.62s |   92.24s | 😔                                                                             |
| [25](https://adventofcode.com/2022/day/25) | [25.rs](src/bin/25.rs) |  18.00ns |        - | -                                                                              |

(Totally unscientific numbers from a single run, will improve these in the future.)

## Previous years

* [2021 in Kotlin](https://github.com/komu/advent-of-code-2021)
* [2015-2020 in Kotlin](https://github.com/komu/advent-of-code)
