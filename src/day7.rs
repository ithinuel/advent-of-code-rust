use std::collections::{HashMap, HashSet};

use anyhow::anyhow;
use aoc_runner_derive::*;
use itertools::Itertools;

const RULES: &str = r"(\d+) ([\w ]+) bags?";

type RuleMap = HashMap<String, HashMap<String, usize>>;

#[aoc_generator(day7)]
fn gen(input: &str) -> anyhow::Result<RuleMap> {
    let re = regex::Regex::new(RULES).expect("invalid regex");

    input
        .lines()
        .map(|l| {
            let bag_color = l
                .split(" bags contain")
                .next()
                .map(str::to_string)
                .ok_or_else(|| anyhow!("invalid bag name"))?;

            let children = re
                .captures_iter(&l)
                .filter_map(|c| {
                    c.iter()
                        .flat_map(|m| m.into_iter())
                        .skip(1)
                        .tuples()
                        .map(|(count, name)| {
                            count
                                .as_str()
                                .parse()
                                .map_err(|_| anyhow!("Invalid count"))
                                .map(|count| (name.as_str().to_string(), count))
                        })
                        .next()
                })
                .try_collect()?;
            Ok((bag_color, children))
        })
        .try_collect()
}

#[aoc(day7, part1)]
fn part1(rules: &RuleMap) -> usize {
    let mut containers: HashSet<&str> = HashSet::new();
    let mut prev_layer: HashSet<_> = ["shiny gold"].iter().copied().collect();
    loop {
        let layer: HashSet<_> = prev_layer
            .iter()
            .flat_map(|bag| rules.iter().filter(move |(_, v)| v.contains_key(*bag)))
            .map(|(k, _)| k.as_str())
            .collect();
        if layer.is_empty() {
            break;
        }
        containers.extend(layer.iter());
        prev_layer = layer;
    }
    containers.len()
}

#[aoc(day7, part2)]
fn part2(rules: &RuleMap) -> usize {
    let mut leaves: HashMap<_, _> = rules
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(k, _)| (k, 0))
        .collect();

    loop {
        let weights: HashMap<_, usize> = rules
            .iter()
            .filter_map(|(k, v)| {
                if !leaves.contains_key(k) && v.keys().all(|k| leaves.contains_key(k)) {
                    Some((k, v.iter().map(|(k, v)| leaves[k] * v + v).sum()))
                } else {
                    None
                }
            })
            .collect();
        if weights.is_empty() {
            break;
        }
        leaves.extend(weights.into_iter());
    }
    leaves[&"shiny gold".to_string()]
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_AS_HASH: &[(&str, &[(&str, usize)])] = &[
        ("dark orange", &[("bright white", 3), ("muted yellow", 4)]),
        ("bright white", &[("shiny gold", 1)]),
        ("shiny gold", &[("dark olive", 1), ("vibrant plum", 2)]),
        ("dotted black", &[]),
        ("faded blue", &[]),
        ("light red", &[("bright white", 1), ("muted yellow", 2)]),
        ("muted yellow", &[("faded blue", 9), ("shiny gold", 2)]),
        ("dark olive", &[("dotted black", 4), ("faded blue", 3)]),
        ("vibrant plum", &[("dotted black", 6), ("faded blue", 5)]),
    ];

    fn example_as_hash() -> super::RuleMap {
        EXAMPLE_AS_HASH
            .iter()
            .map(|(id, v)| {
                (
                    id.to_string(),
                    v.iter().map(|(id, v)| (id.to_string(), *v)).collect(),
                )
            })
            .collect()
    }

    #[test]
    fn generator() {
        let example_as_hash = example_as_hash();
        assert_eq!(Some(example_as_hash), super::gen(EXAMPLE).ok());
    }

    #[test]
    fn part1() {
        let example_as_hash = example_as_hash();
        assert_eq!(4, super::part1(&example_as_hash));
    }

    #[test]
    fn part2() {
        let example_as_hash = example_as_hash();
        assert_eq!(32, super::part2(&example_as_hash));
    }
}
