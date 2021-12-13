use std::collections::BTreeSet;

use aoc_runner_derive::*;
use itertools::Itertools;

type Coord = (i32, i32);

type Gen = (BTreeSet<Coord>, Vec<Coord>);

fn fold((col, line): Coord, (x, y): Coord) -> Coord {
    if col == 0 {
        (x, line - (line - y).abs())
    } else {
        (col - (col - x).abs(), y)
    }
}
#[aoc_generator(day13)]
fn gen(input: &str) -> Gen {
    let (dots, instr) = input.split("\n\n").next_tuple().unwrap();
    let map = dots
        .lines()
        .filter_map(|l| l.split(',').filter_map(|v| v.parse().ok()).next_tuple())
        .collect();
    let instr = instr
        .lines()
        .filter_map(|l| {
            let (axis, val) = l
                .trim_start_matches("fold along ")
                .split('=')
                .next_tuple()?;
            let val: i32 = val.parse().ok()?;
            Some(match axis {
                "x" => (val, 0),
                "y" => (0, val),
                _ => unreachable!(),
            })
        })
        .collect();
    (map, instr)
}

#[aoc(day13, part1)]
fn part1((map, instr): &Gen) -> usize {
    let &fold_line = instr.first().unwrap();
    let map: BTreeSet<_> = map
        .into_iter()
        .map(|&coord| fold(fold_line, coord))
        .collect();
    map.len()
}

fn print(map: &BTreeSet<Coord>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (max_x, max_y) = map.iter().fold((0, 0), |(col, line), &(x, y)| {
        (i32::max(col, x), i32::max(line, y))
    });
    for (y, x) in (0..=max_y).cartesian_product(0..=max_x) {
        match map.get(&(x, y)) {
            Some(_) => write!(f, "#")?,
            None => write!(f, " ")?,
        }
        if x == max_x {
            writeln!(f)?
        }
    }
    Ok(())
}

#[derive(PartialEq)]
struct Map(BTreeSet<Coord>);
impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print(&self.0, f)
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        print(&self.0, f)
    }
}

#[aoc(day13, part2)]
fn part2((map, instr): &Gen) -> Map {
    let map: BTreeSet<_> = instr.into_iter().fold(map.clone(), |map, &fold_line| {
        map.into_iter()
            .map(|coord| fold(fold_line, coord))
            .collect()
    });
    Map(map)
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::gen;

    const EXAMPLE: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    const EXPECT: &str = r"0,0
1,0
2,0
3,0
4,0
0,1
4,1
0,2
4,2
0,3
4,3
0,4
1,4
2,4
3,4
4,4";

    #[test]
    fn part1() {
        assert_eq!(17, super::part1(&gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        let expect = super::Map(
            EXPECT
                .lines()
                .filter_map(|l| l.split(',').filter_map(|v| v.parse().ok()).next_tuple())
                .collect(),
        );
        assert_eq!(expect, super::part2(&gen(EXAMPLE)));
    }
}
