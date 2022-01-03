use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lines = input.lock().lines();
    let checksum: u32 = lines
        .map(|l| {
            let mut line: Vec<u32> = l
                .unwrap()
                .trim()
                .split('\t')
                .map(|s| s.parse().unwrap())
                .collect();
            line.sort_unstable();
            line[line.len() - 1] - line[0]
        })
        .sum();
    println!("{}", checksum);
}
