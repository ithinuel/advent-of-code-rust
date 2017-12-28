use std::io::stdin;
use std::io::BufRead;
use std::collections::BTreeMap;

struct Firewall {
    layers: BTreeMap<usize, usize>
}

impl std::fmt::Debug for Firewall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "bonjour")
    }
}

fn main() {
    let input = stdin();
    let mut firewall = Firewall {
        layers: input.lock().lines()
            .map(|l| {
                let v: Vec<usize> = l.unwrap()
                    .trim()
                    .split(": ")
                    .map(|n| n.parse().unwrap())
                    .collect();
                (v[0], v[1])
            }).collect()
    };
    let layers_cnt = firewall.layers.keys().max().unwrap_or(&0);
    println!("last layer = {}", layers_cnt);
    println!("{:?}", firewall.layers);
    let mut severity = 0;
    for i in 0..*layers_cnt {
        let depth = firewall.layers.get(&i);
        if let Some(d) = depth {
            if (i % (d*2 - 2)) == 0 {
                severity += i*d;
            }
        }
    }
    println!("severity at 0: {}", severity);
}
