use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

// post spoil.
// Itertools are SOOO COOOL !!

fn main() {
    let (passports, _) = std::io::stdin().lock().lines().filter_map(Result::ok).fold(
        (Vec::new(), HashMap::<String, String>::new()),
        |(mut passports, mut passport), line| {
            if line.is_empty() {
                // if passports has all required keys
                if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                    .iter()
                    .all(|k| passport.contains_key(k.to_owned()))
                {
                    passports.push(passport);
                }
                (passports, HashMap::new())
            } else {
                passport.extend(
                    line.split_whitespace()
                        .flat_map(|field| field.split(":"))
                        .map(str::to_owned)
                        .tuples(),
                );
                (passports, passport)
            }
        },
    );

    println!("{:?}", passports.iter().count());
}
