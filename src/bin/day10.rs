use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let mut adapters: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.ok()?.parse().ok())
        .collect();
    adapters.push(0);
    adapters.sort();
    let (ones, threes) = adapters
        .iter()
        .tuple_windows()
        .fold((0, 0), |(ones, threes), (a, b)| match b - a {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });

    println!("{}", ones * (threes + 1));
}
