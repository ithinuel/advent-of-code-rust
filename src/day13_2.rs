extern crate num;

use std::collections::BTreeMap;
use std::io::stdin;
use std::io::BufRead;

use num::integer::Integer;

struct Firewall {
    layers: BTreeMap<usize, usize>,
    tick: usize,
    delay: usize,
}

impl Firewall {
    fn layer_count(&self) -> usize {
        *self.layers.keys().max().unwrap_or(&0) + 1
    }

    fn print_layer(&self, layer: usize) -> Vec<String> {
        let depth = *self.layers.get(&layer).unwrap_or(&0);
        let header = format!("{:^3}", layer);
        let tick = if self.tick >= self.delay {
            self.tick - self.delay
        } else {
            usize::max_value()
        };

        if depth == 0 {
            vec![
                header,
                if layer == tick {
                    "(.)".to_string()
                } else {
                    "...".to_string()
                },
            ]
        } else if depth == 1 {
            vec![
                header,
                if layer == tick {
                    "(S)".to_string()
                } else {
                    "[S]".to_string()
                },
            ]
        } else {
            let mut v = Vec::with_capacity(1 + depth);
            v.push(header);

            let path_len = depth - 1;
            let mut scanner_pos = self.tick % (path_len * 2);
            if scanner_pos > path_len {
                scanner_pos = 2 * path_len - scanner_pos;
            }
            for i in 0..depth {
                let s = if scanner_pos == i { "S" } else { " " };

                if i == 0 && layer == tick {
                    v.push(format!("({})", s));
                } else {
                    v.push(format!("[{}]", s));
                }
            }
            v
        }
    }
}

impl std::fmt::Display for Firewall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let layer_cnt = self.layer_count();
        let v: Vec<Vec<String>> = (0..layer_cnt).map(|l| self.print_layer(l)).collect();
        let v: Vec<String> = transpose(v).iter().map(|l| l.join(" ")).collect();
        write!(f, "{}", v.join("\n"))
    }
}

fn transpose(v: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let max_depth = v.iter().map(|v2| v2.len()).max().unwrap_or(0);

    (0..max_depth)
        .map(|i| {
            v.iter()
                .map(|v2| {
                    if v2.len() > i {
                        v2[i].clone()
                    } else {
                        "   ".to_string()
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let input = stdin();
    let layers: BTreeMap<usize, usize> = input
        .lock()
        .lines()
        .map(|l| {
            let v: Vec<usize> = l
                .unwrap()
                .trim()
                .split(": ")
                .map(|n| n.parse().unwrap())
                .collect();
            (v[0], v[1])
        })
        .collect();

    #[allow(unused_mut)]
    let mut fw = Firewall {
        layers: layers.clone(),
        tick: 0,
        delay: 0,
    };

    let layers_cnt = fw.layer_count();
    println!("last layer = {}", layers_cnt);
    println!("{:?}", layers);
    let mut uniq: Vec<usize> = layers.values().map(|i| (*i - 1) * 2).collect();
    uniq.sort_unstable();
    uniq.dedup();
    let lcm = uniq.iter().skip(1).fold(uniq[0], |n, l| n.lcm(l));
    println!("lcm({:?}): {}", uniq, lcm);

    let mut min = usize::max_value();
    let mut delay = 0;
    for j in 0..(lcm + 1) {
        let mut severity = 0;
        for (i, depth) in layers.iter() {
            if *depth > 1 && ((j + i) % ((*depth - 1) * 2)) == 0 {
                severity += (i + j) * depth;
            }
        }
        if min > severity {
            min = severity;
            delay = j;
        }
        if min == 0 {
            break;
        }
        //println!("severity at {}: {}", j, severity);
        //println!("=================================================");
    }

    fw.delay = delay;
    for i in 0..layers_cnt {
        fw.tick = delay + i;
        println!("{}", fw);
    }
    println!("minimum severity at {}: {}", delay, min);
}
