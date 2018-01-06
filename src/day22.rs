#![feature(entry_and_modify)]

use std::io::stdin;
use std::io::BufRead;

use std::collections::BTreeMap;

fn turn_right(vector: &mut (isize, isize)) {
    match &*vector {
        &(-1, _) => *vector = (0, -1),
        &(1, _) => *vector = (0, 1),
        &(_, 1) => *vector = (-1, 0),
        &(_, -1) => *vector = (1, 0),
        &(_, _) => unreachable!()
    }
}

fn turn_left(vector: &mut (isize, isize)) {
    match &*vector {
        &(-1, _) => *vector = (0, 1),
        &(1, _) => *vector = (0, -1),
        &(_, 1) => *vector = (1, 0),
        &(_, -1) => *vector = (-1, 0),
        &(_, _) => unreachable!()
    }
}

fn main() {
    let input = stdin();
    let raw_map: Vec<String> = input.lock().lines().map(|l| l.unwrap()).collect();
    let center_y_offset = (raw_map.len() / 2) as isize;
    let center_x_offset = (raw_map[0].len() / 2) as isize;

    let mut map: BTreeMap<(isize, isize), bool> = raw_map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| {
                    (
                        x as isize - center_x_offset, 
                        y as isize - center_y_offset, 
                        c
                    )
                })
        }).filter(|&(_, _, c)| c == '#')
        .map(|(x, y, c)| ((x, y), c == '#'))
        .collect();

    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut vector = (0, -1); // up
    let mut infection_caused = 0;
    for _ in 0..10_000 {
        //println!("{:?}", map);
        map.entry((pos_x, pos_y)).and_modify(|v| {
            if *v {
                turn_right(&mut vector);
            } else {
                infection_caused += 1;
                turn_left(&mut vector);
            }
            *v = !*v;
        }).or_insert_with(|| {
            infection_caused += 1;
            turn_left(&mut vector);
            true
        });
        pos_x += vector.0;
        pos_y += vector.1;
    }
    println!("infection caused: {}", infection_caused); 
}
