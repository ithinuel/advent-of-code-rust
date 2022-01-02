use std::{
    collections::{LinkedList, VecDeque},
    io::Write,
};

use aoc_runner_derive::*;
use itertools::Itertools;

#[aoc_generator(day23)]
fn gen(input: &str) -> VecDeque<u8> {
    input.bytes().map(|b| b - b'0').collect()
}

fn play_a_move<
    T: std::ops::Sub<Output = T>
        + Eq
        + Copy
        + Send
        + Sync
        + Ord
        + num::Zero
        + num::One
        + std::ops::Add<Output = T>,
>(
    mut cups: VecDeque<T>,
) -> VecDeque<T> {
    let current = cups.pop_front().unwrap();
    let c1 = cups.pop_front().unwrap();
    let c2 = cups.pop_front().unwrap();
    let c3 = cups.pop_front().unwrap();
    cups.push_back(current);

    let mut diff = T::one();
    let mut target_pos = None;
    loop {
        let target = current - diff;
        if target == T::zero() {
            break;
        }
        target_pos = cups.iter().position(|&v| v == target);
        if target_pos.is_some() {
            break;
        }
        diff = diff + T::one();
    }

    if target_pos.is_none() {
        target_pos = cups
            .iter()
            .enumerate()
            .max_by_key(|(_, &v)| v)
            .map(|(idx, _)| idx);
    }
    let target_pos = target_pos.unwrap() + 1;

    cups.insert(target_pos, c3);
    cups.insert(target_pos, c2);
    cups.insert(target_pos, c1);

    cups
}

#[aoc(day23, part1)]
fn part1(input: &VecDeque<u8>) -> usize {
    let mut cups = input.clone();
    for _ in 0..100 {
        cups = play_a_move(cups);
    }

    while let Some(n) = cups.pop_front() {
        if n == 1 {
            break;
        }
        cups.push_back(n);
    }

    cups.into_iter().fold(0, |acc, v| acc * 10 + (v as usize))
}

#[aoc(day23, part1, with_list)]
fn part1_with_list(input: &VecDeque<u8>) -> usize {
    let mut cups: LinkedList<_> = input.iter().cloned().map(usize::from).collect();
    for _ in 0..100 {
        cups = play_a_move_list(cups);
    }

    while let Some(n) = cups.pop_front() {
        if n == 1 {
            break;
        }
        cups.push_back(n);
    }

    cups.into_iter().fold(0, |acc, v| acc * 10 + (v as usize))
}

//#[cached::proc_macro::cached]
fn play_a_move_list(cups: LinkedList<usize>) -> LinkedList<usize> {
    let mut cups = cups;
    let current = cups.pop_front().unwrap();
    let mut extracted = cups.split_off(3);
    std::mem::swap(&mut extracted, &mut cups);
    cups.push_back(current);

    let mut diff = 1;
    let target = loop {
        if current == diff {
            break *cups.iter().max().unwrap();
        } else if !extracted.iter().contains(&(current - diff)) {
            break (current - diff);
        }
        diff += 1;
    };

    let target_pos = cups.iter().position(|&v| v == target).unwrap();
    let mut end = cups.split_off(target_pos + 1);

    cups.append(&mut extracted);
    cups.append(&mut end);
    cups
}

#[aoc(day23, part2)]
fn part2(input: &VecDeque<u8>) -> usize {
    let mut cups = input
        .iter()
        .cloned()
        .map(usize::from)
        .collect::<VecDeque<_>>();
    let max = *cups.iter().max().unwrap();
    cups.extend((max + 1)..=1_000_000);

    const DOT_PER_LINE: u32 = 100;
    const MOVE_PER_DOT: u32 = 1_000;
    println!("Go get a tea this takes about an hour to complete.");
    let start = std::time::Instant::now();
    for n in 1..=10_000_000 {
        cups = play_a_move(cups);
        if (n % MOVE_PER_DOT) == 0 {
            print!(".");
            std::io::stdout().lock().flush().unwrap();
        }
        if (n % (DOT_PER_LINE * MOVE_PER_DOT)) == 0 {
            let new_ts = std::time::Instant::now();
            let pace = (new_ts - start) / n; // time per move
            println!(": ETA = {:.3?}", (10_000_000 - n) * pace);
        }
    }

    let one_pos = cups.iter().position(|&v| v == 1).unwrap();
    cups[(one_pos + 1) % 1_000_000] * cups[(one_pos + 2) % 1_000_000]
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = "389125467";

    #[test]
    fn part1() {
        assert_eq!(67384529, super::part1(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part1_with_list() {
        assert_eq!(67384529, super::part1_with_list(&super::gen(EXAMPLE)));
    }
    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(149245887792, super::part2(&super::gen(EXAMPLE)));
    }
}
