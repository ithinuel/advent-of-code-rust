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
            if set.is_empty() {
                None
            } else {
                Some(set)
            }
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
                    break;
                }

                let line: HashSet<_> = line.chars().collect();

                if let Some(intersect) = intersect.as_mut() {
                    *intersect = &*intersect & &line;
                } else {
                    intersect = Some(line);
                }
            }
            intersect
        })
        .map(|group| group.len())
        .sum()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part1() {
        assert_eq!(11, super::part1(EXAMPLE));
    }

    #[test]
    fn part2() {
        assert_eq!(6, super::part2(EXAMPLE));
    }
}
