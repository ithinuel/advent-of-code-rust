use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let result: usize = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .batching(|lines| {
            let mut set = HashSet::new();
            for line in lines {
                if line.is_empty() {
                    return Some(set);
                } else {
                    set.extend(line.chars());
                }
            }
            None
        })
        .map(|group| group.len())
        .sum();
    println!("{}", result);
}
