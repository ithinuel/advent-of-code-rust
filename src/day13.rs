use std::{borrow::Cow, collections::BTreeSet};

use yaah::*;
use itertools::Itertools;

type Coord = (i32, i32);
#[derive(Copy, Clone, Debug)]
pub enum Fold {
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
macro_rules! impl_display {
    ($t:ty) => {
        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f)?;
                writeln!(f, "{:?}", self)
            }
        }
    };
}

#[aoc_generator(day13)]
fn gen(input: &str) -> Gen {
    let (dots, instr) = input.split("\n\n").next_tuple().expect("Invalid format");
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
    let &line = instr.first().expect("Invalid input");
    map.iter().map(|&coord| line.fold(coord)).unique().count()
}

#[derive(PartialEq)]
pub struct Map(BTreeSet<Coord>);
impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.0.iter().fold((0, 0), |(col, line), &(x, y)| {
            (i32::max(col, x), i32::max(line, y))
        });
        for (y, x) in (0..=max_y).cartesian_product(0..=max_x) {
            match self.0.get(&(x, y)) {
                Some(_) => write!(f, "#")?,
                None => write!(f, ".")?,
            }
            if x == max_x && y != max_y {
                writeln!(f)?
            }
        }
        Ok(())
    }
}
impl_display!(Map);
#[aoc(day13, part2)]
fn part2((map, instr): &Gen) -> Map {
    Map(instr.iter().fold(map.clone(), |map, &line| {
        map.into_iter().map(|coord| line.fold(coord)).collect()
    }))
}

pub struct BufferedMap(BTreeSet<(i32, i32)>);
impl std::fmt::Debug for BufferedMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.0.iter().fold((0, 0), |(col, line), &(x, y)| {
            (i32::max(col, x), i32::max(line, y))
        });
        let (len_x, len_y) = ((max_x + 1) as usize, (max_y + 1) as usize);

        let mut output = vec![b'.'; len_x * len_y];
        for &(x, y) in &self.0 {
            let (x, y) = (x as usize, y as usize);
            output[x + y * len_x] = b'#';
        }
        itertools::intersperse(
            output.chunks(len_x).map(String::from_utf8_lossy),
            Cow::Borrowed("\n"),
        )
        .try_for_each(|s| write!(f, "{}", s))
    }
}
impl_display!(BufferedMap);
#[aoc(day13, part2, buffered_output)]
fn part2_buffered((map, instr): &Gen) -> BufferedMap {
    BufferedMap(instr.iter().fold(map.clone(), |map, &line| {
        map.into_iter().map(|coord| line.fold(coord)).collect()
    }))
}

#[cfg(test)]
mod test {
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

    const EXPECT: &str = r"#####
#...#
#...#
#...#
#####";

    #[test]
    fn part1() {
        assert_eq!(17, super::part1(&gen(EXAMPLE)));
    }
    #[test]
    fn part2() {
        assert_eq!(EXPECT, format!("{:?}", super::part2(&gen(EXAMPLE))));
    }
    #[test]
    fn part2_buffered() {
        assert_eq!(
            EXPECT,
            format!("{:?}", super::part2_buffered(&gen(EXAMPLE)))
        );
    }
}
