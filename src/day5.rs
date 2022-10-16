use std::{collections::HashMap, iter::repeat};

use yaah::*;
use itertools::Itertools;

type Parsed = Vec<((u32, u32), (u32, u32))>;

#[aoc_generator(day5)]
fn gen(input: &str) -> Parsed {
    input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .filter_map(|coord| coord.split(',').filter_map(|n| n.parse().ok()).next_tuple())
                .next_tuple()
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Parsed) -> usize {
    let input: Vec<_> = input
        .iter()
        .filter(|(a, b)| a.0 == b.0 || a.1 == b.1)
        .map(|(a, b)| {
            (
                (u32::min(a.0, b.0), u32::min(a.1, b.1)),
                (u32::max(a.0, b.0), u32::max(a.1, b.1)),
            )
        })
        .collect();
    let (max_x, max_y) = input.iter().fold((0, 0), |acc, (a, b)| {
        (
            u32::max(acc.0, u32::max(a.0, b.0)),
            u32::max(acc.1, u32::max(a.1, b.1)),
        )
    });
    let size = ((1 + max_x) * (1 + max_y)) as usize;
    let mut map = Vec::new();
    map.resize(size, 0);
    for (a, b) in input {
        for x in a.0..=b.0 {
            for y in a.1..=b.1 {
                map[(x + y * max_x) as usize] += 1;
            }
        }
    }
    map.iter().filter(|&&n| n > 1).count()
}

#[aoc(day5, part2)]
fn part2(input: &Parsed) -> usize {
    let (max_x, max_y) = input.iter().fold((0, 0), |acc, (a, b)| {
        (
            u32::max(acc.0, u32::max(a.0, b.0)),
            u32::max(acc.1, u32::max(a.1, b.1)),
        )
    });
    let size = ((1 + max_x) * (1 + max_y)) as usize;
    let mut map = Vec::new();
    map.resize(size, 0);

    for (a, b) in input {
        use either::Either::*;
        use std::cmp::Ordering::*;

        let x_coords = match a.0.cmp(&b.0) {
            Less => Left(a.0..=b.0),
            Equal => Right(Left(repeat(a.0))),
            Greater => Right(Right((b.0..=a.0).rev())),
        };
        let y_coords = match a.1.cmp(&b.1) {
            Less => Left(a.1..=b.1),
            Equal => Right(Left(repeat(a.1))),
            Greater => Right(Right((b.1..=a.1).rev())),
        };
        x_coords.zip(y_coords).for_each(|(x, y)| {
            map[(x + y * max_x) as usize] += 1;
        });
    }
    map.iter().filter(|&&n| n > 1).count()
}

#[aoc(day5, part2, hashmap)]
fn part2_hashmap(input: &Parsed) -> usize {
    let mut map = HashMap::new();

    for (a, b) in input {
        use either::Either::*;
        use std::cmp::Ordering::*;

        let x_coords = match a.0.cmp(&b.0) {
            Less => Left(a.0..=b.0),
            Equal => Right(Left(repeat(a.0))),
            Greater => Right(Right((b.0..=a.0).rev())),
        };
        let y_coords = match a.1.cmp(&b.1) {
            Less => Left(a.1..=b.1),
            Equal => Right(Left(repeat(a.1))),
            Greater => Right(Right((b.1..=a.1).rev())),
        };
        x_coords.zip(y_coords).for_each(|(x, y)| {
            *map.entry((x, y)).or_insert(0) += 1;
        });
    }
    map.iter().filter(|(_, &n)| n > 1).count()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1() {
        assert_eq!(5, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(12, super::part2(&gen(EXAMPLE)));
    }

    #[test]
    fn part2_hashmap() {
        assert_eq!(12, super::part2_hashmap(&gen(EXAMPLE)));
    }
}
