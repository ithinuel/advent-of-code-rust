extern crate regex;

use std::io::stdin;
use std::io::BufRead;

use regex::Regex;

#[derive(Debug)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Action {
    fn apply(&self, v: &mut Vec<char>) {
        match *self {
            Action::Spin(x) => {
                let v2 = {
                    let (start, end) = v.split_at(v.len() - x);
                    let mut v2 = Vec::with_capacity(v.len());
                    v2.extend_from_slice(end);
                    v2.extend_from_slice(start);
                    v2
                };
                v.copy_from_slice(&v2);
            }
            Action::Exchange(a, b) => {
                v.swap(a, b);
            }
            Action::Partner(c, d) => {
                let e = v.iter().position(|i| i == &c).unwrap();
                let f = v.iter().position(|i| i == &d).unwrap();
                v.swap(e, f);
            }
        }
    }
}

fn main() {
    let input = stdin();
    let line = input.lock().lines().next().unwrap().unwrap();

    let re = Regex::new(r"s(?P<X>\d+)|x(?P<A>\d+)/(?P<B>\d+)|p(?P<C>[a-p])/(?P<D>[a-p])").unwrap();

    let mut dancers: Vec<char> = "abcdefghijklmnop".chars().collect();
    let actions: Vec<Action> = re
        .captures_iter(&line)
        .map(|cap| {
            let r = &cap[0];
            if r.starts_with('s') {
                Action::Spin(cap["X"].parse().unwrap())
            } else if r.starts_with('x') {
                Action::Exchange(cap["A"].parse().unwrap(), cap["B"].parse().unwrap())
            } else if r.starts_with('p') {
                Action::Partner(
                    cap["C"].chars().next().unwrap(),
                    cap["D"].chars().next().unwrap(),
                )
            } else {
                panic!("Invalid match !");
            }
        })
        .collect();
    println!("{}", actions.len());
    for action in &actions {
        action.apply(&mut dancers);
        //println!("{:?}: {}", action, dancers.iter().collect::<String>());
    }
    println!("{}", dancers.iter().collect::<String>());
}
