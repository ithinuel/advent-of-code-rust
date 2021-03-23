use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut passport = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            // if passports has all required keys
            let has_all_required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .all(|k| passport.contains(k));
            passport.clear();
            if has_all_required_keys {
                count += 1;
            }
        } else {
            passport.extend(
                line.split_whitespace()
                    .flat_map(|field| field.split(':').next()),
            );
        }
    }
    count
}

type Passport<'a> = std::collections::HashMap<&'a str, &'a str>;
fn fields_are_valid_part2(p: &Passport) -> bool {
    let has_all_required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|&k| p.contains_key(k));

    let all_values_are_valid = p.iter().all(|(k, v)| {
        match *k {
            "byr" => v.parse().ok().map(|v| (1920..=2002).contains(&v)),
            "iyr" => v.parse().ok().map(|v| (2010..=2020).contains(&v)),
            "eyr" => v.parse().ok().map(|v| (2020..=2030).contains(&v)),
            "hgt" => match v.split_at(v.len() - 2) {
                (v, "in") => v.parse::<u32>().ok().map(|h| (59..=76).contains(&h)),
                (v, "cm") => v.parse::<u32>().ok().map(|h| (150..=193).contains(&h)),
                (_, _) => None,
            },
            "hcl" => Some(
                v.len() == 7 && v.starts_with('#') && v.chars().skip(1).all(|c| c.is_digit(16)),
            ),
            "ecl" => Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)),
            "pid" => Some(v.len() == 9 && v.chars().all(|c| c.is_digit(10))),
            "cid" => Some(true),
            _ => unreachable!(),
        }
        .unwrap_or(false)
    });

    has_all_required_keys && all_values_are_valid
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    let mut count = 0;
    let mut passport = Passport::new();

    for line in input.lines() {
        if line.is_empty() {
            // if passports has all required keys
            if fields_are_valid_part2(&passport) {
                count += 1;
            }
            passport.clear();
        } else {
            passport.extend(
                line.split_whitespace()
                    .flat_map(|field| field.split(':'))
                    .tuples::<(&str, &str)>(),
            );
        }
    }
    count
}
