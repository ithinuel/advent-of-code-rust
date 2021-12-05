use std::{iter::Repeat, ops::RangeInclusive};

use aoc_runner_derive::*;
use itertools::Itertools;
use std::iter::Rev;

type Parsed = Vec<((u32, u32), (u32, u32))>;

#[aoc_generator(day5)]
fn gen(input: &str) -> Parsed {
    input
        .lines()
        .filter_map(|l| {
            l.split(" -> ")
                .filter_map(|coord| coord.split(",").filter_map(|n| n.parse().ok()).next_tuple())
                .next_tuple()
        })
        .collect()
}

#[aoc(day5, part1)]
fn part1(input: &Parsed) -> usize {
    let input: Vec<_> = input
        .iter()
        .filter_map(|(a, b)| {
            if a.0 == b.0 {
                Some(((a.0, u32::min(a.1, b.1)), (a.0, u32::max(a.1, b.1))))
            } else if a.1 == b.1 {
                Some(((u32::min(a.0, b.0), a.1), (u32::max(a.0, b.0), a.1)))
            } else {
                None
            }
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
    println!("{:?}", input);
    for (a, b) in input {
        use either::Either::{self, *};

        let x_coords = match a.0.cmp(&b.0) {
            std::cmp::Ordering::Less => Left(a.0..=b.0),
            std::cmp::Ordering::Equal => Right(
                Either::<_, Rev<RangeInclusive<u32>>>::Left(std::iter::repeat(a.0)).into_iter(),
            ),
            std::cmp::Ordering::Greater => {
                Right(Either::<Repeat<u32>, _>::Right((b.0..=a.0).rev()).into_iter())
            }
        };
        let y_coords = match a.1.cmp(&b.1) {
            std::cmp::Ordering::Less => Left(a.1..=b.1),
            std::cmp::Ordering::Equal => Right(
                Either::<_, Rev<RangeInclusive<u32>>>::Left(std::iter::repeat(a.1)).into_iter(),
            ),
            std::cmp::Ordering::Greater => {
                Right(Either::<Repeat<u32>, _>::Right((b.1..=a.1).rev()).into_iter())
            }
        };
        x_coords.zip(y_coords).for_each(|(x, y)| {
            println!("{},{}", x, y);
            map[(x + y * max_x) as usize] += 1;
        });
    }
    map.iter().filter(|&&n| n > 1).count()
}

#[cfg(test)]
mod test {
    use super::{gen, part1 as solve_part1, part2 as solve_part2};

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
        assert_eq!(5, solve_part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(12, solve_part2(&gen(EXAMPLE)));
    }
}
