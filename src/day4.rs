use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    let mut count = 0;
    let mut passport = Vec::new();

    let mut count_valid_passport = |passport: &Vec<_>| {
        // if passports has all required keys
        if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| passport.contains(k))
        {
            count += 1;
        }
    };

    for line in input.lines() {
        if line.is_empty() {
            count_valid_passport(&passport);
            passport.clear();
        } else {
            passport.extend(
                line.split_whitespace()
                    .flat_map(|field| field.split(':').next()),
            );
        }
    }
    if !passport.is_empty() {
        count_valid_passport(&passport);
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
    if !passport.is_empty() && fields_are_valid_part2(&passport) {
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID: &str = r"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID: &str = r"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part1() {
        assert_eq!(2, super::part1(EXAMPLE));
    }
    #[test]
    fn part2() {
        assert_eq!(2, super::part2(EXAMPLE));
        assert_eq!(0, super::part2(INVALID));
        assert_eq!(4, super::part2(VALID));
    }
}
