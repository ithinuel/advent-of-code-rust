use aoc_runner_derive::*;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
type Gen = (Vec<u8>, BTreeMap<(u8, u8), u8>);

#[aoc_generator(day14)]
fn gen(input: &str) -> Gen {
    let (seed, rules) = input.split("\n\n").next_tuple().expect("Invalid format");
    let rules = rules
        .lines()
        .filter_map(|rule| {
            let (pair, new) = rule.split(" -> ").next_tuple()?;
            let new = new.bytes().next()?;
            pair.bytes().next_tuple().map(|(s, e)| ((s, e), new))
        })
        .collect();
    (seed.into(), rules)
}

#[aoc(day14, part1)]
fn part1(input: &Gen) -> usize {
    let (seed, rules) = input;
    let chain = (1..=10).fold(seed.clone(), |seed, _| {
        seed.iter()
            .copied()
            .tuple_windows()
            .flat_map(|(s, e)| {
                let &new = rules.get(&(s, e)).expect("Unknown pair");
                [s, new].into_iter()
            })
            .chain(seed.iter().copied().last())
            .collect::<Vec<u8>>()
    });

    let elements: BTreeSet<_> = chain.iter().copied().collect();

    elements
        .iter()
        .map(|&b| chain.iter().filter(|&&e| e == b).count())
        .minmax()
        .into_option()
        .map(|(min, max)| max - min)
        .unwrap()
}

#[aoc(day14, part2)]
fn part2(input: &Gen) -> usize {
    let (seed, rules) = input;

    let mut population: BTreeMap<(_, _), _> = BTreeMap::new();
    seed.iter().copied().tuple_windows().for_each(|pair| {
        *population.entry(pair).or_insert(0) += 1;
    });

    let population = (1..=40).fold(population, |population, _| {
        let mut new_cnt = BTreeMap::new();

        for ((a, b), count) in population.into_iter() {
            let &c = rules.get(&(a, b)).expect("Unknown pair");
            *new_cnt.entry((a, c)).or_insert(0) += count;
            *new_cnt.entry((c, b)).or_insert(0) += count;
        }

        new_cnt
    });

    population
        .keys()
        .flat_map(|&(a, b)| [a, b].into_iter())
        .unique()
        .map(|e| {
            population
                .iter()
                .filter_map(|(&(s, _), &count)| (s == e).then(|| count))
                .sum::<usize>()
                + if Some(&e) == seed.iter().last() { 1 } else { 0 }
        })
        .minmax()
        .into_option()
        .map(|(min, max)| max - min)
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::gen;
    const EXAMPLE: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1() {
        assert_eq!(1588, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(2188189693529, super::part2(&gen(EXAMPLE)));
    }
}
