use aoc_helper::*;
use std::{cmp::Reverse, collections::BTreeMap};

type Map = Vec<Vec<usize>>;
type Coord = (usize, usize);

#[aoc_generator(day15, part1)]
fn gen(input: &str) -> Map {
    input
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as usize).collect())
        .collect()
}

fn get_neightbour((col, line): Coord, map: &Map) -> impl Iterator<Item = Coord> + '_ {
    [(-1, 0), (0, -1), (1, 0), (0, 1)]
        .into_iter()
        .filter_map(move |(dcol, dline)| {
            let col = (col as isize).wrapping_add(dcol) as usize;
            let line = (line as isize).wrapping_add(dline) as usize;
            map.get(line).and_then(|l| l.get(col)).map(|_| (col, line))
        })
}

fn a_star<T>(
    origin: Coord,
    target: Coord,
    mut neighbours: impl FnMut(Coord) -> T,
    mut distance: impl FnMut(Coord, Coord) -> usize,
    mut heuristic: impl FnMut(Coord) -> usize,
) -> Result<usize, ()>
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
            //let mut current = current;
            //let mut path = vec![current];
            //while let Some(&origin) = came_from.get(&current) {
            //    current = origin;
            //    path.push(origin);
            //}
            //return Ok((score(&g_score, target), path.into_iter().rev().collect()));
            return Ok(score(&g_score, target));
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

#[aoc(day15, part1)]
fn part1(map: &Map) -> Option<usize> {
    let target = (map[0].len() - 1, map.len() - 1);

    a_star(
        (0, 0),
        target,
        |node| get_neightbour(node, map),
        |_, c| map[c.1][c.0],
        |c| map[c.1][c.0],
    )
    .ok()
}

#[aoc_generator(day15, part2)]
fn gen_part2(input: &str) -> Map {
    let map = gen(input);

    (0..5)
        .flat_map(|rline| {
            map.iter().map(move |line| {
                (0..5)
                    .flat_map(|rcol| {
                        line.iter().copied().map(move |risk| {
                            let risk = rline + rcol + risk;
                            risk - if risk > 9 { 9 } else { 0 }
                        })
                    })
                    .collect()
            })
        })
        .collect()
}

#[aoc(day15, part2)]
fn part2(map: &Map) -> Option<usize> {
    part1(map)
}

#[cfg(test)]
mod test {
    use super::gen;
    use super::gen_part2;

    const EXAMPLE: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1() {
        assert_eq!(Some(40), super::part1(&gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(Some(315), super::part2(&gen_part2(EXAMPLE)));
    }
}
