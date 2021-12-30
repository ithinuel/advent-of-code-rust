use std::collections::{BTreeMap, BTreeSet};
use std::iter::once;

use aoc_runner_derive::*;
use itertools::Itertools;

type Universe3D = BTreeMap<isize, BTreeMap<isize, BTreeSet<isize>>>;

#[aoc_generator(day17, part1)]
fn gen_part1(input: &str) -> Universe3D {
    once((
        0isize,
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                (
                    y as isize,
                    line.chars()
                        .enumerate()
                        .filter(|&(_, c)| c == '#')
                        .map(|(x, _)| x as isize)
                        .collect(),
                )
            })
            .collect(),
    ))
    .collect()
}

#[aoc(day17, part1)]
fn part1(universe: &Universe3D) -> usize {
    let mut universe = universe.clone();
    for _ in 0..6 {
        let (zmin, zmax) = universe
            .keys()
            .copied()
            .minmax()
            .into_option()
            .expect("z min-max");
        let (ymin, ymax) = universe
            .values()
            .flat_map(|plane| plane.keys())
            .copied()
            .minmax()
            .into_option()
            .expect("y min-max");
        let (xmin, xmax) = universe
            .values()
            .flat_map(|plane| plane.values())
            .flat_map(|line| line.iter())
            .copied()
            .minmax()
            .into_option()
            .expect("y min-max");

        let mut new_universe = BTreeMap::new();

        (zmin - 1..=zmax + 1)
            .cartesian_product((ymin - 1..=ymax + 1).cartesian_product(xmin - 1..=xmax + 1))
            .for_each(|(z, (y, x))| {
                let is_active = universe
                    .get(&z)
                    .and_then(|plane| plane.get(&y))
                    .map(|line| line.contains(&x))
                    .unwrap_or(false);

                // check neighboors
                let ref_universe = &universe;
                let count = (z - 1..z + 2)
                    .cartesian_product((y - 1..y + 2).cartesian_product(x - 1..x + 2))
                    .filter(move |&(nz, (ny, nx))| !(nz == z && ny == y && nx == x))
                    .map(move |(z, (y, x))| {
                        ref_universe
                            .get(&z)
                            .and_then(|plane| plane.get(&y))
                            .map(|line| line.contains(&x))
                            .unwrap_or(false)
                    })
                    .filter(|&is_active| is_active)
                    .count();

                if (is_active && count == 2) || count == 3 {
                    new_universe
                        .entry(z)
                        .or_insert_with(BTreeMap::new)
                        .entry(y)
                        .or_insert_with(BTreeSet::new)
                        .insert(x);
                }
            });

        universe = new_universe;
        //println!("After {} cycle", cycle + 1);
        //print_universe(&universe);
    }

    universe
        .values()
        .flat_map(|plane| plane.values())
        .map(|line| line.len())
        .sum()
}

//#[aoc(day17, part2)]
fn _part2(input: &str) -> usize {
    type Universe4D = BTreeMap<isize, BTreeMap<isize, BTreeMap<isize, BTreeSet<isize>>>>;
    let mut universe: Universe4D = once((0isize, gen_part1(input))).collect();
    let cube =
        (-1..2).cartesian_product((-1..2).cartesian_product((-1..2).cartesian_product(-1..2)));

    for _ in 0..6 {
        let mut new_universe = Universe4D::new();

        universe
            .iter()
            // for each active cube
            .flat_map(move |(&w, vol3d)| {
                vol3d.iter().flat_map(move |(&z, plane)| {
                    plane
                        .iter()
                        .flat_map(move |(&y, line)| line.iter().map(move |&x| (w, z, y, x)))
                })
            })
            // and their neighboors
            .flat_map(|(w, z, y, x)| {
                cube.clone()
                    .map(move |(dw, (dz, (dy, dx)))| (w - dw, z + dz, y + dy, x + dx))
            })
            // check them only once
            .unique()
            // process
            .for_each(|(w, z, y, x)| {
                let is_active = universe
                    .get(&w)
                    .and_then(|vol3d| vol3d.get(&z))
                    .and_then(|plane| plane.get(&y))
                    .map(|line| line.contains(&x))
                    .unwrap_or(false);

                // count active neighboors
                let count = cube
                    .clone()
                    .filter(|(dw, (dz, (dy, dx)))| !(dw == &0 && dz == &0 && dy == &0 && dx == &0))
                    .map(move |(dw, (dz, (dy, dx)))| (w - dw, z + dz, y + dy, x + dx))
                    .filter(|(w, z, y, x)| {
                        universe
                            .get(w)
                            .and_then(|vol3d| vol3d.get(z))
                            .and_then(|plane| plane.get(y))
                            .map(|line| line.contains(x))
                            .unwrap_or(false)
                    })
                    .count();

                // add to new_universe if needs to activate
                if (is_active && count == 2) || count == 3 {
                    new_universe
                        .entry(w)
                        .or_insert_with(BTreeMap::new)
                        .entry(z)
                        .or_insert_with(BTreeMap::new)
                        .entry(y)
                        .or_insert_with(BTreeSet::new)
                        .insert(x);
                }
            });

        universe = new_universe;
    }

    universe
        .values()
        .flat_map(|vol3d| vol3d.values())
        .flat_map(|plane| plane.values())
        .map(|line| line.len())
        .sum()
}

type Universe4DAlt1 = std::collections::HashSet<(isize, isize, isize, isize)>;
#[aoc_generator(day17, part2, alt1)]
fn gen2_alt1(input: &str) -> Universe4DAlt1 {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.to_string()
                .into_bytes()
                .into_iter()
                .enumerate()
                .filter(|&(_, c)| c == b'#')
                .map(move |(x, _)| (0isize, 0isize, y as isize, x as isize))
        })
        .collect()
}

#[aoc(day17, part2, alt1)]
fn part2_alt1(ref_universe: &Universe4DAlt1) -> usize {
    use itertools::Itertools;

    let mut universe = ref_universe.clone();

    let cube: Vec<_> = (-1isize..2)
        .cartesian_product(
            (-1isize..2).cartesian_product((-1isize..2).cartesian_product(-1isize..2)),
        )
        .map(|(dw, (dz, (dy, dx)))| (dw, dz, dy, dx))
        .filter(|&(dw, dz, dy, dx)| !(dw == 0 && dz == 0 && dy == 0 && dx == 0))
        .collect();

    for _ in 0..6 {
        // for each active cube
        let mut active_neighboors_count = std::collections::BTreeMap::new();
        cube.iter()
            .flat_map(|&(dw, dz, dy, dx)| {
                universe
                    .iter()
                    // and their neighboors
                    .map(move |&(w, z, y, x)| (w + dw, z + dz, y + dy, x + dx))
            })
            .for_each(|pos| {
                *active_neighboors_count.entry(pos).or_insert(0) += 1;
            });

        universe = active_neighboors_count
            .into_iter()
            .filter(|&(ref coordinates, count)| {
                let is_active = universe.contains(coordinates);

                // add to new_universe if needs to activate
                (is_active && count == 2) || count == 3
            })
            .map(|(pos, _)| pos)
            .collect();
    }
    universe.len()
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    const EXAMPLE: &str = r".#.
..#
###";

    const EXAMPLE_AS_ARRAY: &[(isize, &[isize])] = &[(0, &[1]), (1, &[2]), (2, &[0, 1, 2])];
    const EXAMPLE_AS_TUPLE_ARRAY: &[(isize, isize, isize, isize)] = &[
        (0, 0, 0, 1),
        (0, 0, 1, 2),
        (0, 0, 2, 0),
        (0, 0, 2, 1),
        (0, 0, 2, 2),
    ];

    lazy_static! {
        static ref EXAMPLE_AS_3DMAP: super::Universe3D = std::iter::once((
            0isize,
            EXAMPLE_AS_ARRAY
                .iter()
                .map(|(y, v)| { (*y, v.iter().copied().collect()) })
                .collect(),
        ))
        .collect();
        static ref EXAMPLE_AS_4DMAP: super::Universe4DAlt1 =
            EXAMPLE_AS_TUPLE_ARRAY.iter().copied().collect();
    }

    #[test]
    fn gen_part1() {
        assert_eq!(*EXAMPLE_AS_3DMAP, super::gen_part1(EXAMPLE));
    }

    #[test]
    fn part1() {
        assert_eq!(112, super::part1(&EXAMPLE_AS_3DMAP));
    }

    #[test]
    fn _part2() {
        assert_eq!(848, super::_part2(EXAMPLE));
    }

    #[test]
    fn gen2_alt1() {
        assert_eq!(*EXAMPLE_AS_4DMAP, super::gen2_alt1(EXAMPLE));
    }

    #[test]
    fn part2_alt1() {
        assert_eq!(848, super::part2_alt1(&EXAMPLE_AS_4DMAP));
    }
}
