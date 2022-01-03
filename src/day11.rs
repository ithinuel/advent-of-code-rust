use std::cmp::max;
use std::cmp::min;
use std::io::stdin;

#[derive(Debug)]
struct Path {
    ne: usize,
    nw: usize,
    n: usize,
    se: usize,
    sw: usize,
    s: usize,
}

impl Path {
    fn new(s: &[&str]) -> Path {
        Path {
            ne: s.iter().filter(|d| d == &&"ne").count(),
            nw: s.iter().filter(|d| d == &&"nw").count(),
            n: s.iter().filter(|d| d == &&"n").count(),
            se: s.iter().filter(|d| d == &&"se").count(),
            sw: s.iter().filter(|d| d == &&"sw").count(),
            s: s.iter().filter(|d| d == &&"s").count(),
        }
    }

    fn add_and_reduce(&mut self, dir: &str) {
        match dir {
            "ne" => self.ne += 1,
            "nw" => self.nw += 1,
            "n" => self.n += 1,
            "se" => self.se += 1,
            "sw" => self.sw += 1,
            "s" => self.s += 1,
            _ => panic!("invalid dir"),
        }
        self.reduce();
    }

    fn reduce(&mut self) {
        // cancelling combinations
        if self.s > self.n {
            self.s -= self.n;
            self.n = 0;
        } else {
            self.n -= self.s;
            self.s = 0;
        }

        if self.nw > self.se {
            self.nw -= self.se;
            self.se = 0;
        } else {
            self.se -= self.nw;
            self.nw = 0;
        }

        if self.ne > self.sw {
            self.ne -= self.sw;
            self.sw = 0;
        } else {
            self.sw -= self.ne;
            self.ne = 0;
        }

        // reducing combination
        if self.sw != 0 && self.se != 0 {
            let s = min(self.sw, self.se);
            self.sw -= s;
            self.se -= s;
            self.s += s;
        }
        if self.nw != 0 && self.ne != 0 {
            let n = min(self.nw, self.ne);
            self.nw -= n;
            self.ne -= n;
            self.n += n;
        }
        if self.n != 0 {
            if self.se != 0 {
                let v = min(self.n, self.se);
                self.ne += v;
                self.n -= v;
                self.se -= v;
            } else if self.sw != 0 {
                let v = min(self.n, self.sw);
                self.nw += v;
                self.sw -= v;
                self.n -= v;
            }
        }

        if self.s != 0 {
            if self.ne != 0 {
                let v = min(self.s, self.ne);
                self.se += v;
                self.s -= v;
                self.ne -= v;
            } else if self.nw != 0 {
                let v = min(self.s, self.nw);
                self.sw += v;
                self.nw -= v;
                self.s -= v;
            }
        }
    }

    fn distance(&self) -> usize {
        self.sw + self.se + self.s + self.n + self.ne + self.nw
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let v: Vec<&str> = input.trim().split(',').collect();

    let mut _max = 0;
    let mut p = Path::new(&[]);
    for dir in v {
        p.add_and_reduce(dir);
        _max = max(_max, p.distance());
    }
    println!("distance: {}, max: {}", p.distance(), _max);
}
