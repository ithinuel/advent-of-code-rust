use itertools::Itertools;
use std::collections::BTreeSet;
use std::io::BufRead;

fn main() {
    let mut universe: BTreeSet<(isize, isize, isize, isize)> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .enumerate()
        .flat_map(|(y, line)| {
            line.into_bytes()
                .into_iter()
                .enumerate()
                .filter(|&(_, c)| c == b'#')
                .map(move |(x, _)| (0isize, 0isize, y as isize, x as isize))
        })
        .collect();

    let cube = (-1isize..2)
        .cartesian_product(
            (-1isize..2).cartesian_product((-1isize..2).cartesian_product(-1isize..2)),
        )
        .map(|(dw, (dz, (dy, dx)))| (dw, dz, dy, dx));

    for _ in 0..6 {
        // for each active cube
        universe = universe
            .iter()
            // and their neighboors
            .flat_map(|(w, z, y, x)| {
                cube.clone()
                    .map(move |(dw, dz, dy, dx)| (w - dw, z + dz, y + dy, x + dx))
            })
            .unique() // has an hashset inside 🤷
            .filter(|&(w, z, y, x)| {
                let is_active = universe.contains(&(w, z, y, x));

                // count active neighboors
                let count = cube
                    .clone()
                    .filter(|(dw, dz, dy, dx)| !(dw == &0 && dz == &0 && dy == &0 && dx == &0))
                    .map(move |(dw, dz, dy, dx)| (w - dw, z + dz, y + dy, x + dx))
                    .filter(|coords| universe.contains(coords))
                    .count();

                // add to new_universe if needs to activate
                (is_active && (count == 2 || count == 3)) || (!is_active && count == 3)
            })
            .collect();
    }

    println!("active {}", universe.len());
}
