use std::collections::HashSet;

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

#[aoc(day2, part1)]
fn day2_part1(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" | "B Y" | "C Z" => 3,
            "C X" | "A Y" | "B Z" => 6,
            _ => 0,
        } + match line.chars().last() {
           Some('X') => 1,
           Some('Y') => 2,
           Some('Z') => 3,
           _ => unreachable!(),
        })
        .sum()
}

#[aoc(day2, part2)]
fn day2_part2(input: &'static str) -> usize {
    input
        .lines()
        .map(|line| match line {
            "A X" => 3, // Scissors loss against rock
            "B X" => 1, // Rock loss against paper
            "C X" => 2, // Paper loss against scissors
            "A Y" => 4,
            "B Y" => 5,
            "C Y" => 6,
            "A Z" => 8, // Paper wins on rock
            "B Z" => 9, // Scissors wins on paper
            "C Z" => 7, // Rock wins on scissors
            _ => unreachable!(),
        })
        .sum()
}

#[aoc(day3, part1)]
fn day3_part1(input: &'static str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            let compartiment_size = line.len() / 2;
            let (first, second) = line.split_at(compartiment_size);
            let left: HashSet<_> = first.chars().collect();
            let right: HashSet<_> = second.chars().collect();
            let miss_placed = left.intersection(&right).collect_vec();
            assert_eq!(
                1,
                miss_placed.len(),
                "only one item type should appear in both"
            );
            miss_placed.first().map(|v| **v)
        })
        .map(|c| {
            println!("v: {c}");
            (if c.is_ascii_uppercase() {
                (c as u8) - b'A' + 27
            } else {
                (c as u8) - b'a' + 1
            }) as usize
        })
        .sum()
}
#[aoc(day3, part2)]
fn day3_part2(input: &'static str) -> usize {
    input
        .lines()
        .tuples()
        .filter_map(|(a, b, c)| {
            let a: HashSet<_> = a.chars().collect();
            let b: HashSet<_> = b.chars().collect();
            let c: HashSet<_> = c.chars().collect();
            let a_b: HashSet<_> = a.intersection(&b)
                .cloned()
                .collect();
            a_b
                .intersection(&c)
                .cloned()
                .last()
        })
        .map(|c| {
            (if c.is_ascii_uppercase() {
                (c as u8) - b'A' + 27
            } else {
                (c as u8) - b'a' + 1
            }) as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1() {
        assert_eq!(15, super::day2_part1("A Y\nB X\nC Z"))
    }
    #[test]
    fn day2_part2() {
        assert_eq!(12, super::day2_part2("A Y\nB X\nC Z"))
    }

    const DAY3: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn day3_part1() {
        assert_eq!(157, super::day3_part1(&DAY3));
    }

    #[test]
    fn day3_part2() {
        assert_eq!(70, super::day3_part2(&DAY3));
    }
}

aoc_lib!(with_benchmarks);
