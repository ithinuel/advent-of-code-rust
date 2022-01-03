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
}
