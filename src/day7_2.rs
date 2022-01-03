extern crate regex;

use std::collections::BTreeMap;
use std::io::stdin;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    parent: Option<String>,
    children: Option<Vec<String>>,
}

impl Node {
    fn actual_weight(&self, tree: &BTreeMap<String, Node>) -> u32 {
        (if let Some(ref children) = self.children {
            let mut weights: Vec<(&String, u32)> = children
                .iter()
                .map(|n| (n, tree[n].actual_weight(tree)))
                .collect();
            weights.sort_by_key(|&(_, w)| w);
            let first = weights[0];
            let mid = weights[weights.len() / 2];
            let last = weights[weights.len() - 1];
            let erroneous_child = if first.1 != mid.1 {
                Some(first)
            } else if last.1 != mid.1 {
                Some(last)
            } else {
                None
            };

            if let Some(error) = erroneous_child {
                let err_weight = tree[error.0].weight;
                let corrected_weight =
                    ((err_weight as i64) + ((mid.1 as i64) - (error.1 as i64))) as u32;
                panic!(
                    "{} ({}) should weight {}",
                    error.0, err_weight, corrected_weight
                );
            }
            weights.iter().map(|&(_, w)| w).sum()
        } else {
            0
        }) + self.weight
    }
}

fn main() {
    let mut lines = String::new();
    let mut input = stdin();
    let mut tree = BTreeMap::new();

    let re =
        Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)(?:\s*->\s*(?P<children>\w+(?:,\s*\w+)*))?")
            .unwrap();
    let _ = input.read_to_string(&mut lines);
    for caps in re.captures_iter(&lines) {
        let name = caps["name"].to_string();
        let children: Option<Vec<String>> = caps
            .name("children")
            .map(|m| m.as_str().split(", ").map(|s| s.to_string()).collect());
        {
            // use or_insert_with to prevent unnecessary clone of children.
            let node = tree.entry(name.clone()).or_insert_with(|| Node {
                name: name.clone(),
                weight: 0,
                parent: None,
                children: children.clone(),
            });

            node.weight = caps["weight"].parse().unwrap();
            if node.children.is_none() && children.is_some() {
                node.children = children.clone();
            }
            //println!("{:?}", node);
        }
        if let Some(ref children) = children {
            for child in children.iter() {
                let c = tree.entry(child.clone()).or_insert_with(|| Node {
                    name: child.clone(),
                    weight: 0,
                    parent: None,
                    children: None,
                });
                c.parent = Some(name.clone());
            }
        }
    }
    //println!("{:?}", tree);

    let roots: Vec<&String> = tree
        .values()
        .filter(|n| n.parent.is_none())
        .map(|n| &n.name)
        .collect();
    println!("{:?}", roots);

    for root in roots {
        println!(
            "{}.actual_weight: {}",
            root,
            tree[root].actual_weight(&tree)
        );
    }
}
