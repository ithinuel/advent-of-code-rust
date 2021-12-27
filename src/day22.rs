use std::{collections::HashSet, hash::Hash};

use arrayvec::ArrayVec;

use aoc_runner_derive::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct Block {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}
impl Block {
    fn volume(&self) -> i64 {
        let dx = self.x.1 - self.x.0 + 1;
        let dy = self.y.1 - self.y.0 + 1;
        let dz = self.z.1 - self.z.0 + 1;
        dx * dy * dz
    }

    /// Returns true if self is entirely inside rhs.
    fn contains(&self, rhs: &Self) -> bool {
        (self.x.0 <= rhs.x.0 && rhs.x.1 <= self.x.1)
            && (self.y.0 <= rhs.y.0 && rhs.y.1 <= self.y.1)
            && (self.z.0 <= rhs.z.0 && rhs.z.1 <= self.z.1)
    }
    fn intersects(&self, rhs: &Block) -> bool {
        (self.x.0 <= rhs.x.1 && rhs.x.0 <= self.x.1)
            && (self.y.0 <= rhs.y.1 && rhs.y.0 <= self.y.1)
            && (self.z.0 <= rhs.z.1 && rhs.z.0 <= self.z.1)
    }

    #[allow(dead_code)]
    /// this is either broken (very likely) or creates way too many fragements and exhausts the
    /// memory.
    fn split_in_halves(self) -> impl Iterator<Item = Self> {
        use either::Either::{self, *};
        fn split_axis((min, max): (i64, i64)) -> Either<Option<(i64, i64)>, [(i64, i64); 2]> {
            let diff = max - min + 1;
            if diff == 1 {
                Left(Some((min, max)))
            } else {
                let middle = min + diff / 2;
                Right([(min, middle - 1), (middle, max)])
            }
        }

        let Block { x, y, z } = self.clone();
        split_axis(x)
            .into_iter()
            .cartesian_product(split_axis(y).into_iter())
            .cartesian_product(split_axis(z).into_iter())
            .map(|((x, y), z)| Block { x, y, z })
    }

    fn split_by(self, rhs: &Self) -> impl Iterator<Item = Self> + '_ {
        fn split_along(
            lhs: (i64, i64),
            rhs: (i64, i64),
            copy_with_axis: impl Fn(i64, i64) -> Block,
        ) -> impl Iterator<Item = Block> {
            let (min, max) = (i64::max(lhs.0, rhs.0), i64::min(lhs.1, rhs.1));

            ArrayVec::<_, 3>::into_iter(match (min == lhs.0, max == lhs.1) {
                (true, true) => ArrayVec::from_iter([copy_with_axis(lhs.0, lhs.1)]),
                (false, true) => ArrayVec::from_iter([
                    copy_with_axis(lhs.0, min - 1),
                    copy_with_axis(min, lhs.1),
                ]),
                (true, false) => ArrayVec::from_iter([
                    copy_with_axis(lhs.0, max),
                    copy_with_axis(max + 1, lhs.1),
                ]),
                (false, false) => ArrayVec::from_iter([
                    copy_with_axis(lhs.0, min - 1),
                    copy_with_axis(min, max),
                    copy_with_axis(max + 1, lhs.1),
                ]),
            })
        }

        split_along(self.z, rhs.z, move |min, max| Block {
            z: (min, max),
            ..self
        })
        .flat_map(|block| {
            split_along(block.x, rhs.x, move |min, max| Block {
                x: (min, max),
                ..block
            })
        })
        .flat_map(|block| {
            split_along(block.y, rhs.y, move |min, max| Block {
                y: (min, max),
                ..block
            })
        })
    }

    #[cfg(test)]
    const fn from_const((x, y, z): Tuples) -> Self {
        Self { x, y, z }
    }
}
impl From<Tuples> for Block {
    fn from((x, y, z): Tuples) -> Self {
        Self { x, y, z }
    }
}

type Tuples = ((i64, i64), (i64, i64), (i64, i64));
type Command = (bool, Block);

#[aoc_generator(day22)]
fn gen(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|l| {
            let (action, rest) = if l.starts_with("on ") {
                (true, l.trim_start_matches("on "))
            } else {
                (false, l.trim_start_matches("off "))
            };
            rest.split(',')
                .flat_map(|a| a.split('='))
                .filter_map(|range| {
                    range
                        .split("..")
                        .filter_map(|v| v.parse::<i64>().ok())
                        .collect_tuple()
                })
                .collect_tuple()
                .map(|tuples: Tuples| (action, Block::from(tuples)))
        })
        .collect()
}

#[aoc(day22, part1)]
fn part1(instrs: &[Command]) -> usize {
    let mut map = HashSet::new();

    instrs
        .iter()
        .cloned()
        .filter(|(_, cube)| {
            let x = cube.x;
            let y = cube.y;
            let z = cube.z;
            (x.0 >= -50 && x.1 <= 50) && (y.0 >= -50 && y.1 <= 50) && (z.0 >= -50 && z.1 <= 50)
        })
        .flat_map(|(action, cube)| {
            (cube.x.0..=cube.x.1)
                .cartesian_product(cube.y.0..=cube.y.1)
                .cartesian_product(cube.z.0..=cube.z.1)
                .map(move |((x, y), z)| (action, (x, y, z)))
        })
        .for_each(|(action, coord)| {
            if action {
                map.insert(coord)
            } else {
                map.remove(&coord)
            };
        });
    map.len()
}

#[aoc(day22, part2)]
fn part2(instrs: &[Command]) -> usize {
    // the map does not require extra features (like HashSet/BTreeSet would provide).
    // All the required features are guaranteed by the algorithm.
    let mut map: Vec<Block> = Vec::new();
    for (action, new_block) in instrs.into_iter() {
        map = map
            .into_iter()
            .flat_map(|block| {
                if new_block.contains(&block) {
                    // remove those that are absorbed in new block
                    ArrayVec::<_, 27>::new()
                } else if new_block.intersects(&block) {
                    // undecisive, split & retry
                    block
                        .split_by(new_block)
                        .filter(|b| !new_block.contains(b))
                        .collect()
                } else {
                    // no collision, keep this block
                    ArrayVec::from_iter([block])
                }
            })
            .collect();
        if *action {
            map.push(new_block.clone());
        }
    }
    map.into_iter().map(|block| block.volume() as usize).sum()
}

#[cfg(test)]
mod test {
    use std::collections::{hash_map::RandomState, HashSet};

    use super::Block;
    const EXAMPLE1: &str = r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const EXAMPLE2: &str = r"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    const EXAMPLE3: &str = include_str!("day22/example3.txt");

    // 3x3x3 cube centered on 0,0,0
    const A: Block = Block::from_const(((-1, 1), (-1, 1), (-1, 1)));
    // 4x4x4 cube centered on -3,-3,-3
    const B: Block = Block::from_const(((-5, -2), (-5, -2), (-5, -2)));
    // 2x2x1 block centered on 1,1,0
    const C: Block = Block::from_const(((0, 1), (0, 1), (0, 0)));
    //  3,3,3 cube centered on 1,1,1
    const D: Block = Block::from_const(((0, 2), (0, 2), (0, 2)));

    #[test]
    fn part1() {
        assert_eq!(39, super::part1(&super::gen(EXAMPLE1)));
        assert_eq!(590784, super::part1(&super::gen(EXAMPLE2)));
        assert_eq!(474140, super::part1(&super::gen(EXAMPLE3)));
    }

    #[test]
    //#[ignore]
    fn part2() {
        assert_eq!(39, super::part2(&super::gen(EXAMPLE1)));
        println!("========");
        assert_eq!(2758514936282235, super::part2(&super::gen(EXAMPLE3)));
    }

    #[test]
    fn block_volume() {
        let dut = Block::from(((-1, 1), (-2, 1), (-1, 4)));
        assert_eq!(72, dut.volume());
    }

    #[test]
    fn block_inclusion() {
        // self inclusion
        assert_eq!(true, A.contains(&A));

        // not included (not even coliding)
        assert_eq!(false, A.contains(&B));
        assert_eq!(false, B.contains(&A));

        // c inside a but a not inside c
        assert_eq!(true, A.contains(&C));
        assert_eq!(false, C.contains(&A));

        // intersection but not inclusion
        assert_eq!(false, A.contains(&D));
        assert_eq!(false, D.contains(&A));
    }

    #[test]
    fn block_intersects() {
        // self inclusion
        assert_eq!(true, A.intersects(&A));

        // not included (not even coliding)
        assert_eq!(false, A.intersects(&B));
        assert_eq!(false, B.intersects(&A));

        // intersection but not inclusion
        assert_eq!(true, A.intersects(&D));
        assert_eq!(true, D.intersects(&A));
    }

    #[test]
    fn block_split_halves() {
        const APARTS: [Block; 8] = [
            Block::from_const(((-1, -1), (-1, -1), (-1, -1))),
            Block::from_const(((-1, -1), (-1, -1), (0, 1))),
            Block::from_const(((-1, -1), (0, 1), (-1, -1))),
            Block::from_const(((0, 1), (-1, -1), (-1, -1))),
            Block::from_const(((-1, -1), (0, 1), (0, 1))),
            Block::from_const(((0, 1), (-1, -1), (0, 1))),
            Block::from_const(((0, 1), (0, 1), (-1, -1))),
            Block::from_const(((0, 1), (0, 1), (0, 1))),
        ];
        assert_eq!(
            APARTS.into_iter().collect::<HashSet<_>>(),
            A.split_in_halves().collect()
        );

        const CPARTS: [Block; 4] = [
            Block::from_const(((0, 0), (0, 0), (0, 0))),
            Block::from_const(((0, 0), (1, 1), (0, 0))),
            Block::from_const(((1, 1), (0, 0), (0, 0))),
            Block::from_const(((1, 1), (1, 1), (0, 0))),
        ];
        assert_eq!(
            CPARTS.into_iter().collect::<HashSet<_>>(),
            C.split_in_halves().collect()
        );
    }

    #[test]
    fn block_split_at() {
        // ......
        // ......
        // ..xxxx##
        // ..xxxx##
        // ..xxxx##
        // ..xxxx##
        //   ######
        //   ######
        let a = Block::from(((0, 5), (0, 5), (0, 5)));
        let b = Block::from(((2, 7), (2, 7), (2, 7)));
        let expect = HashSet::<_, RandomState>::from_iter([
            Block::from_const(((0, 1), (0, 1), (0, 1))),
            Block::from_const(((0, 1), (0, 1), (2, 5))),
            Block::from_const(((0, 1), (2, 5), (0, 1))),
            Block::from_const(((2, 5), (0, 1), (0, 1))),
            Block::from_const(((0, 1), (2, 5), (2, 5))),
            Block::from_const(((2, 5), (0, 1), (2, 5))),
            Block::from_const(((2, 5), (2, 5), (0, 1))),
            Block::from_const(((2, 5), (2, 5), (2, 5))),
        ]);
        assert_eq!(expect, a.split_by(&b).collect());
        println!("================");

        //   ......
        //   ......
        //  ########
        //  ########
        //   ......
        //   ......
        // ----------
        //   ......
        //   ......
        //   ..xx..
        //   ..xx..
        //   ......
        //   ......
        let a = Block::from(((0, 5), (0, 5), (0, 5)));
        let b = Block::from(((-1, 6), (2, 3), (2, 3)));
        let expect = HashSet::<_, RandomState>::from_iter([
            Block::from_const(((0, 5), (0, 1), (0, 1))),
            Block::from_const(((0, 5), (2, 3), (0, 1))),
            Block::from_const(((0, 5), (4, 5), (0, 1))),
            Block::from_const(((0, 5), (0, 1), (2, 3))),
            Block::from_const(((0, 5), (2, 3), (2, 3))),
            Block::from_const(((0, 5), (4, 5), (2, 3))),
            Block::from_const(((0, 5), (0, 1), (4, 5))),
            Block::from_const(((0, 5), (2, 3), (4, 5))),
            Block::from_const(((0, 5), (4, 5), (4, 5))),
        ]);
        assert_eq!(expect, a.split_by(&b).collect());
        println!("================");

        //  ......
        //  ......
        //  ..xx..
        //  ..xx..
        //  ......
        //  ......
        // --------
        //  ......
        //  ......
        //  ..xx..
        //  ..xx..
        //  ......
        //  ......
        let a = Block::from(((0, 5), (0, 5), (0, 5)));
        let b = Block::from(((2, 3), (2, 3), (2, 3)));
        let expect = HashSet::<_, RandomState>::from_iter([
            Block::from_const(((0, 1), (0, 1), (0, 1))),
            Block::from_const(((0, 1), (2, 3), (0, 1))),
            Block::from_const(((0, 1), (4, 5), (0, 1))),
            Block::from_const(((0, 1), (0, 1), (2, 3))),
            Block::from_const(((0, 1), (2, 3), (2, 3))),
            Block::from_const(((0, 1), (4, 5), (2, 3))),
            Block::from_const(((0, 1), (0, 1), (4, 5))),
            Block::from_const(((0, 1), (2, 3), (4, 5))),
            Block::from_const(((0, 1), (4, 5), (4, 5))),
            Block::from_const(((2, 3), (0, 1), (0, 1))),
            Block::from_const(((2, 3), (2, 3), (0, 1))),
            Block::from_const(((2, 3), (4, 5), (0, 1))),
            Block::from_const(((2, 3), (0, 1), (2, 3))),
            Block::from_const(((2, 3), (2, 3), (2, 3))),
            Block::from_const(((2, 3), (4, 5), (2, 3))),
            Block::from_const(((2, 3), (0, 1), (4, 5))),
            Block::from_const(((2, 3), (2, 3), (4, 5))),
            Block::from_const(((2, 3), (4, 5), (4, 5))),
            Block::from_const(((4, 5), (0, 1), (0, 1))),
            Block::from_const(((4, 5), (2, 3), (0, 1))),
            Block::from_const(((4, 5), (4, 5), (0, 1))),
            Block::from_const(((4, 5), (0, 1), (2, 3))),
            Block::from_const(((4, 5), (2, 3), (2, 3))),
            Block::from_const(((4, 5), (4, 5), (2, 3))),
            Block::from_const(((4, 5), (0, 1), (4, 5))),
            Block::from_const(((4, 5), (2, 3), (4, 5))),
            Block::from_const(((4, 5), (4, 5), (4, 5))),
        ]);
        assert_eq!(expect, a.split_by(&b).collect());
        println!("================");

        //  ########
        //  ########
        //  #xxxxxx#
        //  #xxxxxx#
        //  #xxxxxx#
        //  #xxxxxx#
        //   ......
        //   ......
        // ----------
        //  ########
        //  ########
        //  #xxxxxx#
        //  #xxxxxx#
        //  #xxxxxx#
        //  #xxxxxx#
        //   ......
        //   ......
        let a = Block::from(((0, 5), (0, 5), (0, 5)));
        let b = Block::from_const(((-1, 6), (-1, 6), (2, 7)));
        let expect = HashSet::<_, RandomState>::from_iter([
            Block::from_const(((0, 5), (0, 5), (0, 1))),
            Block::from_const(((0, 5), (0, 5), (2, 5))),
        ]);
        assert_eq!(expect, a.split_by(&b).collect());
        println!("================");

        //   ......
        //   ......
        //  #xxxxxx#
        //  ########
        // ----------
        //   ......
        //   ......
        //  #xxxxxx#
        //  ########
        let a = Block::from(((0, 5), (0, 5), (0, 2)));
        let b = Block::from_const(((-1, 6), (-1, 6), (-1, 0)));
        let expect = HashSet::<_, RandomState>::from_iter([
            Block::from_const(((0, 5), (0, 5), (1, 2))),
            Block::from_const(((0, 5), (0, 5), (0, 0))),
        ]);
        assert_eq!(expect, a.split_by(&b).collect());
    }
}
