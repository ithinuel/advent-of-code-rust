use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

// post spoil.
// Itertools are SOOO COOOL !!

fn fields_are_valid(p: &HashMap<String, String>) -> bool {
    p.iter().all(|(k, v)| {
        match k.as_str() {
            "byr" => v.parse().ok().map(|v| (1920..=2002).contains(&v)),
            "iyr" => v.parse().ok().map(|v| (2010..=2020).contains(&v)),
            "eyr" => v.parse().ok().map(|v| (2020..=2030).contains(&v)),
            "hgt" => match v.split_at(v.len() - 2) {
                (v, "in") => v.parse::<u32>().ok().map(|h| (59..=76).contains(&h)),
                (v, "cm") => v.parse::<u32>().ok().map(|h| (150..=193).contains(&h)),
                (_, _) => None,
            },
            "hcl" => Some(
                v.len() == 7
                    && v.chars().next() == Some('#')
                    && v.chars().skip(1).all(|c| c.is_digit(16)),
            ),
            "ecl" => Some(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&v.as_str())),
            "pid" => Some(v.chars().all(|c| c.is_digit(10)) && v.len() == 9),
            "cid" => Some(true),
            _ => unreachable!(),
        }
        .unwrap_or(false)
    })
}

fn main() {
    let result = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .batching(|lines| {
            let mut passport = HashMap::new();
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    // if passports has all required keys
                    if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                        .iter()
                        .all(|&k| passport.contains_key(k))
                        && fields_are_valid(&passport)
                    {
                        return Some(passport);
                    }
                    passport.clear();
                } else {
                    passport.extend(
                        line.split_whitespace()
                            .flat_map(|field| field.split(":"))
                            .map(str::to_string)
                            .tuples(),
                    );
                }
            }
            None
        })
        .count();

    println!("{:?}", result);
}
