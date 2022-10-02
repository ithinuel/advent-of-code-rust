use std::{
    collections::{BTreeSet, VecDeque},
    iter::repeat,
};

use aoc_helper::*;
use itertools::Itertools;

#[aoc_generator(day9)]
fn gen(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect_vec())
        .collect_vec()
}

fn get_neighbours(
    input: &[Vec<u8>],
    (x, y): (usize, usize),
) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    let left_col = x.wrapping_sub(1);
    let top_line = y.wrapping_sub(1);

    [(left_col, y), (x, top_line), (x + 1, y), (x, y + 1)]
        .into_iter()
        .filter_map(|(x, y)| {
            input
                .get(y)
                .and_then(|line| line.get(x))
                .map(|&cell| ((x, y), cell))
        })
}
fn find_low_points(input: &[Vec<u8>]) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    input.iter().enumerate().flat_map(move |(y, line)| {
        line.iter().enumerate().filter_map(move |(x, &cell)| {
            if get_neighbours(input, (x, y)).all(|(_, cell2)| cell < cell2) {
                Some(((x, y), cell))
            } else {
                None
            }
        })
    })
}

#[aoc(day9, part1)]
fn part1(input: &[Vec<u8>]) -> usize {
    find_low_points(input)
        .map(|(_, cell)| cell as usize + 1)
        .sum()
}

#[aoc(day9, part2)]
fn part2(input: &[Vec<u8>]) -> usize {
    let mut to_visit: VecDeque<((usize, usize), usize)> = find_low_points(input)
        .enumerate()
        .map(|(bassin_id, (coords, _))| (coords, bassin_id))
        .collect();
    let mut visited: BTreeSet<_> = to_visit.iter().map(|&(coords, _)| coords).collect();
    let mut bassins: Vec<_> = repeat(0).take(to_visit.len()).collect();

    while let Some((coords, id)) = to_visit.pop_front() {
        bassins[id] += 1;
        get_neighbours(input, coords)
            .filter(|&(_, cell)| cell < 9)
            .for_each(|(coords, _)| {
                if visited.insert(coords) {
                    to_visit.push_back((coords, id));
                }
            })
    }

    bassins.sort_unstable();
    bassins.iter().rev().take(3).product()
}

#[aoc(day9, part2, alt)]
fn part2_alt(input: &[Vec<u8>]) -> usize {
    let mut to_visit: VecDeque<((usize, usize), usize)> = find_low_points(input)
        .enumerate()
        .map(|(bassin_id, (coords, _))| (coords, bassin_id))
        .collect();
    let mut visited = input
        .iter()
        .map(|line| repeat(false).take(line.len()).collect_vec())
        .collect_vec();
    to_visit.iter().for_each(|&((x, y), _)| {
        visited[y][x] = true;
    });
    let mut bassins: Vec<_> = repeat(0).take(to_visit.len()).collect();

    while let Some((coords, id)) = to_visit.pop_front() {
        bassins[id] += 1;
        get_neighbours(input, coords)
            .filter(|&(_, cell)| cell < 9)
            .for_each(|(coords, _)| {
                let visited = &mut visited[coords.1][coords.0];
                if !*visited {
                    *visited = true;
                    to_visit.push_back((coords, id));
                }
            })
    }

    bassins.sort_unstable();
    bassins.iter().rev().take(3).product()
}

#[cfg(test)]
mod test {
    use super::gen;

    const EXAMPLE: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1() {
        assert_eq!(15, super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(1134, super::part2(&gen(EXAMPLE)));
    }

    #[test]
    fn part2_alt() {
        assert_eq!(1134, super::part2_alt(&gen(EXAMPLE)));
    }
}
