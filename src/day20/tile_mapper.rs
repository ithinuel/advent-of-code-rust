use std::{collections::HashMap, str::Lines};

use itertools::Itertools;
use num::Zero;

//const _SEA_MONSTER: &str = r"                  #
//#    ##    ##    ###
// #  #  #  #  #  #   ";

fn bit_to_char<T: Eq + Zero>(bit: T) -> char {
    if bit != T::zero() {
        '#'
    } else {
        '.'
    }
}

// Possible transformation are composition of Rotation by 0, 90, 180 or 270° and possibly a
// Vertical Flip (two flip cancel each other and H+V flip are equivalent to a 180° rotation),
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    None,
    CCW90,
    CCW180,
    CW90,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Position {
    Top,
    Right,
    Left,
    Bottom,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Flip {
    None,
    Vertical,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Tile([u16; 10]);
impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for mut line in self.0 {
            for _ in 0..10 {
                write!(f, "{}", bit_to_char(line & 0x200))?;
                line <<= 1;
            }
            writeln!(f)?
        }
        Ok(())
    }
}
impl Tile {
    pub fn image(&self) -> &[u16; 10] {
        &self.0
    }
    pub fn borders(&self) -> Borders {
        Borders::new(self)
    }
    fn rotate(&mut self, rot: Rotation) {
        match rot {
            Rotation::None => {}
            Rotation::CCW90 => {
                let mut v = [0u16; 10];
                v.iter_mut().enumerate().for_each(|(y, line)| {
                    *line = (0..10).fold(0, |acc, x| acc << 1 | ((self.0[x] >> y) & 1));
                });
                self.0 = v;
            }
            Rotation::CCW180 => {
                self.rotate(Rotation::CCW90);
                self.rotate(Rotation::CCW90);
            }
            Rotation::CW90 => {
                self.rotate(Rotation::CCW90);
                self.rotate(Rotation::CCW90);
                self.rotate(Rotation::CCW90);
            }
        }
    }
    fn flip(&mut self, flip: Flip) {
        if flip == Flip::Vertical {
            (0..5).for_each(|line| {
                self.0.swap(line, 9 - line);
            });
        }
    }
}
impl TryFrom<Lines<'_>> for Tile {
    type Error = &'static str;
    fn try_from(lines: Lines<'_>) -> Result<Self, Self::Error> {
        let mut me = [0; 10];
        let last_idx = lines
            .zip(me.iter_mut())
            .map(|(src, dst)| {
                let (val, last_idx) = src.bytes().enumerate().fold((0, 0), |(acc, _), (idx, b)| {
                    (acc << 1 | if b == b'#' { 1 } else { 0 }, idx)
                });
                if last_idx != 9 {
                    // lines must be 10bytes long
                    return Err("Invalid line length");
                }
                *dst = val;
                Ok(())
            })
            .enumerate()
            .try_fold(0, |_, (idx, res)| res.map(|_| idx))?;

        if last_idx != 9 {
            Err("Invalid line count")
        } else {
            Ok(Self(me))
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Borders {
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
}
impl Borders {
    fn new(tile: &Tile) -> Self {
        Self {
            top: tile.0[0],
            left: tile
                .0
                .into_iter()
                .map(|l| (l & 0x200) >> 9)
                .fold(0, |acc, b| acc << 1 | b),
            bottom: tile.0[9],
            right: tile
                .0
                .into_iter()
                .map(|l| (l & 0x01))
                .fold(0, |acc, b| acc << 1 | b),
        }
    }
    pub fn rotate_ccw90(&mut self) {
        self.top = self.top.reverse_bits() >> 6;
        self.bottom = self.bottom.reverse_bits() >> 6;
        std::mem::swap(&mut self.bottom, &mut self.left);
        std::mem::swap(&mut self.top, &mut self.right);
        std::mem::swap(&mut self.left, &mut self.right);
    }

    pub fn vertical_flip(&mut self) {
        self.left = self.left.reverse_bits() >> 6;
        self.right = self.right.reverse_bits() >> 6;
        std::mem::swap(&mut self.top, &mut self.bottom);
    }
}
impl IntoIterator for &Borders {
    type Item = (Position, u16);

    type IntoIter = std::array::IntoIter<(Position, u16), 4>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (Position::Top, self.top),
            (Position::Right, self.right),
            (Position::Bottom, self.bottom),
            (Position::Left, self.left),
        ]
        .into_iter()
    }
}

#[derive(Clone)]
pub struct Image(pub HashMap<(i32, i32), (u32, Tile)>);
impl Image {
    pub fn new(
        tiles: &HashMap<u32, Tile>,
        map: &HashMap<(i32, i32), (u32, Rotation, Flip)>,
    ) -> Self {
        let (xmin, ymin) = map.keys().cloned().fold((0, 0), |acc, c| {
            (i32::min(c.0, acc.0), i32::min(c.1, acc.1))
        });

        Self(
            map.iter()
                .map(|(&coords, &(tileid, rot, flip))| {
                    let mut tile = tiles[&tileid].clone();
                    tile.rotate(rot);
                    tile.flip(flip);
                    ((coords.0 - xmin, coords.1 - ymin), (tileid, tile))
                })
                .collect(),
        )
    }
    fn rotate_ccw90(&mut self) {
        let ymax = self.0.keys().fold(0, |acc, &(_, y)| i32::max(acc, y));

        let mut map = HashMap::new();
        std::mem::swap(&mut self.0, &mut map);
        self.0 = map
            .into_iter()
            .map(|(coord, (id, mut tile))| {
                let coord = (coord.1, ymax - coord.0);
                tile.rotate(Rotation::CCW90);
                (coord, (id, tile))
            })
            .collect();
    }
    pub fn rotate(&mut self, rot: Rotation) {
        match rot {
            Rotation::None => {}
            Rotation::CCW90 => {
                self.rotate_ccw90();
            }
            Rotation::CCW180 => {
                self.rotate_ccw90();
                self.rotate_ccw90();
            }
            Rotation::CW90 => {
                self.rotate_ccw90();
                self.rotate_ccw90();
                self.rotate_ccw90();
            }
        }
    }
    fn vertical_flip(&mut self) {
        let ymax = self.0.keys().fold(0, |acc, &(_, y)| i32::max(acc, y));

        let mut map = HashMap::new();
        std::mem::swap(&mut self.0, &mut map);
        self.0 = map
            .into_iter()
            .map(|(coord, (id, mut tile))| {
                let coord = (coord.0, ymax - coord.1);
                tile.flip(Flip::Vertical);
                (coord, (id, tile))
            })
            .collect();
    }
    pub fn flip(&mut self, flip: Flip) {
        if let Flip::Vertical = flip {
            self.vertical_flip();
        }
    }

    /// Iterates over coordinates matching a sea monster assuming no sea monster overlap.
    pub fn find_monster(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        let max = self.0.keys().fold((0, 0), |acc, &c| {
            (i32::max(acc.0, c.0), i32::max(acc.1, c.1))
        });
        let (xlen, ylen) = ((max.0 + 1) * 8, (max.1 + 1) * 8);
        (0..(ylen - 3))
            .cartesian_product(0..(xlen - 20))
            .filter(move |(img_y, img_x)| {
                use bitvec::prelude::*;
                #[rustfmt::skip]
                let sea_monster  = bitvec![
                    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,
                    1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,1,
                    0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0
                ];
                let mask = sea_monster.clone();
                // view of the image where a monster could be.
                let view: BitVec = (0..3)
                    .cartesian_product(0..20)
                    .map(|(view_y, view_x)| {
                        let y = img_y + view_y;
                        let x = img_x + view_x;

                        // extract bit from img
                        let tile = &self.0[&(x / 8, y / 8)].1;
                        ((tile.image()[1 + (y % 8) as usize] << (1 + (x % 8))) & 0x200) != 0
                    })
                    .collect();
                let masked = view & mask;
                masked == sea_monster
            })
    }
}
impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xmax, ymax) = self.0.keys().fold((0, 0), |acc, &c| {
            (i32::max(acc.0, c.0), i32::max(acc.1, c.1))
        });

        (0..=ymax)
            .cartesian_product(1..=8)
            .try_for_each(|(ytile, y)| {
                (0..=xmax).try_for_each(|xtile| {
                    let line = self.0[&(xtile, ytile)].1.image()[y];
                    (1..=8).try_for_each(|x| write!(f, "{}", bit_to_char((line << x) & 0x200)))
                })?;
                if ytile != ymax || y != 8 {
                    writeln!(f)
                } else {
                    Ok(())
                }
            })
    }
}

#[cfg(test)]
pub mod test {
    use crate::day20::tile_mapper::Borders;

    /// Test sample characteristics:
    /// each border and its flipped version are unique
    /// top: 2 - 256
    /// rigth: 3 - 384
    /// left:  5 - 320
    /// bottom: 24 - 96
    const TILE_EXAMPLE: &str = r"........#.
#....#...#
....##...#
#..#.#....
..#..#....
.#####....
.....#....
.#...#..#.
..######..
.....##...";

    const ROTATED_CCW90: &str = r".##.......
#......#..
........#.
........##
.#########
..#..#..#.
...#.#..#.
....##..#.
.....#.#..
.#.#......";

    const VERTICALLY_FLIPPED: &str = r".....##...
..######..
.#...#..#.
.....#....
.#####....
..#..#....
#..#.#....
....##...#
#....#...#
........#.";

    const TILE_PARSED: Tile = Tile([
        0b00_0000_0010,
        0b10_0001_0001,
        0b00_0011_0001,
        0b10_0101_0000,
        0b00_1001_0000,
        0b01_1111_0000,
        0b00_0001_0000,
        0b01_0001_0010,
        0b00_1111_1100,
        0b00_0001_1000,
    ]);

    use super::Tile;
    #[test]
    fn border_computation() {
        let expect = Borders {
            top: 0b00_0000_0010,
            left: 0b01_0100_0000,
            right: 0b01_1000_0000,
            bottom: 0b00_0001_1000,
        };
        assert_eq!(expect, Borders::new(&TILE_PARSED));
    }

    #[test]
    fn tile_parser() {
        assert_eq!(Ok(TILE_PARSED), Tile::try_from(TILE_EXAMPLE.lines()));
    }

    #[test]
    fn tile_rotate_ccw90() {
        let mut dut = TILE_PARSED;
        let expect = Tile::try_from(ROTATED_CCW90.lines()).unwrap();
        dut.rotate(super::Rotation::CCW90);
        assert_eq!(expect, dut);
    }

    #[test]
    fn tile_vertical_flip() {
        let mut dut = TILE_PARSED;
        let expect = Tile::try_from(VERTICALLY_FLIPPED.lines()).unwrap();
        dut.flip(super::Flip::Vertical);
        assert_eq!(expect, dut);
    }
    #[test]
    fn border_rotate_ccw90() {
        let mut dut = TILE_PARSED.borders();
        let expect = Tile::try_from(ROTATED_CCW90.lines()).unwrap().borders();
        dut.rotate_ccw90();
        assert_eq!(expect, dut);
    }

    #[test]
    fn border_vertical_flip() {
        let mut dut = TILE_PARSED.borders();
        let expect = Tile::try_from(VERTICALLY_FLIPPED.lines())
            .unwrap()
            .borders();
        dut.vertical_flip();
        assert_eq!(expect, dut);
    }
}
