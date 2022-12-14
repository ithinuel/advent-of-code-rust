use std::collections::HashSet;

use itertools::Itertools;
use yaah::{aoc, aoc_generator};

pub type Point = (i32, i32);
pub type Line = (Point, Point);

fn draw_to_vec(lines: &[Line], width: i32, (minx, miny): (i32, i32), map: &mut [char]) {
    lines.iter().for_each(|(start, end)| {
        if start.1 == end.1 {
            // draw vertically
            let min = start.0.min(end.0);
            let max = start.0.max(end.0);

            (min..=max).for_each(|v| map[(v - minx + width * (start.1 - miny)) as usize] = '#');
        } else {
            // draw vertically
            let min = start.1.min(end.1);
            let max = start.1.max(end.1);

            (min..=max).for_each(|v| map[(start.0 - minx + width * (v - miny)) as usize] = '#');
        }
    });
}
struct Map<'a>(&'a [Line]);
impl std::fmt::Debug for Map<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (minx, maxx) = self
            .0
            .iter()
            .flat_map(|v| [v.0 .0, v.1 .0].into_iter())
            .minmax()
            .into_option()
            .ok_or(std::fmt::Error)?;
        let (miny, maxy) = self
            .0
            .iter()
            .flat_map(|v| [v.0 .1, v.1 .1].into_iter())
            .minmax()
            .into_option()
            .ok_or(std::fmt::Error)?;
        let width = (maxx - minx) + 1;
        let mut map = vec!['.'; (width * (maxy - miny + 1)) as usize];

        draw_to_vec(self.0, width, (minx, miny), &mut map);
        writeln!(f)?;
        map.chunks(width as usize)
            .try_for_each(|line| writeln!(f, "{}", line.iter().collect::<String>()))
    }
}
struct FilledMap<'a> {
    walls: &'a [Line],
    blocked: HashSet<Point>,
}
impl std::fmt::Debug for FilledMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (minx, maxx) = self
            .walls
            .iter()
            .flat_map(|v| [v.0 .0, v.1 .0].into_iter())
            .chain(self.blocked.iter().map(|&(x, _)| x))
            .minmax()
            .into_option()
            .ok_or(std::fmt::Error)?;

        let (miny, maxy) = self
            .walls
            .iter()
            .flat_map(|v| [v.0 .1, v.1 .1].into_iter())
            .chain(self.blocked.iter().map(|&(_, y)| y))
            .minmax()
            .into_option()
            .ok_or(std::fmt::Error)?;
        let width = (maxx - minx) + 1;
        let mut map = vec!['.'; (width * (maxy - miny + 1)) as usize];

        draw_to_vec(self.walls, width, (minx, miny), &mut map);

        for &(x, y) in &self.blocked {
            let cell = &mut map[(x - minx + (y - miny) * width) as usize];
            if *cell == '.' {
                *cell = 'o';
            }
        }
        writeln!(f)?;
        map.chunks(width as usize)
            .try_for_each(|line| writeln!(f, "{}", line.iter().collect::<String>()))
    }
}

fn init_blocked(walls: &[Line]) -> HashSet<Point> {
    walls
        .iter()
        .flat_map(|(start, end)| {
            if start.1 == end.1 {
                // draw vertically
                let min = start.0.min(end.0);
                let max = start.0.max(end.0);

                either::Either::Left((min..=max).map(|v| (v, start.1)))
            } else {
                // draw vertically
                let min = start.1.min(end.1);
                let max = start.1.max(end.1);

                either::Either::Right((min..=max).map(|v| (start.0, v)))
            }
        })
        .collect()
}

#[aoc_generator(day14)]
fn gen(input: &'static str) -> Vec<Line> {
    input
        .lines()
        .flat_map(|l| {
            l.split(" -> ")
                .filter_map(|point| {
                    point
                        .split(',')
                        .filter_map(|v| v.parse().ok())
                        .collect_tuple()
                })
                .tuple_windows()
        })
        .collect_vec()
}

#[aoc(day14, part1)]
fn day14_part1(walls: &[Line]) -> Option<usize> {
    let maxy = walls
        .iter()
        .flat_map(|v| [v.0 .1, v.1 .1].into_iter())
        .max()?;

    let mut units = 0;
    let mut blocked = init_blocked(walls);

    'outer: loop {
        let mut unit = (500, 0);
        loop {
            if unit.1 > maxy {
                // falling off the edge, stop simulation
                break 'outer;
            }

            let down = (unit.0, unit.1 + 1);
            if !blocked.contains(&down) {
                unit = down;
                continue;
            }
            let down_left = (unit.0 - 1, down.1);
            if !blocked.contains(&down_left) {
                unit = down_left;
                continue;
            }
            let down_right = (down_left.0 + 2, down_left.1);
            if !blocked.contains(&down_right) {
                unit = down_right;
                continue;
            }

            blocked.insert(unit);
            units += 1;
            // particle blocked, introduce a new one.
            break;
        }
    }

    #[cfg(test)]
    {
        println!("Filled:");
        println!("{:?}", FilledMap { walls, blocked });
    }

    Some(units)
}

#[aoc(day14, part2)]
fn day14_part2(walls: &[Line]) -> Option<usize> {
    let maxy = walls
        .iter()
        .flat_map(|v| [v.0 .1, v.1 .1].into_iter())
        .max()?;

    let mut units = 0;
    let mut blocked = init_blocked(walls);

    'outer: loop {
        let mut unit = (500, 0);
        loop {
            if unit.1 < (maxy + 1) {
                let down = (unit.0, unit.1 + 1);
                if !blocked.contains(&down) {
                    unit = down;
                    continue;
                }
                let down_left = (unit.0 - 1, down.1);
                if !blocked.contains(&down_left) {
                    unit = down_left;
                    continue;
                }
                let down_right = (down_left.0 + 2, down_left.1);
                if !blocked.contains(&down_right) {
                    unit = down_right;
                    continue;
                }
            }

            blocked.insert(unit);
            units += 1;

            if unit == (500, 0) {
                break 'outer;
            }

            // particle blocked, introduce a new one.
            break;
        }
    }

    #[cfg(test)]
    {
        println!("Filled:");
        println!("{:?}", FilledMap { walls, blocked });
    }

    Some(units)
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn gen() {
        let expected = vec![
            ((498, 4), (498, 6)),
            ((498, 6), (496, 6)),
            ((503, 4), (502, 4)),
            ((502, 4), (502, 9)),
            ((502, 9), (494, 9)),
        ];
        assert_eq!(expected, super::gen(EXAMPLE));
        println!("{:?}", super::Map(&expected));
    }

    #[test]
    fn day14_part1() {
        assert_eq!(Some(24), super::day14_part1(&super::gen(EXAMPLE)));
    }
    #[test]
    fn day14_part2() {
        assert_eq!(Some(93), super::day14_part2(&super::gen(EXAMPLE)));
    }
}
