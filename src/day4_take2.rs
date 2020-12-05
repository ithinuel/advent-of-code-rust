use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

// post spoil.
// Itertools are SOOO COOOL !!

fn main() {
    let result = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .batching(|lines| {
            let mut passport: HashMap<String, String> = HashMap::new();
            while let Some(line) = lines.next() {
                if line.is_empty() {
                    // if passports has all required keys
                    if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                        .iter()
                        .all(|k| passport.contains_key(k.to_owned()))
                    {
                        return Some(passport);
                    }
                    passport.clear();
                } else {
                    passport.extend(
                        line.split_whitespace()
                            .flat_map(|field| field.split(":"))
                            .map(str::to_owned)
                            .tuples(),
                    );
                }
            }
            None
        })
        .count();

    println!("{:?}", result);
}
