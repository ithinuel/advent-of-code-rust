use std::collections::HashMap;

use yaah::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Heading {
    East,
    South,
}

pub type Map = HashMap<(usize, usize), Heading>;
struct Printable<'a>(&'a Map);
impl std::fmt::Debug for Printable<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xmax, ymax) = self.0.keys().fold((0, 0), |acc, c| {
            (usize::max(acc.0, c.0), usize::max(acc.1, c.1))
        });
        (0..=ymax).try_for_each(|y| {
            (0..=xmax).try_for_each(|x| {
                write!(
                    f,
                    "{}",
                    self.0
                        .get(&(x, y))
                        .map(|heading| {
                            match heading {
                                Heading::East => '>',
                                Heading::South => 'v',
                            }
                        })
                        .unwrap_or('.')
                )
            })?;
            writeln!(f)
        })
    }
}

#[aoc_generator(day25)]
fn gen(input: &str) -> Map {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.bytes().enumerate().filter_map(move |(col, b)| match b {
                b'.' => None,
                b'>' => Some(((col, row), Heading::East)),
                b'v' => Some(((col, row), Heading::South)),
                _ => unreachable!(),
            })
        })
        .collect()
}

fn step(map: &mut Map, width: usize, height: usize) -> usize {
    // list those that need to move east
    let can_move_east = map
        .iter()
        .filter(|(_, &v)| v == Heading::East)
        .filter_map(|(&from, _)| {
            let to = ((from.0 + 1) % width, from.1);
            (!map.contains_key(&to)).then(|| (from, to))
        })
        .collect_vec();
    // move them
    can_move_east.iter().cloned().for_each(|(from, to)| {
        map.remove(&from);
        map.insert(to, Heading::East);
    });
    // list those that need to move south
    let can_move_south = map
        .iter()
        .filter(|(_, &v)| v == Heading::South)
        .filter_map(|(&from, _)| {
            let to = (from.0, (from.1 + 1) % height);
            (!map.contains_key(&to)).then(|| (from, to))
        })
        .collect_vec();
    // move them
    can_move_south.iter().cloned().for_each(|(from, to)| {
        map.remove(&from);
        map.insert(to, Heading::South);
    });
    can_move_east.len() + can_move_south.len()
}

#[aoc(day25, part1)]
fn part1(map: &Map) -> Option<usize> {
    let (xmax, ymax) = map.keys().fold((0, 0), |acc, c| {
        (usize::max(acc.0, c.0), usize::max(acc.1, c.1))
    });
    let mut map = map.clone();
    (1..)
        .take_while(|_| step(&mut map, xmax + 1, ymax + 1) != 0)
        .last()
        .map(|v| v + 1)
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part1() {
        assert_eq!(Some(58), super::part1(&super::gen(EXAMPLE)))
    }
}
