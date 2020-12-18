use itertools::Itertools;
use std::collections::HashMap;
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

    // convert nodes containing only leaves into leaves
    let mut leaves: HashMap<_, _> = rules
        .iter()
        .filter(|(_, v)| v.len() == 0)
        .map(|(k, _)| (k, 0))
        .collect();

    loop {
        let weights: HashMap<_, usize> = rules
            .iter()
            .filter_map(|(k, v)| {
                if !leaves.contains_key(k) && v.keys().all(|k| leaves.contains_key(k)) {
                    Some((k, v.iter().map(|(k, v)| leaves[k] * v + v).sum()))
                } else {
                    None
                }
            })
            .collect();
        if weights.len() == 0 {
            break;
        }
        leaves.extend(weights.into_iter());
    }
    println!("{}", leaves[&"shiny gold".to_string()]);
}
