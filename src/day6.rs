use std::collections::HashSet;

use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc(day6, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .batching(|lines| {
            let mut set = HashSet::new();
            for line in lines {
                if line.is_empty() {
                    return Some(set);
                } else {
                    set.extend(line.chars());
                }
            }
            None
        })
        .map(|group| group.len())
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .batching(|lines| {
            let mut intersect = None;

            for line in lines {
                if line.is_empty() {
                    return intersect;
                } else {
                    let line: HashSet<_> = line.chars().collect();

                    if let Some(intersect) = intersect.as_mut() {
                        *intersect = &*intersect & &line;
                    } else {
                        intersect = Some(line);
                    }
                }
            }
            None
        })
        .map(|group| group.len())
        .sum()
}
