use std::io::BufRead;

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
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
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
