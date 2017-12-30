use std::io::stdin;
use std::io::BufRead;

fn main() {
    let input = stdin();
    let step: usize = input.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();

    let mut cur_pos = 0;
    let mut pos_1 = 0;
    for i in 0..50_000_000 {
        cur_pos = ((cur_pos + step) % (i+1)) + 1;
        if cur_pos == 1 {
            pos_1 = i+1;
        }
    }
    println!("{}", pos_1);
}
