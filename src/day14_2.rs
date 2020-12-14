use std::collections::BTreeMap;
use std::io::BufRead;

fn write(
    mem: &mut BTreeMap<u64, u64>,
    addr: u64,
    val: u64,
    mut mask: impl Iterator<Item = (usize, char)> + Clone,
) {
    match mask.next() {
        None => *mem.entry(addr).or_default() = val,
        Some((bit, bit_mask)) => match bit_mask {
            '0' => write(mem, addr, val, mask),
            '1' => write(mem, addr | (1 << bit), val, mask),
            'X' => {
                write(mem, addr & !(1 << bit), val, mask.clone());
                write(mem, addr | (1 << bit), val, mask);
            }
            _ => unreachable!(),
        },
    }
}

fn main() {
    let re = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();
    let mut mask = "000000000000000000000000000000000000".to_string();

    for line in std::io::stdin().lock().lines().filter_map(Result::ok) {
        if line.starts_with("mask") {
            mask = line.split_at(7).1.to_string();
        } else {
            let caps = re.captures(&line).expect("invalid mem fmt");
            let addr: u64 = caps[1].parse().ok().expect("invalid mem address fmt");
            let val: u64 = caps[2].parse().ok().expect("invalid mem value fmt");

            write(
                &mut memory,
                addr,
                val,
                mask.chars().enumerate().map(|(bit, mask)| (35 - bit, mask)),
            )
        }
    }

    println!("{:?}", memory.values().sum::<u64>());
}
