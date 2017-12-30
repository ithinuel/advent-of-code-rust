use std::io::stdin;
use std::io::BufRead;

fn main() {
    let input = stdin();
    let step: usize = input.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    let mut vec = vec![0];
    let mut cur_pos = 0;
    for i in 0..2017 {
        cur_pos = ((cur_pos + step) % vec.len()) + 1;
        vec.insert(cur_pos, i+1);
    }
    //println!("{} {:?}", cur_pos, vec);
    println!("{}", vec[(cur_pos+1)%vec.len()]);
}
