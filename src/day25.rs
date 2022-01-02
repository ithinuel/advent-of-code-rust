use aoc_runner_derive::*;

use itertools::Itertools;

#[aoc_generator(day25)]
fn gen(input: &str) -> (usize, usize) {
    input
        .lines()
        .filter_map(|l| l.parse::<usize>().ok())
        .collect_tuple()
        .expect("Invalid format")
}

#[aoc(day25, part1)]
fn part1((a, b): &(usize, usize)) -> usize {
    let mut value = 1;
    let loop_size = (2..)
        .take_while(|_| {
            value = (value * 7) % 20201227;
            value != *a
        })
        .last()
        .unwrap();

    println!("loop_size: {}", loop_size);
    value = 1;
    for _ in 0..loop_size {
        value = (value * b) % 20201227;
    }

    value
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"5764801
17807724";

    #[test]
    fn gen() {
        assert_eq!((5764801, 17807724), super::gen(EXAMPLE));
    }

    #[test]
    fn part1() {
        assert_eq!(14897079, super::part1(&super::gen(EXAMPLE)));
    }
}
