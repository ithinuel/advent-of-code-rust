use std::collections::HashMap;

use aoc_runner_derive::*;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{eof, map},
    multi::many1,
    sequence::terminated,
};

#[derive(Debug, Copy, Clone)]
enum Direction {
    NorthEast,
    NorthWest,
    East,
    West,
    SouthEast,
    SouthWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    White,
    Black,
}
impl Tile {
    fn flip(&mut self) {
        *self = match self {
            Tile::White => Tile::Black,
            Tile::Black => Tile::White,
        };
    }
}

fn parse_direction(input: &str) -> nom::IResult<&str, Vec<Direction>> {
    terminated(
        many1(alt((
            map(tag("ne"), |_| Direction::NorthEast),
            map(tag("nw"), |_| Direction::NorthWest),
            map(tag("e"), |_| Direction::East),
            map(tag("w"), |_| Direction::West),
            map(tag("se"), |_| Direction::SouthEast),
            map(tag("sw"), |_| Direction::SouthWest),
        ))),
        eof,
    )(input)
}

#[aoc_generator(day24)]
fn gen(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| parse_direction(line).expect("Invalid format").1)
        .collect()
}

#[allow(dead_code)]
fn print_room(room: &HashMap<(i32, i32), Tile>) {
    let ((xmin, xmax), (ymin, ymax)) = room.keys().fold(((0, 0), (0, 0)), |(x, y), v| {
        ((x.0.min(v.0), x.1.max(v.0)), (y.0.min(v.1), y.1.max(v.1)))
    });

    (ymin..=ymax)
        .cartesian_product(xmin..=xmax)
        .for_each(|coords| {
            let coords = (coords.1, coords.0);

            if coords.0 == xmin && (coords.1 % 2) != 0 {
                print!(" ");
            }
            match room.get(&coords) {
                None => print!("  "),
                Some(Tile::White) if coords == (0, 0) => print!("X "),
                Some(Tile::Black) if coords == (0, 0) => print!("0 "),
                Some(Tile::White) => print!("# "),
                Some(Tile::Black) => print!("O "),
            }
            if coords.0 == xmax {
                println!()
            }
        });
}

fn build_room(commands: &[Vec<Direction>]) -> HashMap<(i32, i32), Tile> {
    let mut room = HashMap::new();
    commands.iter().for_each(|instrs| {
        let coords = instrs.iter().fold((0i32, 0i32), |mut coords, instr| {
            // (instruction, is_even_row)
            match (instr, coords.1 % 2 == 0) {
                (Direction::NorthEast, true) | (Direction::NorthWest, false) => {
                    coords.1 -= 1;
                }
                (Direction::NorthEast, false) => {
                    coords.1 -= 1;
                    coords.0 += 1;
                }
                (Direction::NorthWest, true) => {
                    coords.0 -= 1;
                    coords.1 -= 1;
                }
                (Direction::East, _) => {
                    coords.0 += 1;
                }
                (Direction::West, _) => {
                    coords.0 -= 1;
                }
                (Direction::SouthEast, true) | (Direction::SouthWest, false) => {
                    coords.1 += 1;
                }
                (Direction::SouthWest, true) => {
                    coords.0 -= 1;
                    coords.1 += 1;
                }
                (Direction::SouthEast, false) => {
                    coords.0 += 1;
                    coords.1 += 1;
                }
            }
            coords
        });

        room.entry(coords).or_insert(Tile::White).flip();
    });
    room
}

#[aoc(day24, part1)]
fn part1(commands: &[Vec<Direction>]) -> usize {
    let room = build_room(commands);
    room.iter().filter(|(_, &v)| v == Tile::Black).count()
}

const EVEN_NEIGHBOURS: [(i32, i32); 6] = [(-1, -1), (0, -1), (-1, 0), (1, 0), (-1, 1), (0, 1)];
const ODD_NEIGHBOURS: [(i32, i32); 6] = [(0, -1), (1, -1), (-1, 0), (1, 0), (0, 1), (1, 1)];

fn live_one_generation(room: &mut HashMap<(i32, i32), Tile>) {
    let ((xmin, xmax), (ymin, ymax)) = room.keys().fold(((0, 0), (0, 0)), |(x, y), v| {
        ((x.0.min(v.0), x.1.max(v.0)), (y.0.min(v.1), y.1.max(v.1)))
    });
    *room = ((ymin - 1)..=(ymax + 1))
        .cartesian_product((xmin - 1)..=(xmax + 1))
        .filter_map(|coords| {
            let coords = (coords.1, coords.0);

            let neighs = if (coords.1) % 2 != 0 {
                ODD_NEIGHBOURS
            } else {
                EVEN_NEIGHBOURS
            };

            let count = neighs
                .into_iter()
                .filter_map(|neigh| {
                    let coords = (coords.0 + neigh.0, coords.1 + neigh.1);
                    room.get(&coords)
                        .filter(|&&tile| tile == Tile::Black)
                        .map(move |&tile| (coords, tile))
                })
                .count();

            match room.get(&coords) {
                Some(Tile::Black) if (1..=2).contains(&count) => Some((coords, Tile::Black)),
                None | Some(Tile::White) if count == 2 => Some((coords, Tile::Black)),
                _ => None,
            }
        })
        .collect();
}

#[aoc(day24, part2)]
fn part2(commands: &[Vec<Direction>]) -> usize {
    println!();
    let mut room = build_room(commands);
    (1..=100).for_each(|_| live_one_generation(&mut room));
    room.iter().filter(|(_, &v)| v == Tile::Black).count()
}

#[cfg(test)]
mod test {
    const EXAMPLE: &str = r"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn part1() {
        assert_eq!(10, super::part1(&super::gen(EXAMPLE)));
    }

    #[test]
    fn part2() {
        assert_eq!(2208, super::part2(&super::gen(EXAMPLE)));
    }
}
