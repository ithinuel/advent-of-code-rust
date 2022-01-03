extern crate regex;

use std::io::stdin;
use std::io::Read;

use regex::Regex;

#[derive(Debug)]
struct Particle {
    pos: [i64; 3],
    speed: [i64; 3],
    acc: [i64; 3],
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(
        r"(?x)
                        p=<\s*(?P<px>-?\d+),(?P<py>-?\d+),(?P<pz>-?\d+)>,\s* # match position
                        v=<\s*(?P<vx>-?\d+),(?P<vy>-?\d+),(?P<vz>-?\d+)>,\s* # match speed
                        a=<\s*(?P<ax>-?\d+),(?P<ay>-?\d+),(?P<az>-?\d+)> # match acceleration",
    )
    .unwrap();

    let mut particles: Vec<Particle> = re
        .captures_iter(&input)
        .map(|cap| Particle {
            pos: [
                cap["px"].parse().unwrap(),
                cap["py"].parse().unwrap(),
                cap["pz"].parse().unwrap(),
            ],
            speed: [
                cap["vx"].parse().unwrap(),
                cap["vy"].parse().unwrap(),
                cap["vz"].parse().unwrap(),
            ],
            acc: [
                cap["ax"].parse().unwrap(),
                cap["ay"].parse().unwrap(),
                cap["az"].parse().unwrap(),
            ],
        })
        .collect();

    //println!("{:?}", particles);
    loop {
        let count = particles
            .iter()
            .filter(|p| {
                (p.acc[0] == 0 || p.speed[0].is_positive() == p.acc[0].is_positive())
                    && (p.acc[1] == 0 || p.speed[1].is_positive() == p.acc[1].is_positive())
                    && (p.acc[2] == 0 || p.speed[2].is_positive() == p.acc[2].is_positive())
            })
            .count();
        if count == particles.len() {
            break;
        }
        //println!("{}", count);
        //println!("{:?}", particles[33]);
        for p in &mut particles {
            for i in 0..3 {
                p.speed[i] += p.acc[i];
                p.pos[i] += p.speed[i];
            }
        }
    }
    for _ in 0..5_000 {
        //println!("{}", count);
        //println!("{:?}", particles[33]);
        for p in &mut particles {
            for i in 0..3 {
                p.speed[i] += p.acc[i];
                p.pos[i] += p.speed[i];
            }
        }
    }

    let mut p2: Vec<(usize, i64, &Particle)> = particles
        .iter()
        .enumerate()
        .map(|(i, p)| (i, p.pos[0].abs() + p.pos[1].abs() + p.pos[2].abs(), p))
        .collect();

    p2.sort_by_key(|&(_, d, _)| d);

    /*for &(i, d, p) in p2.iter() {
        println!("{:>3} {} {:?}", i, d, p);
    }*/
    println!("{:?}", p2[0]);
}
