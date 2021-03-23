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
