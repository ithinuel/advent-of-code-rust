use std::collections::BTreeMap;
use std::io::BufRead;

fn main() {
    let (mut and, mut or): (u64, u64) = (0xFFFFFFFFF, 0);
    let re = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();

    for line in std::io::stdin().lock().lines().filter_map(Result::ok) {
        if line.starts_with("mask") {
            or = 0;
            let mut not_and = 0;
            line.chars().skip(7).enumerate().for_each(|(i, c)| match c {
                'X' => {}
                '0' => not_and |= 1 << (35 - i),
                '1' => or |= 1 << (35 - i),
                _ => unreachable!(),
            });
            and = !not_and;
        } else {
            let caps = re.captures(&line).expect("invalid mem fmt");
            let addr: u64 = caps[1].parse().ok().expect("invalid mem address fmt");
            let val: u64 = caps[2].parse().ok().expect("invalid mem value fmt");

            *memory.entry(addr).or_default() = (val & and) | or;
        }
    }

    println!("{:?}", memory.values().sum::<u64>());
}
