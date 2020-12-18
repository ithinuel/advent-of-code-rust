use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

const RULES: &'static str = r"(\d+) ([\w ]+) bags?";

fn main() {
    let re = regex::Regex::new(RULES).expect("invalid regex");
    let rules: HashMap<_, HashMap<_, _>> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(|l| {
            let bag_color = l
                .split(" bags contain")
                .next()
                .map(str::to_string)
                .expect("bag name");
            let children = re
                .captures_iter(&l)
                .filter_map(|c| {
                    c.iter()
                        .flat_map(|m| m.into_iter())
                        .skip(1)
                        .tuples()
                        .map(|(count, name)| {
                            (
                                name.as_str().to_string(),
                                count.as_str().parse::<usize>().unwrap(),
                            )
                        })
                        .next()
                })
                .collect();
            (bag_color, children)
        })
        .collect();

    let mut containers: HashSet<&str> = HashSet::new();
    let mut prev_layer: HashSet<_> = ["shiny gold"].iter().copied().collect();
    loop {
        let layer: HashSet<_> = prev_layer
            .iter()
            .flat_map(|bag| rules.iter().filter(move |(_, v)| v.contains_key(*bag)))
            .map(|(k, _)| k.as_str())
            .collect();
        if layer.len() == 0 {
            break;
        }
        containers.extend(layer.iter());
        prev_layer = layer;
    }
    println!("{}", containers.len());
}
