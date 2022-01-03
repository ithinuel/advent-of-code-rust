use itertools::Itertools;
use std::io::BufRead;

fn strongest_bridge(from: usize, parts: &[(usize, usize)]) -> usize {
    parts
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(idx, (a, b))| {
            if a == from {
                Some((idx, (a, b)))
            } else if b == from {
                Some((idx, (b, a)))
            } else {
                None
            }
        })
        .map(|(idx, (a, b))| {
            let mut parts = parts.to_vec();
            parts.remove(idx);
            let res = strongest_bridge(b, &parts);
            res + a + b
        })
        .max()
        .unwrap_or(0)
}
fn strongest_longest_bridge(from: usize, parts: &[(usize, usize)]) -> (usize, usize) {
    let bridges = parts
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(idx, (a, b))| {
            if a == from {
                Some((idx, (a, b)))
            } else if b == from {
                Some((idx, (b, a)))
            } else {
                None
            }
        })
        .map(|(idx, (a, b))| {
            let mut parts = parts.to_vec();
            parts.remove(idx);
            let (strength, length) = strongest_longest_bridge(b, &parts);
            (strength + a + b, length + 1)
        })
        .collect_vec();

    bridges
        .iter()
        .map(|&(_, len)| len)
        .max()
        .and_then(|max_len| {
            bridges
                .into_iter()
                .filter(|&(_, len)| len == max_len)
                .max_by_key(|&(strength, _)| strength)
        })
        .unwrap_or((0, 0))
}

fn main() {
    let parts: Vec<(usize, _)> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .filter_map(|line| {
            line.split('/')
                .filter_map(|s| s.parse().ok())
                .tuples()
                .next()
        })
        .collect();

    println!("part1: {}", strongest_bridge(0, &parts));
    println!("part2: {}", strongest_longest_bridge(0, &parts).0)
}
