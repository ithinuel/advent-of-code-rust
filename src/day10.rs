use std::iter::{FlatMap, Scan, Chain};

use smallvec::{smallvec, SmallVec, IntoIter};
use yaah::{aoc, aoc_generator};

#[derive(Clone, Copy)]
pub enum Instr {
    Add(i32),
    Noop,
}

pub struct Display([char; 240]);
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        for c in self.0.chunks(40) {
            writeln!(f, "{}", c.iter().collect::<String>())?;
        }
        Ok(())
    }
}

type State = (i32, i32);
pub type Cycles = Chain<
    std::option::IntoIter<State>,
    Scan<
        FlatMap<
            std::str::Lines<'static>,
            IntoIter<[Instr; 2]>,
            fn(&str) -> IntoIter<[Instr; 2]>,
        >,
        State,
        fn(&mut State, Instr) -> Option<State>,
    >,
>;

#[aoc_generator(day10)]
fn day10(input: &'static str) -> Cycles {
    let parser = |l: &str| {
        let input: SmallVec<[&str; 2]> = l.split(' ').collect();
        let seq: SmallVec<[Instr; 2]> = match input[0] {
            "addx" => smallvec![Instr::Noop, Instr::Add(input[1].parse().unwrap())],
            "noop" => smallvec![Instr::Noop],
            _ => unreachable!(),
        };
        seq.into_iter()
    };
    let scanner = |state: &mut State, instr| {
        match instr {
            Instr::Add(v) => {
                state.0 += v;
                state.1 += 1
            }
            Instr::Noop => state.1 += 1,
        }
        Some(*state)
    };

    // required for a reason that's not yet quite clear to me
    let parser: fn(&str) -> IntoIter<[Instr; 2]> = parser;
    let scanner: fn(&mut State, Instr) -> Option<State> = scanner;
    Some((1, 0))
        .into_iter()
        .chain(input.lines().flat_map(parser).scan((1, 0), scanner))
}

#[aoc(day10, part1)]
fn day10_part1(input: &Cycles) -> i32 {
    input
        .clone()
        .filter(|(_, cycle)| (cycle % 40) == 19)
        .inspect(|(x, c)| println!("{c}: {x}"))
        .fold(0, |signal_strength, (x, cycle)| {
            signal_strength + x * (cycle + 1)
        })
}

#[aoc(day10, part2)]
fn day10_part2(input: &Cycles) -> Display {
    let mut display = ['.'; 240];
    input.clone().take(240).for_each(|(x, c)| {
        let crt_pixel = c % 40;
        println!("{crt_pixel} {x}");
        if ((x - 1)..=(x + 1)).contains(&crt_pixel) {
            display[(c % 240) as usize] = '#'
        }
    });

    Display(display)
}

#[cfg(test)]
mod test {
    #[test]
    fn day10_part1() {
        let input = include_str!("../day10_example.txt");
        assert_eq!(13140, super::day10_part1(&super::day10(input)));
    }
    #[test]
    fn day10_part2() {
        let input = include_str!("../day10_example.txt");
        let expected = r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        let out = format!("{}", super::day10_part2(&super::day10(input)));
        assert_eq!(expected, out);
    }
}
