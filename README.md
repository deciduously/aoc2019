# Advent of Code 2019

[Advent of Code 2019](https://adventofcode.com/2019) in Rust.

Usage `./aoc [DAY]`:

```txt
$ cargo run -- 2
   Compiling aoc2019 v0.1.0 (/home/ben/code/aoc2019)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/aoc 2`
Day 2
4945026
5296
```

Or `cargo test` which includes all solved days against verified answers:

```txt
$ cargo test
   Compiling aoc2019 v0.1.0 (/home/ben/code/aoc2019)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running target/debug/deps/aoc2019-b5d3009c37ed6d9d\

running 6 tests
test day1::test::test_sum_dividends ... ok
test day1::test::test_sum_fuels ... ok
test day1::test::test_solutions ... ok
test intcode::test::test_intcode ... ok
test test::test_get_puzzle_string ... ok
test day2::test::test_solutions ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
