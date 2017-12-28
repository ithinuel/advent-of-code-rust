use std::io::stdin;
use std::io::Read;

mod knothash;
use knothash::KnotHash;


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
}
