use std::io::stdin;
use std::vec::Vec;

mod knothash;

use knothash::KnotHash;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    //let v = Vec::with_capacity(bytes.len() + 5);
    //v.extend_from_slice(bytes);
    //v.extend_from_slice(&[17, 31, 73, 47, 23]);
    let len: usize = input.trim().parse().unwrap();
    input.clear();

    stdin().read_line(&mut input).unwrap();
    let mut sizes: Vec<usize> = input
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as usize)
        .collect();

    sizes.extend_from_slice(&[17, 31, 73, 47, 23]);

    println!("lengths: {:?}", sizes);

    let mut kh = KnotHash::new(&sizes, len);
    for _ in 0..64 {
        kh.round();
        //println!("{:?}", kh);
    }
    println!("{}", kh);
}
