use std::cmp::{max, min};
use std::io::{stdin, BufRead};

fn main() {
    let input = stdin();
    let lines = input.lock().lines();
    let checksum: u32 = lines
        .map(|l| {
            let line: Vec<u32> = l
                .unwrap()
                .trim()
                .split('\t')
                .map(|s| s.parse().unwrap())
                .collect();
            line.iter()
                .enumerate()
                .map(|(i, val)| {
                    line.iter()
                        .skip(i)
                        .map(|val2| {
                            let min = min(val, val2);
                            let max = max(val, val2);
                            if (max % min) == 0 && min != max {
                                max / min
                            } else {
                                0
                            }
                        })
                        .sum::<u32>()
                })
                .sum::<u32>()
        })
        .sum();
    println!("{}", checksum);
}
