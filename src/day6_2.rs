use itertools::Itertools;
use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let result: usize = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .batching(|lines| {
            let mut intersect = None;

            for line in lines {
                if line.is_empty() {
                    return intersect;
                } else {
                    let line: HashSet<_> = line.chars().collect();

                    if let Some(intersect) = intersect.as_mut() {
                        *intersect = &*intersect & &line;
                    } else {
                        intersect = Some(line);
                    }
                }
            }
            None
        })
        .map(|group| group.len())
        .sum();
    println!("{}", result);
}
