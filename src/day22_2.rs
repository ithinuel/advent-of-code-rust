#![feature(entry_and_modify)]

extern crate num;

use std::io::stdin;
use std::io::BufRead;
use std::collections::HashMap;

use num::complex::Complex;

enum Node {
    Clean,
    Weakened,
    Infected,
    Flagged
}

type Vector = Complex<isize>;

struct Virus {
    position: Vector,
    direction: Vector 
}
impl Virus {
    fn new() -> Virus {
        Virus {
            position: Vector::new(0, 0), 
            direction: Vector::new(0, -1)
        }
    }

    fn burst(&mut self, node: &mut Node, infection_caused: &mut usize) {
        use Node::*;
        *node = match *node {
            Clean => {
                self.direction *= Vector::new(0, -1);
                Weakened
            },
            Weakened => {
                *infection_caused += 1;
                Infected
            }
            Infected => {
                self.direction *= Vector::new(0, 1);
                Flagged
            }
            Flagged => {
                self.direction = Vector::new(0, 0)-self.direction;
                Clean
            }
        };

        self.position += self.direction;
    }
}

fn read_map() -> HashMap<Vector, Node> {
    let input = stdin();
    let raw_map: Vec<String> = input.lock().lines().map(|l| l.unwrap()).collect();
    let center_y_offset = (raw_map.len() / 2) as isize;
    let center_x_offset = (raw_map[0].len() / 2) as isize;

    raw_map.iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| {
                    (
                        Vector::new(
                            x as isize - center_x_offset, 
                            y as isize - center_y_offset),
                        Node::Infected
                    )
                })
        }).collect()
}

fn main() {
    let mut map = read_map();
    let mut virus = Virus::new();
    let mut infection_caused = 0;
    for _ in 0..10_000_000 {
        //println!("{:?}", map);
        map.entry(virus.position).and_modify(|n| {
            virus.burst(n, &mut infection_caused);
        }).or_insert_with(|| {
            let mut n = Node::Clean;
            virus.burst(&mut n, &mut infection_caused);
            n
        });
    }
    println!("infection caused: {}", infection_caused); 
}
