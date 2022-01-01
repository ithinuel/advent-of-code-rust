use aoc_runner_derive::*;
use itertools::Itertools;

use std::collections::VecDeque;

#[aoc_generator(day23)]
fn gen(input: &str) -> VecDeque<u8> {
    input.bytes().map(|b| b - b'0').collect()
}

fn play_a_move(mut cups: VecDeque<u8>) -> VecDeque<u8> {
    let current = cups.pop_front().unwrap();
    let c1 = cups.pop_front().unwrap();
    let c2 = cups.pop_front().unwrap();
    let c3 = cups.pop_front().unwrap();
    cups.push_back(current);

    let mut diff = 1;
    let mut target_pos = None;
    loop {
        let target = current - diff;
        if target == 0 {
            break;
        }
        target_pos = cups.iter().position(|&v| v == target);
        if target_pos.is_some() {
            break;
        }
        diff += 1;
    }

    if target_pos.is_none() {
        target_pos = cups.iter().position_max();
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

#[cfg(test)]
mod test {
    const EXAMPLE: &str = "389125467";

    #[test]
    fn part1() {
        assert_eq!(67384529, super::part1(&super::gen(EXAMPLE)));
    }
}
