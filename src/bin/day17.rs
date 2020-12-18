use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufRead;
use std::iter::once;

fn neighboors<'a>(
    universe: &'a BTreeMap<isize, BTreeMap<isize, BTreeSet<isize>>>,
    x: isize,
    y: isize,
    z: isize,
) -> impl Iterator<Item = bool> + 'a {
    (z - 1..z + 2)
        .cartesian_product((y - 1..y + 2).cartesian_product(x - 1..x + 2))
        .filter(move |&(nz, (ny, nx))| !(nz == z && ny == y && nx == x))
        .map(move |(z, (y, x))| {
            universe
                .get(&z)
                .and_then(|plane| plane.get(&y))
                .map(|line| line.contains(&x))
                .unwrap_or(false)
        })
}

fn print_universe(universe: &BTreeMap<isize, BTreeMap<isize, BTreeSet<isize>>>) {
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

    let mut old_y = ymin;
    let mut old_z = zmin;
    println!("z={}", old_z);
    (zmin..=zmax)
        .cartesian_product((ymin..=ymax).cartesian_product(xmin..=xmax))
        .for_each(|(z, (y, x))| {
            let is_active = universe
                .get(&z)
                .and_then(|plane| plane.get(&y))
                .map(|line| line.contains(&x))
                .unwrap_or(false);

            if old_z != z {
                println!("\n\nz={}", z);
            } else if old_y != y {
                println!();
            }

            if is_active {
                print!("#")
            } else {
                print!(".")
            }

            old_y = y;
            old_z = z;
        });
    println!("\n");
}

fn main() {
    let mut universe: BTreeMap<_, BTreeMap<_, BTreeSet<_>>> = once((
        0isize,
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
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
    .collect();

    print_universe(&universe);
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
                //
                // check neighboors
                let count = neighboors(&universe, x, y, z)
                    .filter(|&is_active| is_active)
                    .count();

                if is_active && (count == 2 || count == 3) {
                    new_universe
                        .entry(z)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(y)
                        .or_insert_with(|| BTreeSet::new())
                        .insert(x);
                } else if !is_active && count == 3 {
                    new_universe
                        .entry(z)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(y)
                        .or_insert_with(|| BTreeSet::new())
                        .insert(x);
                }
            });

        universe = new_universe;
        //println!("After {} cycle", cycle + 1);
        //print_universe(&universe);
    }

    let active: usize = universe
        .values()
        .flat_map(|plane| plane.values())
        .map(|line| line.len())
        .sum();

    println!("active {}", active);
}
