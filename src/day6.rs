use std::collections::BTreeSet;
use std::io::stdin;

fn main() {
    let mut line = String::new();
    if stdin().read_line(&mut line).is_ok() {
        let mut v: Vec<u32> = line
            .trim()
            .split('\t')
            .map(|n| n.parse().unwrap())
            .collect();
        let mut s = BTreeSet::new();

        let mut iter_count = 0;
        while !s.contains(&v) {
            s.insert(v.clone());

            // find the index of the biggest bank
            let (mut idx, _) =
                v.iter().enumerate().fold(
                    (0, 0),
                    |prev, (idx, val)| {
                        if *val > prev.1 {
                            (idx, *val)
                        } else {
                            prev
                        }
                    },
                );
            let mut blocks = v[idx];
            v[idx] = 0;
            while blocks > 0 {
                idx += 1;
                if idx == v.len() {
                    idx = 0;
                }
                v[idx] += 1;
                blocks -= 1;
            }
            iter_count += 1;
        }
        println!("{}", iter_count);
    }
}
