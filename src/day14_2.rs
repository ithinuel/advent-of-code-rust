extern crate num;

use std::io::stdin;
use std::io::Read;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use num::integer::Integer;

mod knothash;
use knothash::KnotHash;

#[derive(Copy, Clone)]
enum State {
    Empty,
    Group(u32)
}

fn main() {
    let mut key = String::new();
    stdin().read_to_string(&mut key);
    let out: String = (0..128).map(|i| {
        let mut key = key.trim().to_string();
        key.push_str(&format!("-{}", i));
        let mut seed: Vec<usize> = key.as_bytes()
            .iter()
            .map(|b| *b as usize)
            .collect();

        seed.extend_from_slice(&[17, 31, 73, 47, 23]);
        let mut kh = KnotHash::new(&seed, 256);
        for _ in 0..64 {
            kh.round();
        }
        kh.get_dense_hash_as_vec()
            .iter()
            .map(|n| format!("{:08b}", n))
            .collect::<String>()
    }).collect();
    println!("{}", out.chars().filter(|c| c == &'1').count());

    let mut occupied: BTreeSet<(usize, usize)> = out.chars()
        .enumerate()
        .filter(|&(i, c)| c == '1')
        .map(|(i, c)| i.div_rem(&128))
        .collect();
    let mut tovisit = VecDeque::new();
    let mut visited = BTreeSet::new();
    
    let mut map = vec![State::Empty; 128*128];
    let range = 0..128;
    let mut group = 0;

    while !occupied.is_empty() {
        group += 1;
        {
            let first = *occupied.iter().next().unwrap();
            tovisit.push_back(first);
            occupied.remove(&first);
        }

        while let Some(coords) = tovisit.pop_front() {
            visited.insert(coords);

            map[coords.0 + coords.1 * 128] = State::Group(group);

            let mut neighboors = Vec::with_capacity(4);
            if coords.1 > 0 {
                neighboors.push((coords.0, coords.1 - 1));
            }
            if coords.1 < 127 {
                neighboors.push((coords.0, coords.1 + 1));
            }
            if coords.0 > 0 {
                neighboors.push((coords.0 - 1, coords.1));
            }
            if coords.0 < 127 {
                neighboors.push((coords.0 + 1, coords.1));
            }

            for n in neighboors.iter() {
                if occupied.contains(n) {
                    occupied.remove(n);
                    tovisit.push_back(*n);
                }
            }
        }
    }
    println!("{}", group);
}
