use std::collections::HashMap;

use aoc_runner_derive::*;
use bitvec::prelude::*;

#[derive(Debug, Clone)]
struct Borders {
    top: BitVec,
    right: BitVec,
    bottom: BitVec,
    left: BitVec,
}
impl Borders {
    fn rotate_ccw90(&mut self) {
        self.bottom.reverse();
        self.top.reverse();
        std::mem::swap(&mut self.top, &mut self.right); // top has right, right has top
        std::mem::swap(&mut self.left, &mut self.right); // left has top, right has left
        std::mem::swap(&mut self.bottom, &mut self.right); // bottom has left, right has bottom
    }

    fn vertical_flip(&mut self) {
        self.left.reverse();
        self.right.reverse();
        std::mem::swap(&mut self.top, &mut self.bottom);
    }
}

type Tile = Vec<BitVec>;
// Possible transformation are composition of Rotation by 0, 90, 180 or 270° and possibly a
// Vertical Flip (two flip cancel each other and H+V flip are equivalent to a 180° rotation),
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
enum Rotation {
    None,
    CCW90,
    CCW180,
    CW90,
}
#[derive(Debug, Clone, Copy)]
enum Position {
    Top,
    Right,
    Left,
    Bottom,
}
#[derive(Debug, Clone, Copy)]
enum Flip {
    None,
    Vertical,
}

#[aoc_generator(day20)]
fn gen(input: &str) -> HashMap<u32, Tile> {
    input
        .split("\n\n")
        .map(|tile| {
            let mut lines = tile.lines();
            let id: u32 = lines
                .next()
                .and_then(|line| line[5..9].parse().ok())
                .expect("Invalid tile number");

            let image: Vec<BitVec> = lines
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();

            (id, image)
        })
        .collect()
}

fn rebuild_map(map: &HashMap<u32, Tile>) -> HashMap<u32, HashMap<u32, (Position, Rotation, Flip)>> {
    let signatures: Vec<_> = map
        .iter()
        .map(|(id, tile)| {
            (
                *id,
                Borders {
                    top: tile.first().map(Clone::clone).unwrap(),
                    left: tile.iter().filter_map(|l| l.last()).collect(),
                    bottom: tile.last().map(Clone::clone).unwrap(),
                    right: tile.iter().filter_map(|l| l.first()).collect(),
                },
            )
        })
        .collect();

    signatures
        .iter()
        .map(|(id, borders)| {
            let neighboors = signatures
                .iter()
                .filter(|(id2, _)| id != id2)
                .filter_map(|(id2, borders2)| {
                    let mut borders2 = borders2.clone();
                    let mut transform = None;

                    for &f in &[Flip::None, Flip::Vertical] {
                        for &r in &[
                            Rotation::None,
                            Rotation::CCW90,
                            Rotation::CCW180,
                            Rotation::CW90,
                        ] {
                            if borders.top == borders2.bottom {
                                transform = Some((Position::Top, r, f));
                            } else if borders.right == borders2.left {
                                transform = Some((Position::Right, r, f))
                            } else if borders.bottom == borders2.top {
                                transform = Some((Position::Bottom, r, f))
                            } else if borders.left == borders2.right {
                                transform = Some((Position::Left, r, f))
                            } else {
                                borders2.rotate_ccw90();
                                continue;
                            }
                            break;
                        }
                        borders2.vertical_flip();
                    }

                    transform.map(|t| (*id2, t))
                })
                .collect();

            (*id, neighboors)
        })
        .collect()
}

#[aoc(day20, part1)]
fn part1(map: &HashMap<u32, Tile>) -> u64 {
    //
    let possible_match = rebuild_map(map);
    //println!("{:?}", possible_match);
    possible_match
        .iter()
        // the tiles with only two size mapped are the corners.
        .filter(|(_, n)| n.len() == 2)
        .map(|(id, _)| *id as u64)
        .product()
}

#[aoc(day20, part2)]
fn part2<T>(_input: T) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST: &str = include_str!("../test20.txt");

    #[test]
    fn part1() {
        let map = super::gen(TEST);
        assert_eq!(1951 * 3079 * 2971 * 1171, super::part1(&map));
    }

    #[test]
    fn part2() {
        let map = super::gen(TEST);
        assert_eq!(273, super::part2(&map));
    }
}
