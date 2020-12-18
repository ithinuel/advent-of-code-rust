use std::io::BufRead;
use std::ops::BitAnd;

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}
impl Passport {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    fn is_valid(&self) -> bool {
        let byr_valid = self
            .byr
            .as_ref()
            .and_then(|s| s.parse::<u32>().ok())
            .map(|v| (1920..=2002).contains(&v))
            .unwrap_or(false);
        let iyr_valid = self
            .iyr
            .as_ref()
            .and_then(|s| s.parse::<u32>().ok())
            .map(|v| (2010..=2020).contains(&v))
            .unwrap_or(false);
        let eyr_valid = self
            .eyr
            .as_ref()
            .and_then(|s| s.parse::<u32>().ok())
            .map(|v| (2020..=2030).contains(&v))
            .unwrap_or(false);
        let hgt_valid = self
            .hgt
            .as_ref()
            .and_then(|hgt| match hgt.split_at(hgt.len() - 2) {
                (v, "in") => v.parse::<u32>().ok().map(|h| (59..=76).contains(&h)),
                (v, "cm") => v.parse::<u32>().ok().map(|h| (150..=193).contains(&h)),
                (_, _) => None,
            })
            .unwrap_or(false);
        let hcl_valid = self
            .hcl
            .as_ref()
            .map(|s| {
                s.len() == 7
                    && s.chars().next() == Some('#')
                    && s.chars()
                        .skip(1)
                        .map(|c| c.is_digit(16))
                        .fold(true, bool::bitand)
            })
            .unwrap_or(false);
        let ecl_valid = self
            .ecl
            .as_ref()
            .map(|s| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s.as_str()))
            .unwrap_or(false);
        let pid_valid = self
            .pid
            .as_ref()
            .map(|s| s.chars().map(|c| c.is_digit(10)).fold(true, bool::bitand) && s.len() == 9)
            .unwrap_or(false);

        byr_valid && iyr_valid && eyr_valid && hgt_valid && hcl_valid && ecl_valid && pid_valid
    }
}

fn main() {
    let (passports, _) = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.to_string()
                .split_whitespace()
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .fold(
            (Vec::new(), Passport::new()),
            |(mut passports, mut builder), data| {
                if data.is_empty() {
                    passports.push(builder);
                    (passports, Passport::new())
                } else {
                    for detail in data {
                        match detail.split_at(4) {
                            ("byr:", v) => builder.byr = Some(v.into()),
                            ("iyr:", v) => builder.iyr = Some(v.into()),
                            ("eyr:", v) => builder.eyr = Some(v.into()),
                            ("hgt:", v) => builder.hgt = Some(v.into()),
                            ("hcl:", v) => builder.hcl = Some(v.into()),
                            ("ecl:", v) => builder.ecl = Some(v.into()),
                            ("pid:", v) => builder.pid = Some(v.into()),
                            ("cid:", v) => builder.cid = Some(v.into()),
                            (_, _) => todo!(),
                        };
                    }
                    (passports, builder)
                }
            },
        );

    println!("{:?}", passports.iter().filter(|p| p.is_valid()).count());
}
