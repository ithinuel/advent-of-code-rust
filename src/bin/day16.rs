use itertools::Itertools;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

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

    let _my_ticket: Vec<usize> = blocks
        .next()
        .and_then(|block| {
            block
                .lines()
                .skip(1)
                .flat_map(|line| line.split(","))
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
                .map(|line| line.split(",").map(|v| v.parse()).try_collect())
                .try_collect()
                .ok()
        })
        .expect("Invalid ticket format");

    let error_rate: usize = other_tickets
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
        .sum();
    println!("Error rate: {}", error_rate);
}
