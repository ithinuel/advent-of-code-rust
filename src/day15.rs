use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;
use yaah::{aoc, aoc_generator};

pub type Coord = (i64, i64);

#[aoc_generator(day15)]
fn day15(input: &'static str) -> Vec<(Coord, Coord)> {
    let re = regex::Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    re.captures_iter(input)
        .filter_map(|caps| {
            let sx = caps["sx"].parse().ok()?;
            let sy = caps["sy"].parse().ok()?;
            let bx = caps["bx"].parse().ok()?;
            let by = caps["by"].parse().ok()?;

            Some(((sx, sy), (bx, by)))
        })
        .collect()
}

fn check_line(input: &[(Coord, Coord)], y: i64) -> usize {
    let beacons: HashSet<Coord> = input.iter().map(|&(_, beacon)| beacon).collect();
    let sensor_ranges: Vec<(Coord, i64)> = input
        .iter()
        .map(|&(sensor, beacon)| {
            let effective_range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            (sensor, effective_range)
        })
        .collect();

    let (min_x, max_x) = input
        .iter()
        .flat_map(|&((sx, _), (bx, _))| [sx, bx].into_iter())
        .chain(
            sensor_ranges
                .iter()
                .flat_map(|&(sensor, range)| [(sensor.0 - range), sensor.0 + range].into_iter()),
        )
        .minmax()
        .into_option()
        .unwrap();

    (min_x..=max_x)
        .into_par_iter()
        .filter(|&x| {
            !beacons.contains(&(x, y)) && {
                sensor_ranges.iter().any(|&(coord, range)| {
                    let dist = (coord.0 - x).abs() + (coord.1 - y).abs();
                    dist <= range
                })
            }
        })
        .count()
}

fn find_distress_beacon(input: &[(Coord, Coord)], search_area: i64) -> Option<i64> {
    let sensor_ranges: Vec<(Coord, i64)> = input
        .iter()
        .map(|&(sensor, beacon)| {
            let effective_range = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
            (sensor, effective_range)
        })
        .collect();

    (0..=search_area)
        .into_par_iter()
        .find_map_any(|y| {
            let mut x = 0;
            'outer: loop {
                for &(coord, range) in &sensor_ranges {
                    let dist = (coord.0 - x).abs() + (coord.1 - y).abs();
                    if dist <= range {
                        x = coord.0 + (range - (coord.1 - y).abs()) + 1;
                        continue 'outer;
                    }
                }
                break;
            }
            (x <= search_area).then_some((x, y))
        })
        .map(|(x, y)| x * 4_000_000 + y)
}

#[aoc(day15, part1)]
fn day15_part1(input: &[(Coord, Coord)]) -> usize {
    check_line(input, 2_000_000)
}
#[aoc(day15, part2)]
fn day15_part2(input: &[(Coord, Coord)]) -> Option<i64> {
    find_distress_beacon(input, 4_000_000)
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn day15_gen() {
        assert_eq!(
            &[
                ((2, 18), (-2, 15)),
                ((9, 16), (10, 16)),
                ((13, 2), (15, 3)),
                ((12, 14), (10, 16)),
                ((10, 20), (10, 16)),
                ((14, 17), (10, 16)),
                ((8, 7), (2, 10)),
                ((2, 0), (2, 10)),
                ((0, 11), (2, 10)),
                ((20, 14), (25, 17)),
                ((17, 20), (21, 22)),
                ((16, 7), (15, 3)),
                ((14, 3), (15, 3)),
                ((20, 1), (15, 3))
            ][..],
            &super::day15(EXAMPLE)
        );
    }

    #[test]
    fn day15_part1() {
        assert_eq!(26, super::check_line(&super::day15(EXAMPLE), 10));
    }

    #[test]
    fn day15_part2() {
        assert_eq!(
            Some(56000011),
            super::find_distress_beacon(&super::day15(EXAMPLE), 20)
        );
    }
}
