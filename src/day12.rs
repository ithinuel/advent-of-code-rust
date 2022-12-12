use std::{cmp::Reverse, collections::BTreeMap};

use itertools::Itertools;
use yaah::{aoc, aoc_generator};

pub type Map = Vec<u8>;

const START: u8 = 0;
const END: u8 = 27;

#[aoc_generator(day12)]
fn day12(input: &'static str) -> (Map, usize) {
    (
        input
            .lines()
            .flat_map(|l| {
                l.as_bytes().iter().map(|v| match v {
                    b'S' => START,
                    b'E' => END,
                    v => v - b'a' + 1,
                })
            })
            .collect(),
        input.lines().next().unwrap_or("").len(),
    )
}

type Coord = usize;

// reused from 2021 day 15
fn a_star<T>(
    origin: Coord,
    target: Coord,
    mut neighbours: impl FnMut(Coord) -> T,
    mut distance: impl FnMut(Coord, Coord) -> usize,
    mut heuristic: impl FnMut(Coord) -> usize,
) -> Result<(usize, Vec<Coord>), ()>
where
    T: Iterator<Item = Coord>,
{
    let mut open = priority_queue::PriorityQueue::new();
    let mut came_from: BTreeMap<Coord, Coord> = BTreeMap::new();

    let mut g_score = BTreeMap::new();
    let mut f_score = BTreeMap::new();

    open.push(origin, Reverse(0));
    g_score.insert(origin, 0);
    f_score.insert(origin, heuristic(origin));

    fn score(score_map: &BTreeMap<Coord, usize>, node: Coord) -> usize {
        score_map.get(&node).copied().unwrap_or(usize::max_value())
    }

    while let Some((current, _)) = open.pop() {
        if current == target {
            // rebuild path
            let mut current = current;
            let mut path = vec![current];
            while let Some(&origin) = came_from.get(&current) {
                current = origin;
                path.push(origin);
            }
            return Ok((score(&g_score, target), path.into_iter().rev().collect()));
        }

        let current_g_score = score(&g_score, current);
        for neighbour in neighbours(current) {
            let tentative_g_score = current_g_score + distance(current, neighbour);
            if tentative_g_score < score(&g_score, neighbour) {
                let tentative_f_score = tentative_g_score + heuristic(neighbour);

                came_from.insert(neighbour, current);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_f_score);
                open.push(neighbour, Reverse(tentative_f_score));
            }
        }
    }

    Err(())
}

fn path_len(map: &Map, size: usize, start: Coord, target: Coord) -> Option<usize> {
    let target_c = (target % size, target / size);

    let (_, path) = a_star(
        start,
        target,
        |c| {
            let current_height = map[c];
            [c.wrapping_sub(size), c.wrapping_sub(1), c + 1, c + size]
                .into_iter()
                .filter(move |&v| {
                    let h = map.get(v).cloned().unwrap_or(u8::MAX);
                    h <= (current_height + 1)
                })
        },
        |a, b| {
            let (ax, ay) = (a % size, a / size);
            let (bx, by) = (b % size, b / size);
            ax.abs_diff(bx) + ay.abs_diff(by)
        },
        |a| {
            let (ax, ay) = (a % size, a / size);
            ax.abs_diff(target_c.0) + ay.abs_diff(target_c.1)
        },
    )
    .ok()?;
    Some(path.len() - 1)
}

#[aoc(day12, part1)]
fn day12_part1((map, size): &(Map, usize)) -> Option<usize> {
    let size = *size;
    let start = map.iter().find_position(|&&v| v == START)?.0;
    let target = map.iter().find_position(|&&v| v == END)?.0;

    path_len(map, size, start, target)
}

#[aoc(day12, part2)]
fn day12_part2((map, size): &(Map, usize)) -> Option<usize> {
    let size = *size;
    let target = map.iter().find_position(|&&v| v == END)?.0;

    map.iter()
        .positions(|&v| v == 1)
        .filter_map(|start| path_len(map, size, start, target))
        .min()
}

#[cfg(test)]
mod test {
    // >vbv<<<<
    // avcvyxx^
    // a>vvzEx^
    // acv>>^w^
    // ab>>>>>^
    const EXAMPLE: &str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn day12_part1() {
        assert_eq!(Some(31), super::day12_part1(&super::day12(EXAMPLE)));
    }

    #[test]
    fn day12_part2() {
        assert_eq!(Some(29), super::day12_part2(&super::day12(EXAMPLE)));
    }
}
