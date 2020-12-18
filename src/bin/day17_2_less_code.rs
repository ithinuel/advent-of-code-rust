use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut universe: HashSet<(isize, isize, isize, isize)> = std::io::stdin()
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
        let mut active_neighboors_count = HashMap::new();
        universe
            .iter()
            // and their neighboors
            .flat_map(|(w, z, y, x)| {
                cube.clone()
                    .filter(|&(dw, dz, dy, dx)| !(dw == 0 && dz == 0 && dy == 0 && dx == 0))
                    .map(move |(dw, dz, dy, dx)| (w + dw, z + dz, y + dy, x + dx))
            })
            .for_each(|pos| {
                *active_neighboors_count.entry(pos).or_insert(0) += 1;
            });

        println!("{:?}", active_neighboors_count);

        universe = active_neighboors_count
            .into_iter()
            .filter(|&((w, z, y, x), count)| {
                let is_active = universe.contains(&(w, z, y, x));

                // add to new_universe if needs to activate
                (is_active && (count == 2 || count == 3)) || (!is_active && count == 3)
            })
            .map(|(pos, _)| pos)
            .collect();
    }

    println!("active {}", universe.len());
}
