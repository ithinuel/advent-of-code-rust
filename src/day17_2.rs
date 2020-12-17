use itertools::Itertools;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::io::BufRead;
use std::iter::once;

fn main() {
    let mut universe: BTreeMap<_, BTreeMap<_, BTreeMap<_, BTreeSet<_>>>> = once((
        0,
        once((
            0,
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
        .collect(),
    ))
    .collect();

    let cube =
        (-1..2).cartesian_product((-1..2).cartesian_product((-1..2).cartesian_product(-1..2)));

    for _ in 0..6 {
        let mut new_universe = BTreeMap::new();

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
                            .get(&w)
                            .and_then(|vol3d| vol3d.get(&z))
                            .and_then(|plane| plane.get(&y))
                            .map(|line| line.contains(&x))
                            .unwrap_or(false)
                    })
                    .count();

                // add to new_universe if needs to activate
                if is_active && (count == 2 || count == 3) {
                    new_universe
                        .entry(w)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(z)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(y)
                        .or_insert_with(|| BTreeSet::new())
                        .insert(x);
                } else if !is_active && count == 3 {
                    new_universe
                        .entry(w)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(z)
                        .or_insert_with(|| BTreeMap::new())
                        .entry(y)
                        .or_insert_with(|| BTreeSet::new())
                        .insert(x);
                }
            });

        universe = new_universe;
    }

    let active: usize = universe
        .values()
        .flat_map(|vol3d| vol3d.values())
        .flat_map(|plane| plane.values())
        .map(|line| line.len())
        .sum();

    println!("active {}", active);
}
