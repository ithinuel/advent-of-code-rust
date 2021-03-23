use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc_generator(day10)]
fn gen(input: &str) -> Vec<usize> {
    let mut adapters: Vec<_> = input.lines().filter_map(|s| s.parse().ok()).collect();
    adapters.push(0);
    adapters.sort_unstable();
    adapters
}

#[aoc(day10, part1)]
fn part1(adapters: &[usize]) -> usize {
    let (ones, threes) = adapters
        .iter()
        .tuple_windows()
        .fold((0, 0), |(ones, threes), (a, b)| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });

    ones * (threes + 1)
}
#[aoc(day10, part2)]
fn part2(adapters: &[usize]) -> u64 {
    use std::collections::BTreeMap;
    let mut edges = BTreeMap::new();
    adapters.iter().for_each(|&a| {
        edges.entry(a).or_insert_with(Vec::new).extend(
            (1..4)
                .map(|n| a + n)
                .filter(|b| adapters.binary_search(&b).is_ok()),
        );
    });

    // The input is a Directed Acyclic Graph
    // count number of path from 0 to max
    let mut paths_count = BTreeMap::new();
    adapters.iter().rev().for_each(|&v| {
        let neighbours = &edges[&v];
        if neighbours.is_empty() {
            paths_count.insert(v, 1u64);
        } else {
            paths_count.insert(v, neighbours.iter().map(|n| paths_count[n]).sum());
        }
    });

    paths_count[&0]
}
