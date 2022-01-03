use std::io::stdin;
use std::io::BufRead;

struct Generator {
    factor: u64,
    value: u64,
}

impl Iterator for Generator {
    // we will be counting with usize
    type Item = u64;

    // next() is the only required method
    fn next(&mut self) -> Option<u64> {
        self.value *= self.factor;
        self.value %= 2147483647;
        Some(self.value)
    }
}

fn main() {
    let input = stdin();
    let seeds: Vec<u64> = input
        .lock()
        .lines()
        .map(|l| l.unwrap().split(' ').last().unwrap().parse().unwrap())
        .collect();

    let mut a = Generator {
        factor: 16807,
        value: seeds[0],
    };
    let mut b = Generator {
        factor: 48271,
        value: seeds[1],
    };
    //println!("--Gen. A--  --Gen. B--");
    //for _ in 0..5 {
    //    println!("{:>10}  {:>10}", a.next().unwrap(), b.next().unwrap());
    //}

    let matching_count = (0..40_000_000)
        .map(|_| (a.next().unwrap(), b.next().unwrap()))
        .filter(|&(an, bn)| (an & 0xFFFF) == (bn & 0xFFFF))
        .count();
    println!("matching pairs : {}", matching_count);
}
