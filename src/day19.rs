use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::*;
use itertools::Itertools;

use crate::day19::matrices::rotations;

mod matrices;
type Report = Vec<(i32, i32, i32)>;

#[aoc_generator(day19)]
fn gen(input: &str) -> Vec<Report> {
    input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .skip(1)
                .filter_map(|line| {
                    line.split(',')
                        .filter_map(|n| n.parse().ok())
                        .collect_tuple()
                })
                .collect()
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Object {
    Beacon(usize, usize),
    Scanner(usize),
}
impl Object {
    fn is_beacon(&self) -> bool {
        matches!(self, Object::Beacon(_, _))
    }
    fn is_scanner(&self) -> bool {
        matches!(self, Object::Scanner(_))
    }
}

#[aoc(day19, part1)]
fn part1(input: &[Report]) -> usize {
    let mut unmapped_reports: BTreeSet<_> = (1..input.len()).collect();
    let mut map: BTreeMap<matrices::Coord, Object> = input[0]
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, coord)| (coord, Object::Beacon(0, idx)))
        .collect();
    map.insert((0, 0, 0), Object::Scanner(0));

    let mut rotated: BTreeMap<_, _> = unmapped_reports
        .iter()
        .map(|&scan_id| {
            (
                scan_id,
                rotations()
                    .map(|r| input[scan_id].iter().cloned().map(r).collect_vec())
                    .collect_vec(),
            )
        })
        .collect();

    while !unmapped_reports.is_empty() {
        // find a report with a rotation that matches at least 12 points in the current map.
        if let Some((
            reference,
            scanner_id,
            scanner_coord,
            target_beacon_id,
            rotation_id,
            matched_image,
        )) = unmapped_reports
            .iter()
            .cloned()
            .cartesian_product(0..24)
            .cartesian_product(
                map.iter()
                    .filter_map(|(&coord, &obj)| obj.is_beacon().then(|| coord)),
            )
            .flat_map(|((scan_id, rot_id), reference_beacon)| {
                let rotated = &rotated;
                rotated[&scan_id][rot_id].iter().enumerate().map(
                    move |(target_beacon_id, relative_target_beacon)| {
                        // if target_beacon_id in scand_id's report match the reference_beacon
                        // coordinates, then scan_id's loc is:
                        let scanner_absolute = (
                            reference_beacon.0 - relative_target_beacon.0,
                            reference_beacon.1 - relative_target_beacon.1,
                            reference_beacon.2 - relative_target_beacon.2,
                        );

                        // compute where would scand_id's
                        (
                            reference_beacon,
                            scan_id,
                            scanner_absolute,
                            target_beacon_id,
                            rot_id,
                            rotated[&scan_id][rot_id]
                                .iter()
                                .cloned()
                                .map(move |(x, y, z)| {
                                    (
                                        scanner_absolute.0 + x,
                                        scanner_absolute.1 + y,
                                        scanner_absolute.2 + z,
                                    )
                                }),
                        )
                    },
                )
            })
            // those things could probably be cached
            .find(|(_, _, _, _, _, report)| {
                report.clone().filter(|a| map.keys().contains(a)).count() >= 12
            })
        {
            // expand map with newly matched beacons & scanner
            println!(
                "we found one: {} using {} on {:?} with rotation {}",
                scanner_id, target_beacon_id, reference, rotation_id
            );
            map.extend(
                matched_image
                    .into_iter()
                    .enumerate()
                    .map(|(idx, coord)| (coord, Object::Beacon(scanner_id, idx)))
                    .chain(std::iter::once((
                        scanner_coord,
                        Object::Scanner(scanner_id),
                    ))),
            );
            unmapped_reports.remove(&scanner_id);
            rotated.remove(&scanner_id);
        } else {
            println!("We couldn't find a match for the remaining reports:");
            println!("{:?}", unmapped_reports);
            println!();
            println!("The current maps is: {:?}", map);
            //panic!("Something went teribly wrong");
            break;
        }
    }

    println!(
        "map: {:?}",
        map.iter().filter(|(_, obj)| obj.is_scanner()).collect_vec()
    );

    map.into_iter().filter(|(_, obj)| obj.is_beacon()).count()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = include_str!("day19/example.txt");
    const ROTATION_EXAMPLE: &str = include_str!("day19/rotation_example.txt");

    #[test]
    fn gen() {
        let res = super::gen(EXAMPLE);
        assert_eq!(5, res.len());
        [
            (404, -588, -901),
            (528, -643, 409),
            (-838, 591, 734),
            (390, -675, -793),
        ]
        .iter()
        .for_each(|coords| assert!(res[0].contains(coords)));
    }

    #[test]
    fn rotations() {
        let expect = super::gen(ROTATION_EXAMPLE);
        let first = expect.last().unwrap();

        let res: Vec<_> = super::matrices::rotations()
            .map(|r| first.iter().cloned().map(r).collect())
            .inspect(|v| println!("{:2?}", v))
            .collect();

        expect
            .iter()
            .enumerate()
            .for_each(|(idx, v)| assert!(res.contains(v), "Missing {} from results: {:?}", idx, v));
    }

    #[test]
    fn part1() {
        assert_eq!(79, super::part1(&super::gen(EXAMPLE)));
    }
}
