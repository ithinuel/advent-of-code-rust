use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    let lengths = input
        .lines()
        .map(|l| {
            l.split(" | ")
                .skip(1)
                .flat_map(|s| s.split_ascii_whitespace())
                .map(|s| s.as_bytes().len())
                .collect_vec()
        })
        .collect_vec();

    lengths
        .iter()
        .map(|lens| lens.iter().filter(|l| [2, 3, 4, 7].contains(l)).count())
        .sum()
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
/// cagedb : contains ab but not uw => 0
/// cefabd : contains ab & uw => 9
/// cdfgeb :  => 6
///
/// abcdf : contains ab => 3
/// cbdef : contains uw => 5
/// acdfg : => 2

fn infer<'a>(input: &'a [HashSet<u8>]) -> HashMap<usize, &'a HashSet<u8>> {
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
    let uw: HashSet<_> = first_pass[&4]
        .difference(&first_pass[&1])
        .cloned()
        .collect();

    let mut infered: HashMap<_, _> = input
        .iter()
        .filter_map(|n| {
            if n.len() == 5 {
                if first_pass[&1].is_subset(n) {
                    Some((3, n))
                } else if uw.is_subset(n) {
                    Some((5, n))
                } else {
                    Some((2, n))
                }
            } else if n.len() == 6 {
                if first_pass[&1].is_subset(n) {
                    if uw.is_subset(n) {
                        Some((9, n))
                    } else {
                        Some((0, n))
                    }
                } else {
                    Some((6, n))
                }
            } else {
                None
            }
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
            let (patterns, digits) = line.split(" | ").next_tuple().unwrap();
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
                .map(|digit| {
                    map.iter()
                        .find(|(_, &pat)| pat == digit)
                        .map(|(&k, _)| k)
                        .unwrap()
                })
                .fold(0, |acc, d| acc * 10 + d)
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
}
