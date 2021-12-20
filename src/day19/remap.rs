use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub type Coord = (i32, i32, i32);
pub type Report = Vec<(i32, i32, i32)>;

fn rotations() -> impl Iterator<Item = fn(Coord) -> Coord> {
    // x increases is ahead
    [
        // front facing, rotate around x
        |(x, y, z): Coord| (x, y, z),
        |(x, y, z): Coord| (x, z, -y),
        |(x, y, z): Coord| (x, -y, -z),
        |(x, y, z): Coord| (x, -z, y),
        // looking one side, rotate around y
        |(x, y, z): Coord| (y, -x, z),
        |(x, y, z): Coord| (-z, -x, y),
        |(x, y, z): Coord| (-y, -x, -z),
        |(x, y, z): Coord| (z, -x, -y),
        // looking back, rotate around x
        |(x, y, z): Coord| (-x, -y, z),
        |(x, y, z): Coord| (-x, z, y),
        |(x, y, z): Coord| (-x, y, -z),
        |(x, y, z): Coord| (-x, -z, -y),
        // looking the other side, rotate around y
        |(x, y, z): Coord| (-y, x, z),
        |(x, y, z): Coord| (-z, x, -y),
        |(x, y, z): Coord| (y, x, -z),
        |(x, y, z): Coord| (z, x, y),
        // looking up, rotate around z
        |(x, y, z): Coord| (-z, y, x),
        |(x, y, z): Coord| (y, z, x),
        |(x, y, z): Coord| (z, -y, x),
        |(x, y, z): Coord| (-y, -z, x),
        // looking down, rorate around z
        |(x, y, z): Coord| (z, y, -x),
        |(x, y, z): Coord| (y, -z, -x),
        |(x, y, z): Coord| (-y, z, -x),
        |(x, y, z): Coord| (-z, -y, -x),
    ]
    .into_iter()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Object {
    Beacon(usize, usize),
    Scanner(usize),
}
impl Object {
    pub fn is_beacon(&self) -> bool {
        matches!(self, Object::Beacon(_, _))
    }
    pub fn is_scanner(&self) -> bool {
        matches!(self, Object::Scanner(_))
    }
}

fn transform_images_relative_to<'a>(
    reference_beacon: Coord,
    keys: impl Iterator<Item = usize> + 'a,
    rotated: &'a HashMap<usize, Vec<Vec<Coord>>>,
) -> impl Iterator<Item = ((usize, Coord, usize, usize), (Coord, Vec<Coord>))> + 'a {
    keys.cartesian_product(0..24)
        .flat_map(move |(scan_id, rot_id)| {
            rotated[&scan_id][rot_id]
                .iter()
                .enumerate()
                .map(move |(beacon_id, beacon_coord)| (scan_id, rot_id, beacon_id, beacon_coord))
        })
        .map(move |(scan_id, rot_id, beacon_rel_id, beacon_rel_coord)| {
            // if beacon_rel_id in scan_id's report match the reference_beacon
            // coordinates, then scan_id's location is:
            let scanner_absolute = (
                reference_beacon.0 - beacon_rel_coord.0,
                reference_beacon.1 - beacon_rel_coord.1,
                reference_beacon.2 - beacon_rel_coord.2,
            );

            // what would the report look like in absolute coord space
            (
                (scan_id, reference_beacon, beacon_rel_id, rot_id),
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
        })
}

pub fn rebuild_map(input: &[Report]) -> HashMap<Coord, Object> {
    let mut unmapped_reports: HashSet<_> = (1..input.len()).collect();
    let mut map: HashMap<Coord, Object> = input[0]
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, coord)| (coord, Object::Beacon(0, idx)))
        .collect();
    map.insert((0, 0, 0), Object::Scanner(0));

    let rotated: HashMap<_, _> = unmapped_reports
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

    let mut relative: HashMap<_, _> = map
        .iter()
        .filter_map(|(&coord, &obj)| obj.is_beacon().then(|| coord))
        .flat_map(|reference_beacon| {
            transform_images_relative_to(
                reference_beacon,
                unmapped_reports.iter().cloned(),
                &rotated,
            )
        })
        .collect();

    while !unmapped_reports.is_empty() {
        // find a report with a rotation that matches at least 12 points in the current map.
        if let Some((
            &(scanner_id, reference, target_beacon, rotation_id),
            (scanner_coord, matched_image),
        )) = relative
            .iter()
            .find(|(_, (_, report))| report.iter().filter(|a| map.contains_key(a)).count() >= 12)
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
            }));
        } else {
            println!("We couldn't find a match for the remaining reports:");
            println!("{:?}", unmapped_reports);
            println!();
            println!("The current maps is: {:?}", map);
            // So weird... using panic instead of break doubles the execution time even if this
            // path's never taken Oo
            //unreachable!("Something went teribly wrong");
            break;
        }
    }
    map
}

#[cfg(test)]
mod test {
    const ROTATION_EXAMPLE: &str = include_str!("rotation_example.txt");

    #[test]
    fn rotations() {
        let expect = super::super::gen(ROTATION_EXAMPLE);
        let first = expect.last().unwrap();

        let res: Vec<_> = super::rotations()
            .map(|r| first.iter().cloned().map(r).collect())
            .inspect(|v| println!("{:2?}", v))
            .collect();

        expect
            .iter()
            .enumerate()
            .for_each(|(idx, v)| assert!(res.contains(v), "Missing {} from results: {:?}", idx, v));
    }
}
