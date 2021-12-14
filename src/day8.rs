use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split(" | ")
                .skip(1)
                .flat_map(|s| s.split_ascii_whitespace())
                .map(|s| s.as_bytes().len())
                .filter(|l| [2, 3, 4, 7].contains(l))
        })
        .count()
}

/// ```text
///  tttt
/// u    v
/// u    v
///  wwww
/// x    y
/// x    y
///  zzzz
/// ```
///
/// 1 : ab : ab is either vy or yv
/// 4 : abef : ef is either uw or wu
/// 7 : abd : d is t
///
/// cdfgeb : does not contain 1 => 6
/// cefabd : contains (4-1) => 9
/// cagedb :  => 0
///
/// abcdf : contains 1 => 3
/// cbdef : contains (4-1) => 5
/// acdfg : => 2

fn infer(input: &[HashSet<u8>]) -> HashMap<usize, &'_ HashSet<u8>> {
    let first_pass: HashMap<_, _> = input
        .iter()
        .filter_map(|n| match n.len() {
            2 => Some((1, n)),
            3 => Some((7, n)),
            4 => Some((4, n)),
            7 => Some((8, n)),
            _ => None,
        })
        .collect();
    let one = first_pass[&1];
    let uw = first_pass[&4] - one;

    let mut infered: HashMap<_, _> = input
        .iter()
        .filter_map(|n| match n.len() {
            6 => Some(if !one.is_subset(n) {
                (6, n)
            } else if uw.is_subset(n) {
                (9, n)
            } else {
                (0, n)
            }),
            5 => Some(if one.is_subset(n) {
                (3, n)
            } else if uw.is_subset(n) {
                (5, n)
            } else {
                (2, n)
            }),
            _ => None,
        })
        .collect();
    infered.extend(first_pass);
    infered
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (patterns, digits) = line.split(" | ").next_tuple().expect("Invalid format");
            let patterns: Vec<HashSet<_>> = patterns
                .split_ascii_whitespace()
                .map(|n| n.bytes().collect())
                .collect();
            let digits: Vec<HashSet<_>> = digits
                .split_ascii_whitespace()
                .map(|n| n.bytes().collect())
                .collect();

            let map = infer(&patterns);

            digits
                .iter()
                .filter_map(|digit| map.iter().find(|(_, &pat)| pat == digit).map(|(&k, _)| k))
                .fold(0, |acc, d| acc * 10 + d)
        })
        .sum()
}

fn infer_bitmask(patterns: &[u8]) -> [u8; 10] {
    let mut result = [0; 10];
    patterns.iter().cloned().for_each(|n| {
        if let Some(idx) = match n.count_ones() {
            5 | 6 => None,
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => unreachable!(),
        } {
            result[idx] = n
        }
    });

    let one = result[1];
    let uw = result[4] ^ one;

    patterns.iter().cloned().for_each(|n| {
        if let Some(idx) = match n.count_ones() {
            6 if (n & one) != one => Some(6),
            6 if (n & uw) == uw => Some(9),
            6 => Some(0),
            5 if (n & one) == one => Some(3),
            5 if (n & uw) == uw => Some(5),
            5 => Some(2),
            _ => None,
        } {
            result[idx] = n
        }
    });
    result
}

#[aoc(day8, part2, bitmask)]
fn part2_bitmask(input: &str) -> u32 {
    let to_bitmask = |s: &str| {
        s.bytes().fold(0, |acc, b| {
            acc | match b {
                b'a' => 0b000_0001,
                b'b' => 0b000_0010,
                b'c' => 0b000_0100,
                b'd' => 0b000_1000,
                b'e' => 0b001_0000,
                b'f' => 0b010_0000,
                b'g' => 0b100_0000,
                _ => unreachable!(),
            }
        })
    };

    input
        .lines()
        .map(|line| {
            let (patterns, digits) = line.split(" | ").next_tuple().expect("Invalid format");
            let patterns: Vec<u8> = patterns.split_ascii_whitespace().map(to_bitmask).collect();

            let map = infer_bitmask(&patterns);
            digits
                .split_ascii_whitespace()
                .map(to_bitmask)
                .filter_map(|n| map.iter().find_position(|&&m| n == m))
                .fold(0, |acc, n| acc * 10 + n.0 as u32)
        })
        .sum()
}

fn infer_bitmask_transposed(patterns: &[u8]) -> [Option<u32>; 128] {
    let mut result = [None; 128];
    let mut one = 0;
    let mut four = 0;
    patterns.iter().cloned().map(usize::from).for_each(|n| {
        result[n] = match n.count_ones() {
            5 | 6 => None,
            2 => {
                one = n;
                Some(1)
            }
            3 => Some(7),
            4 => {
                four = n;
                Some(4)
            }
            7 => Some(8),
            _ => unreachable!(),
        }
    });

    let uw = four ^ one;

    patterns.iter().cloned().map(usize::from).for_each(|n| {
        result[n] = match n.count_ones() {
            6 if (n & one) != one => Some(6),
            6 if (n & uw) == uw => Some(9),
            6 => Some(0),
            5 if (n & one) == one => Some(3),
            5 if (n & uw) == uw => Some(5),
            5 => Some(2),
            _ => return,
        }
    });
    result
}

#[aoc(day8, part2, bitmask_transposed)]
fn part2_bitmask_transposed(input: &str) -> u32 {
    let to_bitmask = |s: &str| {
        s.bytes().fold(0, |acc, b| {
            acc | match b {
                b'a' => 0b000_0001,
                b'b' => 0b000_0010,
                b'c' => 0b000_0100,
                b'd' => 0b000_1000,
                b'e' => 0b001_0000,
                b'f' => 0b010_0000,
                b'g' => 0b100_0000,
                _ => unreachable!(),
            }
        })
    };

    input
        .lines()
        .map(|line| {
            let (patterns, digits) = line.split(" | ").next_tuple().expect("Invalid format");
            let patterns: Vec<u8> = patterns.split_ascii_whitespace().map(to_bitmask).collect();

            let map = infer_bitmask_transposed(&patterns);
            digits
                .split_ascii_whitespace()
                .map(to_bitmask)
                .map(usize::from)
                .filter_map(|v| map[v])
                .fold(0, |acc, n| acc * 10 + n)
        })
        .sum()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1() {
        assert_eq!(26, super::part1(EXAMPLE));
    }
    #[test]
    fn part2() {
        assert_eq!(61229, super::part2(EXAMPLE));
    }
    #[test]
    fn part2_bitmask() {
        assert_eq!(61229, super::part2_bitmask(EXAMPLE));
    }
    #[test]
    fn part2_bitmask_transposed() {
        assert_eq!(61229, super::part2_bitmask_transposed(EXAMPLE));
    }
}
