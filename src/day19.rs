use std::collections::{BTreeMap, BTreeSet};

use aoc_runner_derive::*;
use itertools::Itertools;

use crate::day19::matrices::rotations;

use self::matrices::Coord;

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

fn transform_images_relative_to(
    reference_beacon: Coord,
    keys: impl Iterator<Item = usize>,
    rotated: &BTreeMap<usize, Vec<Vec<Coord>>>,
) -> BTreeMap<(usize, Coord, usize, usize), (Coord, Vec<Coord>)> {
    keys.flat_map(|scan_id| {
        (0..24).flat_map(move |rot_id| {
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
                        (scan_id, reference_beacon, target_beacon_id, rot_id),
                        (
                            scanner_absolute,
                            rotated[&scan_id][rot_id]
                                .iter()
                                .cloned()
                                .map(move |(x, y, z)| {
                                    (
                                        scanner_absolute.0 + x,
                                        scanner_absolute.1 + y,
                                        scanner_absolute.2 + z,
                                    )
                                })
                                .collect_vec(),
                        ),
                    )
                },
            )
        })
    })
    .collect()
}

fn rebuild_map(input: &[Report]) -> BTreeMap<Coord, Object> {
    let mut unmapped_reports: BTreeSet<_> = (1..input.len()).collect();
    let mut map: BTreeMap<matrices::Coord, Object> = input[0]
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, coord)| (coord, Object::Beacon(0, idx)))
        .collect();
    map.insert((0, 0, 0), Object::Scanner(0));

    let rotated: BTreeMap<_, _> = unmapped_reports
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

    let mut relative: BTreeMap<_, _> = map
        .iter()
        .filter_map(|(&coord, &obj)| obj.is_beacon().then(|| coord))
        .flat_map(|reference_beacon| {
            transform_images_relative_to(
                reference_beacon,
                unmapped_reports.iter().cloned(),
                &rotated,
            )
            .into_iter()
        })
        .collect();

    while !unmapped_reports.is_empty() {
        // find a report with a rotation that matches at least 12 points in the current map.
        if let Some((
            &(scanner_id, reference, target_beacon, rotation_id),
            (scanner_coord, matched_image),
        )) = relative
            .iter()
            .find(|(_, (_, report))| report.iter().filter(|a| map.keys().contains(a)).count() >= 12)
        {
            let scanner_coord = *scanner_coord;
            println!(
                "we found: {} to be from {:?} using {:?} on beacon {}:{} and rotation {}",
                scanner_id, scanner_coord, reference, scanner_id, target_beacon, rotation_id
            );
            map.extend(
                matched_image
                    .iter()
                    .cloned()
                    .enumerate()
                    .map(|(idx, coord)| (coord, Object::Beacon(scanner_id, idx)))
                    .chain(std::iter::once((
                        scanner_coord,
                        Object::Scanner(scanner_id),
                    ))),
            );
            unmapped_reports.remove(&scanner_id);

            let matched_image: Vec<Coord> = matched_image.clone();
            let to_remove = relative
                .keys()
                .cloned()
                .filter(|&(id, _, _, _)| id == scanner_id)
                .collect_vec();
            to_remove.into_iter().for_each(|key| {
                relative.remove(&key);
            });
            relative.extend(matched_image.into_iter().flat_map(|coord| {
                transform_images_relative_to(coord, unmapped_reports.iter().cloned(), &rotated)
                    .into_iter()
            }));
        } else {
            println!("We couldn't find a match for the remaining reports:");
            println!("{:?}", unmapped_reports);
            println!();
            println!("The current maps is: {:?}", map);
            panic!("Something went teribly wrong");
            //break;
        }
    }
    map
}

#[aoc(day19, part1)]
fn part1(input: &[Report]) -> usize {
    rebuild_map(input)
        .into_iter()
        .filter(|(_, obj)| obj.is_beacon())
        .count()
}

#[aoc(day19, part2)]
fn part2(input: &[Report]) -> Option<i32> {
    let scanners = rebuild_map(input)
        .into_iter()
        .filter_map(|(key, obj)| obj.is_scanner().then(|| key))
        .collect_vec();
    scanners
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs())
        .max()
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
    #[test]
    fn part2() {
        assert_eq!(Some(3621), super::part2(&super::gen(EXAMPLE)));
    }
}
