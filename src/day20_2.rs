extern crate regex;

use std::io::stdin;
use std::io::Read;
use std::collections::BTreeMap;

use regex::Regex;

#[derive(Debug)]
struct Particle {
    id: usize,
    pos: [i64; 3],
    speed: [i64; 3],
    acc: [i64; 3]
}

fn move_particle(particles: &mut Vec<Particle>) {
    let mut collisions = BTreeMap::new();

    //println!("{}", count);
    //println!("{:?}", particles[33]);
    for p in particles.iter_mut() {
        for i in 0..3 {
            p.speed[i] += p.acc[i];
            p.pos[i] += p.speed[i];
        }

        let cell = collisions.entry(p.pos.clone())
            .or_insert_with(|| Vec::new());
        cell.push(p.id);
    }

    // for all cells containing more than 1 item 
    for (_, vec) in collisions.iter()
        .filter(|&(_, vec)| vec.len() > 1) {
        //println!("[{:>3},{:>3},{:>3}] particles colliding at {:?}", cell[0], cell[1], cell[2], vec);
        for id in vec.iter() {
            let idx = particles.iter().position(|p| p.id == *id).unwrap();
            //println!("{:?}", particles[idx]);
            particles.remove(idx);
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let re = Regex::new(r"(?x)
                        p=<\s*(?P<px>-?\d+),(?P<py>-?\d+),(?P<pz>-?\d+)>,\s* # match position
                        v=<\s*(?P<vx>-?\d+),(?P<vy>-?\d+),(?P<vz>-?\d+)>,\s* # match speed
                        a=<\s*(?P<ax>-?\d+),(?P<ay>-?\d+),(?P<az>-?\d+)> # match acceleration").unwrap();

    let mut particles: Vec<Particle> = re.captures_iter(&input)
        .enumerate()
        .map(|(id, cap)| {
            Particle {
                id: id,
                pos: [cap["px"].parse().unwrap(),
                      cap["py"].parse().unwrap(),
                      cap["pz"].parse().unwrap()],
                speed: [cap["vx"].parse().unwrap(),
                        cap["vy"].parse().unwrap(),
                        cap["vz"].parse().unwrap()],
                acc: [cap["ax"].parse().unwrap(),
                      cap["ay"].parse().unwrap(),
                      cap["az"].parse().unwrap()]
            }
        }).collect();

    //println!("{:?}", particles);
    for _ in 0..1_000 {
        move_particle(&mut particles);
    }


    let mut p2: Vec<(usize, i64, &Particle)> = particles.iter()
        .enumerate()
        .map(|(i, p)| (i, p.pos[0].abs()+p.pos[1].abs()+p.pos[2].abs(), p))
        .collect();

    p2.sort_by_key(|&(_, d, _)| d);

    /*for &(i, d, p) in p2.iter() {
        println!("{:>3} {} {:?}", i, d, p);
    }*/
    println!("{}", particles.len());
}
