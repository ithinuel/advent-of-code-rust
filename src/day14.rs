use aoc_runner_derive::*;
use std::collections::BTreeMap;

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let (mut and, mut or): (u64, u64) = (0xFFFFFFFFF, 0);
    let re = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();

    for line in input.lines() {
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
            let addr: u64 = caps[1].parse().expect("invalid mem address fmt");
            let val: u64 = caps[2].parse().expect("invalid mem value fmt");

            *memory.entry(addr).or_default() = (val & and) | or;
        }
    }

    memory.values().sum()
}

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

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let re = regex::Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    let mut memory = BTreeMap::new();
    let mut mask = "000000000000000000000000000000000000";

    for line in input.lines() {
        if line.starts_with("mask") {
            mask = line.split_at(7).1;
        } else {
            let caps = re.captures(&line).expect("invalid mem fmt");
            let addr: u64 = caps[1].parse().expect("invalid mem address fmt");
            let val: u64 = caps[2].parse().expect("invalid mem value fmt");

            write(
                &mut memory,
                addr,
                val,
                mask.chars().enumerate().map(|(bit, mask)| (35 - bit, mask)),
            )
        }
    }

    memory.values().sum()
}
