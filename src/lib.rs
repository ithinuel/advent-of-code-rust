use itertools::Itertools;
use yaah::{aoc, aoc_lib, aoc_year};

aoc_year!(2022);

#[aoc(day1, part1)]
fn day1_part1(input: &'static str) -> Option<usize> {
    input
        .split("\n\n")
        .map(|s| s.split("\n").filter_map(|v| v.parse::<usize>().ok()).sum())
        .max()
}

#[aoc(day1, part2)]
fn day1_part2(input: &'static str) -> usize {
    input
        .split("\n\n")
        .map(|s| -> usize { s.split("\n").filter_map(|v| v.parse::<usize>().ok()).sum() })
        .sorted_unstable()
        .rev()
        .take(3)
        .sum::<usize>()
}

aoc_lib!(with_benchmarks);
