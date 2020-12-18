use std::collections::BTreeMap;
use std::io::BufRead;

fn main() {
    let mut adapters: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|s| s.ok()?.parse().ok())
        .collect();
    adapters.push(0);
    adapters.sort();

    let mut edges = BTreeMap::new();
    adapters.iter().for_each(|&a| {
        edges.entry(a).or_insert_with(|| Vec::new()).extend(
            (1..4)
                .map(|n| a + n)
                .filter(|b| adapters.binary_search(&b).is_ok()),
        );
    });

    // The input is a Directed Acyclic Graph
    // count number of path from 0 to max
    let mut paths_count = BTreeMap::new();
    adapters.iter().rev().for_each(|&v| {
        let neighbours = &edges[&v];
        if neighbours.is_empty() {
            paths_count.insert(v, 1u64);
        } else {
            paths_count.insert(v, neighbours.iter().map(|n| paths_count[n]).sum());
        }
    });

    println!("{:?}", paths_count[&0]);
}
