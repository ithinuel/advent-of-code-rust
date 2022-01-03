use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::io::stdin;
use std::io::BufRead;

#[derive(Debug)]
struct Node {
    id: u32,
    neighboors: Vec<u32>,
}

fn main() {
    let input = stdin();
    let network: BTreeMap<u32, Node> = input
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let a: Vec<&str> = l.split(" <-> ").collect();
            Node {
                id: a[0].parse().unwrap(),
                neighboors: a[1].split(", ").map(|n| n.parse().unwrap()).collect(),
            }
        })
        .map(|n| (n.id, n))
        .collect();

    let mut visited = BTreeSet::new();
    let mut tovisit = VecDeque::new();
    tovisit.push_back(0);
    while let Some(id) = tovisit.pop_front() {
        let node = &network[&id];
        if !visited.contains(&id) {
            visited.insert(id);
            for neighboor in &node.neighboors {
                if !visited.contains(neighboor) {
                    tovisit.push_back(*neighboor);
                }
            }
        }
    }

    println!("{:?}", visited.len());
}
