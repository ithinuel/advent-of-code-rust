use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use aoc_runner_derive::*;
use itertools::Itertools;

type Input = (
    HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>,
    Vec<usize>,
    Vec<Vec<usize>>,
);

#[aoc_generator(day16)]
fn gen(input: &str) -> Input {
    let rules_regex = regex::Regex::new(r"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$").unwrap();

    let mut blocks = input.split("\n\n");
    let rules: HashMap<_, _> = blocks
        .next()
        .map(|rules| rules.lines())
        .expect("Invalid input format")
        .map(|line| {
            let captures = rules_regex.captures(line).expect("Invalid rule format");
            let field = captures[1].to_string();
            let a: usize = captures[2].parse().expect("Invalid range bound");
            let b = captures[3].parse().expect("Invalid range bound");
            let c: usize = captures[4].parse().expect("Invalid range bound");
            let d = captures[5].parse().expect("Invalid range bound");
            (field, (a..=b, c..=d))
        })
        .collect();

    let my_ticket: Vec<usize> = blocks
        .next()
        .and_then(|block| {
            block
                .lines()
                .skip(1)
                .flat_map(|line| line.split(','))
                .map(|v| v.parse())
                .try_collect()
                .ok()
        })
        .expect("Invalid ticket format");

    let other_tickets: Vec<Vec<usize>> = blocks
        .next()
        .and_then(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.split(',').map(|v| v.parse()).try_collect())
                .try_collect()
                .ok()
        })
        .expect("Invalid ticket format");
    (rules, my_ticket, other_tickets)
}

#[aoc(day16, part1)]
fn part1((rules, _my_ticket, other_tickets): &Input) -> usize {
    other_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|v| {
                    !rules
                        .iter()
                        .any(|(_, (range_1, range_2))| range_1.contains(v) || range_2.contains(v))
                })
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day16, part2)]
fn parte2((rules, my_ticket, other_tickets): &Input) -> usize {
    // prepare all rules for all fields
    let mut possible_mapping: Vec<HashMap<_, _>> = (0..my_ticket.len())
        .map(|_| rules.iter().collect())
        .collect();
    // for each ticket that have no fields matching at least one rule,for each field in those remove
    // from the possible_mapping the rules that do not validate that field.
    other_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .find(|v| {
                    // find a field that matches no rules
                    !rules
                        .iter()
                        .any(|(_, (range_1, range_2))| range_1.contains(v) || range_2.contains(v))
                })
                .is_none()
        })
        .for_each(|ticket| {
            ticket.iter().enumerate().for_each(|(i, v)| {
                let rules = &mut possible_mapping[i];
                rules.retain(|_, (range_1, range_2)| range_1.contains(v) || range_2.contains(v))
            });
        });

    // collapse findings
    loop {
        let found_mapping: HashSet<_> = possible_mapping
            .iter()
            .filter_map(|rules| rules.iter().exactly_one().ok().map(|(&k, _)| k))
            .collect();

        let mut changed = false;
        possible_mapping
            .iter_mut()
            .filter(|rules| rules.len() != 1)
            .for_each(|rules| {
                let prev_len = rules.len();
                rules.retain(|k, _| !found_mapping.contains(k));
                changed |= prev_len != rules.len();
            });
        if !changed {
            break;
        }
    }

    // at this stage we still may have fields with more that 1 possible rule but the input is
    // guarateed to be enough

    // compute result
    possible_mapping
        .into_iter()
        .enumerate()
        .filter_map(|(i, rules)| rules.into_iter().next().map(|(k, _)| (k, i)))
        .filter(|(k, _)| k.starts_with("departure"))
        .map(|(_, i)| my_ticket[i])
        .product()
}
