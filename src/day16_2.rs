extern crate regex;

use std::io::stdin;
use std::io::BufRead;
use std::collections::BTreeSet;

use regex::Regex;

#[derive(Debug)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

impl Action {
    fn apply(&self, v: &mut Vec<char>) {
        match self {
            &Action::Spin(x) => {
                let v2 = {
                    let (start, end) = v.split_at(v.len() - x);
                    let mut slice = [
                        'a', 'b', 'c', 'd',
                        'e', 'f', 'g', 'h',
                        'i', 'j', 'k', 'l',
                        'm', 'n', 'o', 'p'];
                    slice[..end.len()].copy_from_slice(end);
                    slice[end.len()..].copy_from_slice(start);
                    slice
                };
                v.copy_from_slice(&v2);
            }
            &Action::Exchange(a, b) => {
                v.swap(a, b);
            }
            &Action::Partner(c, d) => {
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
    let actions: Vec<Action> = re.captures_iter(&line)
        .map(|cap| {
            let r = &cap[0];
            if r.starts_with("s") {
                Action::Spin(cap["X"].parse().unwrap())
            } else if r.starts_with("x") {
                Action::Exchange(
                    cap["A"].parse().unwrap(),
                    cap["B"].parse().unwrap())
            } else if r.starts_with("p") {
                Action::Partner(
                    cap["C"].chars().next().unwrap(),
                    cap["D"].chars().next().unwrap())
            } else {
                panic!("Invalid match !");
            }
        }).collect();
    println!("{}", actions.len());
    let mut cache = BTreeSet::new();
    let mut history = Vec::new();
    let mut repeat_at = None;

    cache.insert(dancers.clone());
    history.push(dancers.clone());

    for i in 0..1_000_000_000 {
        for action in &actions {
            action.apply(&mut dancers);
            //println!("{:?}: {}", action, dancers.iter().collect::<String>());
        }
        if cache.contains(&dancers) {
            repeat_at = Some(i);
            break;
        }
        cache.insert(dancers.clone());
        history.push(dancers.clone());
    }
    if let Some(i) = repeat_at {
        let n = 1_000_000_000 % (i+1);
        println!("repeats after {} rounds", i);
        println!("taking the reminder from the history[{}]", n);
        println!("{}", history[n].iter().collect::<String>());
    } else {
        println!("{}", dancers.iter().collect::<String>());
    }
}
