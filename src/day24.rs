#![cfg(test)]
//! This challenge was about reverse engineering.
//!
#[doc = include_str!("day24/notes.md")]
use yaah::*;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{eof, map, map_res, opt},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[aoc_generator(day24)]
fn gen(input: &str) -> Option<Vec<Instr>> {
    input.lines().map(Instr::try_from).try_collect().ok()
}

fn parse_reg(input: &str) -> IResult<&str, Reg> {
    use nom::character::complete::char;
    alt((
        map(char('x'), |_| Reg::X),
        map(char('y'), |_| Reg::Y),
        map(char('z'), |_| Reg::Z),
        map(char('w'), |_| Reg::W),
    ))(input)
}
fn parse_imm(input: &str) -> IResult<&str, isize> {
    use nom::character::complete::char;
    map_res(tuple((opt(char('-')), digit1)), |(sign, value)| {
        isize::from_str_radix(value, 10).map(|v| if sign.is_some() { -1 } else { 1 } * v)
    })(input)
}
fn parse_args(input: &str) -> IResult<&str, (Reg, Op2)> {
    separated_pair(
        parse_reg,
        space1,
        alt((map(parse_reg, Op2::Reg), map(parse_imm, Op2::Imm))),
    )(input)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Reg {
    X,
    Y,
    Z,
    W,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op2 {
    Reg(Reg),
    Imm(isize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instr {
    Inp(Reg),
    Add(Reg, Op2),
    Mul(Reg, Op2),
    Div(Reg, Op2),
    Mod(Reg, Op2),
    Eql(Reg, Op2),
}
impl TryFrom<&str> for Instr {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, instr) = terminated(
            alt((
                map(preceded(tag("inp "), parse_reg), Instr::Inp),
                map(preceded(tag("add "), parse_args), |(r, o)| Instr::Add(r, o)),
                map(preceded(tag("mul "), parse_args), |(r, o)| Instr::Mul(r, o)),
                map(preceded(tag("div "), parse_args), |(r, o)| Instr::Div(r, o)),
                map(preceded(tag("mod "), parse_args), |(r, o)| Instr::Mod(r, o)),
                map(preceded(tag("eql "), parse_args), |(r, o)| Instr::Eql(r, o)),
            )),
            eof,
        )(value)
        .map_err(|e| format!("{:?}", e))?;
        Ok(instr)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
struct Alu {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Alu {
    fn new(x: isize, y: isize, z: isize, w: isize) -> Self {
        Self { x, y, z, w }
    }
    fn set(&mut self, reg: Reg, val: isize) {
        match reg {
            Reg::X => self.x = val,
            Reg::Y => self.y = val,
            Reg::Z => self.z = val,
            Reg::W => self.w = val,
        }
    }
    fn get(&self, op2: Op2) -> isize {
        match op2 {
            Op2::Reg(Reg::X) => self.x,
            Op2::Reg(Reg::Y) => self.y,
            Op2::Reg(Reg::Z) => self.z,
            Op2::Reg(Reg::W) => self.w,
            Op2::Imm(v) => v,
        }
    }
    fn execute(&mut self, program: &[Instr], input: impl IntoIterator<Item = isize>) {
        let mut input = input.into_iter();
        for instr in program.iter().cloned() {
            match instr {
                Instr::Inp(r) => self.set(r, input.next().unwrap()),
                Instr::Add(r, o) => self.set(r, self.get(Op2::Reg(r)) + self.get(o)),
                Instr::Mul(r, o) => self.set(r, self.get(Op2::Reg(r)) * self.get(o)),
                Instr::Div(r, o) => self.set(r, self.get(Op2::Reg(r)) / self.get(o)),
                Instr::Mod(r, o) => self.set(r, self.get(Op2::Reg(r)) % self.get(o)),
                Instr::Eql(r, o) => self.set(r, (self.get(Op2::Reg(r)) == self.get(o)) as isize),
            }
            if let Instr::Add(Reg::Z, Op2::Reg(Reg::Y)) = instr {
                println!("{:?}", self);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day24::Alu;

    use super::Instr;
    use super::Op2;
    use super::Reg;

    #[test]
    fn parser() {
        assert_eq!(
            Ok(Instr::Add(Reg::Z, Op2::Imm(-3))),
            Instr::try_from("add z -3")
        );
    }

    #[test]
    fn alu() {
        let program = super::gen(
            r"inp x
mul x -1",
        )
        .expect("Invalid program");

        let mut alu = Alu::default();
        alu.execute(&program, [3]);
        assert_eq!(Alu::new(-3, 0, 0, 0), alu);

        let program = super::gen(
            r"inp z
inp x
mul z 3
eql z x",
        )
        .expect("Invalid program");

        let mut alu = Alu::default();
        alu.execute(&program, [3, 9]);
        assert_eq!(Alu::new(9, 0, 1, 0), alu);

        let mut alu = Alu::default();
        alu.execute(&program, [3, 4]);
        assert_eq!(Alu::new(4, 0, 0, 0), alu);

        let program = super::gen(
            r"inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2",
        )
        .expect("Invalid program");

        let mut alu = Alu::default();
        alu.execute(&program, [13]);
        assert_eq!(Alu::new(1, 0, 1, 1), alu);
    }

    #[test]
    fn part1() {
        let prog = super::gen(include_str!("../input/2021/day24.txt")).expect("Invalid program");
        let mut alu = Alu::default();
        alu.execute(&prog, [9, 1, 2, 9, 7, 3, 9, 5, 9, 1, 9, 9, 9, 3]);
        assert_eq!(0, alu.z, "{:?}", alu);
    }
    #[test]
    fn part2() {
        let prog = super::gen(include_str!("../input/2021/day24.txt")).expect("Invalid program");
        let mut alu = Alu::default();
        alu.execute(&prog, [7, 1, 1, 3, 1, 1, 5, 1, 9, 1, 7, 8, 9, 1]);
        assert_eq!(0, alu.z, "{:?}", alu);
    }
}
