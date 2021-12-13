use std::collections::BTreeSet;

use aoc_runner_derive::*;
use itertools::Itertools;

type Coord = (i32, i32);
#[derive(Copy, Clone, Debug)]
enum Fold {
    Col(i32),
    Line(i32),
}
impl Fold {
    fn fold(&self, (x, y): Coord) -> Coord {
        match &self {
            Fold::Line(line) => (x, line - (line - y).abs()),
            Fold::Col(col) => (col - (col - x).abs(), y),
        }
    }
}
type Gen = (BTreeSet<Coord>, Vec<Fold>);

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
            let val = val.parse().ok()?;
            Some(match axis {
                "x" => Fold::Col(val),
                "y" => Fold::Line(val),
                _ => unreachable!(),
            })
        })
        .collect();
    (map, instr)
}

#[aoc(day13, part1)]
fn part1((map, instr): &Gen) -> usize {
    let &line = instr.first().unwrap();
    let map: BTreeSet<_> = map.into_iter().map(|&coord| line.fold(coord)).collect();
    map.len()
}

fn print(map: &BTreeSet<Coord>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (max_x, max_y) = map.iter().fold((0, 0), |(col, line), &(x, y)| {
        (i32::max(col, x), i32::max(line, y))
    });
    for (y, x) in (0..=max_y).cartesian_product(0..=max_x) {
        match map.get(&(x, y)) {
            Some(_) => write!(f, "*")?,
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
    Map(instr.into_iter().fold(map.clone(), |map, &line| {
        map.into_iter().map(|coord| line.fold(coord)).collect()
    }))
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
